use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::shader::ShaderRef;
use bevy::sprite_render::{Material2d, Material2dPlugin};
use crate::bodies::building_blocks::clouds::{Clouds, CloudsUniform};
use crate::bodies::building_blocks::planetunder::{PlanetUnder, PlanetUnderUniform};

pub fn build(app: &mut App) {
    if !app.is_plugin_added::<Material2dPlugin<PlanetUnder>>() {
       app.add_plugins(Material2dPlugin::<PlanetUnder>::default());
    }
    app.add_plugins(Material2dPlugin::<Lakes>::default());
    if !app.is_plugin_added::<Material2dPlugin<Clouds>>() {
        app.add_plugins(Material2dPlugin::<Clouds>::default());
    }

    app.add_observer(on_ice_world_added);

    #[cfg(feature = "dynamic")]
    app.add_systems(Update, on_ice_world_changed);
}

#[derive(Component, Debug)]
pub struct IceWorldParams {
    pub mesh_radius: f32,
    pub pixels: f32,
    pub time_speed: f32,
    pub light_origin: Vec2,
    pub land_params: LandParams,
    pub lake_params: LakeParams,
    pub cloud_params: CloudParams
}
impl Default for IceWorldParams {
    fn default() -> Self {
        IceWorldParams {
            mesh_radius: 100.0,
            pixels: 100.0,
            time_speed: 0.25,
            light_origin: Vec2::new(0.3, 0.3),
            land_params: Default::default(),
            lake_params: Default::default(),
            cloud_params: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct LandParams {
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
impl Default for LandParams {
    fn default() -> Self {
        LandParams {
            rotation: 0.0,
            dither_size: 2.0,
            should_dither: true,
            light_border_1: 0.48,
            light_border_2: 0.632,
            colors: [
                Srgba::hex("faffff").unwrap().into(),
                Srgba::hex("c7d4e1").unwrap().into(),
                Srgba::hex("928fb8").unwrap().into(),
            ],
            size: 8.0,
            seed: 1.036,
            octaves: 2,
        }
    }
}

#[derive(Debug)]
pub struct LakeParams {
    pub lake_cutoff: f32,
    pub rotation: f32,
    pub light_border_1: f32,
    pub light_border_2: f32,
    pub colors: [LinearRgba; 3],
    pub size: f32,
    pub seed: f32,
    pub octaves: u32
}
impl Default for LakeParams {
    fn default() -> Self {
        LakeParams {
            lake_cutoff: 0.55,
            rotation: 0.0,
            light_border_1: 0.024,
            light_border_2: 0.047,
            colors: [
                Srgba::hex("4fa4b8").unwrap().into(),
                Srgba::hex("4c6885").unwrap().into(),
                Srgba::hex("3a3f5e").unwrap().into(),
            ],
            size: 10.0,
            seed: 1.14,
            octaves: 3,
        }
    }
}

#[derive(Debug)]
pub struct CloudParams {
    pub cloud_cover: f32,
    pub cloud_curve: f32,
    pub stretch: f32,
    pub rotation: f32,
    pub light_border_1: f32,
    pub light_border_2: f32,
    pub colors: [LinearRgba; 4],
    pub size: f32,
    pub seed: f32,
    pub octaves: u32
}
impl Default for CloudParams {
    fn default() -> Self {
        CloudParams {
            cloud_cover: 0.546,
            cloud_curve: 1.3,
            stretch: 2.5,
            rotation: 0.0,
            light_border_1: 0.566,
            light_border_2: 0.781,
            colors: [
                Srgba::hex("e1f2ff").unwrap().into(),
                Srgba::hex("c0e3ff").unwrap().into(),
                Srgba::hex("5e70a5").unwrap().into(),
                Srgba::hex("404973").unwrap().into(),
            ],
            size: 4.0,
            seed: 1.14,
            octaves: 4,
        }
    }
}

#[cfg(feature = "dynamic")]
#[derive(Component)]
struct IceWorldHandles {
    mesh: Handle<Mesh>,
    land: Handle<PlanetUnder>,
    lakes: Handle<Lakes>,
    clouds: Handle<Clouds>
}

fn on_ice_world_added(
    trigger: On<Add, IceWorldParams>,
    query: Query<&IceWorldParams>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut land_materials: ResMut<Assets<PlanetUnder>>,
    mut lake_materials: ResMut<Assets<Lakes>>,
    mut cloud_materials: ResMut<Assets<Clouds>>,
    mut commands: Commands
) {
    info!("Ice World added!");

    let params = query.get(trigger.entity).unwrap();

    let mesh = Mesh2d(meshes.add(Circle::new(params.mesh_radius)));
    let land = MeshMaterial2d(land_materials.add(PlanetUnder::from(params)));
    let lakes = MeshMaterial2d(lake_materials.add(Lakes::from(params)));
    let clouds = MeshMaterial2d(cloud_materials.add(Clouds::from(params)));

    #[cfg(feature = "dynamic")]
    commands.entity(trigger.entity).insert(IceWorldHandles {
        mesh: mesh.0.clone(),
        land: land.0.clone(),
        lakes: lakes.0.clone(),
        clouds: clouds.0.clone(),
    });

    commands.entity(trigger.entity).insert((
        mesh.clone(),
        land
    )).with_children(|parent| {
        parent.spawn((
            mesh.clone(),
            lakes,
            Transform::from_xyz(0.0, 0.0, 0.1)
        ));
        parent.spawn((
            mesh.clone(),
            clouds,
            Transform::from_xyz(0.0, 0.0, 0.2)
        ));
    });
}

#[cfg(feature = "dynamic")]
fn on_ice_world_changed(
    query: Query<(&IceWorldParams, &IceWorldHandles), Changed<IceWorldParams>>,
    mut land_materials: ResMut<Assets<PlanetUnder>>,
    mut lake_materials: ResMut<Assets<Lakes>>,
    mut cloud_materials: ResMut<Assets<Clouds>>,
) {
    for (params, handles) in query {
        if let Some(mut land) = land_materials.get_mut(handles.land.id()) {
            *land = PlanetUnder::from(params);
        }
        if let Some(mut lakes) = lake_materials.get_mut(handles.lakes.id()) {
            *lakes = Lakes::from(params);
        }
        if let Some(mut clouds) = cloud_materials.get_mut(handles.clouds.id()) {
            *clouds = Clouds::from(params);
        }
    }
}

impl From<&IceWorldParams> for PlanetUnder {
    fn from(value: &IceWorldParams) -> Self {
        PlanetUnder {
            params: PlanetUnderUniform {
                pixels: value.pixels,
                rotation: value.land_params.rotation,
                light_origin: value.light_origin,
                time_speed: value.time_speed,
                dither_size: value.land_params.dither_size,
                light_border_1: value.land_params.light_border_1,
                light_border_2: value.land_params.light_border_2,
                should_dither: if value.land_params.should_dither { 1 } else { 0 },
                colors: value.land_params.colors,
                size: value.land_params.size,
                seed: value.land_params.seed,
                octaves: value.land_params.octaves,
            }
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct Lakes {
    #[uniform(0)]
    params: LakesUniform
}
#[derive(ShaderType, Debug, Clone)]
struct LakesUniform {
    lake_cutoff: f32,
    colors: [LinearRgba; 3],
    light_border_1: f32,
    light_border_2: f32,
    rotation: f32,
    pixels: f32,
    light_origin: Vec2,
    time_speed: f32,
    size: f32,
    seed: f32,
    octaves: u32,
}
impl Material2d for Lakes {
    fn fragment_shader() -> ShaderRef { "shaders/iceworld/lakes.wgsl".into() }
}
impl From<&IceWorldParams> for Lakes {
    fn from(value: &IceWorldParams) -> Self {
        Lakes {
            params: LakesUniform {
                lake_cutoff: value.lake_params.lake_cutoff,
                colors: value.lake_params.colors,
                light_border_1: value.lake_params.light_border_1,
                light_border_2: value.lake_params.light_border_2,
                rotation: value.lake_params.rotation,
                pixels: value.pixels,
                light_origin: value.light_origin,
                time_speed: value.time_speed,
                size: value.lake_params.size,
                seed: value.lake_params.seed,
                octaves: value.lake_params.octaves,
            },
        }
    }
}

impl From<&IceWorldParams> for Clouds {
    fn from(value: &IceWorldParams) -> Self {
        Clouds {
            params: CloudsUniform {
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
}