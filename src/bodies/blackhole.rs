use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, ShaderType};
use bevy::shader::ShaderRef;
use bevy::sprite_render::{Material2d, Material2dPlugin};

pub fn build(app: &mut App) {
    app
        .add_plugins((
            Material2dPlugin::<Shadow>::default(),
            Material2dPlugin::<AccretionDisk>::default()
        ))
        .add_observer(on_black_hole_added);

    #[cfg(feature = "dynamic")]
    app.add_systems(Update, on_black_hole_changed);
}

#[derive(Component, Debug)]
pub struct BlackHoleParams {
    pub mesh_radius: f32,
    pub accretion_disk_mesh_radius: f32,
    pub time_speed: f32,
    pub light_origin: Vec2,
    pub shadow_params: ShadowParams,
    pub accretion_disk_params: AccretionDiskParams
}
impl Default for BlackHoleParams {
    fn default() -> Self {
        BlackHoleParams {
            mesh_radius: 100.0,
            accretion_disk_mesh_radius: 300.0,
            time_speed: 0.2,
            light_origin: Vec2::new(0.607, 0.444),
            shadow_params: Default::default(),
            accretion_disk_params: Default::default(),
        }
    }
}

#[derive(Debug)]
pub struct ShadowParams {
    pub pixels: f32,
    pub radius: f32,
    pub light_width: f32,
    pub colors: [LinearRgba; 3],
}
impl Default for ShadowParams {
    fn default() -> Self {
        ShadowParams {
            pixels: 100.0,
            radius: 0.247,
            light_width: 0.028,
            colors: [
                Srgba::hex("272736").unwrap().into(),
                Srgba::hex("ffffeb").unwrap().into(),
                Srgba::hex("ed7b39").unwrap().into()
            ],
        }
    }
}

#[derive(Debug)]
pub struct AccretionDiskParams {
    pub disk_width: f32,
    pub ring_perspective: f32,
    pub should_dither: bool,
    pub pixels: f32,
    pub rotation: f32,
    pub colors: [LinearRgba; 5],
    pub num_colors: u32,
    pub size: f32,
    pub seed: f32,
    pub octaves: u32
}
impl Default for AccretionDiskParams {
    fn default() -> Self {
        AccretionDiskParams {
            disk_width: 0.065,
            ring_perspective: 14.0,
            should_dither: true,
            pixels: 300.0,
            rotation: 0.766,
            colors: [
                Srgba::hex("ffffeb").unwrap().into(),
                Srgba::hex("fff540").unwrap().into(),
                Srgba::hex("ffb84a").unwrap().into(),
                Srgba::hex("ed7b39").unwrap().into(),
                Srgba::hex("bd4035").unwrap().into(),
            ],
            num_colors: 5,
            size: 6.598,
            seed: 8.175,
            octaves: 3,
        }
    }
}


#[cfg(feature = "dynamic")]
#[derive(Component)]
struct BlackHoleHandles {
    shadow_mesh: Handle<Mesh>,
    accretion_disk_mesh: Handle<Mesh>,
    shadow: Handle<Shadow>,
    accretion_disk: Handle<AccretionDisk>
}

fn on_black_hole_added(
    trigger: On<Add, BlackHoleParams>,
    query: Query<&BlackHoleParams>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut shadow_materials: ResMut<Assets<Shadow>>,
    mut accretion_disk_materials: ResMut<Assets<AccretionDisk>>,
    mut commands: Commands
) {
    info!("Black Hole added!");

    let params = query.get(trigger.entity).unwrap();

    let shadow_mesh = Mesh2d(meshes.add(Circle::new(params.mesh_radius)));
    let accretion_disk_mesh = Mesh2d(meshes.add(Circle::new(params.accretion_disk_mesh_radius)));
    let shadow = MeshMaterial2d(shadow_materials.add(Shadow::from(params)));
    let accretion_disk = MeshMaterial2d(accretion_disk_materials.add(AccretionDisk::from(params)));

    #[cfg(feature = "dynamic")]
    commands.entity(trigger.entity).insert(BlackHoleHandles {
        shadow_mesh: shadow_mesh.0.clone(),
        accretion_disk_mesh: accretion_disk_mesh.0.clone(),
        shadow: shadow.0.clone(),
        accretion_disk: accretion_disk.0.clone(),
    });

    commands.entity(trigger.entity).insert((
        shadow_mesh,
        shadow
    )).with_children(|parent| {
        parent.spawn((
            accretion_disk_mesh,
            accretion_disk,
            Transform::from_xyz(0.0, 0.0, 0.1)
        ));
    });
}

#[cfg(feature = "dynamic")]
fn on_black_hole_changed(
    query: Query<(&BlackHoleParams, &BlackHoleHandles), Changed<BlackHoleParams>>,
    mut shadow_materials: ResMut<Assets<Shadow>>,
    mut accretion_disk_materials: ResMut<Assets<AccretionDisk>>
) {
    for (params, handles) in query {
        if let Some(mut shadow) = shadow_materials.get_mut(handles.shadow.id()) {
            *shadow = Shadow::from(params);
        }
        if let Some(mut accretion_disk) = accretion_disk_materials.get_mut(handles.accretion_disk.id()) {
            *accretion_disk = AccretionDisk::from(params);
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct Shadow {
    #[uniform(0)]
    params: ShadowUniform
}
#[derive(ShaderType, Debug, Clone)]
struct ShadowUniform {
    pixels: f32,
    colors: [LinearRgba; 3],
    radius: f32,
    light_width: f32,
}
impl Material2d for Shadow {
    fn fragment_shader() -> ShaderRef { "shaders/blackhole/shadow.wgsl".into() }
}
impl From<&BlackHoleParams> for Shadow {
    fn from(value: &BlackHoleParams) -> Self {
        Shadow {
            params: ShadowUniform {
                pixels: value.shadow_params.pixels,
                colors: value.shadow_params.colors,
                radius: value.shadow_params.radius,
                light_width: value.shadow_params.light_width,
            }
        }
    }
}

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct AccretionDisk {
    #[uniform(0)]
    params: AccretionDiskUniform
}
#[derive(ShaderType, Clone, Debug)]
struct AccretionDiskUniform {
    pixels: f32,
    rotation: f32,
    light_origin: Vec2,
    time_speed: f32,
    disk_width: f32,
    ring_perspective: f32,
    should_dither: u32,
    colors: [LinearRgba; 5],
    num_colors: u32,
    size: f32,
    seed: f32,
    octaves: u32,
}
impl Material2d for AccretionDisk {
    fn fragment_shader() -> ShaderRef { "shaders/blackhole/accretiondisk.wgsl".into() }
}
impl From<&BlackHoleParams> for AccretionDisk {
    fn from(value: &BlackHoleParams) -> Self {
        AccretionDisk {
            params: AccretionDiskUniform {
                pixels: value.accretion_disk_params.pixels,
                rotation: value.accretion_disk_params.rotation,
                light_origin: value.light_origin,
                time_speed: value.time_speed,
                disk_width: value.accretion_disk_params.disk_width,
                ring_perspective: value.accretion_disk_params.ring_perspective,
                should_dither: if value.accretion_disk_params.should_dither { 1 } else { 0 },
                colors: value.accretion_disk_params.colors,
                num_colors: value.accretion_disk_params.num_colors,
                size: value.accretion_disk_params.size,
                seed: value.accretion_disk_params.seed,
                octaves: value.accretion_disk_params.octaves,
            }
        }
    }
}