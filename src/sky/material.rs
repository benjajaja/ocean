use bevy::core_pipeline::node::MAIN_PASS_DEPENDENCIES;
use bevy::render::renderer::RenderContext;
use bevy::render::RenderApp;
use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    pbr::{MaterialPipeline, SpecializedMaterial},
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_asset::{PrepareAssetError, RenderAsset},
        render_graph::{self, RenderGraph},
        render_resource::{
            std140::{AsStd140, Std140},
            CompareFunction, *,
        },
        renderer::RenderDevice,
    },
};

#[derive(Debug, Clone, TypeUuid)]
#[uuid = "d9645c26-9290-4651-bbb1-4a83fb071b38"]
pub struct SkyMaterial {
    pub color: Color,
}

#[derive(Clone)]
pub struct GpuCustomMaterial {
    _buffer: Buffer,
    bind_group: BindGroup,
}

impl RenderAsset for SkyMaterial {
    type ExtractedAsset = SkyMaterial;
    type PreparedAsset = GpuCustomMaterial;
    type Param = (SRes<RenderDevice>, SRes<MaterialPipeline<Self>>);
    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        (render_device, material_pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let color = Vec4::from_slice(&extracted_asset.color.as_linear_rgba_f32());
        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            contents: color.as_std140().as_bytes(),
            label: None,
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: None,
            layout: &material_pipeline.material_layout,
        });

        Ok(GpuCustomMaterial {
            _buffer: buffer,
            bind_group,
        })
    }
}

impl SpecializedMaterial for SkyMaterial {
    type Key = String;
    fn key(_: &<SkyMaterial as RenderAsset>::PreparedAsset) -> Self::Key {
        String::from("sky")
    }

    fn specialize(key: Self::Key, descriptor: &mut RenderPipelineDescriptor) {
        if let Some(depth_stencil_state) = &mut descriptor.depth_stencil {
            depth_stencil_state.depth_compare = CompareFunction::GreaterEqual;
            depth_stencil_state.depth_write_enabled = false;
        }
    }

    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/sky.wgsl"))
        // None
    }
    fn vertex_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        // Some(asset_server.load("shaders/sky.wgsl"))
        None
    }

    fn bind_group(render_asset: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
        &render_asset.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: BufferSize::new(Vec4::std140_size_static() as u64),
                },
                count: None,
            }],
            label: None,
        })
    }
}

pub struct GameOfLifeComputePlugin;

impl Plugin for GameOfLifeComputePlugin {
    fn build(&self, app: &mut App) {
        let render_app = app.sub_app_mut(RenderApp);
        // render_app
        // .init_resource::<GameOfLifePipeline>()
        // .add_system_to_stage(RenderStage::Extract, extract_game_of_life_image)
        // .add_system_to_stage(RenderStage::Queue, queue_bind_group);

        let mut render_graph = render_app.world.get_resource_mut::<RenderGraph>().unwrap();
        render_graph.add_node("game_of_life", DispatchGameOfLife::default());
        render_graph
            .add_node_edge("game_of_life", MAIN_PASS_DEPENDENCIES)
            .unwrap();
    }
}

enum Initialized {
    Default,
    No,
    Yes,
}
struct DispatchGameOfLife {
    initialized: Initialized,
}
impl Default for DispatchGameOfLife {
    fn default() -> Self {
        Self {
            initialized: Initialized::Default,
        }
    }
}
impl render_graph::Node for DispatchGameOfLife {
    fn update(&mut self, _world: &mut World) {
        match self.initialized {
            Initialized::Default => self.initialized = Initialized::No,
            Initialized::No => self.initialized = Initialized::Yes,
            Initialized::Yes => {}
        }
    }

    fn run(
        &self,
        _graph: &mut render_graph::RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), render_graph::NodeRunError> {
        Ok(())
    }
}
