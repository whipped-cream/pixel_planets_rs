#import bevy_sprite::mesh2d_view_bindings::globals
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

#import "shaders/planet_common.wgsl"::{tiled_rand, rand, spherify, rotate, dither, pixelize, compute_circle_mask, circle_noise, cloud_alpha}

override fn "shaders/planet_common.wgsl"::rand(coord: vec2<f32>, seed: f32) -> f32 {
    return tiled_rand(coord, seed, u_params.size, 1.);
}

struct Params {
    pixels: f32,
    rotation: f32,
    light_origin: vec2<f32>,
    time_speed: f32,
    light_border: f32,
    colors: array<vec4<f32>, 2>,
    size: f32,
    seed: f32,
}

// Uniforms
@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> u_params: Params;


fn crater(uv: vec2<f32>, seed: f32, size: f32, time: f32, time_speed: f32) -> f32 {
    var c = 1.0;
    for (var i: u32 = 0; i < 2; i++) {
        c *= circle_noise((uv * size) + f32(i) + 11. + vec2f(time * time_speed * 0.5, 0.0), seed);
    }
    return 1. - c;
}


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let pixel_uv = pixelize(in.uv, u_params.pixels);

    var d_light = distance(pixel_uv, u_params.light_origin);

    let a = compute_circle_mask(pixel_uv);

    let rotated_uv = rotate(pixel_uv, u_params.rotation);

    let sphere_uv = spherify(rotated_uv);

    let c1 = crater(sphere_uv, u_params.seed, u_params.size, globals.time, u_params.time_speed);
    let c2 = crater(sphere_uv + (u_params.light_origin - 0.5) * 0.03, u_params.seed, u_params.size, globals.time, u_params.time_speed);
    let crater_mask = step(0.5, c1);

    // now we can assign colors based on distance to light origin
    var level: u32;
    if (c2 < c1 - (0.5 - d_light) * 2. || d_light > u_params.light_border) {
        level = 1;
    } else {
        level = 0;
    }
    let col = u_params.colors[level];

    if (a == 0. || crater_mask == 0.) {
        discard;
    }
    return col;
}
