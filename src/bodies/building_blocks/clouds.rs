use bevy::asset::Asset;
use bevy::color::LinearRgba;
use bevy::math::Vec2;
use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;
use bevy::sprite_render::Material2d;


#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub(crate) struct Clouds {
    #[uniform(0)]
    pub pixels: f32,
    #[uniform(1)]
    pub rotation: f32,
    #[uniform(2)]
    pub cloud_cover: f32,
    #[uniform(3)]
    pub light_origin: Vec2,
    #[uniform(4)]
    pub time_speed: f32,
    #[uniform(5)]
    pub stretch: f32,
    #[uniform(6)]
    pub cloud_curve: f32,
    #[uniform(7)]
    pub light_border_1: f32,
    #[uniform(8)]
    pub light_border_2: f32,
    #[uniform(9)]
    pub colors: [LinearRgba; 4],
    #[uniform(10)]
    pub size: f32,
    #[uniform(11)]
    pub seed: f32,
    #[uniform(12)]
    pub octaves: u32
}
impl Material2d for Clouds {
    fn fragment_shader() -> ShaderRef {
        "shaders/landmasses/clouds.wgsl".into()
    }
}