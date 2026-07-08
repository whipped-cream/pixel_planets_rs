use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::shader::ShaderRef;
use bevy::sprite_render::{Material2d, Material2dPlugin};
use rand::{Rng, RngExt};
use crate::bodies::{generate_random_colorscheme, PixelPlanet, Random};

pub fn build(app: &mut App) {
    app
        .add_plugins((
            Material2dPlugin::<Body>::default(),
            Material2dPlugin::<Blobs>::default(),
            Material2dPlugin::<Flares>::default()
        ))
        .add_observer(on_star_added);

    #[cfg(feature = "dynamic")]
    app.add_systems(Update, on_star_changed);
}

#[derive(Component, Debug, Clone)]
#[require(PixelPlanet)]
pub struct StarParams {
    pub mesh_radius: f32,
    pub outer_mesh_radius: f32,
    pub time_speed: f32,
    pub body_params: BodyParams,
    pub blob_params: BlobParams,
    pub flare_params: FlareParams,
}
impl Default for StarParams {
    fn default() -> Self {
        StarParams {
            mesh_radius: 100.0,
            outer_mesh_radius: 200.0,
            time_speed: 1.0,
            body_params: Default::default(),
            blob_params: Default::default(),
            flare_params: Default::default()
        }
    }
}
impl Random for StarParams {
    fn random(rng: &mut impl Rng) -> Self {
        let mut colors = generate_random_colorscheme(rng, 0.2..0.4, 2.0, 4.0, 0.9, 4.0, 0.8);
        colors[0] = colors[0].lighter(0.8);
        StarParams {
            body_params: BodyParams {
                colors,
                seed: rng.random_range(0.0..100.0),
                ..default()
            },
            blob_params: BlobParams {
                colors: [colors[0]],
                seed: rng.random_range(0.0..100.0),
                ..default()
            },
            flare_params: FlareParams {
                colors: [colors[1], colors[0]],
                seed: rng.random_range(0.0..100.0),
                ..default()
            },
            ..default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct BodyParams {
    pub time_speed_multiplier: f32,
    pub pixels: f32,
    pub rotation: f32,
    pub colors: [Color; 4],
    pub num_colors: u32,
    pub should_dither: bool,
    pub size: f32,
    pub seed: f32,
    pub octaves: u32,
    pub tiles: u32
}
impl Default for BodyParams {
    fn default() -> Self {
        BodyParams {
            time_speed_multiplier: 0.005,
            pixels: 100.0,
            rotation: 0.0,
            colors: [
                Srgba::hex("f5ffe8").unwrap().into(),
                Srgba::hex("77d6c1").unwrap().into(),
                Srgba::hex("1c92a7").unwrap().into(),
                Srgba::hex("033e5e").unwrap().into(),
            ],
            num_colors: 4,
            should_dither: true,
            size: 4.463,
            seed: 4.837,
            octaves: 4,
            tiles: 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct BlobParams {
    pub time_speed_multiplier: f32,
    pub pixels: f32,
    pub rotation: f32,
    pub circle_amount: f32,
    pub circle_size: f32,
    pub colors: [Color; 1],
    pub size: f32,
    pub seed: f32,
    pub octaves: u32,
}
impl Default for BlobParams {
    fn default() -> Self {
        BlobParams {
            time_speed_multiplier: 0.01,
            pixels: 200.0,
            rotation: 0.0,
            circle_amount: 2.0,
            circle_size: 1.0,
            colors: [
                Srgba::hex("ffffe4").unwrap().into()
            ],
            size: 4.93,
            seed: 3.078,
            octaves: 4,
        }
    }
}

#[derive(Debug, Clone)]
pub struct FlareParams {
    pub time_speed_multiplier: f32,
    pub pixels: f32,
    pub rotation: f32,
    pub should_dither: bool,
    pub storm_width: f32,
    pub storm_dither_width: f32,
    pub circle_amount: f32,
    pub circle_scale: f32,
    pub scale: f32,
    pub colors: [Color; 2],
    pub size: f32,
    pub seed: f32,
    pub octaves: u32,
}
impl Default for FlareParams {
    fn default() -> Self {
        FlareParams {
            time_speed_multiplier: 0.015,
            pixels: 200.0,
            rotation: 0.0,
            should_dither: true,
            storm_width: 0.3,
            storm_dither_width: 0.0,
            circle_amount: 2.0,
            circle_scale: 1.0,
            scale: 1.0,
            colors: [
                Srgba::hex("77d6c1").unwrap().into(),
                Srgba::hex("ffffe4").unwrap().into(),
            ],
            size: 1.6,
            seed: 3.078,
            octaves: 4,
        }
    }
}


#[cfg(feature = "dynamic")]
#[derive(Component)]
struct StarHandles {
    mesh: Handle<Mesh>,
    outer_mesh: Handle<Mesh>,
    body: Handle<Body>,
    blobs: Handle<Blobs>,
    flares: Handle<Flares>
}

fn on_star_added(
    trigger: On<Add, StarParams>,
    query: Query<&StarParams>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut body_materials: ResMut<Assets<Body>>,
    mut blob_materials: ResMut<Assets<Blobs>>,
    mut flare_materials: ResMut<Assets<Flares>>,
    mut commands: Commands
) {
    info!("Star added!");

    let params = query.get(trigger.entity).unwrap();

    let mesh = Mesh2d(meshes.add(Circle::new(params.mesh_radius)));
    let outer_mesh = Mesh2d(meshes.add(Circle::new(params.outer_mesh_radius)));
    let body = MeshMaterial2d(body_materials.add(Body::from(params)));
    let blobs = MeshMaterial2d(blob_materials.add(Blobs::from(params)));
    let flares = MeshMaterial2d(flare_materials.add(Flares::from(params)));

    #[cfg(feature = "dynamic")]
    commands.entity(trigger.entity).insert(StarHandles {
        mesh: mesh.0.clone(),
        outer_mesh: outer_mesh.0.clone(),
        body: body.0.clone(),
        blobs: blobs.0.clone(),
        flares: flares.0.clone()
    });

    commands.entity(trigger.entity).insert((
        mesh,
        body
    )).with_children(|parent| {
        parent.spawn((
            outer_mesh.clone(),
            blobs,
            Transform::from_xyz(0.0, 0.0, -0.1)
        ));
        parent.spawn((
            outer_mesh,
            flares,
            Transform::from_xyz(0.0, 0.0, 0.1)
        ));
    });
}

#[cfg(feature = "dynamic")]
fn on_star_changed(
    query: Query<(&StarParams, &StarHandles), Changed<StarParams>>,
    mut body_materials: ResMut<Assets<Body>>,
    mut blob_materials: ResMut<Assets<Blobs>>,
    mut flare_materials: ResMut<Assets<Flares>>
) {
    for (params, handles) in query {
        if let Some(mut body) = body_materials.get_mut(handles.body.id()) {
            *body = Body::from(params);
        }
        if let Some(mut blobs) = blob_materials.get_mut(handles.blobs.id()) {
            *blobs = Blobs::from(params);
        }
        if let Some(mut flares) = flare_materials.get_mut(handles.flares.id()) {
            *flares = Flares::from(params);
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct Body {
    #[uniform(0)]
    params: BodyUniform
}
#[derive(ShaderType, Debug, Clone)]
struct BodyUniform {
    pixels: f32,
    time_speed: f32,
    rotation: f32,
    colors: [LinearRgba; 4],
    num_colors: u32,
    should_dither: u32,
    size: f32,
    seed: f32,
    octaves: u32,
    tiles: u32
}
impl Material2d for Body {
    fn fragment_shader() -> ShaderRef { "shaders/star/body.wgsl".into() }
}
impl From<&StarParams> for Body {
    fn from(value: &StarParams) -> Self {
        Body {
            params: BodyUniform {
                pixels: value.body_params.pixels,
                time_speed: value.time_speed * value.body_params.time_speed_multiplier * value.body_params.size.round() * 2.0,
                rotation: value.body_params.rotation,
                colors: value.body_params.colors.map(|c| c.to_linear()),
                num_colors: value.body_params.num_colors,
                should_dither: if value.body_params.should_dither { 1 } else { 0 },
                size: value.body_params.size,
                seed: value.body_params.seed,
                octaves: value.body_params.octaves,
                tiles: value.body_params.tiles,
            }
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct Blobs {
    #[uniform(0)]
    params: BlobUniform
}
#[derive(ShaderType, Clone, Debug)]
struct BlobUniform {
    pixels: f32,
    time_speed: f32,
    rotation: f32,
    circle_amount: f32,
    circle_size: f32,
    colors: [LinearRgba; 1],
    size: f32,
    seed: f32,
    octaves: u32,
}
impl Material2d for Blobs {
    fn fragment_shader() -> ShaderRef { "shaders/star/blobs.wgsl".into() }
}
impl From<&StarParams> for Blobs {
    fn from(value: &StarParams) -> Self {
        Blobs {
            params: BlobUniform {
                pixels: value.blob_params.pixels,
                time_speed: value.time_speed * value.blob_params.time_speed_multiplier * value.blob_params.size.round() * 2.0,
                rotation: value.blob_params.rotation,
                circle_amount: value.blob_params.circle_amount,
                circle_size: value.blob_params.circle_size,
                colors: value.blob_params.colors.map(|c| c.to_linear()),
                size: value.blob_params.size,
                octaves: value.blob_params.octaves,
                seed: value.blob_params.seed,
            }
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct Flares {
    #[uniform(0)]
    params: FlareUniform
}
#[derive(ShaderType, Clone, Debug)]
struct FlareUniform {
    pixels: f32,
    time_speed: f32,
    rotation: f32,
    should_dither: u32,
    storm_width: f32,
    storm_dither_width: f32,
    circle_amount: f32,
    circle_scale: f32,
    scale: f32,
    colors: [LinearRgba; 2],
    size: f32,
    seed: f32,
    octaves: u32,
}
impl Material2d for Flares {
    fn fragment_shader() -> ShaderRef { "shaders/star/flares.wgsl".into() }
}
impl From<&StarParams> for Flares {
    fn from(value: &StarParams) -> Self {
        Flares {
            params: FlareUniform {
                pixels: value.flare_params.pixels,
                time_speed: value.time_speed * value.flare_params.time_speed_multiplier * value.flare_params.size.round() * 2.0,
                rotation: value.flare_params.rotation,
                should_dither: if value.flare_params.should_dither { 1 } else { 0 },
                storm_width: value.flare_params.storm_width,
                storm_dither_width: value.flare_params.storm_dither_width,
                circle_amount: value.flare_params.circle_amount,
                circle_scale: value.flare_params.circle_scale,
                scale: value.flare_params.scale,
                colors: value.flare_params.colors.map(|c| c.to_linear()),
                size: value.flare_params.size,
                octaves: value.flare_params.octaves,
                seed: value.flare_params.seed,
            }
        }
    }
}