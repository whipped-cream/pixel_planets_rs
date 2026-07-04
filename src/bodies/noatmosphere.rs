use bevy::prelude::*;
use bevy::sprite_render::Material2dPlugin;
use crate::bodies::building_blocks::craters::{Craters, CratersUniform};
use crate::bodies::building_blocks::surface::{Surface, SurfaceUniform};
use crate::bodies::PixelPlanet;

pub fn build(app: &mut App) {
    app
        .add_plugins((
            Material2dPlugin::<Surface>::default(),
            Material2dPlugin::<Craters>::default(),
        ))
        .add_observer(on_no_atmosphere_added);

    #[cfg(feature = "dynamic")]
    app.add_systems(Update, on_no_atmosphere_changed);
}

#[derive(Component, Debug, Clone)]
#[require(PixelPlanet)]
pub struct NoAtmosphereParams {
    pub mesh_radius: f32,
    pub pixels: f32,
    pub time_speed: f32,
    pub light_origin: Vec2,
    pub surface_params: SurfaceParams,
    pub craters_params: CratersParams,
}
impl Default for NoAtmosphereParams {
    fn default() -> Self {
        NoAtmosphereParams {
            mesh_radius: 100.0,
            pixels: 100.0,
            time_speed: 1.0,
            light_origin: Vec2::new(0.25, 0.25),
            surface_params: Default::default(),
            craters_params: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
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
            light_border_1: 0.615,
            light_border_2: 0.729,
            colors: [
                Srgba::hex("a3a7c2").unwrap().into(),
                Srgba::hex("4c6885").unwrap().into(),
                Srgba::hex("3a3f5e").unwrap().into(),
            ],
            size: 8.0,
            seed: 1.012,
            octaves: 4,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CratersParams {
    // TODO: The pixels value for this one is different from the Surface.
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
            light_border: 0.465,
            colors: [
                Srgba::hex("4c6885").unwrap().into(),
                Srgba::hex("3a3f5e").unwrap().into(),
            ],
            size: 5.0,
            seed: 5.517
        }
    }
}

#[cfg(feature = "dynamic")]
#[derive(Component)]
struct NoAtmosphereHandles {
    mesh: Handle<Mesh>,
    surface: Handle<Surface>,
    craters: Handle<Craters>
}

fn on_no_atmosphere_added(
    trigger: On<Add, NoAtmosphereParams>,
    query: Query<&NoAtmosphereParams>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut surface_materials: ResMut<Assets<Surface>>,
    mut craters_materials: ResMut<Assets<Craters>>,
    mut commands: Commands
) {
    info!("No Atmosphere planet added!");

    let no_atmosphere_params = query.get(trigger.entity).unwrap();

    let mesh = Mesh2d(meshes.add(Circle::new(no_atmosphere_params.mesh_radius)));
    let surface = MeshMaterial2d(surface_materials.add(Surface::from(no_atmosphere_params)));
    let craters = MeshMaterial2d(craters_materials.add(Craters::from(no_atmosphere_params)));

    #[cfg(feature = "dynamic")]
    commands.entity(trigger.entity).insert(NoAtmosphereHandles {
        mesh: mesh.0.clone(),
        surface: surface.0.clone(),
        craters: craters.0.clone()
    });

    commands.entity(trigger.entity).insert((
        mesh.clone(),
        surface
    )).with_children(|parent| {
        parent.spawn((
            mesh,
            craters,
            Transform::from_xyz(0.0, 0.0, 0.1)
        ));
    });
}

#[cfg(feature = "dynamic")]
fn on_no_atmosphere_changed(
    query: Query<(&NoAtmosphereParams, &NoAtmosphereHandles), Changed<NoAtmosphereParams>>,
    mut surface_materials: ResMut<Assets<Surface>>,
    mut craters_materials: ResMut<Assets<Craters>>,
) {
    for (no_atmosphere_params, handles) in query {
        if let Some(mut surface) = surface_materials.get_mut(handles.surface.id()) {
            *surface = Surface::from(no_atmosphere_params);
        }
        if let Some(mut craters) = craters_materials.get_mut(handles.craters.id()) {
            *craters = Craters::from(no_atmosphere_params);
        }
    }
}


impl From<&NoAtmosphereParams> for Surface {
    fn from(value: &NoAtmosphereParams) -> Self {
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

impl From<&NoAtmosphereParams> for Craters {
    fn from(value: &NoAtmosphereParams) -> Self {
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