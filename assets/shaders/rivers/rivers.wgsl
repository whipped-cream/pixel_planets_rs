#import bevy_sprite::mesh2d_view_bindings::globals
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

#import "shaders/planet_common.wgsl"::{pixelize, rand, tiled_rand, spherify, fbm, rotate, dither}


override fn "shaders/planet_common.wgsl"::rand(coord: vec2<f32>, seed: f32) -> f32 {
    return tiled_rand(coord, seed, u_size, 2.);
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
@group(#{MATERIAL_BIND_GROUP}) @binding(8) var<uniform> u_river_cutoff: f32;

@group(#{MATERIAL_BIND_GROUP}) @binding(9) var<uniform> u_colors: array<vec4<f32>, 6>;

@group(#{MATERIAL_BIND_GROUP}) @binding(10) var<uniform> u_size: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(11) var<uniform> u_seed: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(12) var<uniform> u_octaves: u32;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    // pixelize uv
    let pixel_uv = pixelize(in.uv, u_pixels);

    let dith = dither(pixel_uv, in.uv, u_pixels);
    // stepping over 0.5 instead of 0.49999 makes some pixels a little buggy
    let a = step(length(pixel_uv - vec2f(0.5)), 0.49999);

    if (a == 0.0) {
        return vec4(0.);
    }

    // map to sphere
    let sphere_uv = spherify(pixel_uv);
    var d_light = distance(sphere_uv, u_light_origin);

    // give planet a tilt
    let rotated_uv = rotate(sphere_uv, u_rotation);

    // some scrolling noise for landmasses
    let base_fbm_uv = rotated_uv * u_size + vec2f(globals.time * u_time_speed, 0.);

    // use multiple fbm's at different places so we can determine what color land gets
    let fbm1 = fbm(base_fbm_uv, u_octaves, u_seed);
    var fbm2 = fbm(base_fbm_uv - u_light_origin * fbm1, u_octaves, u_seed);
    var fbm3 = fbm(base_fbm_uv - u_light_origin * 1.5 * fbm1, u_octaves, u_seed);
    var fbm4 = fbm(base_fbm_uv - u_light_origin * 2.0 * fbm1, u_octaves, u_seed);

    let river_fbm = step(u_river_cutoff, fbm(base_fbm_uv + fbm1 * 6.0, u_octaves, u_seed));

    // size of edge in which colors should be dithered
    let dither_border = (1.0 / u_pixels) * u_dither_size;
    // lots of magic numbers here
    // you can mess with them, it changes the color distribution
    if (d_light < u_light_border_1) {
        fbm4 *= 0.9;
    } else {
        fbm2 *= 1.05;
        fbm3 *= 1.05;
        fbm4 *= 1.05;
    }
    if (d_light > u_light_border_2) {
        fbm2 *= 1.3;
        fbm3 *= 1.4;
        fbm4 *= 1.8;

        if (d_light < u_light_border_2 + dither_border) {
            if (dith || !(u_should_dither == 1)) {
                fbm4 *= 0.5;
            }
        }
    }

    // increase contrast on d_light
    d_light = pow(d_light, 2.0) * 0.4;

    var level: u32;
    if (river_fbm < fbm1 * 0.5) {
        if (fbm4 + d_light < fbm1 * 1.5) {
            level = 4;
        } else {
            level = 5;
        }
    } else if (fbm2 + d_light < fbm1) {
        level = 0;
    } else if (fbm3 + d_light < fbm1 * 1.0) {
        level = 1;
    } else if (fbm4 + d_light < fbm1 * 1.5) {
        level = 2;
    } else {
        level = 3;
    }
    let col = u_colors[level];

    return vec4f(col.rgb, a * col.a);
}

