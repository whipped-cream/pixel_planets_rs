#import bevy_sprite::mesh2d_view_bindings::globals
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

#import "shaders/planet_common.wgsl"::{rand, tiled_rand, spherify, rotate, dither, pixelize, fbm, compute_circle_mask, circle_noise, posterize}



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
    ring_width: f32,
    ring_perspective: f32,
    scale: f32,
    colors: array<vec4<f32>, 3>,
    dark_colors: array<vec4<f32>, 3>,
    num_colors: u32,
    size: f32,
    seed: f32,
    octaves: u32,
}

// Uniforms
@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> u_params: Params;


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let pixel_uv = pixelize(in.uv, u_params.pixels);

    let d_light = distance(pixel_uv, u_params.light_origin);

    let rotated_uv = rotate(pixel_uv, u_params.rotation);

    let center_uv = rotated_uv - vec2f(0.0, 0.5);

    // tilt ring
    let tilted_uv = center_uv * vec2f(1.0, u_params.ring_perspective);
    let d_center = distance(tilted_uv, vec2f(0.5, 0.0));

    // cut out 2 circles of different sizes and only intersection of the 2.
    var ring = smoothstep(0.5 - u_params.ring_width * 2.0, 0.5 - u_params.ring_width, d_center);
    ring *= smoothstep(d_center - u_params.ring_width, d_center, 0.4);

    // pretend like the ring goes behind the planet by removing it if it's in the upper half.
    if (rotated_uv.y < 0.5) {
        ring *= step(1.0 / u_params.scale, distance(rotated_uv, vec2f(0.5)));
    }

    // rotate material in ring
    let rotated_center_uv = rotate(center_uv + vec2f(0.0, 0.5), globals.time * u_params.time_speed);
    // some noise
    ring *= fbm(rotated_center_uv * u_params.size, u_params.octaves, u_params.seed);

    // apply some colors based on final value
    let posterized = min(floor((ring + pow(d_light, 2.0) * 2.0) * 4.0) / 4.0, 2.0);
    var col: vec4<f32>;
    if (posterized <= 1.0) {
        col = u_params.colors[u32(posterized * f32(u_params.num_colors - 1))];
    } else {
        col = u_params.dark_colors[u32((posterized-1.0) * f32(u_params.num_colors - 1))];
    }

    let ring_a = step(0.28, ring);
    if (ring_a == 0.0) {
        discard;
    }
    return vec4f(col.rgb, ring_a * col.a);
}
