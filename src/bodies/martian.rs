use bevy::asset::{Asset, Assets};
use bevy::color::{LinearRgba, Srgba};
use bevy::math::Vec2;
use bevy::mesh::{Mesh, Mesh2d};
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::shader::ShaderRef;
use bevy::sprite_render::{Material2d, Material2dPlugin};
use crate::bodies::PixelPlanet;

pub fn build(app: &mut App) {
    app
        .add_plugins(Material2dPlugin::<Martian>::default())
        .add_observer(on_martian_added);

    #[cfg(feature = "dynamic")]
    app.add_systems(Update, on_martian_changed);
}


#[derive(Component, Debug, Clone)]
#[require(PixelPlanet)]
pub struct MartianParams {
    pub mesh_radius: f32,
    pub pixels: f32,
    pub rotation: f32,
    pub light_origin: Vec2,
    pub light_border_1: f32,
    pub light_border_2: f32,
    pub time_speed: f32,
    pub time_speed_multiplier: f32,
    pub dither_size: Option<f32>,
    pub colors: [Color; 5],
    pub num_colors: u32,
    pub size: f32,
    pub seed: f32,
    pub octaves: u32
}
impl Default for MartianParams {
    fn default() -> Self {
        MartianParams {
            mesh_radius: 100.0,
            pixels: 100.0,
            rotation: 0.0,
            light_origin: Vec2::new(0.4, 0.3),
            light_border_1: 0.362,
            light_border_2: 0.525,
            time_speed: 1.0,
            time_speed_multiplier: 0.02,
            dither_size: Some(2.0),
            colors: [
                Srgba::hex("ff8933").unwrap().into(),
                Srgba::hex("e64539").unwrap().into(),
                Srgba::hex("ad2f45").unwrap().into(),
                Srgba::hex("52333f").unwrap().into(),
                Srgba::hex("3d2936").unwrap().into(),
            ],
            num_colors: 5,
            size: 8.0,
            seed: 1.175,
            octaves: 3,
        }
    }
}

// Observers
fn on_martian_added(
    trigger: On<Add, MartianParams>,
    query: Query<&MartianParams>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Martian>>,
    mut commands: Commands
) {
    info!("Martian planet added!");


    let params = query.get(trigger.entity).unwrap();

    // TODO: Can we do this without manually maintaining meshes
    let mesh = Mesh2d(meshes.add(Circle::new(params.mesh_radius)));
    let martian = MeshMaterial2d(materials.add(Martian::from(params)));

    #[cfg(feature = "dynamic")]
    {
        commands.entity(trigger.entity).insert(MartianHandles {
            mesh: mesh.0.clone(),
            martian: martian.0.clone()
        });
    }

    commands.entity(trigger.entity).insert((
        mesh,
        martian,
    ));
}
#[cfg(feature = "dynamic")]
fn on_martian_changed(
    query: Query<(&MartianParams, &MartianHandles), Changed<MartianParams>>,
    mut materials: ResMut<Assets<Martian>>
) {
    for (params, handles) in query {
        if let Some(mut martian) = materials.get_mut(handles.martian.id()) {
            *martian = Martian::from(params);
        }
    }
}
#[cfg(feature = "dynamic")]
#[derive(Component)]
struct MartianHandles {
    mesh: Handle<Mesh>,
    martian: Handle<Martian>
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct Martian {
    #[uniform(0)]
    params: MartianUniform
}
#[derive(ShaderType, Debug, Clone)]
struct MartianUniform {
    pixels: f32,
    rotation: f32,
    light_origin: Vec2,
    light_border_1: f32,
    light_border_2: f32,
    time_speed: f32,
    dither_size: f32,
    should_dither: u32,
    colors: [LinearRgba; 5],
    num_colors: u32,
    size: f32,
    seed: f32,
    octaves: u32
}
impl Material2d for Martian {
    fn fragment_shader() -> ShaderRef { "shaders/dryterran/dryterran.wgsl".into() }
}
impl From<&MartianParams> for Martian {
    fn from(value: &MartianParams) -> Self {
        Martian {
            params: MartianUniform {
                pixels: value.pixels,
                rotation: value.rotation,
                light_origin: value.light_origin,
                light_border_1: value.light_border_1,
                light_border_2: value.light_border_2,
                time_speed: value.time_speed * value.time_speed_multiplier * value.size.round() * 2.0,
                dither_size: value.dither_size.unwrap_or(1.0),
                should_dither: if value.dither_size.is_some() { 1 } else { 0 },
                colors: value.colors.map(|c| c.to_linear()),
                num_colors: value.num_colors,
                size: value.size,
                seed: value.seed,
                octaves: value.octaves,
            }
        }
    }
}
// fn make_color_array(value: &Vec<LinearRgba>) -> [LinearRgba; 5] {
//     if value.len() > 5 {
//         warn!("Number of colors for Martian must be less than 5!")
//     }
//     let mut array = [Srgba::new(0.0, 0.0, 0.0, 0.0).into(); 5];
//     for (index, color) in value.iter().enumerate().take(5) {
//         array[index] = *color;
//     }
//     array
// }