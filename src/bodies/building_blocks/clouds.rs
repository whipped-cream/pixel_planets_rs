use bevy::asset::Asset;
use bevy::color::LinearRgba;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::shader::ShaderRef;
use bevy::sprite_render::Material2d;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub(crate) struct Clouds {
    #[uniform(0)]
    pub(crate) params: CloudsUniform
}
#[derive(ShaderType, Debug, Clone)]
pub(crate) struct CloudsUniform {
    pub pixels: f32,
    pub rotation: f32,
    pub cloud_cover: f32,
    pub light_origin: Vec2,
    pub time_speed: f32,
    pub stretch: f32,
    pub cloud_curve: f32,
    pub light_border_1: f32,
    pub light_border_2: f32,
    pub colors: [LinearRgba; 4],
    pub size: f32,
    pub seed: f32,
    pub octaves: u32
}
impl Material2d for Clouds {
    fn fragment_shader() -> ShaderRef {
        "shaders/landmasses/clouds.wgsl".into()
    }
}