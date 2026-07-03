use bevy::asset::Asset;
use bevy::color::LinearRgba;
use bevy::math::Vec2;
use bevy::prelude::TypePath;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::shader::ShaderRef;
use bevy::sprite_render::Material2d;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub(crate) struct Craters {
    #[uniform(0)]
    pub(crate) params: CratersUniform
}
#[derive(ShaderType, Debug, Clone)]
pub(crate) struct CratersUniform {
    pub(crate) pixels: f32,
    pub(crate) rotation: f32,
    pub(crate) light_origin: Vec2,
    pub(crate) time_speed: f32,
    pub(crate) light_border: f32,
    pub(crate) colors: [LinearRgba; 2],
    pub(crate) size: f32,
    pub(crate) seed: f32,
}
impl Material2d for Craters {
    fn fragment_shader() -> ShaderRef { "shaders/noatmosphere/craters.wgsl".into() }
}