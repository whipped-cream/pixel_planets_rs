use bevy::asset::{Asset, Assets};
use bevy::color::{LinearRgba, Srgba};
use bevy::math::Vec2;
use bevy::mesh::{Mesh, Mesh2d};
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::shader::ShaderRef;
use bevy::sprite_render::{Material2d, Material2dPlugin};
use rand::{Rng, RngExt};
use crate::bodies::{generate_random_colorscheme, CommonParams, NewWithCommon, PixelPlanet, PixelPlanetParams, Random};

pub fn build(app: &mut App) {
    app
        .add_plugins(Material2dPlugin::<Martian>::default())
        .add_observer(on_martian_added);

    #[cfg(feature = "dynamic")]
    app.add_systems(Update, on_martian_changed);
}


#[derive(Component, Debug, Clone)]
#[require(PixelPlanet)]
pub struct MartianParams {
    pub common_params: CommonParams,
    pub light_border_1: f32,
    pub light_border_2: f32,
    pub time_speed_multiplier: f32,
    pub dither_size: Option<f32>,
    pub colors: [Color; 5],
    pub num_colors: u32,
    pub size: f32,
    pub seed: f32,
    pub octaves: u32
}
impl PixelPlanetParams for MartianParams {
    fn common_params(&self) -> &CommonParams { &self.common_params }
    fn common_params_mut(&mut self) -> &mut CommonParams { &mut self.common_params }
}
impl NewWithCommon for MartianParams {
    fn new(common_params: CommonParams) -> Self {
        MartianParams {
            common_params,
            light_border_1: 0.362,
            light_border_2: 0.525,
            time_speed_multiplier: 0.02,
            dither_size: Some(2.0),
            colors: [
                Srgba::hex("ff8933").unwrap().into(),
                Srgba::hex("e64539").unwrap().into(),
                Srgba::hex("ad2f45").unwrap().into(),
                Srgba::hex("52333f").unwrap().into(),
                Srgba::hex("3d2936").unwrap().into(),
            ],
            num_colors: 5,
            size: 8.0,
            seed: 1.175,
            octaves: 3,
        }
    }
}
impl Random for MartianParams {
    fn random(rng: &mut impl Rng, common_params: CommonParams) -> Self {
        MartianParams {
            colors: generate_random_colorscheme(rng, 0.3..0.65, 1.0, 5.0, 1.0, 5.0, 0.2),
            seed: rng.random_range(0.0..100.0),
            ..Self::new(common_params)
        }
    }
}

// Observers
fn on_martian_added(
    trigger: On<Add, MartianParams>,
    query: Query<&MartianParams>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<Martian>>,
    mut commands: Commands
) {
    info!("Martian planet added!");


    let params = query.get(trigger.entity).unwrap();

    // TODO: Can we do this without manually maintaining meshes
    let mesh = Mesh2d(meshes.add(Circle::new(params.common_params.mesh_diameter.unwrap_or(params.common_params.pixels) / 2.0)));
    let martian = MeshMaterial2d(materials.add(Martian::from(params)));

    #[cfg(feature = "dynamic")]
    {
        commands.entity(trigger.entity).insert(MartianHandles {
            mesh: mesh.0.clone(),
            martian: martian.0.clone()
        });
    }

    commands.entity(trigger.entity).insert((
        mesh,
        martian,
    ));
}
#[cfg(feature = "dynamic")]
fn on_martian_changed(
    query: Query<(&MartianParams, &MartianHandles), Changed<MartianParams>>,
    mut materials: ResMut<Assets<Martian>>
) {
    for (params, handles) in query {
        if let Some(mut martian) = materials.get_mut(handles.martian.id()) {
            *martian = Martian::from(params);
        }
    }
}
#[cfg(feature = "dynamic")]
#[derive(Component)]
struct MartianHandles {
    mesh: Handle<Mesh>,
    martian: Handle<Martian>
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct Martian {
    #[uniform(0)]
    params: MartianUniform
}
#[derive(ShaderType, Debug, Clone)]
struct MartianUniform {
    pixels: f32,
    rotation: f32,
    light_origin: Vec2,
    light_border_1: f32,
    light_border_2: f32,
    time_speed: f32,
    dither_size: f32,
    should_dither: u32,
    colors: [LinearRgba; 5],
    num_colors: u32,
    size: f32,
    seed: f32,
    octaves: u32
}
impl Material2d for Martian {
    fn fragment_shader() -> ShaderRef { "shaders/dryterran/dryterran.wgsl".into() }
}
impl From<&MartianParams> for Martian {
    fn from(value: &MartianParams) -> Self {
        Martian {
            params: MartianUniform {
                pixels: value.common_params.pixels,
                rotation: value.common_params.rotation,
                light_origin: value.common_params.light_origin,
                light_border_1: value.light_border_1,
                light_border_2: value.light_border_2,
                time_speed: value.common_params.time_speed * value.time_speed_multiplier * value.size.round() * 2.0,
                dither_size: value.dither_size.unwrap_or(1.0),
                should_dither: if value.dither_size.is_some() { 1 } else { 0 },
                colors: value.colors.map(|c| c.to_linear()),
                num_colors: value.num_colors,
                size: value.size,
                seed: value.seed,
                octaves: value.octaves,
            }
        }
    }
}