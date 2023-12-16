use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::sprite::Material2d;

#[derive(Asset, AsBindGroup, Default, Reflect, Debug, Clone)]
#[reflect(Default, Debug)]
pub struct ChalkMaterial {
    #[uniform(0)]
    pub material_color: Color,
}

impl Material2d for ChalkMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/chalk.wgsl".into()
    }
}
