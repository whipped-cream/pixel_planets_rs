use std::array;
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::shader::ShaderRef;
use bevy::sprite_render::{Material2d, Material2dPlugin};
use rand::{Rng, RngExt};
use crate::bodies::building_blocks::clouds::{Clouds, CloudsUniform};
use crate::bodies::building_blocks::planetunder::{PlanetUnder, PlanetUnderUniform};
use crate::bodies::{generate_colorscheme_base, CommonParams, NewWithCommon, PixelPlanet, PixelPlanetParams, Random};

pub fn build(app: &mut App) {
    if !app.is_plugin_added::<Material2dPlugin<PlanetUnder>>() {
        app.add_plugins(Material2dPlugin::<PlanetUnder>::default());
    }
    if !app.is_plugin_added::<Material2dPlugin<Clouds>>() {
        app.add_plugins(Material2dPlugin::<Clouds>::default());
    }

    app
        .add_plugins((
            Material2dPlugin::<Landmass>::default(),
        ))
        .add_observer(on_islands_added);

    #[cfg(feature = "dynamic")]
    app.add_systems(Update, on_islands_changed);
}


#[derive(Component, Debug, Clone)]
#[require(PixelPlanet)]
pub struct IslandsParams {
    pub common_params: CommonParams,
    pub ocean_params: OceanParams,
    pub landmass_params: LandmassParams,
    pub cloud_params: CloudParams
}
impl PixelPlanetParams for IslandsParams {
    fn common_params(&self) -> &CommonParams { &self.common_params }
    fn common_params_mut(&mut self) -> &mut CommonParams { &mut self.common_params }
}
impl NewWithCommon for IslandsParams {
    fn new(common_params: CommonParams) -> Self {
        IslandsParams {
            common_params,
            ocean_params: Default::default(),
            landmass_params: Default::default(),
            cloud_params: Default::default()
        }
    }
}
impl Random for IslandsParams {
    fn random(rng: &mut impl Rng, common_params: CommonParams) -> Self {
        let saturation = rng.random_range(0.45..0.55);
        let hue_diff = rng.random_range(0.7..1.0);
        let seed_colors: [_; 3] = generate_colorscheme_base(rng, hue_diff, saturation);
        IslandsParams {
            landmass_params: LandmassParams {
                colors: array::from_fn(|i| {
                    let new_color = Hsva::from(seed_colors[0].mix(&Color::BLACK, i as f32 / 4.0));
                    Color::hsv(new_color.hue + (0.2 * (i as f32 / 4.0)) * 360.0, new_color.saturation, new_color.value)
                }),
                seed: rng.random_range(0.0..100.0),
                ..default()
            },
            ocean_params: OceanParams {
                colors: array::from_fn(|i| {
                    let new_color = Hsva::from(seed_colors[1].mix(&Color::BLACK, i as f32 / 5.0));
                    Color::hsv(new_color.hue + (0.1 * (i as f32 / 2.0)) * 360.0, new_color.saturation, new_color.value) // The factors here cancel. Kept this way to match Godot
                }),
                seed: rng.random_range(0.0..100.0),
                ..default()
            },
            cloud_params: CloudParams {
                colors: array::from_fn(|i| {
                    let new_color = Hsva::from(seed_colors[2].mix(&Color::WHITE, (1.0 - i as f32 / 4.0) * 0.8));
                    Color::hsv(new_color.hue + (0.2 * (i as f32 / 4.0)) * 360.0, new_color.saturation, new_color.value)
                }),
                seed: rng.random_range(0.0..100.0),
                ..default()
            },
            ..Self::new(common_params)
        }
    }
}

#[derive(Debug, Clone)]
pub struct OceanParams {
    pub time_speed_multiplier: f32,
    pub rotation_offset: f32,
    pub dither_size: Option<f32>,
    pub light_border_1: f32,
    pub light_border_2: f32,
    pub colors: [Color; 3],
    pub size: f32,
    pub seed: f32,
    pub octaves: u32
}
impl Default for OceanParams {
    fn default() -> Self {
        OceanParams {
            time_speed_multiplier: 0.02,
            rotation_offset: 0.0,
            dither_size: Some(2.0),
            light_border_1: 0.4,
            light_border_2: 0.6,
            colors: [
                Srgba::hex("92e8c0").unwrap().into(),
                Srgba::hex("4fa4b8").unwrap().into(),
                Srgba::hex("2c354d").unwrap().into(),
            ],
            size: 5.228,
            seed: 10.0,
            octaves: 3,
        }
    }
}

