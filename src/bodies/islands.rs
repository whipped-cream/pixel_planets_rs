use bevy::prelude::*;
use bevy::render::render_resource::AsBindGroup;
use bevy::shader::ShaderRef;
use bevy::sprite_render::{Material2d, Material2dPlugin};
use crate::bodies::building_blocks::clouds::Clouds;
use crate::bodies::building_blocks::planetunder::PlanetUnder;

pub fn build(app: &mut App) {
    app
        .add_plugins((
            Material2dPlugin::<PlanetUnder>::default(),
            Material2dPlugin::<Landmass>::default(),
        ))
        .add_observer(on_islands_added);

    if !app.is_plugin_added::<Material2dPlugin<Clouds>>() {
        app.add_plugins(Material2dPlugin::<Clouds>::default());
    }

    #[cfg(feature = "dynamic")]
    app.add_systems(Update, on_islands_changed);
}


#[derive(Component, Debug)]
pub struct IslandsParams {
    pub mesh_radius: f32, // TODO: I think this is a radius and pixels is a diameter and these should stay in sync
    pub pixels: f32,
    pub time_speed: f32,
    pub light_origin: Vec2,
    pub ocean_params: OceanParams,
    pub landmass_params: LandmassParams,
    pub cloud_params: CloudParams
}
impl Default for IslandsParams {
    fn default() -> Self {
        IslandsParams {
            mesh_radius: 100.0,
            pixels: 100.0,
            time_speed: 0.2,
            light_origin: Vec2::new(0.39, 0.39),
            ocean_params: Default::default(),
            landmass_params: Default::default(),
            cloud_params: Default::default()
        }
    }
}

#[derive(Debug)]
pub struct OceanParams {
    pub rotation: f32,
    pub dither_size: f32,
    pub should_dither: bool,
    pub light_border_1: f32,
    pub light_border_2: f32,
    pub colors: [LinearRgba; 3],
    pub size: f32,
    pub seed: f32,
    pub octaves: u32
}
impl Default for OceanParams {
    fn default() -> Self {
        OceanParams {
            rotation: 100.0,
            dither_size: 2.0,
            should_dither: true,
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

#[derive(Debug)]
pub struct LandmassParams {
    pub rotation: f32,
    // pub dither_size: f32,
    pub light_border_1: f32,
    pub light_border_2: f32,
    pub land_cutoff: f32,
    pub colors: [LinearRgba; 4],
    pub size: f32,
    pub seed: f32,
    pub octaves: u32
}
impl Default for LandmassParams {
    fn default() -> Self {
        LandmassParams {
            rotation: 0.2,
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

#[derive(Debug)]
pub struct CloudParams {
    pub rotation: f32,
    pub cloud_cover: f32,
    pub cloud_curve: f32,
    pub stretch: f32,
    pub light_border_1: f32,
    pub light_border_2: f32,
    pub colors: [LinearRgba; 4],
    pub size: f32,
    pub seed: f32,
    pub octaves: u32,
}
impl Default for CloudParams {
    fn default() -> Self {
        CloudParams {
            rotation: 0.0,
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

    let islands_params = query.get(trigger.entity).unwrap();

    let mesh = Mesh2d(meshes.add(Circle::new(islands_params.mesh_radius)));
    let ocean = MeshMaterial2d(ocean_materials.add(PlanetUnder::from(islands_params)));
    let landmass = MeshMaterial2d(landmass_materials.add(Landmass::from(islands_params)));
    let cloud = MeshMaterial2d(cloud_materials.add(Clouds::from(islands_params)));

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
    pixels: f32,
    #[uniform(1)]
    rotation: f32,
    #[uniform(2)]
    light_origin: Vec2,
    #[uniform(3)]
    time_speed: f32,
    #[uniform(4)]
    light_border_1: f32,
    #[uniform(5)]
    light_border_2: f32,
    #[uniform(6)]
    land_cutoff: f32,
    #[uniform(7)]
    colors: [LinearRgba; 4],
    #[uniform(8)]
    size: f32,
    #[uniform(9)]
    seed: f32,
    #[uniform(10)]
    octaves: u32,
}
impl Material2d for Landmass {
    fn fragment_shader() -> ShaderRef { "shaders/landmasses/landmass.wgsl".into() }
}
impl From<&IslandsParams> for Landmass {
    fn from(value: &IslandsParams) -> Self {
        Landmass {
            pixels: value.pixels,
            rotation: value.landmass_params.rotation,
            light_origin: value.light_origin,
            time_speed: value.time_speed,
            light_border_1: value.landmass_params.light_border_1,
            light_border_2: value.landmass_params.light_border_2,
            land_cutoff: value.landmass_params.land_cutoff,
            colors: value.landmass_params.colors,
            size: value.landmass_params.size,
            seed: value.landmass_params.seed,
            octaves: value.landmass_params.octaves,
        }
    }
}

impl From<&IslandsParams> for PlanetUnder {
    fn from(value: &IslandsParams) -> Self {
        PlanetUnder {
            pixels: value.pixels,
            rotation: value.ocean_params.rotation,
            light_origin: value.light_origin,
            time_speed: value.time_speed,
            dither_size: value.ocean_params.dither_size,
            light_border_1: value.ocean_params.light_border_1,
            light_border_2: value.ocean_params.light_border_2,
            should_dither: if value.ocean_params.should_dither { 1 } else { 0 },
            colors: value.ocean_params.colors,
            size: value.ocean_params.size,
            seed: value.ocean_params.seed,
            octaves: value.ocean_params.octaves,
        }
    }
}

impl From<&IslandsParams> for Clouds {
    fn from(value: &IslandsParams) -> Self {
        Clouds {
            pixels: value.pixels,
            rotation: value.cloud_params.rotation,
            cloud_cover: value.cloud_params.cloud_cover,
            light_origin: value.light_origin,
            time_speed: value.time_speed,
            stretch: value.cloud_params.stretch,
            cloud_curve: value.cloud_params.cloud_curve,
            light_border_1: value.cloud_params.light_border_1,
            light_border_2: value.cloud_params.light_border_2,
            colors: value.cloud_params.colors,
            size: value.cloud_params.size,
            seed: value.cloud_params.seed,
            octaves: value.cloud_params.octaves,
        }
    }
}