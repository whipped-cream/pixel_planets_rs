use bevy::prelude::*;

pub mod bodies;

pub struct PixelPlanetsPlugin;
impl Plugin for PixelPlanetsPlugin {
    fn build(&self, app: &mut App) {
        info!("Building PixelPlanetsPlugin");
        bodies::terran::build(app);
        bodies::martian::build(app);
        bodies::islands::build(app);
        bodies::noatmosphere::build(app);
        bodies::stormygasgiant::build(app);
        bodies::bandedgasgiant::build(app);
        bodies::iceworld::build(app);
        bodies::lavaworld::build(app);
        bodies::asteroid::build(app);
        bodies::blackhole::build(app);
        bodies::galaxy::build(app);
        bodies::star::build(app);
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
