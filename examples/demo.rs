use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass};
use bevy_rand::prelude::*;
use rand::Rng;
use pixel_planets_rs::{BodyType, PixelPlanetsPlugin};

use pixel_planets_rs::bodies::terran::TerranParams;
use pixel_planets_rs::bodies::asteroid::AsteroidParams;
use pixel_planets_rs::bodies::bandedgasgiant::BandedGasGiantParams;
use pixel_planets_rs::bodies::martian::MartianParams;
use pixel_planets_rs::bodies::islands::IslandsParams;
use pixel_planets_rs::bodies::noatmosphere::NoAtmosphereParams;
use pixel_planets_rs::bodies::stormygasgiant::StormyGasGiantParams;
use pixel_planets_rs::bodies::blackhole::BlackHoleParams;
use pixel_planets_rs::bodies::galaxy::GalaxyParams;
use pixel_planets_rs::bodies::iceworld::IceWorldParams;
use pixel_planets_rs::bodies::lavaworld::LavaWorldParams;
use pixel_planets_rs::bodies::Random;
use pixel_planets_rs::bodies::star::StarParams;


#[derive(Resource)]
struct UiState {
    planet_type: BodyType,
    seed: f32,
    pixels: f32,
    rotation: f32,
    dither: bool,
    colors: Vec<[f32; 4]>,
    current_planet: Option<Entity>,
}
impl Default for UiState {
    fn default() -> Self {
        UiState {
            planet_type: BodyType::Terran,
            seed: 8.98,
            pixels: 100.0,
            rotation: 0.0,
            dither: true,
            colors: vec![
                [1.0, 0.0, 0.0, 1.0],
                [0.0, 1.0, 0.0, 1.0],
                [0.0, 0.0, 1.0, 1.0],
            ],
            current_planet: None,
        }
    }
}


fn main() {
    App::new()
        .add_plugins((DefaultPlugins, EguiPlugin::default(), PixelPlanetsPlugin))
        .add_plugins(EntropyPlugin::<WyRand>::default())
        .init_resource::<UiState>()
        .add_systems(Startup, setup)
        .add_systems(EguiPrimaryContextPass, ui)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d::default(),
        Transform::from_xyz(0., 0., 0.),
    ));
}

fn spawn_planet(planet_type: &BodyType, commands: &mut Commands) -> Entity {
    match planet_type {
        BodyType::Terran => commands.spawn((TerranParams::default(), Transform::default())).id(),
        BodyType::Asteroid => commands.spawn((AsteroidParams::default(), Transform::default())).id(),
        BodyType::BandedGasGiant => commands.spawn((BandedGasGiantParams::default(), Transform::default())).id(),
        BodyType::Martian => commands.spawn((MartianParams::default(), Transform::default())).id(),
        BodyType::Islands => commands.spawn((IslandsParams::default(), Transform::default())).id(),
        BodyType::NoAtmosphere => commands.spawn((NoAtmosphereParams::default(), Transform::default())).id(),
        BodyType::StormyGasGiant => commands.spawn((StormyGasGiantParams::default(), Transform::default())).id(),
        BodyType::BlackHole => commands.spawn((BlackHoleParams::default(), Transform::default())).id(),
        BodyType::Galaxy => commands.spawn((GalaxyParams::default(), Transform::default())).id(),
        BodyType::IceWorld => commands.spawn((IceWorldParams::default(), Transform::default())).id(),
        BodyType::LavaWorld => commands.spawn((LavaWorldParams::default(), Transform::default())).id(),
        BodyType::Star => commands.spawn((StarParams::default(), Transform::default())).id(),
    }
}

