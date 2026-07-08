use bevy::prelude::*;

pub mod bodies;

pub use bodies::{
    terran, martian, islands, noatmosphere, stormygasgiant, bandedgasgiant,
    iceworld, lavaworld, asteroid, blackhole, galaxy, star,
    PixelPlanet, BodyType
};

pub struct PixelPlanetsPlugin;
impl Plugin for PixelPlanetsPlugin {
    fn build(&self, app: &mut App) {
        info!("Building PixelPlanetsPlugin");
        terran::build(app);
        martian::build(app);
        islands::build(app);
        noatmosphere::build(app);
        stormygasgiant::build(app);
        bandedgasgiant::build(app);
        iceworld::build(app);
        lavaworld::build(app);
        asteroid::build(app);
        blackhole::build(app);
        galaxy::build(app);
        star::build(app);
    }
}

#[cfg(test)]
mod tests {
    use bevy::{
        prelude::*, reflect::TypePath, render::render_resource::AsBindGroup, shader::ShaderRef,
    };
    use bevy::sprite_render::{Material2d, Material2dPlugin};
    use super::*;

    // #[test]
    // fn it_works() {
    //     let result = add(2, 2);
    //     assert_eq!(result, 4);
    // }




}
