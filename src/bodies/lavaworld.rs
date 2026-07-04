use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::shader::ShaderRef;
use bevy::sprite_render::{Material2d, Material2dPlugin};
use crate::bodies::building_blocks::craters::{Craters, CratersUniform};
use crate::bodies::building_blocks::surface::{Surface, SurfaceUniform};

pub fn build(app: &mut App) {
    if !app.is_plugin_added::<Material2dPlugin<Surface>>() {
        app.add_plugins(Material2dPlugin::<Surface>::default());
    }
    if !app.is_plugin_added::<Material2dPlugin<Craters>>() {
        app.add_plugins(Material2dPlugin::<Craters>::default());
    }
    if !app.is_plugin_added::<Material2dPlugin<Rivers>>() {
        app.add_plugins(Material2dPlugin::<Rivers>::default());
    }
    
    app.add_observer(on_lava_world_added);

    #[cfg(feature = "dynamic")]
    app.add_systems(Update, on_lava_world_changed);
}

#[derive(Component, Debug)]
pub struct LavaWorldParams {
    pub mesh_radius: f32,
    pub pixels: f32,
    pub time_speed: f32,
    pub light_origin: Vec2,
    pub surface_params: SurfaceParams,
    pub craters_params: CratersParams,
    pub lava_rivers_params: LavaRiversParams
}
impl Default for LavaWorldParams {
    fn default() -> Self {
        LavaWorldParams {
            mesh_radius: 100.0,
            pixels: 100.0,
            time_speed: 1.0,
            light_origin: Vec2::new(0.3, 0.3),
            surface_params: Default::default(),
            craters_params: Default::default(),
            lava_rivers_params: Default::default()
        }
    }
}

#[derive(Debug)]
pub struct SurfaceParams {
    pub time_speed_multiplier: f32,
    pub rotation: f32,
    pub dither_size: Option<f32>,
    pub light_border_1: f32,
    pub light_border_2: f32,
    pub colors: [Color; 3],
    pub size: f32,
    pub seed: f32,
    pub octaves: u32
}
impl Default for SurfaceParams {
    fn default() -> Self {
        SurfaceParams {
            time_speed_multiplier: 0.02,
            rotation: 0.0,
            dither_size: Some(2.0),
            light_border_1: 0.4,
            light_border_2: 0.6,
            colors: [
                Srgba::hex("8f4d57").unwrap().into(),
                Srgba::hex("52333f").unwrap().into(),
                Srgba::hex("3d2936").unwrap().into(),
            ],
            size: 10.0,
            seed: 1.551,
            octaves: 3,
        }
    }
}

#[derive(Debug)]
pub struct CratersParams {
    // TODO: The pixels value is different on this one
    pub time_speed_multiplier: f32,
    pub rotation: f32,
    pub light_border: f32,
    pub colors: [Color; 2],
    pub size: f32,
    pub seed: f32,
    // pub octaves: f32
}
impl Default for CratersParams {
    fn default() -> Self {
        CratersParams {
            time_speed_multiplier: 0.02,
            rotation: 0.0,
            light_border: 0.4,
            colors: [
                Srgba::hex("52333f").unwrap().into(),
                Srgba::hex("3d2936").unwrap().into(),
            ],
            size: 3.5,
            seed: 1.561
        }
    }
}

#[derive(Debug)]
pub struct LavaRiversParams {
    pub time_speed_multiplier: f32,
    pub rotation: f32,
    pub light_border_1: f32,
    pub light_border_2: f32,
    pub river_cutoff: f32,
    pub colors: [Color; 3],
    pub size: f32,
    pub seed: f32,
    pub octaves: u32,
}
impl Default for LavaRiversParams {
    fn default() -> Self {
        LavaRiversParams {
            time_speed_multiplier: 0.02,
            rotation: 0.0,
            light_border_1: 0.019,
            light_border_2: 0.036,
            river_cutoff: 0.579,
            colors: [
                Srgba::hex("ff8933").unwrap().into(),
                Srgba::hex("e64539").unwrap().into(),
                Srgba::hex("ad2f45").unwrap().into(),
            ],
            size: 10.0,
            seed: 2.527,
            octaves: 4,
        }
    }
}

#[cfg(feature = "dynamic")]
#[derive(Component)]
struct LavaWorldHandles {
    mesh: Handle<Mesh>,
    surface: Handle<Surface>,
    craters: Handle<Craters>,
    lava_rivers: Handle<Rivers>
}

