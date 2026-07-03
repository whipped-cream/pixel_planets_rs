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
    cloud_cover: f32,
    cloud_curve: f32,
    time_speed: f32,
    stretch: f32,
    light_border_1: f32,
    light_border_2: f32,
    bands: f32,
    should_dither: u32,
    colors: array<vec4<f32>, 3>,
    dark_colors: array<vec4<f32>, 3>,
    num_colors: u32,
    size: f32,
    seed: f32,
    octaves: u32,
}


// Uniforms
@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> u_params: Params;


fn turbulance(uv: vec2<f32>, size: f32, time: f32, time_speed: f32, seed: f32) -> f32 {
    var c_noise = 0.0;

    let offset = vec2f(time * time_speed, 0.0);

    // more iterations for more turbulence
    for (var i = 0; i < 9; i++) {
        c_noise += circle_noise((uv * size * 0.3) + f32(i) + 11. + offset, seed);
    }

    return c_noise;
}


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let pixel_uv = pixelize(in.uv, u_params.pixels);

    let dith = dither(pixel_uv, in.uv, u_params.pixels);

    let d_light = distance(pixel_uv, u_params.light_origin);

    let circle_mask = compute_circle_mask(pixel_uv);
    if (circle_mask == 0.0) {
//        return vec4f(0.);
        discard;
    }

//    let d_to_center = distance(pixel_uv, vec2f(0.5));

    let rotated_uv = rotate(pixel_uv, u_params.rotation);

    var sphere_uv = spherify(rotated_uv);

    // a band is just one dimensional noise
    let band = fbm(vec2f(0.0, sphere_uv.y * u_params.size * u_params.bands), u_params.octaves, u_params.seed);

    // turbulence value is circles on top of each other
    let turbulance_val = turbulance(sphere_uv, u_params.size, globals.time, u_params.time_speed, u_params.seed);

    // by layering multiple noise values & combining with turbulence and bands
    // we get some dynamic looking shape
    let fbm1 = fbm(sphere_uv * u_params.size, u_params.octaves, u_params.seed);
    var fbm2 = fbm(sphere_uv * vec2f(1.0, 2.0) * u_params.size + fbm1 + vec2f(-globals.time * u_params.time_speed, 0.0) + turbulance_val, u_params.octaves, u_params.seed);

    // all of this is just increasing some contrast & applying light
    fbm2 *= pow(band, 2.0) * 7.0;
    let light = fbm2 + d_light * 1.8;
    fbm2 += pow(d_light, 1.0) - 0.3; // Raise to the first power? Leaving it cause itll be optimised out anyway
    fbm2 = smoothstep(-0.2, 4.0 - fbm2, light);

    // apply the dither value
    if (dith && u_params.should_dither == 1) {
        fbm2 *= 1.1;
    }

    // finally add colors
    let posterized = floor(fbm2 * 4.0) / 2.0;
    var col: vec4<f32>;
    if (fbm2 < 0.625) {
        col = u_params.colors[u32(posterized * f32(u_params.num_colors - 1))];
    } else {
        col = u_params.dark_colors[u32((posterized - 1.0) * f32(u_params.num_colors - 1))];
    }

    return vec4(col.rgb, col.a * circle_mask);
}
