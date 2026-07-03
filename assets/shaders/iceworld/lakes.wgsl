#import bevy_sprite::mesh2d_view_bindings::globals
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

#import "shaders/planet_common.wgsl"::{rand, tiled_rand, spherify, rotate, dither, pixelize, fbm, compute_circle_mask, circle_noise, posterize}

override fn "shaders/planet_common.wgsl"::rand(coord: vec2<f32>, seed: f32) -> f32 {
    return tiled_rand(coord, seed, u_params.size, 2.);
}

struct Params {
    lake_cutoff: f32,
    colors: array<vec4<f32>, 3>,
    light_border_1: f32,
    light_border_2: f32,
    rotation: f32,
    pixels: f32,
    light_origin: vec2<f32>,
    time_speed: f32,
    size: f32,
    seed: f32,
    octaves: u32,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> u_params: Params;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let pixel_uv = pixelize(in.uv, u_params.pixels);

    let rotated_uv = rotate(pixel_uv, u_params.rotation);

    let d_circle = distance(rotated_uv, vec2f(0.5));

    let sphere_uv = spherify(rotated_uv);

    let lake = fbm(sphere_uv * u_params.size + vec2f(globals.time * u_params.time_speed, 0.0), u_params.octaves, u_params.seed);

    var d_light = distance(pixel_uv, u_params.light_origin);
    d_light = pow(d_light, 2.0) * 0.4;
    d_light -= d_light * lake;

    var level: u32;
    if (d_light > u_params.light_border_2) {
        level = 2;
    } else if (d_light > u_params.light_border_1) {
        level = 1;
    } else {
        level = 0;
    }
    let col = u_params.colors[level];

    let a = step(u_params.lake_cutoff, lake);
    if (a == 0.0 || d_circle > 0.5) {
        discard;
    }
    return col;
}