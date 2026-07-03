use bevy::asset::Asset;
use bevy::color::LinearRgba;
use bevy::math::Vec2;
use bevy::prelude::TypePath;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::shader::ShaderRef;
use bevy::sprite_render::Material2d;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub(crate) struct Surface {
    #[uniform(0)]
    pub(crate) params: SurfaceUniform
}
#[derive(ShaderType, Debug, Clone)]
pub(crate) struct SurfaceUniform {
    pub(crate) pixels: f32,
    pub(crate) rotation: f32,
    pub(crate) light_origin: Vec2,
    pub(crate) time_speed: f32,
    pub(crate) dither_size: f32,
    pub(crate) should_dither: u32,
    pub(crate) light_border_1: f32,
    pub(crate) light_border_2: f32,
    pub(crate) colors: [LinearRgba; 3],
    pub(crate) size: f32,
    pub(crate) seed: f32,
    pub(crate) octaves: u32
}
impl Material2d for Surface {
    fn fragment_shader() -> ShaderRef { "shaders/noatmosphere/surface.wgsl".into() }
}