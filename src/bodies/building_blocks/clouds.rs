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
    pub(crate) pixels: f32,
    pub(crate) rotation: f32,
    pub(crate) cloud_cover: f32,
    pub(crate) light_origin: Vec2,
    pub(crate) time_speed: f32,
    pub(crate) stretch: f32,
    pub(crate) cloud_curve: f32,
    pub(crate) light_border_1: f32,
    pub(crate) light_border_2: f32,
    pub(crate) colors: [LinearRgba; 4],
    pub(crate) size: f32,
    pub(crate) seed: f32,
    pub(crate) octaves: u32
}
impl Material2d for Clouds {
    fn fragment_shader() -> ShaderRef {
        "shaders/landmasses/clouds.wgsl".into()
    }
}