#import bevy_sprite::mesh2d_view_bindings::globals
#import bevy_render::view::View
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

#import "shaders/planet_common.wgsl"::{rand, tiled_rand, spherify, rotate, dither, pixelize, fbm, compute_circle_mask, circle_noise}



override fn "shaders/planet_common.wgsl"::rand(coord: vec2<f32>, seed: f32) -> f32 {
    return tiled_rand(coord, seed, u_size, 2.);
}


// Uniforms
@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> u_pixels: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var<uniform> u_rotation: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var<uniform> u_cloud_cover: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(3) var<uniform> u_light_origin: vec2<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(4) var<uniform> u_time_speed: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(5) var<uniform> u_stretch: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(6) var<uniform> u_cloud_curve: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(7) var<uniform> u_light_border_1: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(8) var<uniform> u_light_border_2: f32;

@group(#{MATERIAL_BIND_GROUP}) @binding(9) var<uniform> u_colors: array<vec4<f32>, 4>;

@group(#{MATERIAL_BIND_GROUP}) @binding(10) var<uniform> u_size: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(11) var<uniform> u_seed: f32;
@group(#{MATERIAL_BIND_GROUP}) @binding(12) var<uniform> u_octaves: u32;

fn cloud_alpha(uv: vec2<f32>, size: f32, time: f32, time_speed: f32, octaves: u32, seed: f32) -> f32 {
    var c_noise = 0.0;

    let offset = vec2f(time * time_speed, 0.0);

    // more iterations for more turbulence
    for (var i = 0; i < 9; i++) {
        c_noise += circle_noise((uv * size * 0.3) + f32(i) + 11. + offset, seed);
    }

    let fbm_val = fbm(uv * size + c_noise + offset, octaves, seed);

    return fbm_val; // step(a_cutoff, fbm); ?
}


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let pixel_uv = pixelize(in.uv, u_pixels);

    let d_light = distance(pixel_uv, u_light_origin);

    let circle_mask = compute_circle_mask(pixel_uv);
    if (circle_mask == 0.0) {
//        return vec4f(0.);
        discard;
    }

    let d_to_center = distance(pixel_uv, vec2f(0.5));

    let rotated_uv = rotate(pixel_uv, u_rotation);

    var sphere_uv = spherify(rotated_uv);

    // slightly make uv go down on the right, and up in the left
    sphere_uv.y += smoothstep(0.0, u_cloud_curve, abs(sphere_uv.x - 0.4));

    var c = cloud_alpha(sphere_uv * vec2f(1.0, u_stretch), u_size, globals.time, u_time_speed, u_octaves, u_seed);
    let cloud_mask = step(u_cloud_cover, c);
    if (cloud_mask == 0.0) {
//        return vec4f(0.);
        discard;
    }

    var level: u32;
    if (d_light + c * 0.2 > u_light_border_2) {
        level = 3;
    } else if (d_light + c * 0.2 > u_light_border_1) {
        level = 2;
    } else if (c < u_cloud_cover + 0.03) {
        level = 1;
    } else {
        level = 0;
    }
    let col = u_colors[level];

    return vec4f(col.rgb, col.a);
}