fn on_lava_world_added(
    trigger: On<Add, LavaWorldParams>,
    query: Query<&LavaWorldParams>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut surface_materials: ResMut<Assets<Surface>>,
    mut craters_materials: ResMut<Assets<Craters>>,
    mut lava_rivers_materials: ResMut<Assets<Rivers>>,
    mut commands: Commands
) {
    info!("Lava Rivers planet added!");

    let params = query.get(trigger.entity).unwrap();

    let mesh = Mesh2d(meshes.add(Circle::new(params.mesh_radius)));
    let surface = MeshMaterial2d(surface_materials.add(Surface::from(params)));
    let craters = MeshMaterial2d(craters_materials.add(Craters::from(params)));
    let lava_rivers = MeshMaterial2d(lava_rivers_materials.add(Rivers::from(params)));

    #[cfg(feature = "dynamic")]
    commands.entity(trigger.entity).insert(LavaWorldHandles {
        mesh: mesh.0.clone(),
        surface: surface.0.clone(),
        craters: craters.0.clone(),
        lava_rivers: lava_rivers.0.clone()
    });

    commands.entity(trigger.entity).insert((
        mesh.clone(),
        surface
    )).with_children(|parent| {
        parent.spawn((
            mesh.clone(),
            craters,
            Transform::from_xyz(0.0, 0.0, 0.1)
        ));
        parent.spawn((
            mesh,
            lava_rivers,
            Transform::from_xyz(0.0, 0.0, 0.2)
        ));
    });
}

#[cfg(feature = "dynamic")]
fn on_lava_world_changed(
    query: Query<(&LavaWorldParams, &LavaWorldHandles), Changed<LavaWorldParams>>,
    mut surface_materials: ResMut<Assets<Surface>>,
    mut craters_materials: ResMut<Assets<Craters>>,
    mut lava_rivers_materials: ResMut<Assets<Rivers>>
) {
    for (params, handles) in query {
        if let Some(mut surface) = surface_materials.get_mut(handles.surface.id()) {
            *surface = Surface::from(params);
        }
        if let Some(mut craters) = craters_materials.get_mut(handles.craters.id()) {
            *craters = Craters::from(params);
        }
        if let Some(mut lava_rivers) = lava_rivers_materials.get_mut(handles.lava_rivers.id()) {
            *lava_rivers = Rivers::from(params);
        }
    }
}


impl From<&LavaWorldParams> for Surface {
    fn from(value: &LavaWorldParams) -> Self {
        Surface {
            params: SurfaceUniform {
                pixels: value.pixels,
                rotation: value.surface_params.rotation,
                light_origin: value.light_origin,
                time_speed: value.time_speed * value.surface_params.time_speed_multiplier * value.surface_params.size.round() * 2.0,
                dither_size: value.surface_params.dither_size.unwrap_or(1.0),
                should_dither: if value.surface_params.dither_size.is_some() { 1 } else { 0 },
                light_border_1: value.surface_params.light_border_1,
                light_border_2: value.surface_params.light_border_2,
                colors: value.surface_params.colors.map(|c| c.to_linear()),
                size: value.surface_params.size,
                seed: value.surface_params.seed,
                octaves: value.surface_params.octaves,
            }
        }
    }
}

impl From<&LavaWorldParams> for Craters {
    fn from(value: &LavaWorldParams) -> Self {
        Craters {
            params: CratersUniform {
                pixels: value.pixels * 87.419 / 100.0,
                rotation: value.craters_params.rotation,
                light_origin: value.light_origin,
                time_speed: value.time_speed * value.craters_params.time_speed_multiplier * value.craters_params.size.round() * 2.0,
                light_border: value.craters_params.light_border,
                colors: value.craters_params.colors.map(|c| c.to_linear()),
                size: value.craters_params.size,
                seed: value.craters_params.seed,
            }
        }
    }
}

#[derive(ShaderType, Debug, Clone)]
pub(crate) struct RiversUniform {
    pixels: f32,
    rotation: f32,
    light_origin: Vec2,
    time_speed: f32,
    light_border_1: f32,
    light_border_2: f32,
    river_cutoff: f32,
    colors: [LinearRgba; 3],
    size: f32,
    seed: f32,
    octaves: u32,
}
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub(crate) struct Rivers {
    #[uniform(0)]
    params: RiversUniform
}
impl Material2d for Rivers {
    fn fragment_shader() -> ShaderRef { "shaders/lavaworld/rivers.wgsl".into() }
}
impl From<&LavaWorldParams> for Rivers {
    fn from(value: &LavaWorldParams) -> Self {
        Rivers {
            params: RiversUniform {
                pixels: value.pixels,
                rotation: value.lava_rivers_params.rotation,
                light_origin: value.light_origin,
                time_speed: value.time_speed * value.lava_rivers_params.time_speed_multiplier * value.lava_rivers_params.size.round() * 2.0,
                light_border_1: value.lava_rivers_params.light_border_1,
                light_border_2: value.lava_rivers_params.light_border_2,
                river_cutoff: value.lava_rivers_params.river_cutoff,
                colors: value.lava_rivers_params.colors.map(|c| c.to_linear()),
                size: value.lava_rivers_params.size,
                seed: value.lava_rivers_params.seed,
                octaves: value.lava_rivers_params.octaves,
            },
        }
    }
}