use std::num::NonZeroU32;

use bevy::{
    core_pipeline::Transparent3d,
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    pbr::{
        DrawMesh, MaterialPipeline, MeshPipeline, MeshPipelineKey, MeshUniform, SetMeshBindGroup,
        SetMeshViewBindGroup, SpecializedMaterial,
    },
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_asset::{PrepareAssetError, RenderAsset},
        render_phase::{
            AddRenderCommand, DrawFunctions, EntityRenderCommand, RenderCommandResult, RenderPhase,
            SetItemPipeline, TrackedRenderPass,
        },
        render_resource::std140::{AsStd140, Std140},
        render_resource::*,
        renderer::{RenderDevice, RenderQueue},
        view::{ExtractedView, Msaa},
        RenderApp, RenderStage,
    },
};

#[derive(Component)]
pub struct CustomMaterial;

pub struct CustomMaterialPlugin;

impl Plugin for CustomMaterialPlugin {
    fn build(&self, app: &mut App) {
        let render_device = app.world.get_resource::<RenderDevice>().unwrap();

        let buffer = render_device.create_buffer(&BufferDescriptor {
            label: Some("time uniform buffer"),
            // size: std::mem::size_of::<f32>() as u64,
            size: ExtractedTime::std140_size_static() as u64,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        app.sub_app_mut(RenderApp)
            .add_render_command::<Transparent3d, DrawCustom>()
            .insert_resource(TimeMeta {
                buffer,
                bind_group: None,
            })
            .init_resource::<CustomPipeline>()
            .init_resource::<SpecializedPipelines<CustomPipeline>>()
            .add_system_to_stage(RenderStage::Extract, extract_time)
            .add_system_to_stage(RenderStage::Extract, extract_custom_material)
            .add_system_to_stage(RenderStage::Prepare, prepare_time)
            .add_system_to_stage(RenderStage::Queue, queue_custom)
            .add_system_to_stage(RenderStage::Queue, queue_time_bind_group);
    }
}

// extract the `CustomMaterial` component into the render world
fn extract_custom_material(
    mut commands: Commands,
    mut previous_len: Local<usize>,
    mut query: Query<Entity, With<CustomMaterial>>,
) {
    let mut values = Vec::with_capacity(*previous_len);
    for entity in query.iter_mut() {
        values.push((entity, (CustomMaterial,)));
    }
    *previous_len = values.len();
    commands.insert_or_spawn_batch(values);
}

// add each entity with a mesh and a `CustomMaterial` to every view's `Transparent3d` render phase using the `CustomPipeline`
fn queue_custom(
    transparent_3d_draw_functions: Res<DrawFunctions<Transparent3d>>,
    custom_pipeline: Res<CustomPipeline>,
    msaa: Res<Msaa>,
    mut pipelines: ResMut<SpecializedPipelines<CustomPipeline>>,
    mut pipeline_cache: ResMut<RenderPipelineCache>,
    material_meshes: Query<(Entity, &MeshUniform), (With<Handle<Mesh>>, With<CustomMaterial>)>,
    mut views: Query<(&ExtractedView, &mut RenderPhase<Transparent3d>)>,
) {
    let draw_custom = transparent_3d_draw_functions
        .read()
        .get_id::<DrawCustom>()
        .unwrap();

    let key = MeshPipelineKey::from_msaa_samples(msaa.samples)
        | MeshPipelineKey::from_primitive_topology(PrimitiveTopology::TriangleList);
    let pipeline = pipelines.specialize(&mut pipeline_cache, &custom_pipeline, key);

    for (view, mut transparent_phase) in views.iter_mut() {
        let view_matrix = view.transform.compute_matrix();
        let view_row_2 = view_matrix.row(2);
        for (entity, mesh_uniform) in material_meshes.iter() {
            transparent_phase.add(Transparent3d {
                entity,
                pipeline,
                draw_function: draw_custom,
                distance: view_row_2.dot(mesh_uniform.transform.col(3)),
            });
        }
    }
}

#[derive(Default, AsStd140)]
struct ExtractedTime {
    color: Vec4,
    time: f32,
    camera: Vec3,
    wave1: Vec4,
    wave2: Vec4,
    wave3: Vec4,
}

// extract the passed time into a resource in the render world
fn extract_time(mut commands: Commands, time: Res<Time>, query: Query<&super::Water>) {
    if let Ok(water) = query.get_single() {
        commands.insert_resource(ExtractedTime {
            color: water.color.into(),
            time: time.seconds_since_startup() as f32,
            camera: Vec3::ZERO,
            wave1: water.waves[0].to_vec4(),
            wave2: water.waves[1].to_vec4(),
            wave3: water.waves[2].to_vec4(),
        });
    }
}

struct TimeMeta {
    buffer: Buffer,
    bind_group: Option<BindGroup>,
}

// write the extracted time into the corresponding uniform buffer
fn prepare_time(
    time: Res<ExtractedTime>,
    time_meta: ResMut<TimeMeta>,
    render_queue: Res<RenderQueue>,
    // render_device: Res<RenderDevice>,
) {
    let value_std140 = time.as_std140();

    // let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
    // contents: value_std140.as_bytes(),
    // label: Some("water_material_uniform_buffer"),
    // usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
    // });
    render_queue.write_buffer(
        &time_meta.buffer,
        0,
        value_std140.as_bytes(),
        // bevy::core::cast_slice(&[time.seconds_since_startup]),
    );
}

// create a bind group for the time uniform buffer
fn queue_time_bind_group(
    render_device: Res<RenderDevice>,
    mut time_meta: ResMut<TimeMeta>,
    pipeline: Res<CustomPipeline>,
) {
    let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
        label: None,
        layout: &pipeline.time_bind_group_layout,
        entries: &[BindGroupEntry {
            binding: 0,
            resource: time_meta.buffer.as_entire_binding(),
        }],
    });
    time_meta.bind_group = Some(bind_group);
}

