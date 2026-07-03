virtual fn rand(coord: vec2<f32>, seed: f32) -> f32 {
    // Using a virtual function here allows shaders to decide whether they want to tile or not by only overriding one function
    // Do NOT put this comment above this function: See https://github.com/bevyengine/naga_oil/issues/105
    return base_rand(coord, seed);
}

fn tiled_rand(coord: vec2<f32>, seed: f32, size: f32, multiple: f32) -> f32 {
    // land has to be tiled (or the contintents on this planet have to be changing very fast)
    // tiling only works for integer values, thus the rounding
    // it would probably be better to only allow integer sizes
    // multiply by vec2(2,1) to simulate planet having another side
    let resized = mod_floor_vec2_2(coord, (vec2f(multiple, 1.) * round(size)));
    return base_rand(resized, seed);
}

fn base_rand(coord: vec2<f32>, seed: f32) -> f32 {
    return fract(sin(dot(coord.xy, vec2f(12.9898,78.233))) * 15.5453 * seed);
}

//fn tiled_noise(coord: vec2<f32>, seed: f32, size: f32, multiple: f32) -> f32 {
//    let i = floor(coord);
//    let f = fract(coord);
//
//    let a = tiled_rand(i, seed, size, multiple);
//    let b = tiled_rand(i + vec2f(1., 0.), seed, size, multiple);
//    let c = tiled_rand(i + vec2f(0., 1.), seed, size, multiple);
//    let d = tiled_rand(i + vec2f(1., 1.), seed, size, multiple);
//
//    let cubic = f * f * (3. - 2. * f);
//
//    return mix(a, b, cubic.x) + (c - a) * cubic.y * (1. - cubic.x) + (d - b) * cubic.x * cubic.y;
//}
//
//fn tiled_fbm(coord: vec2<f32>, octaves: u32, seed: f32, size: f32, multiple: f32) -> f32 {
//    var value = 0.;
//    var scale = 0.5;
//    var freq = 1.;
//
//    for (var i: u32 = 0u; i < octaves; i++) {
//        value += tiled_noise(coord * freq, seed, size, multiple) * scale;
//        freq *= 2.;
//        scale *= 0.5;
//    }
//    return value;
//}

fn noise(coord: vec2<f32>, seed: f32) -> f32 {
    let i = floor(coord);
    let f = fract(coord);

    let a = rand(i, seed);
    let b = rand(i + vec2f(1., 0.), seed);
    let c = rand(i + vec2f(0., 1.), seed);
    let d = rand(i + vec2f(1., 1.), seed);

    let cubic = f * f * (3. - 2. * f);

    return mix(a, b, cubic.x) + (c - a) * cubic.y * (1. - cubic.x) + (d - b) * cubic.x * cubic.y;
}

fn fbm(coord: vec2<f32>, octaves: u32, seed: f32) -> f32 {
    var value = 0.;
    var scale = 0.5;
    var freq = 1.;

    for (var i: u32 = 0u; i < octaves; i++) {
        value += noise(coord * freq, seed) * scale;
        freq *= 2.;
        scale *= 0.5;
    }
    return value;
}

fn spherify(uv: vec2<f32>) -> vec2<f32> {
    let centered = uv * 2. - 1.;
    let z = sqrt(1. - dot(centered.xy, centered.xy));
    let sphere = centered / (z + 1.);

    return sphere * 0.5 + 0.5;
}

fn rotate(coord: vec2<f32>, angle: f32) -> vec2<f32> {
    return (
        (coord - 0.5) * mat2x2(
            cos(angle), -sin(angle), sin(angle), cos(angle)
        )
    ) + 0.5;
}

fn dither(uv_pixel: vec2<f32>, uv_real: vec2<f32>, pixels: f32) -> bool {
    return mod_floor_f32((uv_pixel.x + uv_real.y), (2. / pixels)) <= 1. / pixels;
}

fn pixelize(uv_real: vec2<f32>, pixels: f32) -> vec2<f32> {
    // If we are rendering to a sphere, we can get nice round borders by rendering as if the mesh was slightly larger
    // This is done at minimal cost as we are rendering to a circle mesh
    return floor((uv_real + 0.03) * pixels) * 0.97 / pixels;
}

fn compute_circle_mask(pixel_uv: vec2<f32>) -> f32 {
    // stepping over 0.5 instead of 0.49999 makes some pixels a little buggy
    return step(distance(pixel_uv, vec2f(0.5)), 0.49999);
}

// by Leukbaars from https://www.shadertoy.com/view/4tK3zR
// TODO: This is licensed under CC-BY-SA 3.0 originally. Should probably publish this bit under CC-BY-SA
fn circle_noise(uv: vec2<f32>, seed: f32) -> f32 { // TODO: Whether or not this should be tiled
    var uv_mut = uv;
    uv_mut.x += floor(uv_mut.y) * 0.31;
    let f = fract(uv_mut);
    let h = rand(floor(uv_mut), seed);
    let m = length(f - 0.25 - (h / 2.));
    let r = h * 0.25;
    return smoothstep(0.0, r, m * 0.75);
}

fn posterize(val: f32, num_colors: u32) -> u32 {
    let buckets = f32(num_colors - 1u);
    let quantized = min(floor(val * buckets) / buckets, 1.0);
    return u32(quantized * buckets);
}

// Just found out % isnt mod. Thats annoying lol
fn mod_floor_f32(val: f32, modulus: f32) -> f32 {
    return val - modulus * floor(val / modulus);
}
fn mod_floor_vec2(val: vec2<f32>, modulus: f32) -> vec2<f32> {
    return val - modulus * floor(val / modulus);
}
fn mod_floor_vec2_2(val: vec2<f32>, modulus: vec2<f32>) -> vec2<f32> {
    return vec2f(mod_floor_f32(val.x, modulus.x), mod_floor_f32(val.y, modulus.y));
}