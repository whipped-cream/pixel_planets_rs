use bevy::asset::Asset;
use bevy::color::LinearRgba;
use bevy::math::Vec2;
use bevy::prelude::TypePath;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;
use bevy::sprite_render::Material2d;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub(crate) struct Craters {
    #[uniform(0)]
    pub(crate) pixels: f32,
    #[uniform(1)]
    pub(crate) rotation: f32,
    #[uniform(2)]
    pub(crate) light_origin: Vec2,
    #[uniform(3)]
    pub(crate) time_speed: f32,
    #[uniform(4)]
    pub(crate) light_border: f32,
    #[uniform(5)]
    pub(crate) colors: [LinearRgba; 2],
    #[uniform(6)]
    pub(crate) size: f32,
    #[uniform(7)]
    pub(crate) seed: f32,
}
impl Material2d for Craters {
    fn fragment_shader() -> ShaderRef { "shaders/noatmosphere/craters.wgsl".into() }
}