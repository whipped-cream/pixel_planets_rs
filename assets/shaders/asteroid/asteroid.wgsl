#import bevy_sprite::mesh2d_view_bindings::globals
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

#import "shaders/planet_common.wgsl"::{rand, noise, fbm, spherify, rotate, dither, pixelize, compute_circle_mask, circle_noise}


struct Params {
    pixels: f32,
    rotation: f32,
    light_origin: vec2<f32>,
    should_dither: u32,
    colors: array<vec4<f32>, 3>,
    size: f32,
    seed: f32,
    octaves: u32,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> u_params: Params;


fn crater(uv: vec2<f32>, seed: f32, size: f32) -> f32 {
    var c = 1.0;
    for (var i: u32 = 0; i < 2; i++) {
        c *= circle_noise((uv * size) + f32(i) + 11., seed);
    }
    return 1. - c;
}


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let pixel_uv = pixelize(in.uv, u_params.pixels);

    let dith = dither(pixel_uv, in.uv, u_params.pixels);

    let d_to_center = distance(pixel_uv, vec2f(0.5));

    let rotated_uv = rotate(pixel_uv, u_params.rotation);

    // two noise values with one slightly offset according to light source, to create shadows later
    let n = fbm(rotated_uv * u_params.size, u_params.octaves, u_params.seed);
    let n2 = fbm(rotated_uv * u_params.size + (rotate(u_params.light_origin, u_params.rotation) - 0.5) * 0.5, u_params.octaves, u_params.seed);

    // step noise values to determine where the edge of the asteroid is
    // step cutoff value depends on distance from center
    let n_step = step(0.2, n - d_to_center);
    if (n_step == 0.0) {
        discard;
    }

    let n2_step = step(0.2, n2 - d_to_center);

    // with this val we can determine where the shadows should be
    let noise_rel = (n2_step + n2) - (n_step + n);

    // two crater values, again one extra for the shadows
    let c1 = crater(rotated_uv, u_params.seed, u_params.size);
    let c2 = crater(rotated_uv + (u_params.light_origin - 0.5) * 0.03, u_params.seed, u_params.size);

    // now we just assign colors depending on noise values and crater values

    var level: u32;
    // craters
    if (c2 < c1) {
        level = 2;
    } else if (c1 > 0.4) {
        level = 1;
    } else
    // noise
    if (noise_rel > 0.05 || (noise_rel > 0.03 && (dith && u_params.should_dither == 1))) {
        level = 2;
    } else if (noise_rel < -0.06 || (noise_rel < -0.04 && (dith && u_params.should_dither == 1))) {
        level = 0;
    }
    // base
    else {
        level = 1;
    }

    return u_params.colors[level];
}
