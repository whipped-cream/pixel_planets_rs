/// ***************************** ///
/// THIS IS THE DEFAULT 2D SHADER ///
/// You can always get back to this with `python3 scripts/reset-2d.py` ///
/// ***************************** ///

#import bevy_sprite::mesh2d_view_bindings::globals
#import bevy_render::view::View
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

#import "shaders/planet_common.wgsl"::{tiled_rand, tiled_noise, fbm, spherify, rotate, dither, pixelize, compute_circle_mask, circle_noise, cloud_alpha}
//#import "shaders/.wgsl"::{cloud_alpha}

@group(0) @binding(0) var<uniform> view: View;


// Uniforms
const u_pixels: f32 = 100.;
const u_cloud_cover: f32 = 0.4;
const u_light_origin: vec2<f32> = vec2f(0.39);
const u_time_speed: f32 = 0.2;
const u_stretch: f32 = 2.0;
const u_cloud_curve: f32 = 1.3;
const u_light_border_1: f32 = 0.52;
const u_light_border_2: f32 = 0.62;

const u_rotation: f32 = 0.;

const u_colors = array<vec4<f32>, 4>(
    vec4f(0.85, 0.45, 0.25, 1.0),  // deep orange
    vec4f(0.35, 0.65, 0.25, 1.0),  // leafy green
    vec4f(0.25, 0.55, 0.85, 1.0),  // ocean blue
    vec4f(0.95, 0.85, 0.35, 1.0),  // sunny yellow
);

const u_size: f32 = 50.;
const u_octaves: u32 = 4;
const u_seed: f32 = 1.;


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let pixel_uv = pixelize(in.uv, u_pixels);

    let d_light = distance(pixel_uv, u_light_origin);

    let a = step(length(pixel_uv - vec2f(0.5)), 0.49999);

    let rotated_uv = rotate(pixel_uv, u_rotation);

    var sphere_uv = spherify(rotated_uv);

    // slightly make uv go down on the right, and up in the left
    sphere_uv.y += smoothstep(0.0, u_cloud_curve, abs(sphere_uv.x - 0.4));

    let c = cloud_alpha(sphere_uv * vec2f(1., u_stretch), u_size, globals.time, u_time_speed, u_octaves, u_seed, 1.);

    // assign some colors based on cloud depth & distance from light
    var layer: u32;
    if (d_light + c * 0.2 > u_light_border_2) {
        layer = 3;
    } else if (d_light + c * 0.2 > u_light_border_1) {
        layer = 2;
    } else if (c < u_cloud_cover + 0.03) {
        layer = 1;
    } else {
        layer = 0;
    }

    let a_cc = step(u_cloud_cover, c);
    if (a_cc == 0. || a == 0.) {
        return vec4f(0.);
    }

    let col = u_colors[layer];
    return vec4f(col.rgb, a_cc * a * col.a);
}