pub struct CustomPipeline {
    shader_frag: Handle<Shader>,
    shader_vert: Handle<Shader>,
    mesh_pipeline: MeshPipeline,
    time_bind_group_layout: BindGroupLayout,
}

impl FromWorld for CustomPipeline {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();
        let asset_server = world.get_resource::<AssetServer>().unwrap();

        let render_device = world.get_resource_mut::<RenderDevice>().unwrap();
        let time_bind_group_layout =
            render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
                label: Some("time bind group"),
                entries: &[BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX_FRAGMENT,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        // min_binding_size: BufferSize::new(std::mem::size_of::<f32>() as u64),
                        min_binding_size: BufferSize::new(
                            ExtractedTime::std140_size_static() as u64
                        ),
                    },
                    count: NonZeroU32::new(6),
                }],
            });

        let mesh_pipeline = world.get_resource::<MeshPipeline>().unwrap();

        CustomPipeline {
            shader_vert: asset_server.load("shaders/water.vert"),
            shader_frag: asset_server.load("shaders/water.frag"),
            mesh_pipeline: mesh_pipeline.clone(),
            time_bind_group_layout,
        }
    }
}

impl SpecializedPipeline for CustomPipeline {
    type Key = MeshPipelineKey;

    fn specialize(&self, key: Self::Key) -> RenderPipelineDescriptor {
        let mut descriptor = self.mesh_pipeline.specialize(key);
        descriptor.vertex.shader = self.shader_vert.clone();
        descriptor.vertex.entry_point = "main".into();
        descriptor.fragment.as_mut().unwrap().shader = self.shader_frag.clone();
        descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
        descriptor.layout = Some(vec![
            self.mesh_pipeline.view_layout.clone(),
            self.mesh_pipeline.mesh_layout.clone(),
            self.time_bind_group_layout.clone(),
        ]);
        descriptor
    }
}

type DrawCustom = (
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMeshBindGroup<1>,
    SetTimeBindGroup<2>,
    DrawMesh,
);

struct SetTimeBindGroup<const I: usize>;
impl<const I: usize> EntityRenderCommand for SetTimeBindGroup<I> {
    type Param = SRes<TimeMeta>;

    fn render<'w>(
        _view: Entity,
        _item: Entity,
        time_meta: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let time_bind_group = time_meta.into_inner().bind_group.as_ref().unwrap();
        pass.set_bind_group(I, time_bind_group, &[]);

        RenderCommandResult::Success
    }
}

