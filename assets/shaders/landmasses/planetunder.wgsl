// This is the PlanetUnder shader from PixelPlanets

#import bevy_sprite::mesh2d_view_bindings::globals
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

#import "shaders/planet_common.wgsl"::{tiled_rand, rand, fbm, spherify, rotate, dither, pixelize, compute_circle_mask, circle_noise, cloud_alpha}

override fn "shaders/planet_common.wgsl"::rand(coord: vec2<f32>, seed: f32) -> f32 {
    return tiled_rand(coord, seed, u_params.size, 2.);
}

struct Params {
    pixels: f32,
    rotation: f32,
    light_origin: vec2<f32>,
    time_speed: f32,
    dither_size: f32,
    light_border_1: f32,
    light_border_2: f32,
    should_dither: u32,
    colors: array<vec4<f32>, 3>,
    size: f32,
    seed: f32,
    octaves: u32,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> u_params: Params;


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let pixel_uv = pixelize(in.uv, u_params.pixels);

    let dith = dither(pixel_uv, in.uv, u_params.pixels);

    var d_light = distance(pixel_uv, u_params.light_origin);

    let a = compute_circle_mask(pixel_uv);

    var sphere_uv = spherify(pixel_uv);
    let rotated_uv = rotate(sphere_uv, u_params.rotation);


    // get a noise value with light distance added
    d_light += fbm(rotated_uv * u_params.size + vec2f(globals.time * u_params.time_speed, 0.), u_params.octaves, u_params.seed) * 0.3; // change the magic 0.3 here for different light strengths

    // size of edge in which colors should be dithered
    let dither_border = (1. / u_params.pixels) * u_params.dither_size;

    // now we can assign colors based on distance to light origin
    var level: u32;
    if (d_light > u_params.light_border_2) {
        if (d_light < u_params.light_border_2 + dither_border && (dith && u_params.should_dither == 1)) {
            level = 1;
        } else {
            level = 2;
        }
    } else if (d_light > u_params.light_border_1) {
        if (d_light < u_params.light_border_2 + dither_border && (dith && u_params.should_dither == 1)) {
            level = 0;
        } else {
            level = 1;
        }
    } else {
        level = 0;
    }
    let col = u_params.colors[level];

    if (a == 0.) {
        return vec4f(0.);
    }
    return col;
}
