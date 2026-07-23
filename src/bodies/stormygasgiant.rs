use std::array;
use bevy::prelude::*;
use bevy::sprite_render::Material2dPlugin;
use rand::{Rng, RngExt};
use crate::bodies::building_blocks::clouds::{Clouds, CloudsUniform};
use crate::bodies::{generate_colorscheme_base, PixelPlanet, Random};

pub fn build(app: &mut App) {
    if !app.is_plugin_added::<Material2dPlugin<Clouds>>() {
        app.add_plugins(Material2dPlugin::<Clouds>::default());
    }

    app.add_observer(on_stormy_gas_giant_added);

    #[cfg(feature = "dynamic")]
    app.add_systems(Update, on_stormy_gas_giant_changed);
}

#[derive(Component, Debug, Clone)]
#[require(PixelPlanet)]
pub struct StormyGasGiantParams {
    pub pixels: f32,
    pub mesh_diameter: Option<f32>,
    pub time_speed: f32,
    pub light_origin: Vec2,
    pub base_layer: CloudParams,
    pub storm_layer: CloudParams
}
impl Default for StormyGasGiantParams {
    fn default() -> Self {
        StormyGasGiantParams {
            pixels: 100.0,
            mesh_diameter: None,
            time_speed: 1.0,
            light_origin: Vec2::new(0.25, 0.25),
            base_layer: CloudParams::base_default(),
            storm_layer: CloudParams::storm_default(),
        }
    }
}
impl Random for StormyGasGiantParams {
    fn random(rng: &mut impl Rng) -> Self {
        let hue_diff = rng.random_range(0.3..0.8);
        let seed_colors: [Color; 8] = generate_colorscheme_base(rng, hue_diff, 1.0);

        StormyGasGiantParams {
            base_layer: CloudParams {
                colors: array::from_fn(|i| {
                    seed_colors[i]
                        .mix(&Color::BLACK, i as f32 / 6.0)
                        .mix(&Color::BLACK, 0.7)
                }),
                seed: rng.random_range(0.0..100.0),
                ..CloudParams::base_default()
            },
            storm_layer: CloudParams {
                colors: array::from_fn(|i| {
                    seed_colors[i + 4]
                        .mix(&Color::BLACK, i as f32 / 4.0)
                        .mix(&Color::WHITE, 1.0 - (i as f32 / 4.0) * 0.5)
                }),
                seed: rng.random_range(0.0..100.0),
                ..CloudParams::storm_default()
            },
            ..default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct CloudParams {
    pub time_speed_multiplier: f32,
    pub rotation: f32,
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

impl CloudParams {
    pub fn base_default() -> CloudParams {
        CloudParams {
            time_speed_multiplier: 0.005,
            rotation: 0.0,
            cloud_cover: 0.0,
            cloud_curve: 1.3,
            stretch: 1.0,
            light_border_1: 0.692,
            light_border_2: 0.666,
            colors: [
                Srgba::hex("3b2027").unwrap().into(),
                Srgba::hex("3b2027").unwrap().into(),
                Srgba::hex("000000").unwrap().into(),
                Srgba::hex("21181b").unwrap().into(),
            ],
            size: 9.0,
            seed: 5.939,
            octaves: 5,
        }
    }
    pub fn storm_default() -> CloudParams {
        CloudParams {
            time_speed_multiplier: 0.005,
            rotation: 0.0,
            cloud_cover: 0.538,
            cloud_curve: 1.3,
            stretch: 1.0,
            light_border_1: 0.439,
            light_border_2: 0.746,
            colors: [
                Srgba::hex("f0b541").unwrap().into(),
                Srgba::hex("cf752b").unwrap().into(),
                Srgba::hex("ab5130").unwrap().into(),
                Srgba::hex("7d3833").unwrap().into(),
            ],
            size: 9.0,
            seed: 5.939,
            octaves: 5,
        }
    }
}

#[cfg(feature = "dynamic")]
#[derive(Component)]
struct StormyGasGiantHandles {
    mesh: Handle<Mesh>,
    base_layer: Handle<Clouds>,
    storm_layer: Handle<Clouds>
}

fn on_stormy_gas_giant_added(
    trigger: On<Add, StormyGasGiantParams>,
    query: Query<&StormyGasGiantParams>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Clouds>>,
    mut commands: Commands
) {
    info!("Stormy Gas Giant added!");

    let params = query.get(trigger.entity).unwrap();

    let mesh = Mesh2d(meshes.add(Circle::new(params.mesh_diameter.unwrap_or(params.pixels) / 2.0)));
    let base_layer = MeshMaterial2d(materials.add(make_base_layer(params)));
    let storm_layer = MeshMaterial2d(materials.add(make_storm_layer(params)));

    #[cfg(feature = "dynamic")]
    commands.entity(trigger.entity).insert(StormyGasGiantHandles {
        mesh: mesh.0.clone(),
        base_layer: base_layer.0.clone(),
        storm_layer: storm_layer.0.clone()
    });

    commands.entity(trigger.entity).insert((
        mesh.clone(),
        base_layer
    )).with_children(|parent| {
        parent.spawn((
            mesh,
            storm_layer,
            Transform::from_xyz(0.0, 0.0, 0.1)
        ));
    });
}

#[cfg(feature = "dynamic")]
fn on_stormy_gas_giant_changed(
    query: Query<(&StormyGasGiantParams, &StormyGasGiantHandles), Changed<StormyGasGiantParams>>,
    mut materials: ResMut<Assets<Clouds>>
) {
    for (stormy_gas_giant_params, handles) in query {
        if let Some(mut base_layer) = materials.get_mut(handles.base_layer.id()) {
            *base_layer = make_base_layer(stormy_gas_giant_params);
        }
        if let Some(mut storm_layer) = materials.get_mut(handles.storm_layer.id()) {
            *storm_layer = make_storm_layer(stormy_gas_giant_params);
        }
    }
}

fn make_base_layer(value: &StormyGasGiantParams) -> Clouds {
    Clouds {
        params: CloudsUniform {
            pixels: value.pixels,
            rotation: value.base_layer.rotation,
            cloud_cover: value.base_layer.cloud_cover,
            cloud_curve: value.base_layer.cloud_curve,
            light_origin: value.light_origin,
            time_speed: value.time_speed * value.base_layer.time_speed_multiplier * value.base_layer.size.round() * 2.0,
            stretch: value.base_layer.stretch,
            light_border_1: value.base_layer.light_border_1,
            light_border_2: value.base_layer.light_border_2,
            colors: value.base_layer.colors.map(|c| c.to_linear()),
            size: value.base_layer.size,
            seed: value.base_layer.seed,
            octaves: value.base_layer.octaves,
        }
    }
}
fn make_storm_layer(value: &StormyGasGiantParams) -> Clouds {
    Clouds {
        params: CloudsUniform {
            pixels: value.pixels,
            rotation: value.storm_layer.rotation,
            cloud_cover: value.storm_layer.cloud_cover,
            cloud_curve: value.storm_layer.cloud_curve,
            light_origin: value.light_origin,
            time_speed: value.time_speed * value.storm_layer.time_speed_multiplier * value.storm_layer.size.round() * 2.0,
            stretch: value.storm_layer.stretch,
            light_border_1: value.storm_layer.light_border_1,
            light_border_2: value.storm_layer.light_border_2,
            colors: value.storm_layer.colors.map(|c| c.to_linear()),
            size: value.storm_layer.size,
            seed: value.storm_layer.size,
            octaves: value.storm_layer.octaves,
        }
    }
}