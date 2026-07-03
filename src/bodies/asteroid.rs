use bevy::asset::{Asset, Assets};
use bevy::color::{LinearRgba, Srgba};
use bevy::math::Vec2;
use bevy::mesh::{Mesh, Mesh2d};
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::shader::ShaderRef;
use bevy::sprite_render::{Material2d, Material2dPlugin};

pub fn build(app: &mut App) {
    app
        .add_plugins(Material2dPlugin::<Asteroid>::default())
        .add_observer(on_asteroid_added);

    #[cfg(feature = "dynamic")]
    app.add_systems(Update, on_asteroid_changed);
}


#[derive(Component)]
pub struct AsteroidParams {
    pub mesh_radius: f32,
    pub pixels: f32,
    pub should_dither: bool,
    pub rotation: f32,
    pub light_origin: Vec2,
    pub colors: [LinearRgba; 3],
    pub size: f32,
    pub seed: f32,
    pub octaves: u32
}
impl Default for AsteroidParams {
    fn default() -> Self {
        AsteroidParams {
            mesh_radius: 100.0,
            pixels: 100.0,
            should_dither: true,
            rotation: 0.0,
            light_origin: Vec2::new(0.0, 0.0),
            colors: [
                Srgba::hex("a3a7c2").unwrap().into(),
                Srgba::hex("4c6885").unwrap().into(),
                Srgba::hex("3a3f5e").unwrap().into(),
            ],
            size: 5.294,
            seed: 1.567,
            octaves: 2,
        }
    }
}

fn on_asteroid_added(
    trigger: On<Add, AsteroidParams>,
    query: Query<&AsteroidParams>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Asteroid>>,
    mut commands: Commands
) {
    info!("Asteroid added!");

    let params = query.get(trigger.entity).unwrap();

    // TODO: Can we do this without manually maintaining meshes
    let mesh = Mesh2d(meshes.add(Circle::new(params.mesh_radius)));
    let asteroid = MeshMaterial2d(materials.add(Asteroid::from(params)));

    #[cfg(feature = "dynamic")]
    commands.entity(trigger.entity).insert(AsteroidHandles {
        mesh: mesh.0.clone(),
        asteroid: asteroid.0.clone()
    });

    commands.entity(trigger.entity).insert((
        mesh,
        asteroid,
    ));
}
#[cfg(feature = "dynamic")]
fn on_asteroid_changed(
    query: Query<(&AsteroidParams, &AsteroidHandles), Changed<AsteroidParams>>,
    mut materials: ResMut<Assets<Asteroid>>
) {
    for (params, handles) in query {
        if let Some(mut asteroid) = materials.get_mut(handles.asteroid.id()) {
            *asteroid = Asteroid::from(params);
        }
    }
}
#[cfg(feature = "dynamic")]
#[derive(Component)]
struct AsteroidHandles {
    mesh: Handle<Mesh>,
    asteroid: Handle<Asteroid>
}

#[derive(ShaderType, Debug, Clone)]
struct AsteroidUniform {
    pixels: f32,
    rotation: f32,
    light_origin: Vec2,
    should_dither: u32,
    colors: [LinearRgba; 3],
    size: f32,
    seed: f32,
    octaves: u32
}
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct Asteroid {
    #[uniform(0)]
    params: AsteroidUniform
}
impl Material2d for Asteroid {
    fn fragment_shader() -> ShaderRef { "shaders/asteroid/asteroid.wgsl".into() }
}
impl From<&AsteroidParams> for Asteroid {
    fn from(value: &AsteroidParams) -> Self {
        Asteroid {
            params: AsteroidUniform {
                pixels: value.pixels,
                rotation: value.rotation,
                light_origin: value.light_origin,
                should_dither: if value.should_dither { 1 } else { 0 },
                colors: value.colors,
                size: value.size,
                seed: value.seed,
                octaves: value.octaves,
            },
        }
    }
}