#[derive(Debug, Clone)]
pub struct LandmassParams {
    pub time_speed_multiplier: f32,
    pub rotation_offset: f32,
    // pub dither_size: f32,
    pub light_border_1: f32,
    pub light_border_2: f32,
    pub land_cutoff: f32,
    pub colors: [Color; 4],
    pub size: f32,
    pub seed: f32,
    pub octaves: u32
}
impl Default for LandmassParams {
    fn default() -> Self {
        LandmassParams {
            time_speed_multiplier: 0.02,
            rotation_offset: 0.2,
            light_border_1: 0.32,
            light_border_2: 0.534,
            land_cutoff: 0.633,
            colors: [
                Srgba::hex("c8d45d").unwrap().into(),
                Srgba::hex("63ab3f").unwrap().into(),
                Srgba::hex("2f5753").unwrap().into(),
                Srgba::hex("283540").unwrap().into(),
            ],
            size: 4.292,
            seed: 7.947,
            octaves: 6,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CloudParams {
    pub time_speed_multiplier: f32,
    pub rotation_offset: f32,
    pub cloud_cover: f32,
    pub cloud_curve: f32,
    pub stretch: f32,
    pub light_border_1: f32,
    pub light_border_2: f32,
    pub colors: [Color; 4],
    pub size: f32,
    pub seed: f32,
    pub octaves: u32,
}
impl Default for CloudParams {
    fn default() -> Self {
        CloudParams {
            time_speed_multiplier: 0.01,
            rotation_offset: 0.0,
            cloud_cover: 0.415,
            cloud_curve: 1.3,
            stretch: 2.0,
            light_border_1: 0.52,
            light_border_2: 0.62,
            colors: [
                Srgba::hex("dfe0e8").unwrap().into(),
                Srgba::hex("a3a7c2").unwrap().into(),
                Srgba::hex("686f99").unwrap().into(),
                Srgba::hex("404973").unwrap().into(),
            ],
            size: 7.745,
            seed: 5.939,
            octaves: 2,
        }
    }
}

#[cfg(feature = "dynamic")]
#[derive(Component)]
struct IslandsHandles {
    mesh: Handle<Mesh>,
    landmass: Handle<Landmass>,
    ocean: Handle<PlanetUnder>,
    cloud: Handle<Clouds>
}

fn on_islands_added(
    trigger: On<Add, IslandsParams>,
    query: Query<&IslandsParams>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut ocean_materials: ResMut<Assets<PlanetUnder>>,
    mut landmass_materials: ResMut<Assets<Landmass>>,
    mut cloud_materials: ResMut<Assets<Clouds>>,
    mut commands: Commands
) {
    info!("Islands planet added!");

    let params = query.get(trigger.entity).unwrap();

    let mesh = Mesh2d(meshes.add(Circle::new(params.common_params.mesh_diameter.unwrap_or(params.common_params.pixels) / 2.0)));
    let ocean = MeshMaterial2d(ocean_materials.add(PlanetUnder::from(params)));
    let landmass = MeshMaterial2d(landmass_materials.add(Landmass::from(params)));
    let cloud = MeshMaterial2d(cloud_materials.add(Clouds::from(params)));

    #[cfg(feature = "dynamic")]
    commands.entity(trigger.entity).insert(IslandsHandles {
        mesh: mesh.0.clone(),
        ocean: ocean.0.clone(),
        landmass: landmass.0.clone(),
        cloud: cloud.0.clone()
    });

    commands.entity(trigger.entity).insert((
        mesh.clone(),
        ocean
    )).with_children(|parent| {
        parent.spawn((
            mesh.clone(),
            landmass,
            Transform::from_xyz(0.0, 0.0, 0.1)
        ));
    }).with_children(|parent| {
        parent.spawn((
            mesh,
            cloud,
            Transform::from_xyz(0.0, 0.0, 0.2)
        ));
    });
}

#[cfg(feature = "dynamic")]
fn on_islands_changed(
    query: Query<(&IslandsParams, &IslandsHandles), Changed<IslandsParams>>,
    mut ocean_materials: ResMut<Assets<PlanetUnder>>,
    mut landmass_materials: ResMut<Assets<Landmass>>,
    mut cloud_materials: ResMut<Assets<Clouds>>
) {
    for (islands_params, handles) in query {
        if let Some(mut ocean) = ocean_materials.get_mut(handles.ocean.id()) {
            *ocean = PlanetUnder::from(islands_params);
        }
        if let Some(mut landmass) = landmass_materials.get_mut(handles.landmass.id()) {
            *landmass = Landmass::from(islands_params);
        }
        if let Some(mut cloud) = cloud_materials.get_mut(handles.cloud.id()) {
            *cloud = Clouds::from(islands_params);
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct Landmass {
    #[uniform(0)]
    params: LandmassUniform
}
#[derive(ShaderType, Debug, Clone)]
struct LandmassUniform {
    pixels: f32,
    rotation: f32,
    light_origin: Vec2,
    time_speed: f32,
    light_border_1: f32,
    light_border_2: f32,
    land_cutoff: f32,
    colors: [LinearRgba; 4],
    size: f32,
    seed: f32,
    octaves: u32,
}
impl Material2d for Landmass {
    fn fragment_shader() -> ShaderRef { "shaders/landmasses/landmass.wgsl".into() }
}
impl From<&IslandsParams> for Landmass {
    fn from(value: &IslandsParams) -> Self {
        Landmass {
            params: LandmassUniform {
                pixels: value.common_params.pixels,
                rotation: value.common_params.rotation + value.landmass_params.rotation_offset,
                light_origin: value.common_params.light_origin,
                time_speed: value.common_params.time_speed * value.landmass_params.time_speed_multiplier * value.landmass_params.size.round() * 2.0,
                light_border_1: value.landmass_params.light_border_1,
                light_border_2: value.landmass_params.light_border_2,
                land_cutoff: value.landmass_params.land_cutoff,
                colors: value.landmass_params.colors.map(|c| c.to_linear()),
                size: value.landmass_params.size,
                seed: value.landmass_params.seed,
                octaves: value.landmass_params.octaves,
            }
        }
    }
}

impl From<&IslandsParams> for PlanetUnder {
    fn from(value: &IslandsParams) -> Self {
        PlanetUnder {
            params: PlanetUnderUniform {
                pixels: value.common_params.pixels,
                rotation: value.common_params.rotation + value.ocean_params.rotation_offset,
                light_origin: value.common_params.light_origin,
                time_speed: value.common_params.time_speed * value.ocean_params.time_speed_multiplier * value.ocean_params.size.round() * 2.0,
                dither_size: value.ocean_params.dither_size.unwrap_or(1.0),
                light_border_1: value.ocean_params.light_border_1,
                light_border_2: value.ocean_params.light_border_2,
                should_dither: if value.ocean_params.dither_size.is_some() { 1 } else { 0 },
                colors: value.ocean_params.colors.map(|c| c.to_linear()),
                size: value.ocean_params.size,
                seed: value.ocean_params.seed,
                octaves: value.ocean_params.octaves,
            }
        }
    }
}

impl From<&IslandsParams> for Clouds {
    fn from(value: &IslandsParams) -> Self {
        Clouds {
            params: CloudsUniform {
                pixels: value.common_params.pixels,
                rotation: value.common_params.rotation + value.cloud_params.rotation_offset,
                cloud_cover: value.cloud_params.cloud_cover,
                light_origin: value.common_params.light_origin,
                time_speed: value.common_params.time_speed * value.cloud_params.time_speed_multiplier * value.cloud_params.size.round() * 2.0,
                stretch: value.cloud_params.stretch,
                cloud_curve: value.cloud_params.cloud_curve,
                light_border_1: value.cloud_params.light_border_1,
                light_border_2: value.cloud_params.light_border_2,
                colors: value.cloud_params.colors.map(|c| c.to_linear()),
                size: value.cloud_params.size,
                seed: value.cloud_params.seed,
                octaves: value.cloud_params.octaves,
            }
        }
    }
}