use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    pbr::{MaterialPipeline, SpecializedMaterial},
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_asset::{PrepareAssetError, RenderAsset},
        render_resource::std140::{AsStd140, Std140},
        render_resource::*,
        renderer::RenderDevice,
    },
};

#[derive(Default, Debug, Clone, TypeUuid, AsStd140)]
#[uuid = "463e4b8a-d555-4fc2-ba9f-4c880063ba92"]
pub struct WaterMaterial {
    pub time: f32,
    pub color: Vec4,
    pub camera: Vec3,
    pub wave1: Vec4,
    pub wave2: Vec4,
    pub wave3: Vec4,
}

#[derive(Clone)]
pub struct GpuCustomMaterial {
    _buffer: Buffer,
    bind_group: BindGroup,
}

impl RenderAsset for WaterMaterial {
    type ExtractedAsset = WaterMaterial;
    type PreparedAsset = GpuCustomMaterial;
    type Param = (SRes<RenderDevice>, SRes<MaterialPipeline<Self>>);
    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        (render_device, material_pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        // println!("prepare asset: {:?}", extracted_asset.as_std140());

        let value_std140 = extracted_asset.as_std140();

        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            contents: value_std140.as_bytes(),
            label: Some("water_material_uniform_buffer"),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("water_material_bind_group"),
            layout: &material_pipeline.material_layout,
        });

        Ok(GpuCustomMaterial {
            _buffer: buffer,
            bind_group,
        })
    }
}

impl SpecializedMaterial for WaterMaterial {
    type Key = ();

    fn key(_: &<WaterMaterial as RenderAsset>::PreparedAsset) -> Self::Key {}

    fn specialize(_: Self::Key, descriptor: &mut RenderPipelineDescriptor) {
        descriptor.vertex.entry_point = "main".into();
        descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
    }

    fn vertex_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/water.vert"))
    }

    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/water.frag"))
    }

    fn bind_group(render_asset: &<Self as RenderAsset>::PreparedAsset) -> &BindGroup {
        &render_asset.bind_group
    }

    fn bind_group_layout(render_device: &RenderDevice) -> BindGroupLayout {
        println!("bind_group_layout");
        render_device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            entries: &[BindGroupLayoutEntry {
                binding: 0,
                visibility: ShaderStages::VERTEX_FRAGMENT,
                ty: BindingType::Buffer {
                    ty: BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    // min_binding_size: BufferSize::new(Vec4::std140_size_static() as u64),
                    min_binding_size: BufferSize::new(WaterMaterial::std140_size_static() as u64),
                },
                count: None,
            }],
            label: None,
        })
    }
}
