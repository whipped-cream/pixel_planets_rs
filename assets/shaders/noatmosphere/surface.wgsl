#import bevy_sprite::mesh2d_view_bindings::globals
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

#import "shaders/planet_common.wgsl"::{tiled_rand, rand, fbm, spherify, rotate, dither, pixelize, compute_circle_mask, circle_noise, cloud_alpha}

override fn "shaders/planet_common.wgsl"::rand(coord: vec2<f32>, seed: f32) -> f32 {
    return tiled_rand(coord, seed, u_size, 1.);
}

// Uniforms
@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> u_pixels: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> u_rotation: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> u_light_origin: vec2<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var<uniform> u_time_speed: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var<uniform> u_dither_size: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(5) var<uniform> u_should_dither: u32;
@group(#{MATERIAL_BIND_GROUP}) @binding(6) var<uniform> u_light_border_1: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(7) var<uniform> u_light_border_2: f32;

@group(#{MATERIAL_BIND_GROUP}) @binding(8) var<uniform> u_colors: array<vec4<f32>, 3>;

@group(#{MATERIAL_BIND_GROUP}) @binding(9) var<uniform> u_size: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(10) var<uniform> u_seed: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(11) var<uniform> u_octaves: u32;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let pixel_uv = pixelize(in.uv, u_pixels);

    var d_light = distance(pixel_uv, u_light_origin);

    let a = compute_circle_mask(pixel_uv);

    let dith = dither(pixel_uv, in.uv, u_pixels);

    let rotated_uv = rotate(pixel_uv, u_rotation);

    // get a noise value with light distance added
    // this creates a moving dynamic shape
    let fbm1 = fbm(rotated_uv, u_octaves, u_seed);
    d_light += fbm(rotated_uv * u_size + fbm1 + vec2f(globals.time * u_time_speed, 0.), u_octaves, u_seed) * 0.3; // change the magic 0.3 here for different light strengths

    // size of edge in which colors should be dithered
    let dither_border = (1. / u_pixels) * u_dither_size;

    // now we can assign colors based on distance to light origin
    var level: u32;
    if (d_light > u_light_border_2) {
        if (d_light < u_light_border_2 + dither_border && (dith && u_should_dither == 1)) {
            level = 1;
        } else {
            level = 2;
        }
    } else if (d_light > u_light_border_1) {
        if (d_light < u_light_border_2 + dither_border && (dith && u_should_dither == 1)) {
            level = 0;
        } else {
            level = 1;
        }
    } else {
        level = 0;
    }
    let col = u_colors[level];

    if (a == 0.) {
        discard;
    }
    return col;
}
