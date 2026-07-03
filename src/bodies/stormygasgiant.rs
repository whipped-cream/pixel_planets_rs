use bevy::prelude::*;
use bevy::sprite_render::Material2dPlugin;
use crate::bodies::building_blocks::clouds::Clouds;
use crate::bodies::terran::CloudParams;

pub fn build(app: &mut App) {
    if !app.is_plugin_added::<Material2dPlugin<Clouds>>() {
        app.add_plugins(Material2dPlugin::<Clouds>::default());
    }

    app.add_observer(on_stormy_gas_giant_added);

    #[cfg(feature = "dynamic")]
    app.add_systems(Update, on_stormy_gas_giant_changed);
}

#[derive(Component, Debug)]
pub struct StormyGasGiantParams {
    pub mesh_radius: f32,
    pub pixels: f32,
    pub time_speed: f32,
    pub light_origin: Vec2,
    pub base_layer: CloudParams,
    pub storm_layer: CloudParams
}
impl Default for StormyGasGiantParams {
    fn default() -> Self {
        StormyGasGiantParams {
            mesh_radius: 100.0,
            pixels: 100.0,
            time_speed: 0.47,
            light_origin: Vec2::new(0.25, 0.25),
            base_layer: base_default(),
            storm_layer: storm_default(),
        }
    }
}

pub fn base_default() -> CloudParams {
    CloudParams {
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

    let stormy_gas_giant_params = query.get(trigger.entity).unwrap();

    let mesh = Mesh2d(meshes.add(Circle::new(stormy_gas_giant_params.mesh_radius)));
    let base_layer = MeshMaterial2d(materials.add(make_base_layer(stormy_gas_giant_params)));
    let storm_layer = MeshMaterial2d(materials.add(make_storm_layer(stormy_gas_giant_params)));

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

fn make_base_layer(stormy_gas_giant_params: &StormyGasGiantParams) -> Clouds {
    Clouds {
        pixels: stormy_gas_giant_params.pixels,
        rotation: stormy_gas_giant_params.base_layer.rotation,
        cloud_cover: stormy_gas_giant_params.base_layer.cloud_cover,
        cloud_curve: stormy_gas_giant_params.base_layer.cloud_curve,
        light_origin: stormy_gas_giant_params.light_origin,
        time_speed: stormy_gas_giant_params.time_speed,
        stretch: stormy_gas_giant_params.base_layer.stretch,
        light_border_1: stormy_gas_giant_params.base_layer.light_border_1,
        light_border_2: stormy_gas_giant_params.base_layer.light_border_2,
        colors: stormy_gas_giant_params.base_layer.colors,
        size: stormy_gas_giant_params.base_layer.size,
        seed: stormy_gas_giant_params.base_layer.seed,
        octaves: stormy_gas_giant_params.base_layer.octaves,
    }
}
fn make_storm_layer(stormy_gas_giant_params: &StormyGasGiantParams) -> Clouds {
    Clouds {
        pixels: stormy_gas_giant_params.pixels,
        rotation: stormy_gas_giant_params.storm_layer.rotation,
        cloud_cover: stormy_gas_giant_params.storm_layer.cloud_cover,
        cloud_curve: stormy_gas_giant_params.storm_layer.cloud_curve,
        light_origin: stormy_gas_giant_params.light_origin,
        time_speed: stormy_gas_giant_params.time_speed,
        stretch: stormy_gas_giant_params.storm_layer.stretch,
        light_border_1: stormy_gas_giant_params.storm_layer.light_border_1,
        light_border_2: stormy_gas_giant_params.storm_layer.light_border_2,
        colors: stormy_gas_giant_params.storm_layer.colors,
        size: stormy_gas_giant_params.storm_layer.size,
        seed: stormy_gas_giant_params.storm_layer.size,
        octaves: stormy_gas_giant_params.storm_layer.octaves,
    }
}