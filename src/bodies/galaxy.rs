// TODO: Calling a galaxy a body is not really correct

use bevy::asset::{Asset, Assets};
use bevy::color::{LinearRgba, Srgba};
use bevy::mesh::{Mesh, Mesh2d};
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::shader::ShaderRef;
use bevy::sprite_render::{Material2d, Material2dPlugin};

pub fn build(app: &mut App) {
    app
        .add_plugins(Material2dPlugin::<Galaxy>::default())
        .add_observer(on_galaxy_added);

    #[cfg(feature = "dynamic")]
    app.add_systems(Update, on_galaxy_changed);
}


#[derive(Component)]
pub struct GalaxyParams {
    pub mesh_radius: f32,
    pub pixels: f32,
    pub rotation: f32,
    pub time_speed: f32,
    pub time_speed_multiplier: f32,
    pub dither_size: Option<f32>,
    pub colors: [Color; 7],
    pub num_colors: u32,
    pub size: f32,
    pub seed: f32,
    pub octaves: u32,
    pub tilt: f32,
    pub num_layers: f32,
    pub layer_height: f32,
    pub zoom: f32,
    pub swirl: f32
}
impl Default for GalaxyParams {
    fn default() -> Self {
        GalaxyParams {
            mesh_radius: 200.0,
            pixels: 200.0,
            rotation: 0.674,
            time_speed: -1.0, // Changed from Godot because clockwise looks nicer than anticlockwise IMO
            time_speed_multiplier: 1.0,
            dither_size: Some(2.0),
            colors: [
                Srgba::hex("ffffeb").unwrap().into(),
                Srgba::hex("ffe98d").unwrap().into(),
                Srgba::hex("b5e066").unwrap().into(),
                Srgba::hex("65a566").unwrap().into(),
                Srgba::hex("395d64").unwrap().into(),
                Srgba::hex("32394d").unwrap().into(),
                Srgba::hex("322947").unwrap().into(),
            ],
            num_colors: 6, // TODO: The Godot uses 6 here when there are definitely 7 colors?
            size: 7.0,
            seed: 5.881,
            octaves: 1,
            tilt: 3.0,
            num_layers: 4.0,
            layer_height: 0.4,
            zoom: 1.375,
            swirl: -9.0,
        }
    }
}

fn on_galaxy_added(
    trigger: On<Add, GalaxyParams>,
    query: Query<&GalaxyParams>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Galaxy>>,
    mut commands: Commands
) {
    info!("Galaxy added!");

    let params = query.get(trigger.entity).unwrap();

    // TODO: Can we do this without manually maintaining meshes
    let mesh = Mesh2d(meshes.add(Circle::new(params.mesh_radius)));
    let galaxy = MeshMaterial2d(materials.add(Galaxy::from(params)));

    #[cfg(feature = "dynamic")]
    commands.entity(trigger.entity).insert(GalaxyHandles {
        mesh: mesh.0.clone(),
        galaxy: galaxy.0.clone()
    });

    commands.entity(trigger.entity).insert((
        mesh,
        galaxy,
    ));
}
#[cfg(feature = "dynamic")]
fn on_galaxy_changed(
    query: Query<(&GalaxyParams, &GalaxyHandles), Changed<GalaxyParams>>,
    mut materials: ResMut<Assets<Galaxy>>
) {
    for (params, handles) in query {
        if let Some(mut galaxy) = materials.get_mut(handles.galaxy.id()) {
            *galaxy = Galaxy::from(params);
        }
    }
}
#[cfg(feature = "dynamic")]
#[derive(Component)]
struct GalaxyHandles {
    mesh: Handle<Mesh>,
    galaxy: Handle<Galaxy>
}

#[derive(ShaderType, Debug, Clone)]
struct GalaxyUniform {
    pixels: f32,
    rotation: f32,
    time_speed: f32,
    dither_size: f32,
    should_dither: u32,
    colors: [LinearRgba; 7],
    num_colors: u32,
    size: f32,
    seed: f32,
    octaves: u32,
    tilt: f32,
    num_layers: f32,
    layer_height: f32,
    zoom: f32,
    swirl: f32,
}
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct Galaxy {
    #[uniform(0)]
    params: GalaxyUniform
}
impl Material2d for Galaxy {
    fn fragment_shader() -> ShaderRef { "shaders/galaxy/galaxy.wgsl".into() }
}
impl From<&GalaxyParams> for Galaxy {
    fn from(value: &GalaxyParams) -> Self {
        Galaxy {
            params: GalaxyUniform {
                pixels: value.pixels,
                rotation: value.rotation,
                time_speed: value.time_speed,
                dither_size: value.dither_size.unwrap_or(1.0),
                should_dither: if value.dither_size.is_some() { 1 } else { 0 },
                colors: value.colors.map(|c| c.to_linear()),
                num_colors: value.num_colors,
                size: value.size,
                seed: value.seed,
                octaves: value.octaves,
                tilt: value.tilt,
                num_layers: value.num_layers,
                layer_height: value.layer_height,
                zoom: value.zoom,
                swirl: value.swirl,
            },
        }
    }
}