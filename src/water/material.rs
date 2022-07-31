use bevy::pbr::{MaterialPipeline, MaterialPipelineKey};
use bevy::render::mesh::MeshVertexBufferLayout;
use bevy::render::render_resource::{AsBindGroup, ShaderRef, RenderPipelineDescriptor, SpecializedMeshPipelineError};
use bevy::{
    prelude::*,
    reflect::TypeUuid,
};

#[derive(AsBindGroup, TypeUuid, Clone)]
#[uuid = "463e4b8a-d555-4fc2-ba9f-4c880063ba92"]
pub struct WaterMaterial {
    #[uniform(0)]
    pub time: f32,
    #[uniform(1)]
    pub color: Vec4,
    #[uniform(2)]
    pub camera: Vec3,
    #[uniform(3)]
    pub wave1: Vec4,
    #[uniform(4)]
    pub wave2: Vec4,
    #[uniform(5)]
    pub wave3: Vec4,
}

impl Material for WaterMaterial {
    // fn fragment_shader() -> ShaderRef {
        // "shaders/water.frag".into()
    // }
    // fn vertex_shader() -> ShaderRef {
        // "shaders/water.vert".into()
    // }
    // Bevy assumes by default that vertex shaders use the "vertex" entry point
    // and fragment shaders use the "fragment" entry point (for WGSL shaders).
    // GLSL uses "main" as the entry point, so we must override the defaults here
    fn specialize(
        _pipeline: &MaterialPipeline<Self>,
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: MaterialPipelineKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        // descriptor.vertex.entry_point = "main".into();
        // descriptor.fragment.as_mut().unwrap().entry_point = "main".into();
        Ok(())
    }
}

