use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    pbr::{MaterialPipeline, SpecializedMaterial},
    prelude::*,
    reflect::TypeUuid,
    render::{
        render_asset::{PrepareAssetError, RenderAsset},
        render_resource::{
            std140::{AsStd140, Std140},
            CompareFunction, *,
        },
        renderer::RenderDevice,
    },
};

#[derive(Default, Debug, Clone, TypeUuid, AsStd140)]
#[uuid = "d9645c26-9290-4651-bbb1-4a83fb071b38"]
pub struct SkyStarMaterial {
    pub color: Vec4,
    pub background: i32,
}

#[derive(Clone)]
pub struct GpuCustomMaterial {
    _buffer: Buffer,
    bind_group: BindGroup,
}

impl RenderAsset for SkyStarMaterial {
    type ExtractedAsset = SkyStarMaterial;
    type PreparedAsset = GpuCustomMaterial;
    type Param = (SRes<RenderDevice>, SRes<MaterialPipeline<Self>>);
    fn extract_asset(&self) -> Self::ExtractedAsset {
        self.clone()
    }

    fn prepare_asset(
        extracted_asset: Self::ExtractedAsset,
        (render_device, material_pipeline): &mut SystemParamItem<Self::Param>,
    ) -> Result<Self::PreparedAsset, PrepareAssetError<Self::ExtractedAsset>> {
        let value_std140 = extracted_asset.as_std140();

        let buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            contents: value_std140.as_bytes(),
            label: Some("sky_material_uniform_buffer"),
            usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
        });
        let bind_group = render_device.create_bind_group(&BindGroupDescriptor {
            entries: &[BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
            label: Some("sky_material_bind_group"),
            layout: &material_pipeline.material_layout,
        });

        Ok(GpuCustomMaterial {
            _buffer: buffer,
            bind_group,
        })
    }
}

impl SpecializedMaterial for SkyStarMaterial {
    type Key = String;
    fn key(_: &<SkyStarMaterial as RenderAsset>::PreparedAsset) -> Self::Key {
        String::from("sky_stars")
    }

    fn specialize(_key: Self::Key, descriptor: &mut RenderPipelineDescriptor) {
        if let Some(depth_stencil_state) = &mut descriptor.depth_stencil {
            depth_stencil_state.depth_compare = CompareFunction::GreaterEqual;
            depth_stencil_state.depth_write_enabled = false;
        }
    }

    fn fragment_shader(asset_server: &AssetServer) -> Option<Handle<Shader>> {
        Some(asset_server.load("shaders/sky.wgsl"))
    }
    fn vertex_shader(_asset_server: &AssetServer) -> Option<Handle<Shader>> {
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
                    min_binding_size: BufferSize::new(SkyStarMaterial::std140_size_static() as u64),
                },
                count: None,
            }],
            label: None,
        })
    }
}
