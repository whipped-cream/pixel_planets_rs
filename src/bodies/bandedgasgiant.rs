// TODO: The rings on this shader dont quite match the Godot and I'm not 100% sure why
// The rings on the Godot are rougher on the edges but this one makes very smooth edges.
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::shader::ShaderRef;
use bevy::sprite_render::{Material2d, Material2dPlugin};

pub fn build(app: &mut App) {
    app
        .add_plugins((
            Material2dPlugin::<Base>::default(),
            Material2dPlugin::<Ring>::default()
        ))
        .add_observer(on_banded_gas_giant_added);

    #[cfg(feature = "dynamic")]
    app.add_systems(Update, on_banded_gas_giant_changed);
}

#[derive(Component, Debug)]
pub struct BandedGasGiantParams {
    pub mesh_radius: f32,
    pub ring_mesh_radius: f32,
    pub time_speed: f32,
    pub light_origin: Vec2,
    pub base_layer_params: BaseParams,
    pub ring_params: RingParams
}
impl Default for BandedGasGiantParams {
    fn default() -> Self {
        BandedGasGiantParams {
            mesh_radius: 100.0,
            ring_mesh_radius: 300.0,
            time_speed: 1.0,
            light_origin: Vec2::new(-0.1, 0.3),
            base_layer_params: Default::default(),
            ring_params: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct BaseParams {
    pub pixels: f32,
    pub time_speed_multiplier: f32,
    pub rotation: f32,
    pub cloud_cover: f32,
    pub cloud_curve: f32,
    pub stretch: f32,
    pub light_border_1: f32,
    pub light_border_2: f32,
    pub bands: f32,
    pub should_dither: bool,
    pub colors: [Color; 3],
    pub dark_colors: [Color; 3],
    pub num_colors: u32,
    pub size: f32,
    pub seed: f32,
    pub octaves: u32
}
impl Default for BaseParams {
    fn default() -> Self {
        BaseParams {
            pixels: 100.0,
            time_speed_multiplier: 0.004,
            rotation: 0.0,
            cloud_cover: 0.61,
            cloud_curve: 1.376,
            stretch: 2.204,
            light_border_1: 0.52,
            light_border_2: 0.62,
            bands: 0.892,
            should_dither: true,
            colors: [
                Srgba::hex("eec39a").unwrap().into(),
                Srgba::hex("d9a066").unwrap().into(),
                Srgba::hex("8f563b").unwrap().into(),
            ],
            dark_colors: [
                Srgba::hex("663931").unwrap().into(),
                Srgba::hex("45283c").unwrap().into(),
                Srgba::hex("222034").unwrap().into(),
            ],
            num_colors: 3,
            size: 10.107,
            seed: 6.314,
            octaves: 3,
        }
    }
}

#[derive(Debug)]
pub struct RingParams {
    pub pixels: f32,
    pub time_speed_multiplier: f32,
    pub rotation: f32,
    pub light_border_1: f32,
    pub light_border_2: f32,
    pub ring_width: f32,
    pub ring_perspective: f32,
    pub scale: f32,
    pub colors: [Color; 3],
    pub dark_colors: [Color; 3],
    pub num_colors: u32,
    pub size: f32,
    pub seed: f32,
    pub octaves: u32
}
impl Default for RingParams {
    fn default() -> Self {
        RingParams {
            pixels: 300.0,
            time_speed_multiplier: 314.15 * 0.004 * 0.2,
            rotation: 0.7,
            light_border_1: 0.52,
            light_border_2: 0.62,
            ring_width: 0.127,
            ring_perspective: 6.0,
            scale: 6.0,
            colors: [
                Srgba::hex("eec39a").unwrap().into(),
                Srgba::hex("b37a50").unwrap().into(),
                Srgba::hex("8f563b").unwrap().into(),
            ],
            dark_colors: [
                Srgba::hex("553036").unwrap().into(),
                Srgba::hex("322337").unwrap().into(),
                Srgba::hex("222034").unwrap().into(),
            ],
            num_colors: 3,
            size: 15.0,
            seed: 8.461,
            octaves: 4,
        }
    }
}


#[cfg(feature = "dynamic")]
#[derive(Component)]
struct BandedGasGiantHandles {
    mesh: Handle<Mesh>,
    ring_mesh: Handle<Mesh>,
    base_layer: Handle<Base>,
    ring: Handle<Ring>
}

fn on_banded_gas_giant_added(
    trigger: On<Add, BandedGasGiantParams>,
    query: Query<&BandedGasGiantParams>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut base_layer_materials: ResMut<Assets<Base>>,
    mut ring_materials: ResMut<Assets<Ring>>,
    mut commands: Commands
) {
    info!("Banded Gas Giant added!");

    let banded_gas_giant_params = query.get(trigger.entity).unwrap();

    let base_mesh = Mesh2d(meshes.add(Circle::new(banded_gas_giant_params.mesh_radius)));
    let ring_mesh = Mesh2d(meshes.add(Circle::new(banded_gas_giant_params.ring_mesh_radius)));
    let base = MeshMaterial2d(base_layer_materials.add(Base::from(banded_gas_giant_params)));
    let ring = MeshMaterial2d(ring_materials.add(Ring::from(banded_gas_giant_params)));

    #[cfg(feature = "dynamic")]
    commands.entity(trigger.entity).insert(BandedGasGiantHandles {
        mesh: base_mesh.0.clone(),
        ring_mesh: ring_mesh.0.clone(),
        base_layer: base.0.clone(),
        ring: ring.0.clone(),
    });

    commands.entity(trigger.entity).insert((
        base_mesh,
        base
    )).with_children(|parent| {
        parent.spawn((
            ring_mesh,
            ring,
            Transform::from_xyz(0.0, 0.0, 0.1)
        ));
    });
}

#[cfg(feature = "dynamic")]
fn on_banded_gas_giant_changed(
    query: Query<(&BandedGasGiantParams, &BandedGasGiantHandles), Changed<BandedGasGiantParams>>,
    mut base_materials: ResMut<Assets<Base>>,
    mut ring_materials: ResMut<Assets<Ring>>
) {
    for (params, handles) in query {
        if let Some(mut base) = base_materials.get_mut(handles.base_layer.id()) {
            *base = Base::from(params);
        }
        if let Some(mut ring) = ring_materials.get_mut(handles.ring.id()) {
            *ring = Ring::from(params);
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct Base {
    #[uniform(0)]
    params: BaseUniform
}
#[derive(ShaderType, Debug, Clone)]
struct BaseUniform {
    pixels: f32,
    rotation: f32,
    light_origin: Vec2,
    cloud_cover: f32,
    cloud_curve: f32,
    time_speed: f32,
    stretch: f32,
    light_border_1: f32,
    light_border_2: f32,
    bands: f32,
    should_dither: u32,
    colors: [LinearRgba; 3],
    dark_colors: [LinearRgba; 3],
    num_colors: u32,
    size: f32,
    seed: f32,
    octaves: u32
}
impl Material2d for Base {
    fn fragment_shader() -> ShaderRef { "shaders/gasplanet/base.wgsl".into() }
}
impl From<&BandedGasGiantParams> for Base {
    fn from(value: &BandedGasGiantParams) -> Self {
        Base {
            params: BaseUniform {
                pixels: value.base_layer_params.pixels,
                rotation: value.base_layer_params.rotation,
                light_origin: value.light_origin,
                cloud_cover: value.base_layer_params.cloud_cover,
                cloud_curve: value.base_layer_params.cloud_curve,
                time_speed: value.time_speed * value.base_layer_params.time_speed_multiplier * value.base_layer_params.size.round() * 2.0,
                stretch: value.base_layer_params.stretch,
                light_border_1: value.base_layer_params.light_border_1,
                light_border_2: value.base_layer_params.light_border_2,
                bands: value.base_layer_params.bands,
                should_dither: if value.base_layer_params.should_dither { 1 } else { 0 },
                colors: value.base_layer_params.colors.map(|c| c.to_linear()),
                dark_colors: value.base_layer_params.dark_colors.map(|c| c.to_linear()),
                num_colors: value.base_layer_params.num_colors,
                size: value.base_layer_params.size,
                seed: value.base_layer_params.seed,
                octaves: value.base_layer_params.octaves,
            }
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct Ring {
    #[uniform(0)]
    params: RingUniform
}
#[derive(ShaderType, Clone, Debug)]
struct RingUniform {
    pixels: f32,
    rotation: f32,
    light_origin: Vec2,
    time_speed: f32,
    light_border_1: f32,
    light_border_2: f32,
    ring_width: f32,
    ring_perspective: f32,
    scale: f32,
    colors: [LinearRgba; 3],
    dark_colors: [LinearRgba; 3],
    num_colors: u32,
    size: f32,
    seed: f32,
    octaves: u32
}
impl Material2d for Ring {
    fn fragment_shader() -> ShaderRef { "shaders/gasplanet/ring.wgsl".into() }
}
impl From<&BandedGasGiantParams> for Ring {
    fn from(value: &BandedGasGiantParams) -> Self {
        Ring {
            params: RingUniform {
                pixels: value.ring_params.pixels,
                rotation: value.ring_params.rotation,
                light_origin: value.light_origin,
                time_speed: value.time_speed * value.ring_params.time_speed_multiplier, // This is deliberately different from the others to match the Godot
                light_border_1: value.ring_params.light_border_1,
                light_border_2: value.ring_params.light_border_2,
                ring_width: value.ring_params.ring_width,
                ring_perspective: value.ring_params.ring_perspective,
                scale: value.ring_params.scale,
                colors: value.ring_params.colors.map(|c| c.to_linear()),
                dark_colors: value.ring_params.dark_colors.map(|c| c.to_linear()),
                num_colors: value.ring_params.num_colors,
                size: value.ring_params.size,
                seed: value.ring_params.seed,
                octaves: value.ring_params.octaves,
            }
        }
    }
}