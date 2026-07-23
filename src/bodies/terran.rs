use std::array;
use bevy::asset::{Asset, Assets};
use bevy::color::{LinearRgba, Srgba};
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::shader::ShaderRef;
use bevy::sprite_render::{Material2d, Material2dPlugin};
use rand::{Rng, RngExt};
use crate::bodies::building_blocks::clouds::{Clouds, CloudsUniform};
use crate::bodies::{generate_colorscheme_base, CommonParams, NewWithCommon, PixelPlanet, PixelPlanetParams, Random};

pub fn build(app: &mut App) {
    app
        .add_plugins(Material2dPlugin::<Land>::default())
        .add_observer(on_terran_added);

    if !app.is_plugin_added::<Material2dPlugin<Clouds>>() {
        app.add_plugins(Material2dPlugin::<Clouds>::default());
    }

    #[cfg(feature = "dynamic")]
    app.add_systems(Update, on_terran_changed);
}
#[derive(Component, Debug, Clone)]
#[require(PixelPlanet)]
pub struct TerranParams {
    pub common_params: CommonParams,
    pub land_params: LandParams,
    pub cloud_params: CloudParams
}
impl PixelPlanetParams for TerranParams {
    fn common_params(&self) -> &CommonParams { &self.common_params }
    fn common_params_mut(&mut self) -> &mut CommonParams { &mut self.common_params }
}
impl NewWithCommon for TerranParams {
    fn new(common_params: CommonParams) -> Self {
        TerranParams {
            common_params,
            land_params: Default::default(),
            cloud_params: Default::default()
        }
    }
}
impl Random for TerranParams {
    fn random(rng: &mut impl Rng, common_params: CommonParams) -> Self {
        let hue_diff = rng.random_range(0.7..1.0);
        let saturation = rng.random_range(0.45..0.55);
        let seed_colors: [_; 3] = generate_colorscheme_base(rng, hue_diff, saturation);

        let land_colors_1: [_; 4] = array::from_fn(|i| {
            let new_color = Hsva::from(seed_colors[0].mix(&Color::BLACK, i as f32 / 4.0));
            Color::hsv(new_color.hue + (0.2 * (i as f32 / 4.0)) * 360.0, new_color.saturation, new_color.value)
        });
        let land_colors_2: [_; 2] = array::from_fn(|i| {
            let new_color = Hsva::from(seed_colors[1].mix(&Color::BLACK, i as f32 / 2.0));
            Color::hsv(new_color.hue + (0.2 * (i as f32 / 2.0)) * 360.0, new_color.saturation, new_color.value)
        });

        TerranParams {
            land_params: LandParams {
                colors: [land_colors_1[0], land_colors_1[1], land_colors_1[2], land_colors_1[3], land_colors_2[0], land_colors_2[1]],
                seed: rng.random_range(0.0..100.0),
                ..default()
            },
            cloud_params: CloudParams {
                colors: array::from_fn(|i| {
                    let new_color = Hsva::from(seed_colors[2].mix(&Color::WHITE, (1.0 - (i as f32 / 4.0)) * 0.8));
                    Color::hsv(new_color.hue + (0.2 * (i as f32 / 4.0)) * 360.0, new_color.saturation, new_color.value)
                }),
                seed: rng.random_range(0.0..100.0),
                ..default()
            },
            ..Self::new(common_params)
        }
    }
}

