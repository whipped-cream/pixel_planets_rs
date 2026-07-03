#import bevy_sprite::mesh2d_view_bindings::globals
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

#import "shaders/planet_common.wgsl"::{rand, tiled_rand, fbm, spherify, rotate, dither, pixelize, compute_circle_mask, circle_noise, cloud_alpha}

override fn "shaders/planet_common.wgsl"::rand(coord: vec2<f32>, seed: f32) -> f32 {
    return tiled_rand(coord, seed, u_params.size, 2.);
}

struct Params {
    pixels: f32,
    rotation: f32,
    light_origin: vec2<f32>,
    time_speed: f32,
    light_border_1: f32,
    light_border_2: f32,
    river_cutoff: f32,
    colors: array<vec4<f32>, 3>,
    size: f32,
    seed: f32,
    octaves: u32,
}

// Uniforms
@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> u_params: Params;


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let pixel_uv = pixelize(in.uv, u_params.pixels);

    let a = compute_circle_mask(pixel_uv);

    let rotated_uv = rotate(pixel_uv, u_params.rotation);

    var sphere_uv = spherify(rotated_uv);

    // some scrolling noise for landmasses
    let fbm1 = fbm(sphere_uv * u_params.size + vec2f(globals.time * u_params.time_speed, 0.), u_params.octaves, u_params.seed);
    let river_fbm = fbm(sphere_uv + fbm1 * 2.5, u_params.octaves, u_params.seed);

    // increase contrast on d_light
    var d_light = distance(pixel_uv, u_params.light_origin);
    d_light = pow(d_light, 2.) * 0.4;
    d_light -= d_light * river_fbm;

    // now we can assign colors
    var level: u32;
    if (d_light > u_params.light_border_2) {
        level = 2;
    } else if (d_light > u_params.light_border_1) {
        level = 1;
    } else {
        level = 0;
    }
    let col = u_params.colors[level];

    let river_mask = step(u_params.river_cutoff, river_fbm);
    if (river_mask == 0. || a == 0.) {
        discard;
    }
    return col;
}