fn spawn_planet_random(rng: &mut impl Rng, planet_type: &BodyType, commands: &mut Commands) -> Entity {
    match planet_type {
        BodyType::Terran => commands.spawn((TerranParams::random(rng), Transform::default())).id(),
        BodyType::Asteroid => commands.spawn((AsteroidParams::random(rng), Transform::default())).id(),
        BodyType::BandedGasGiant => commands.spawn((BandedGasGiantParams::random(rng), Transform::default())).id(),
        BodyType::Martian => commands.spawn((MartianParams::random(rng), Transform::default())).id(),
        BodyType::Islands => commands.spawn((IslandsParams::random(rng), Transform::default())).id(),
        BodyType::NoAtmosphere => commands.spawn((NoAtmosphereParams::random(rng), Transform::default())).id(),
        BodyType::StormyGasGiant => commands.spawn((StormyGasGiantParams::random(rng), Transform::default())).id(),
        BodyType::BlackHole => commands.spawn((BlackHoleParams::random(rng), Transform::default())).id(),
        BodyType::Galaxy => commands.spawn((GalaxyParams::random(rng), Transform::default())).id(),
        BodyType::IceWorld => commands.spawn((IceWorldParams::random(rng), Transform::default())).id(),
        BodyType::LavaWorld => commands.spawn((LavaWorldParams::random(rng), Transform::default())).id(),
        BodyType::Star => commands.spawn((StarParams::random(rng), Transform::default())).id(),
    }
}

fn ui(
    mut contexts: EguiContexts,
    mut state: ResMut<UiState>,
    mut commands: Commands,
    mut rng: Single<&mut WyRand>
) -> Result {
    let ctx = contexts.ctx_mut()?;
    let mut viewport_ui = egui::Ui::new(
        ctx.clone(),
        "viewport".into(),
        egui::UiBuilder::new()
            .layer_id(egui::LayerId::background())
            .max_rect(ctx.viewport_rect()),
    );

    egui::Panel::right("controls")
        .default_size(220.0)
        .show(&mut viewport_ui, |ui| {

            ui.separator();

            // Planet type
            ui.label("PLANET TYPE:");
            egui::ComboBox::from_id_salt("planet_type")
                .selected_text(state.planet_type.label())
                .show_ui(ui, |ui| {
                    for planet_type in BodyType::all() {
                        let selected = *planet_type == state.planet_type;
                        if ui.selectable_label(selected, planet_type.label()).clicked() && !selected {
                            state.planet_type = planet_type.clone();
                            if let Some(entity) = state.current_planet.take() {
                                commands.entity(entity).despawn();
                            }
                            state.current_planet = Some(spawn_planet(&state.planet_type, &mut commands));
                        }
                    }
                });

            ui.separator();
            ui.label("SEED:");
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(&mut state.seed).range(1.0..=10.0).speed(0.01));
                if ui.button("RAND").clicked() {
                    state.seed = rand::random::<f32>() * 9.0 + 1.0;
                }
            });

            ui.separator();
            ui.label("PIXELS:");
            ui.add(egui::DragValue::new(&mut state.pixels).range(10.0..=300.0).speed(1.0));

            ui.separator();
            ui.label("ROTATION:");
            ui.add(egui::Slider::new(&mut state.rotation, 0.0..=6.28).text("rad"));

            ui.separator();
            ui.horizontal(|ui| {
                ui.label("DITHER:");
                let dither_label = if state.dither { "ON" } else { "OFF" };
                ui.toggle_value(&mut state.dither, dither_label);
            });

            ui.separator();
            ui.label("COLORS:");
            ui.horizontal_wrapped(|ui| {
                for color in state.colors.iter_mut() {
                    ui.color_edit_button_rgba_unmultiplied(color);
                }
            });
            ui.horizontal(|ui| {
                if ui.button("RANDOM").clicked() {
                    for color in state.colors.iter_mut() {
                        *color = [rand::random(), rand::random(), rand::random(), 1.0];
                    }
                }
                if ui.button("RESET").clicked() {
                    // TODO: reset to planet type defaults
                }
            });

            ui.separator();
            ui.label("LAYERS:");
            egui::ComboBox::from_id_salt("layers")
                .selected_text("Select layers...")
                .show_ui(ui, |ui| {
                    ui.label("(no layers)");
                });

            ui.separator();
            ui.label("EXPORT:");
            ui.horizontal(|ui| {
                if ui.button("PNG").clicked() { /* TODO */ }
                if ui.button("GIF").clicked() { /* TODO */ }
                if ui.button("SPRITESHEET").clicked() { /* TODO */ }
            });

            ui.separator();
            if ui.button(if state.current_planet.is_some() { "DESPAWN" } else { "SPAWN" }).clicked() {
                if let Some(entity) = state.current_planet.take() {
                    commands.entity(entity).despawn();
                } else {
                    state.current_planet = Some(spawn_planet_random(rng.as_mut(), &state.planet_type, &mut commands));
                }
            }
        });

    Ok(())
}