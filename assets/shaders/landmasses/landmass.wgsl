#import bevy_sprite::mesh2d_view_bindings::globals
#import bevy_render::view::View
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

#import "shaders/planet_common.wgsl"::{tiled_rand, rand, fbm, spherify, rotate, pixelize, compute_circle_mask, circle_noise, cloud_alpha}

@group(0) @binding(0) var<uniform> view: View;


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
    land_cutoff: f32,
    colors: array<vec4<f32>, 4>,
    size: f32,
    seed: f32,
    octaves: u32,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> u_params: Params;


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let pixel_uv = pixelize(in.uv, u_params.pixels);

    var d_light = distance(pixel_uv, u_params.light_origin);

    let a = compute_circle_mask(pixel_uv);

    let rotated_uv = rotate(pixel_uv, u_params.rotation);

    var sphere_uv = spherify(rotated_uv);

    let base_fbm_uv = sphere_uv * u_params.size + vec2f(globals.time * u_params.time_speed, 0.);

    // use multiple fbm's at different places so we can determine what color land gets
    let fbm1 = fbm(base_fbm_uv, u_params.octaves, u_params.seed);
    var fbm2 = fbm(base_fbm_uv - u_params.light_origin * fbm1, u_params.octaves, u_params.seed);
    var fbm3 = fbm(base_fbm_uv - u_params.light_origin * 1.5 * fbm1, u_params.octaves, u_params.seed);
    var fbm4 = fbm(base_fbm_uv - u_params.light_origin * 2.0 * fbm1, u_params.octaves, u_params.seed);

    // lots of magic numbers here
    // you can mess with them, it changes the color distribution
    if (d_light < u_params.light_border_1) {
        fbm4 *= 0.9;
    } else {
        fbm2 *= 1.05;
        fbm3 *= 1.05;
        fbm4 *= 1.05;
    }
    if (d_light > u_params.light_border_2) {
        fbm2 *= 1.3;
        fbm3 *= 1.4;
        fbm4 *= 1.8;
    }

    // increase contrast on d_light
    d_light = pow(d_light, 2.0) * 0.1;

    var level: u32;
    if (fbm2 + d_light < fbm1) {
        level = 0;
    } else if (fbm3 + d_light < fbm1 * 1.0) {
        level = 1;
    } else if (fbm4 + d_light < fbm1 * 1.5) {
        level = 2;
    } else {
        level = 3;
    }
    let col = u_params.colors[level];

    let land_mask = step(u_params.land_cutoff, fbm1);
    if (land_mask == 0. || a == 0.) {
        discard;
    }
    return col;
}