// TODO: Move more things from the params into the Rivers which should be shared between the
// TODO: cloud and land shaders
#[derive(Debug, Clone)]
pub struct LandParams {
    pub time_speed_multiplier: f32,
    pub rotation_offset: f32,
    pub dither_size: Option<f32>,
    pub light_border_1: f32,
    pub light_border_2: f32,
    pub river_cutoff: f32,
    pub colors: [Color; 6],
    pub size: f32,
    pub seed: f32,
    pub octaves: u32,
}
impl Default for LandParams {
    fn default() -> Self {
        LandParams {
            time_speed_multiplier: 0.02,
            rotation_offset: 0.2,
            // time_speed: 0.1,
            dither_size: Some(3.951),
            light_border_1: 0.287,
            light_border_2: 0.476,
            river_cutoff: 0.368,
            colors: [
                Srgba::hex("63ab3f").unwrap().into(),
                Srgba::hex("3b7d4f").unwrap().into(),
                Srgba::hex("2f5753").unwrap().into(),
                Srgba::hex("283540").unwrap().into(),
                Srgba::hex("4fa4b8").unwrap().into(),
                Srgba::hex("404973").unwrap().into(),
            ],
            size: 4.6,
            seed: 8.98,
            octaves: 6,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CloudParams {
    pub time_speed_multiplier: f32,
    pub rotation_offset: f32,
    // pub time_speed: f32,
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
            // time_speed: 0.1,
            cloud_cover: 0.47,
            cloud_curve: 1.3,
            stretch: 2.0,
            light_border_1: 0.52,
            light_border_2: 0.62,
            colors: [
                Srgba::hex("f5ffe8").unwrap().into(),
                Srgba::hex("dfe0e8").unwrap().into(),
                Srgba::hex("686f99").unwrap().into(),
                Srgba::hex("404973").unwrap().into(),
            ],
            size: 7.315,
            seed: 5.939,
            octaves: 2,
        }
    }
}

#[cfg(feature = "dynamic")]
#[derive(Component)]
struct RiversHandles {
    mesh: Handle<Mesh>,
    land: Handle<Land>,
    cloud: Handle<Clouds>
}

// I hate observer patterns so much, but they are so useful
fn on_terran_added(
    trigger: On<Add, TerranParams>,
    query: Query<&TerranParams>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut land_materials: ResMut<Assets<Land>>,
    mut cloud_materials: ResMut<Assets<Clouds>>,
    mut commands: Commands
) {
    info!("Terran planet added!");

    let params = query.get(trigger.entity).unwrap();

    // TODO: Can we do this without manually maintaining meshes
    let mesh = Mesh2d(meshes.add(Circle::new(params.common_params.mesh_diameter.unwrap_or(params.common_params.pixels) / 2.0)));
    let land = MeshMaterial2d(land_materials.add(Land::from(params)));
    let cloud = MeshMaterial2d(cloud_materials.add(Clouds::from(params)));

    #[cfg(feature = "dynamic")]
    {
        commands.entity(trigger.entity).insert(RiversHandles {
            mesh: mesh.0.clone(),
            land: land.0.clone(),
            cloud: cloud.0.clone(),
        });
    }

    commands.entity(trigger.entity).insert((
        mesh.clone(),
        land,
    )).with_children(|parent| {
        parent.spawn((
            mesh,
            cloud,
            Transform::from_xyz(0.0, 0.0, 0.1)
        ));
    });
}
#[cfg(feature = "dynamic")]
fn on_terran_changed(
    query: Query<(&TerranParams, &RiversHandles), Changed<TerranParams>>,
    mut land_materials: ResMut<Assets<Land>>,
    mut cloud_materials: ResMut<Assets<Clouds>>
) {
    for (params, handles) in query {
        if let Some(mut land) = land_materials.get_mut(handles.land.id()) {
            *land = Land::from(params);
        }
        if let Some(mut cloud) = cloud_materials.get_mut(handles.cloud.id()) {
            *cloud = Clouds::from(params);
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct Land {
    #[uniform(0)]
    params: LandUniform
}
#[derive(ShaderType, Debug, Clone)]
struct LandUniform {
    pixels: f32,
    rotation: f32,
    light_origin: Vec2,
    time_speed: f32,
    dither_size: f32,
    should_dither: u32,
    light_border_1: f32,
    light_border_2: f32,
    river_cutoff: f32,
    colors: [LinearRgba; 6],
    size: f32,
    seed: f32,
    octaves: u32,
}
impl Material2d for Land {
    fn fragment_shader() -> ShaderRef {
        "shaders/rivers/rivers.wgsl".into()
    }
}
impl From<&TerranParams> for Land {
    fn from(value: &TerranParams) -> Self {
        Land {
            params: LandUniform {
                pixels: value.common_params.pixels,
                rotation: value.common_params.rotation + value.land_params.rotation_offset,
                light_origin: value.common_params.light_origin,
                time_speed: value.common_params.time_speed * value.land_params.time_speed_multiplier * value.land_params.size.round() * 2.0,
                dither_size: value.land_params.dither_size.unwrap_or(1.0),
                should_dither: if value.land_params.dither_size.is_some() { 1 } else { 0 },
                light_border_1: value.land_params.light_border_1,
                light_border_2: value.land_params.light_border_2,
                river_cutoff: value.land_params.river_cutoff,
                colors: value.land_params.colors.map(|c| c.to_linear()),
                size: value.land_params.size,
                seed: value.land_params.seed,
                octaves: value.land_params.octaves,
            }
        }
    }
}

impl From<&TerranParams> for Clouds {
    fn from(value: &TerranParams) -> Self {
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