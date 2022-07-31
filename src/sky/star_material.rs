use bevy::pbr::{MaterialPipeline, MaterialPipelineKey};
use bevy::render::mesh::MeshVertexBufferLayout;
use bevy::render::render_resource::{AsBindGroup, ShaderRef, RenderPipelineDescriptor, SpecializedMeshPipelineError, CompareFunction};
use bevy::{
    prelude::*,
    reflect::TypeUuid,
};

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "d9645c26-9290-4651-bbb1-4a83fb071b38"]
pub struct SkyStarMaterial {
    #[uniform(0)]
    pub color: Vec4,
    #[uniform(1)]
    pub background: i32,
}

impl Material for SkyStarMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/sky.wgsl".into()
    }
    fn specialize(
            _pipeline: &MaterialPipeline<Self>,
            descriptor: &mut RenderPipelineDescriptor,
            _layout: &MeshVertexBufferLayout,
            _key: MaterialPipelineKey<Self>,
        ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.depth_stencil.as_mut().unwrap().depth_compare = CompareFunction::GreaterEqual;
        descriptor.depth_stencil.as_mut().unwrap().depth_write_enabled = false;
        Ok(())
    }
}
