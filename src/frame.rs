use bevy::prelude::*;
use bevy::render::mesh::MeshVertexBufferLayout;
use bevy::render::render_resource::{
    AsBindGroup, PolygonMode, RenderPipelineDescriptor, ShaderRef,
    SpecializedMeshPipelineError,
};
use bevy::sprite::{Material2d, Material2dKey};

#[derive(Asset, AsBindGroup, Default, Reflect, Debug, Clone)]
#[reflect(Default, Debug)]
pub struct FrameMaterial {
    #[uniform(0)]
    pub material_color: Color,
}

impl Material2d for FrameMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/frame.wgsl".into()
    }

    fn specialize(
        descriptor: &mut RenderPipelineDescriptor,
        _layout: &MeshVertexBufferLayout,
        _key: Material2dKey<Self>,
    ) -> Result<(), SpecializedMeshPipelineError> {
        descriptor.primitive.polygon_mode = PolygonMode::Line;
        Ok(())
    }
}
