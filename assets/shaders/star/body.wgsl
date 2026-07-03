#import bevy_sprite::mesh2d_view_bindings::globals
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

#import "shaders/planet_common.wgsl"::{tiled_rand, spherify, rotate, dither, pixelize, compute_circle_mask, circle_noise, cloud_alpha, posterize, mod_floor_vec2}

struct Params {
    pixels: f32,
    time_speed: f32,
    rotation: f32,
    colors: array<vec4<f32>, 4>,
    num_colors: u32,
    should_dither: u32,
    size: f32,
    seed: f32,
    octaves: u32,
    tiles: u32,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> u_params: Params;

fn hash2(p: vec2<f32>) -> vec2<f32> {
    let r = 523.0*sin(dot(p, vec2(53.3158, 43.6143)));
    return vec2f(fract(15.32354 * r), fract(17.25865 * r));
}

// Tileable cell noise by Dave_Hoskins from shadertoy: https://www.shadertoy.com/view/4djGRh
fn cells(uv: vec2<f32>, num_cells: f32, no_tiles: u32) -> f32 {
    let tiles = uv * num_cells;
    var d = 1.0e10;
    for (var x: i32 = -1; x <= 1; x++) {
        for (var y: i32 = -1; y <= 1; y++) {
            var tp = floor(tiles) + vec2f(f32(x), f32(y));
            tp = tiles - tp - hash2(mod_floor_vec2(tp, num_cells) / f32(no_tiles));
            d = min(d, dot(tp, tp));
        }
    }
    return sqrt(d);
}


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let pixel_uv = pixelize(in.uv, u_params.pixels);

    let a = compute_circle_mask(pixel_uv);

    let dith = dither(pixel_uv, in.uv, u_params.pixels);

    let rotated_uv = rotate(pixel_uv, u_params.rotation);

    let sphere_uv = spherify(rotated_uv);

    var n = cells(sphere_uv - vec2f(globals.time * u_params.time_speed * 2., 0.), 10, u_params.tiles) * cells(sphere_uv - vec2f(globals.time * u_params.time_speed * 1., 0.), 20, u_params.tiles);;

    // adjust cell value to get better looking stuff
    n *= 2.;
    n = clamp(n, 0., 1.);
    if (dith && u_params.should_dither == 1) {
        n *= 1.3;
    }

    // constrain values 4 possibilities and then choose color based on those
    let col = u_params.colors[posterize(n, u_params.num_colors)];

    if (a == 0.) {
        discard;
    }

    return vec4f(col.rgb, a * col.a);
}
