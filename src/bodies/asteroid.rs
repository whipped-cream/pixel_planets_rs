use crate::bodies::{generate_random_colorscheme, PixelPlanet, Random};
use bevy::asset::{Asset, Assets};
use bevy::color::{LinearRgba, Srgba};
use bevy::math::Vec2;
use bevy::mesh::{Mesh, Mesh2d};
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::shader::ShaderRef;
use bevy::sprite_render::{Material2d, Material2dPlugin};
use rand::{Rng, RngExt};

pub fn build(app: &mut App) {
    app
        .add_plugins(Material2dPlugin::<Asteroid>::default())
        .add_observer(on_asteroid_added);

    #[cfg(feature = "dynamic")]
    app.add_systems(Update, on_asteroid_changed);
}


#[derive(Component, Debug, Clone)]
#[require(PixelPlanet)]
pub struct AsteroidParams {
    pub pixels: f32,
    pub mesh_diameter: Option<f32>,
    pub should_dither: bool,
    pub rotation: f32,
    pub light_origin: Vec2,
    pub colors: [Color; 3],
    pub size: f32,
    pub seed: f32,
    pub octaves: u32
}
impl Default for AsteroidParams {
    fn default() -> Self {
        AsteroidParams {
            pixels: 100.0,
            mesh_diameter: None,
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
impl Random for AsteroidParams {
    fn random(rng: &mut impl Rng) -> Self {
        AsteroidParams {
            colors: generate_random_colorscheme(rng, 0.3..0.6, 0.7, 3.0, 1.0, 3.0, 0.2),
            seed: rng.random_range(0.0..100.0),
            ..default()
        }
    }

    // fn random_default_colors(rng: &mut impl Rng) -> Self {
    //     AsteroidParams {
    //         colors: Default::default(),
    //         ..Self::random(rng)
    //     }
    // }
}

// fn generate_random_colorscheme(rng: &mut impl Rng) -> [Color; 3] {
//     let hue_diff = rng.random_range(0.3..0.6);
//     let seed_colors: [_; 3] = generate_colorscheme_base(rng, hue_diff, 0.7);
//
//     array::from_fn(|i| {
//         seed_colors[i].mix(&Color::BLACK, (i as f32 / 3.0).mix(&Color::WHITE, (1.0 - (i as f32 / 3.0) * 0.2)
//     })
// }

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
    let mesh = Mesh2d(meshes.add(Circle::new(params.mesh_diameter.unwrap_or(params.pixels) / 2.0)));
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
                colors: value.colors.map(|c| c.to_linear()),
                size: value.size,
                seed: value.seed,
                octaves: value.octaves,
            },
        }
    }
}