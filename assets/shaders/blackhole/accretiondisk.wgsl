#import bevy_sprite::mesh2d_view_bindings::globals
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

#import "shaders/planet_common.wgsl"::{rand, tiled_rand, fbm, spherify, rotate, dither, pixelize, compute_circle_mask, circle_noise}

override fn "shaders/planet_common.wgsl"::rand(coord: vec2<f32>, seed: f32) -> f32 {
    return tiled_rand(coord, seed, u_params.size, 2.);
}

struct Params {
    pixels: f32,
    rotation: f32,
    light_origin: vec2<f32>,
    time_speed: f32,
    disk_width: f32,
    ring_perspective: f32,
    should_dither: u32,
    colors: array<vec4<f32>, 5>,
    n_colors: u32,
    size: f32,
    seed: f32,
    octaves: u32,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> u_params: Params;

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let pixel_uv = pixelize(in.uv, u_params.pixels) - 0.009; // The accretion disk needs to be offset slightly to sit on top of the black hole for some reason. This was just found empirically

    // ? The source swaps the pixel and real uv here for some reason. Mistake maybe?
    let dith = dither(pixel_uv, in.uv, u_params.pixels);

    let rotated_uv = rotate(pixel_uv, u_params.rotation);

    var compressed_uv = rotated_uv;
    // compress uv along the x axis, or the accretion disk will look to stretched out
    compressed_uv.x = ((compressed_uv.x - 0.5) * 1.3) + 0.5;

    // add a bit of movement to the accretion disk by wobbling it, completely optional and can be disabled.
    var wobbled_uv = rotate(compressed_uv, sin(globals.time * u_params.time_speed * 2.) * 0.01);

    // l_origin will be used to determine how to color the pixels
    // TODO: Could introduce a constant in the common file for this
    var l_origin = vec2f(0.5);
    // d_width will be the final width of the accretion disk
    var d_width = u_params.disk_width;

    // here we distort the uvs to achieve the shape of the accretion disk
    if (wobbled_uv.y < 0.5) {
        // if we are in the top half of the image, then add to the uv.y based on how close we are to the center
        wobbled_uv.y += smoothstep(distance(vec2(0.5), wobbled_uv), 0.5, 0.2);
        // and also the ring width has to be adjusted or it will look to stretched out
        d_width += smoothstep(distance(vec2(0.5), wobbled_uv), 0.5, 0.3);

        // another optional thing that changes the color distribution, I like it, but can be disabled.
        l_origin.y -= smoothstep(distance(vec2(0.5), wobbled_uv), 0.5, 0.2);
    }
    // we don't check for exactly uv.y > 0.5 because we want a small area where the ring
    // is unaffected by stretching, the middle part that goes over the black hole.
    else if (wobbled_uv.y > 0.53) {

        // same steps as before, but uv.y and light is stretched the other way, the disk width is slightly smaller here for visual effect.
        wobbled_uv.y -= smoothstep(distance(vec2(0.5), wobbled_uv), 0.4, 0.17);
        d_width += smoothstep(distance(vec2(0.5), wobbled_uv), 0.5, 0.2);
        l_origin.y += smoothstep(distance(vec2(0.5), wobbled_uv), 0.5, 0.2);
    }

    // get distance to light origin based on unaltered uv's we saved earlier, some math to account for perspective
    let light_d = distance(rotated_uv * vec2f(1., u_params.ring_perspective), l_origin * vec2f(1., u_params.ring_perspective)) * 0.3;

    // center is used to determine ring position
    var uv_center = wobbled_uv - vec2f(0., 0.5);

    // tilt ring
    uv_center *= vec2f(1., u_params.ring_perspective);
    let center_d = distance(uv_center, vec2f(0.5, 0.));

    // cut out 2 circles of different sizes and only intersection of the 2.
    // this actually makes the disk
    var disk = smoothstep(0.1 - d_width * 2., 0.5 - d_width, center_d);
    disk *= smoothstep(center_d - d_width, center_d, 0.4);

    // rotate noise in the disk
    let uv_center_rotated = rotate(uv_center + vec2f(0., 0.5), globals.time * u_params.time_speed * 3.);

    // some noise
    disk *= pow(fbm(uv_center * u_params.size, u_params.octaves, u_params.seed), 0.5);

    // apply dithering
    if (dith && u_params.should_dither == 1) {
        disk *= 1.2;
    }

    // apply some colors based on final value
    let n_posterized = f32(u_params.n_colors - 1);
    var posterized = floor((disk + light_d) * n_posterized);
    posterized = min(posterized, n_posterized);
    var col = u_params.colors[u32(posterized)];

    // this can be toggled on to achieve a more "realistic" black hole, with red and blue shifting. This was just me messing around so can probably be more optimized and done cleaner
//    col.rgb *= 1.0 - pow(wobbled_uv.x, 1.); TODO: Cant do this in WGSL?
//    col.gb *= 1.0 - pow(wobbled_uv.x, 2.);
//    col.b *= 3.0 - pow(wobbled_uv.x, 4.);
//    col.gb *= 2.0 - pow(wobbled_uv.x, 2.);
//    col.rgb *= pow(wobbled_uv.x, 0.15);

    let disk_a = step(0.15, disk);
    if (disk_a == 0.0) {
        discard;
    }
    return vec4f(col.rgb, disk_a * col.a);
}
