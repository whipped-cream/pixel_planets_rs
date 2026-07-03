#import bevy_sprite::mesh2d_view_bindings::globals
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

#import "shaders/planet_common.wgsl"::{tiled_rand, fbm, spherify, rotate, dither, pixelize, compute_circle_mask, circle_noise, cloud_alpha, mod_floor_f32, mod_floor_vec2}

struct Params {
    pixels: f32,
    time_speed: f32,
    rotation: f32,
    should_dither: u32,
    storm_width: f32,
    storm_dither_width: f32,
    circle_amount: f32,
    circle_scale: f32,
    scale: f32,
    colors: array<vec4<f32>, 2>,
    size: f32,
    seed: f32,
    octaves: u32,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> u_params: Params;

override fn "shaders/planet_common.wgsl"::rand(coord: vec2<f32>, seed: f32) -> f32 {
    return tiled_rand(coord, seed, u_params.size, 1.);
}

fn circle(uv: vec2<f32>, circle_amount: f32, circle_scale: f32, seed: f32, size: f32) -> f32 {
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
    return smoothstep(circle, circle + 0.5, invert * circle_scale * tiled_rand(rand_co * 1.5, seed, size, 1.));
}


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let pixel_uv = pixelize(in.uv, u_params.pixels);

    let dith = dither(pixel_uv, in.uv, u_params.pixels);

    let rotated_uv = rotate(pixel_uv, u_params.rotation);

    // angle from centered uv's
    let angle = atan2(rotated_uv.y - 0.5, rotated_uv.x - 0.5) * 0.4;
    let d = distance(pixel_uv, vec2f(0.5));

    // we make uv circular here to have eternally outward moving stuff
    let circle_uv = vec2f(d, angle);

    // two types of noise values
    let n = fbm(circle_uv * u_params.size - globals.time * u_params.time_speed, u_params.octaves, u_params.seed);
    var nc = circle(circle_uv * u_params.scale - globals.time * u_params.time_speed + n, u_params.circle_amount, u_params.circle_scale, u_params.seed, u_params.size);

    nc *= 1.5;
    let n2 = fbm(circle_uv * u_params.size - globals.time - vec2f(100.), u_params.octaves, u_params.seed);
    nc -= n2 * 0.1;

    // our alpha, default 0
    var a: f32;
    if (1. - d > nc) {
        // now we generate very thin strips of positive alpha if our noise has certain values and is close enough to center
        if (nc > u_params.storm_width - u_params.storm_dither_width + d && (dith && u_params.should_dither == 1)) {
            a = 1.0;
        } else if (nc > u_params.storm_width + d) { // could use an or statement instead, but this looks more clear to me
            a = 1.0;
        }
    } else {
        a = 0.;
    }

    // use our two noise values to assign colors
    let interpolated = floor(n2 + nc);
    let col = u_params.colors[u32(interpolated)];

    // final step to not have everything appear from the center
    a *= step(n2 * 0.25, d);
    if (a == 0.) {
        discard;
    }
    return vec4f(col.rgb, a * col.a);
}
