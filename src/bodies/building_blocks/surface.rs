use bevy::asset::Asset;
use bevy::color::LinearRgba;
use bevy::math::Vec2;
use bevy::prelude::TypePath;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;
use bevy::sprite_render::Material2d;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub(crate) struct Surface {
    #[uniform(0)]
    pub(crate) pixels: f32,
    #[uniform(1)]
    pub(crate) rotation: f32,
    #[uniform(2)]
    pub(crate) light_origin: Vec2,
    #[uniform(3)]
    pub(crate) time_speed: f32,
    #[uniform(4)]
    pub(crate) dither_size: f32,
    #[uniform(5)]
    pub(crate) should_dither: u32,
    #[uniform(6)]
    pub(crate) light_border_1: f32,
    #[uniform(7)]
    pub(crate) light_border_2: f32,
    #[uniform(8)]
    pub(crate) colors: [LinearRgba; 3],
    #[uniform(9)]
    pub(crate) size: f32,
    #[uniform(10)]
    pub(crate) seed: f32,
    #[uniform(11)]
    pub(crate) octaves: u32
}
impl Material2d for Surface {
    fn fragment_shader() -> ShaderRef { "shaders/noatmosphere/surface.wgsl".into() }
}