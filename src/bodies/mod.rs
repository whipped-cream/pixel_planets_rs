use std::array;
use std::cmp::max;
use bevy::prelude::*;
use bevy::prelude::ops::cos;
use rand::{Rng, RngExt};
use rand::distr::uniform::SampleRange;

pub mod terran;
pub mod lavaworld;
pub mod martian;
pub mod islands;
pub mod noatmosphere;
pub mod stormygasgiant;
pub mod bandedgasgiant;
pub mod iceworld;
mod building_blocks;
pub mod asteroid;
pub mod blackhole;
pub mod galaxy;
pub mod star;

#[derive(Component, Default)]
pub struct PixelPlanet;

/// The Randomizable trait states that a struct can be constructed randomly
pub trait Random {
    /// Generate a random body.
    /// The generated struct is guaranteed to be identical between calls if the state of rng is the same.
    fn random(rng: &mut impl Rng) -> Self;
    // Generate a random body with the default colors.
    // The generated struct is guaranteed to be identical between calls if the state of rng is the same.
    // As the normal randomization logic is used in this function, the generated struct will match
    // that of `random`, but with the default colors
    // fn random_default_colors(rng: &mut impl Rng) -> Self;
}

// Using ideas from https://www.iquilezles.org/www/articles/palettes/palettes.htm
pub(crate) fn generate_colorscheme_base<const NUM_COLORS: usize>(rng: &mut impl Rng, hue_diff: f32, saturation: f32) -> [Color; NUM_COLORS] {
    // let hue_diff = hue_diff.unwrap_or(0.9);
    // let saturation = saturation.unwrap_or(0.9);

    let a = Vec3::new(0.5, 0.5, 0.5);
    let b = Vec3::new(0.5, 0.5, 0.5) * saturation;
    let c = Vec3::new(rng.random_range(0.5..1.5), rng.random_range(0.5..1.5), rng.random_range(0.5..1.5)) * hue_diff;
    let d = Vec3::new(rng.random_range(0.0..1.0), rng.random_range(0.0..1.0), rng.random_range(0.0..1.0)) * rng.random_range(1.0..3.0);

    let n = max(1, NUM_COLORS - 1) as f32;

    array::from_fn(|i| { // Not sure if the i / n is integer division or not. I dont think so unless Godot max returns int when argument is a float which wouldnt make sense
        Srgba::new(
            a.x + b.x * cos(6.28318 * (c.x * (i as f32 / n) + d.x)),
            a.y + b.y * cos(6.28318 * (c.y * (i as f32 / n) + d.y)),
            a.z + b.z * cos(6.28318 * (c.z * (i as f32 / n) + d.z)),
            1.0
        ).into()
    })
}

pub(crate) fn generate_random_colorscheme<const NUM_COLORS: usize>(
    rng: &mut impl Rng,
    hue_diff_range: impl SampleRange<f32>,
    saturation: f32,
    a: f32,
    b: f32,
    c: f32,
    d: f32
) -> [Color; NUM_COLORS] {
    let hue_diff = rng.random_range(hue_diff_range);
    let seed_colors: [_; NUM_COLORS] = generate_colorscheme_base(rng, hue_diff, saturation);
    array::from_fn(|i| {
        seed_colors[i]
            .mix(&Color::BLACK, i as f32 / a * b)
            .mix(&Color::WHITE, (1.0 - (i as f32 / c)) * d)
    })
}

#[derive(Debug, Clone, PartialEq)]
pub enum BodyType {
    Terran,
    Asteroid,
    BandedGasGiant,
    Martian,
    Islands,
    NoAtmosphere,
    StormyGasGiant,
    BlackHole,
    Galaxy,
    IceWorld,
    LavaWorld,
    Star
}

impl BodyType {
    pub fn all() -> &'static [BodyType] {
        &[
            BodyType::Terran,
            BodyType::Asteroid,
            BodyType::BandedGasGiant,
            BodyType::Martian,
            BodyType::Islands,
            BodyType::NoAtmosphere,
            BodyType::StormyGasGiant,
            BodyType::BlackHole,
            BodyType::Galaxy,
            BodyType::IceWorld,
            BodyType::LavaWorld,
            BodyType::Star,
        ]
    }

    pub fn label(&self) -> &'static str {
        match self {
            BodyType::Terran => "Terran",
            BodyType::Asteroid => "Asteroid",
            BodyType::BandedGasGiant => "Banded Gas Giant",
            BodyType::Martian => "Martian",
            BodyType::Islands => "Islands",
            BodyType::NoAtmosphere => "No Atmosphere",
            BodyType::StormyGasGiant => "Stormy Gas Giant",
            BodyType::BlackHole => "Black Hole",
            BodyType::Galaxy => "Galaxy",
            BodyType::IceWorld => "Ice World",
            BodyType::LavaWorld => "Lava World",
            BodyType::Star => "Star",
        }
    }
}