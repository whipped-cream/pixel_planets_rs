#import bevy_sprite::mesh2d_view_bindings::globals
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

#import "shaders/planet_common.wgsl"::{tiled_rand, spherify, rotate, dither, pixelize, compute_circle_mask, circle_noise, cloud_alpha, mod_floor_vec2, mod_floor_f32}


struct Params {
    pixels: f32,
    time_speed: f32,
    rotation: f32,
    circle_amount: f32,
    circle_size: f32,
    colors: array<vec4<f32>, 1>,
    size: f32,
    seed: f32,
    octaves: u32,
}


@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> u_params: Params;



fn circle(uv: vec2<f32>, circle_amount: f32, circle_size: f32, seed: f32, size: f32) -> f32 {
    var circle_uv = uv;

    let invert = 1. / circle_amount;

    if (mod_floor_f32(uv.y, (invert * 2.)) < invert) {
        circle_uv.x += invert * 0.5;
    }
    let rand_co = floor(circle_uv * circle_amount) / circle_amount;
    circle_uv = mod_floor_vec2(uv, invert) * circle_amount;

    var r = tiled_rand(rand_co, seed, size, 1.);
    r = clamp(r, invert, 1. - invert);
    let circle = distance(circle_uv, vec2f(r));
    return smoothstep(circle, circle + 0.5, invert * circle_size * tiled_rand(rand_co * 1.5, seed, size, 1.));
}


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let pixel_uv = pixelize(in.uv, u_params.pixels);

    let rotated_uv = rotate(pixel_uv, u_params.rotation);

    // angle from centered uv's
    let angle = atan2(rotated_uv.x - 0.5, rotated_uv.y - 0.5);
    let d = distance(pixel_uv, vec2f(0.5));

    var c = 0.;
    for (var i = 0; i < 15; i++) {
        let r = tiled_rand(vec2f(f32(i)), u_params.seed, u_params.size, 1.);
        let circle_uv = vec2f(d, angle);
        c += circle(circle_uv * u_params.size - globals.time * u_params.time_speed - (1./d) * 0.1 + r, u_params.circle_amount, u_params.circle_size, u_params.seed, u_params.size);
    }
    c *= 0.37 - d;
    c = step(0.07, c - d);

    if (c == 0.) {
        discard;
    }

    return vec4(u_params.colors[0].rgb, c * u_params.colors[0].a);
}
