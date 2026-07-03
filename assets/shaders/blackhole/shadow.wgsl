#import bevy_sprite::mesh2d_view_bindings::globals
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

#import "shaders/planet_common.wgsl"::{rand, noise, fbm, spherify, rotate, dither, pixelize, compute_circle_mask, circle_noise}


struct Params {
    pixels: f32,
    colors: array<vec4<f32>, 3>,
    radius: f32,
    light_width: f32,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> u_params: Params;


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let pixel_uv = pixelize(in.uv, u_params.pixels);

    let d_to_center = distance(pixel_uv, vec2f(0.5));

    let a = step(d_to_center, u_params.radius - 0.0001);
    if (a == 0.0) {
        discard;
    }

    var level: u32;
    if (d_to_center > u_params.radius - u_params.light_width * 0.5) {
        level = 2;
    } else if (d_to_center > u_params.radius - u_params.light_width) {
        level = 1;
    } else {
        level = 0;
    }

    return u_params.colors[level];
}
