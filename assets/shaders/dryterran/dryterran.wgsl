#import bevy_sprite::mesh2d_view_bindings::globals
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

#import "shaders/planet_common.wgsl"::{pixelize, rand, tiled_rand, spherify, fbm, rotate, dither, posterize, compute_circle_mask}


override fn "shaders/planet_common.wgsl"::rand(coord: vec2<f32>, seed: f32) -> f32 {
    return tiled_rand(coord, seed, u_params.size, 2.);
}

struct Params {
    pixels: f32,
    rotation: f32,
    light_origin: vec2<f32>,
    light_border_1: f32,
    light_border_2: f32,
    time_speed: f32,
    dither_size: f32,
    should_dither: u32,
    colors: array<vec4<f32>, 5>,
    num_colors: u32,
    size: f32,
    seed: f32,
    octaves: u32,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> u_params: Params;


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // pixelize uv
    let pixel_uv = pixelize(in.uv, u_params.pixels);

    let dith = dither(pixel_uv, in.uv, u_params.pixels);

    let a = compute_circle_mask(pixel_uv);

    if (a == 0.0) {
        discard;
    }

    // map to sphere
    let sphere_uv = spherify(pixel_uv);
    var light_distance = distance(sphere_uv, u_params.light_origin);

    // give planet a tilt
    let rotated_uv = rotate(sphere_uv, u_params.rotation);

    // noise
    let fbm_val = fbm(rotated_uv * u_params.size + vec2f(globals.time * u_params.time_speed, 0), u_params.octaves, u_params.seed);

    // remap light
    var light_distance_remapped = smoothstep(-0.3, 1.2, light_distance);
    if (light_distance_remapped < u_params.light_border_1) {
        light_distance_remapped *= 0.9;
    } else if (light_distance_remapped < u_params.light_border_2) {
        light_distance_remapped *= 0.9 * 0.9;
    }

    // Change magic numbers for different light strengths
    var light_strength = light_distance_remapped * pow(fbm_val, 0.8) * 3.5;

    // Apply dithering
    if (dith && u_params.should_dither == 1) {
       light_strength += 0.02;
       light_strength *= 1.05;
    }

    let col = u_params.colors[posterize(light_strength, u_params.num_colors)];
    return vec4f(col.rgb, a * col.a);
}

