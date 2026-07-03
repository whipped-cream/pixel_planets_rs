#import bevy_sprite::mesh2d_view_bindings::globals
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

#import "shaders/planet_common.wgsl"::{pixelize, rand, tiled_rand, spherify, fbm, rotate, dither, posterize, compute_circle_mask}


override fn "shaders/planet_common.wgsl"::rand(coord: vec2<f32>, seed: f32) -> f32 {
    return tiled_rand(coord, seed, u_size, 2.);
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> u_pixels: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> u_rotation: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> u_light_origin: vec2<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var<uniform> u_light_border_1: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var<uniform> u_light_border_2: f32;

@group(#{MATERIAL_BIND_GROUP}) @binding(5) var<uniform> u_time_speed: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(6) var<uniform> u_dither_size: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(7) var<uniform> u_should_dither: u32;

@group(#{MATERIAL_BIND_GROUP}) @binding(8) var<uniform> u_colors: array<vec4<f32>, 5>;
@group(#{MATERIAL_BIND_GROUP}) @binding(9) var<uniform> u_num_colors: u32;

@group(#{MATERIAL_BIND_GROUP}) @binding(10) var<uniform> u_size: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(11) var<uniform> u_seed: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(12) var<uniform> u_octaves: u32;


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // pixelize uv
    let pixel_uv = pixelize(in.uv, u_pixels);

    let dith = dither(pixel_uv, in.uv, u_pixels);

    let a = compute_circle_mask(pixel_uv);

    if (a == 0.0) {
        discard;
    }

    // map to sphere
    let sphere_uv = spherify(pixel_uv);
    var light_distance = distance(sphere_uv, u_light_origin);

    // give planet a tilt
    let rotated_uv = rotate(sphere_uv, u_rotation);

    // noise
    let fbm_val = fbm(rotated_uv * u_size + vec2f(globals.time * u_time_speed, 0), u_octaves, u_seed);

    // remap light
    var light_distance_remapped = smoothstep(-0.3, 1.2, light_distance);
    if (light_distance_remapped < u_light_border_1) {
        light_distance_remapped *= 0.9;
    } else if (light_distance_remapped < u_light_border_2) {
        light_distance_remapped *= 0.9 * 0.9;
    }

    // Change magic numbers for different light strengths
    var light_strength = light_distance_remapped * pow(fbm_val, 0.8) * 3.5;

    // Apply dithering
    if (dith && u_should_dither == 1) {
       light_strength += 0.02;
       light_strength *= 1.05;
    }

    let col = u_colors[posterize(light_strength, u_num_colors)];
    return vec4f(col.rgb, a * col.a);
}

