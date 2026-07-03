use bevy::prelude::*;

pub mod terran;
pub mod lavaworld;
pub mod martian;
pub mod islands;
pub mod noatmosphere;
pub mod stormygasgiant;
pub mod bandedgasgiant;
pub mod iceworld;
mod building_blocks;
pub mod asteroid;
pub mod blackhole;
pub mod galaxy;
pub mod star;

#[derive(Component)]
// #[require(Mesh2d, MeshMaterial2d<_>, Transform)]
pub struct Body;