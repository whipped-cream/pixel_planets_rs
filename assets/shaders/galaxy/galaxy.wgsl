#import bevy_sprite::mesh2d_view_bindings::globals
#import bevy_sprite::mesh2d_vertex_output::VertexOutput

#import "shaders/planet_common.wgsl"::{tiled_rand, tiled_noise, fbm, spherify, rotate, dither, pixelize, compute_circle_mask, circle_noise}

struct Params {
    pixels: f32,
    rotation: f32,
    time_speed: f32,
    dither_size: f32,
    should_dither: u32,
    colors: array<vec4<f32>, 7>,
    num_colors: u32,
    size: f32,
    seed: f32,
    octaves: u32,
    tilt: f32,
    num_layers: f32,
    layer_height: f32,
    zoom: f32,
    swirl: f32,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> u_params: Params;


@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let pixel_uv = pixelize(in.uv, u_params.pixels);

    let dith = dither(pixel_uv, in.uv, u_params.pixels);

    // I added a little zooming functionality so I dont have to mess with other values to get correct sizing.
    let zoomed_uv = (pixel_uv * u_params.zoom) - (u_params.zoom - 1.) / 2.;

    // overall rotation of galaxy
    let rotated_uv = rotate(zoomed_uv, u_params.rotation);

    // this uv is used to determine where the "layers" will be
    let tilted_uv_layers = vec2f(rotated_uv.x, (rotated_uv.y * u_params.tilt) - (u_params.tilt - 1.) / 2.);

    let d_to_center_layers = distance(tilted_uv_layers, vec2f(0.5));

    // swirl uv around the center, the further from the center the more rotated.
    let rot_layers = u_params.swirl * pow(d_to_center_layers, 0.4);
    let swirled_uv_layers = rotate(tilted_uv_layers, rot_layers + globals.time * u_params.time_speed);

    // fbm will decide where the layers are
    var fbm_val_layers = fbm(swirled_uv_layers * u_params.size, u_params.octaves, u_params.seed);
    // quantize to a few different values, so layers don't blur through each other
    fbm_val_layers = floor(fbm_val_layers * u_params.num_layers) / u_params.num_layers;

    // tilt so it looks like it's an angle.
    let tilted_uv = vec2f(rotated_uv.x, (rotated_uv.y * u_params.tilt) - (u_params.tilt - 1.) / 2. + fbm_val_layers * u_params.layer_height);
//    let tilted_uv = vec2f(tilted_uv_layers.x, tilted_uv_layers.y + f1 * u_params.layer_height); // Pretty sure this is the same as above

    // now do the same stuff as before, but for the actual galaxy image, not the layers
    let d_to_center = distance(tilted_uv, vec2f(0.5));
    let rot = u_params.swirl * pow(d_to_center, 0.4);
    let swirled_uv = rotate(tilted_uv, rot + globals.time * u_params.time_speed);

    // I offset the second fbm by some amount so the don't all use the same noise, try it wihout and the layers are very obvious
    var fbm_val = fbm(swirled_uv * u_params.size + vec2f(fbm_val_layers) * 10., u_params.octaves, u_params.seed);

    // alpha
    let a = step(fbm_val + d_to_center, 0.7);

    // some final steps to choose a nice color
    fbm_val *= 2.3;
    if (dith && u_params.should_dither == 1) { // this is || in other places?
        fbm_val *= 0.93;
    }

    fbm_val = floor(fbm_val * f32(u_params.num_colors));
    fbm_val = min(fbm_val, f32(u_params.num_colors));
    let col = u_params.colors[u32(fbm_val)];

    if (a == 0.) {
        discard;
    }
    return vec4f(col.rgb, a * col.a);
}
