pub const SHADER_PRESETS: &[(&str, &str)] = &[
    ("Grayscale", GRAYSCALE_WGSL),
    ("Invert", INVERT_WGSL),
    ("Sepia", SEPIA_WGSL),
    ("CRT Scanlines", CRT_WGSL),
    ("Vignette", VIGNETTE_WGSL),
    ("Chromatic Aberration", CHROMATIC_ABERRATION_WGSL),
    ("Noise Grain", NOISE_WGSL),
    ("Posterize", POSTERIZE_WGSL),
    ("Edge Detection", EDGE_DETECTION_WGSL),
    ("Pixelate", PIXELATE_WGSL),
    ("VHS Glitch", VHS_GLITCH_WGSL),
];

pub fn get_shader(name: &str) -> Option<&'static str> {
    SHADER_PRESETS
        .iter()
        .find(|(n, _)| *n == name)
        .map(|(_, s)| *s)
}

pub fn get_shader_param_labels(name: &str) -> [&'static str; 4] {
    match name {
        "Grayscale" | "Invert" | "Sepia" | "Edge Detection" | "Vignette" => {
            ["Intensity", "Unused", "Unused", "Unused"]
        },
        "CRT Scanlines" => ["Darkness", "Unused", "Unused", "Unused"],
        "Chromatic Aberration" => ["Shift X", "Unused", "Unused", "Unused"],
        "Noise Grain" => ["Amount", "Unused", "Unused", "Unused"],
        "Posterize" => ["Levels", "Unused", "Unused", "Unused"],
        "Pixelate" => ["Block Size", "Unused", "Unused", "Unused"],
        "VHS Glitch" => ["Tear Width", "Unused", "Unused", "Unused"],
        _ => ["Param 1", "Param 2", "Param 3", "Param 4"],
    }
}

const GRAYSCALE_WGSL: &str = r#"
@group(0) @binding(0) var<storage, read_write> pixels: array<u32>;
@group(0) @binding(1) var<uniform> dimensions: vec2<u32>;

fn unpack_color(c: u32) -> vec4<f32> {
    return vec4<f32>(f32(c & 255u), f32((c >> 8u) & 255u), f32((c >> 16u) & 255u), f32((c >> 24u) & 255u)) / 255.0;
}
fn pack_color(v: vec4<f32>) -> u32 {
    return (u32(v.a * 255.0) << 24u) | (u32(v.b * 255.0) << 16u) | (u32(v.g * 255.0) << 8u) | u32(v.r * 255.0);
}
@group(0) @binding(2) var<uniform> params: vec4<f32>;
@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {

    if (global_id.x >= dimensions.x || global_id.y >= dimensions.y) { return; }
    let idx = global_id.y * dimensions.x + global_id.x;
    let color = unpack_color(pixels[idx]);
    let gray = dot(color.rgb, vec3<f32>(0.299, 0.587, 0.114));
    let final_color = mix(color.rgb, vec3<f32>(gray, gray, gray), clamp(params.x, 0.0, 1.0));
    pixels[idx] = pack_color(vec4<f32>(final_color, color.a));

}
"#;

const INVERT_WGSL: &str = r#"
@group(0) @binding(0) var<storage, read_write> pixels: array<u32>;
@group(0) @binding(1) var<uniform> dimensions: vec2<u32>;
fn unpack_color(c: u32) -> vec4<f32> { return vec4<f32>(f32(c & 255u), f32((c >> 8u) & 255u), f32((c >> 16u) & 255u), f32((c >> 24u) & 255u)) / 255.0; }
fn pack_color(v: vec4<f32>) -> u32 { return (u32(v.a * 255.0) << 24u) | (u32(v.b * 255.0) << 16u) | (u32(v.g * 255.0) << 8u) | u32(v.r * 255.0); }
@group(0) @binding(2) var<uniform> params: vec4<f32>;
@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {

    if (global_id.x >= dimensions.x || global_id.y >= dimensions.y) { return; }
    let idx = global_id.y * dimensions.x + global_id.x;
    let color = unpack_color(pixels[idx]);
    let inv = vec3<f32>(1.0 - color.r, 1.0 - color.g, 1.0 - color.b);
    let final_color = mix(color.rgb, inv, clamp(params.x, 0.0, 1.0));
    pixels[idx] = pack_color(vec4<f32>(final_color, color.a));

}
"#;

const SEPIA_WGSL: &str = r#"
@group(0) @binding(0) var<storage, read_write> pixels: array<u32>;
@group(0) @binding(1) var<uniform> dimensions: vec2<u32>;
fn unpack_color(c: u32) -> vec4<f32> { return vec4<f32>(f32(c & 255u), f32((c >> 8u) & 255u), f32((c >> 16u) & 255u), f32((c >> 24u) & 255u)) / 255.0; }
fn pack_color(v: vec4<f32>) -> u32 {
    return (u32(clamp(v.a * 255.0, 0.0, 255.0)) << 24u) |
           (u32(clamp(v.b * 255.0, 0.0, 255.0)) << 16u) |
           (u32(clamp(v.g * 255.0, 0.0, 255.0)) << 8u) |
           u32(clamp(v.r * 255.0, 0.0, 255.0));
}
@group(0) @binding(2) var<uniform> params: vec4<f32>;
@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {

    if (global_id.x >= dimensions.x || global_id.y >= dimensions.y) { return; }
    let idx = global_id.y * dimensions.x + global_id.x;
    let c = unpack_color(pixels[idx]);
    let tr = (c.r * 0.393) + (c.g * 0.769) + (c.b * 0.189);
    let tg = (c.r * 0.349) + (c.g * 0.686) + (c.b * 0.168);
    let tb = (c.r * 0.272) + (c.g * 0.534) + (c.b * 0.131);
    let final_color = mix(c.rgb, vec3<f32>(tr, tg, tb), clamp(params.x, 0.0, 1.0));
    pixels[idx] = pack_color(vec4<f32>(final_color, c.a));

}
"#;

const CRT_WGSL: &str = r#"
@group(0) @binding(0) var<storage, read_write> pixels: array<u32>;
@group(0) @binding(1) var<uniform> dimensions: vec2<u32>;
fn unpack_color(c: u32) -> vec4<f32> { return vec4<f32>(f32(c & 255u), f32((c >> 8u) & 255u), f32((c >> 16u) & 255u), f32((c >> 24u) & 255u)) / 255.0; }
fn pack_color(v: vec4<f32>) -> u32 {
    return (u32(clamp(v.a * 255.0, 0.0, 255.0)) << 24u) | (u32(clamp(v.b * 255.0, 0.0, 255.0)) << 16u) | (u32(clamp(v.g * 255.0, 0.0, 255.0)) << 8u) | u32(clamp(v.r * 255.0, 0.0, 255.0));
}
@group(0) @binding(2) var<uniform> params: vec4<f32>;
@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {

    if (global_id.x >= dimensions.x || global_id.y >= dimensions.y) { return; }
    let idx = global_id.y * dimensions.x + global_id.x;
    let color = unpack_color(pixels[idx]);
    var mult = 1.0;
    if (global_id.y % 3u == 0u) {
        mult = 1.0 - clamp(params.x * 0.3, 0.0, 1.0);
    }
    pixels[idx] = pack_color(vec4<f32>(color.r * mult, color.g * mult, color.b * mult, color.a));

}
"#;

const VIGNETTE_WGSL: &str = r#"
@group(0) @binding(0) var<storage, read_write> pixels: array<u32>;
@group(0) @binding(1) var<uniform> dimensions: vec2<u32>;
fn unpack_color(c: u32) -> vec4<f32> { return vec4<f32>(f32(c & 255u), f32((c >> 8u) & 255u), f32((c >> 16u) & 255u), f32((c >> 24u) & 255u)) / 255.0; }
fn pack_color(v: vec4<f32>) -> u32 { return (u32(clamp(v.a * 255.0, 0.0, 255.0)) << 24u) | (u32(clamp(v.b * 255.0, 0.0, 255.0)) << 16u) | (u32(clamp(v.g * 255.0, 0.0, 255.0)) << 8u) | u32(clamp(v.r * 255.0, 0.0, 255.0)); }
@group(0) @binding(2) var<uniform> params: vec4<f32>;
@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {

    if (global_id.x >= dimensions.x || global_id.y >= dimensions.y) { return; }
    let idx = global_id.y * dimensions.x + global_id.x;
    let color = unpack_color(pixels[idx]);
    let center = vec2<f32>(f32(dimensions.x) / 2.0, f32(dimensions.y) / 2.0);
    let dist = distance(vec2<f32>(f32(global_id.x), f32(global_id.y)), center);
    let max_dist = length(center);
    var vignette = smoothstep(max_dist, max_dist * 0.4, dist);
    vignette = mix(1.0, vignette, clamp(params.x, 0.0, 1.0));
    pixels[idx] = pack_color(vec4<f32>(color.rgb * vignette, color.a));

}
"#;

const CHROMATIC_ABERRATION_WGSL: &str = r#"
@group(0) @binding(0) var<storage, read_write> pixels: array<u32>;
@group(0) @binding(1) var<uniform> dimensions: vec2<u32>;
fn get_idx(x: u32, y: u32) -> u32 { return y * dimensions.x + x; }
fn unpack_color(c: u32) -> vec4<f32> { return vec4<f32>(f32(c & 255u), f32((c >> 8u) & 255u), f32((c >> 16u) & 255u), f32((c >> 24u) & 255u)) / 255.0; }
fn pack_color(v: vec4<f32>) -> u32 { return (u32(clamp(v.a * 255.0, 0.0, 255.0)) << 24u) | (u32(clamp(v.b * 255.0, 0.0, 255.0)) << 16u) | (u32(clamp(v.g * 255.0, 0.0, 255.0)) << 8u) | u32(clamp(v.r * 255.0, 0.0, 255.0)); }
@group(0) @binding(2) var<uniform> params: vec4<f32>;
@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {

    if (global_id.x >= dimensions.x || global_id.y >= dimensions.y) { return; }
    let idx = get_idx(global_id.x, global_id.y);
    let shift = u32(max(0.0, 5.0 * params.x));

    var r_x = global_id.x;
    if (r_x >= shift) { r_x -= shift; }

    var b_x = global_id.x + shift;
    if (b_x >= dimensions.x) { b_x = dimensions.x - 1u; }

    let color_r = unpack_color(pixels[get_idx(r_x, global_id.y)]).r;
    let color_g = unpack_color(pixels[idx]).g;
    let color_b = unpack_color(pixels[get_idx(b_x, global_id.y)]).b;
    let a = unpack_color(pixels[idx]).a;

    pixels[idx] = pack_color(vec4<f32>(color_r, color_g, color_b, a));

}
"#;

const NOISE_WGSL: &str = r#"
@group(0) @binding(0) var<storage, read_write> pixels: array<u32>;
@group(0) @binding(1) var<uniform> dimensions: vec2<u32>;
fn unpack_color(c: u32) -> vec4<f32> { return vec4<f32>(f32(c & 255u), f32((c >> 8u) & 255u), f32((c >> 16u) & 255u), f32((c >> 24u) & 255u)) / 255.0; }
fn pack_color(v: vec4<f32>) -> u32 { return (u32(clamp(v.a * 255.0, 0.0, 255.0)) << 24u) | (u32(clamp(v.b * 255.0, 0.0, 255.0)) << 16u) | (u32(clamp(v.g * 255.0, 0.0, 255.0)) << 8u) | u32(clamp(v.r * 255.0, 0.0, 255.0)); }
fn rand(co: vec2<f32>) -> f32 {
    return fract(sin(dot(co.xy ,vec2<f32>(12.9898,78.233))) * 43758.5453);
}
@group(0) @binding(2) var<uniform> params: vec4<f32>;
@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {

    if (global_id.x >= dimensions.x || global_id.y >= dimensions.y) { return; }
    let idx = global_id.y * dimensions.x + global_id.x;
    let color = unpack_color(pixels[idx]);
    let noise = (rand(vec2<f32>(f32(global_id.x), f32(global_id.y))) - 0.5) * 0.2 * params.x;
    pixels[idx] = pack_color(vec4<f32>(color.r + noise, color.g + noise, color.b + noise, color.a));

}
"#;

const POSTERIZE_WGSL: &str = r#"
@group(0) @binding(0) var<storage, read_write> pixels: array<u32>;
@group(0) @binding(1) var<uniform> dimensions: vec2<u32>;
fn unpack_color(c: u32) -> vec4<f32> { return vec4<f32>(f32(c & 255u), f32((c >> 8u) & 255u), f32((c >> 16u) & 255u), f32((c >> 24u) & 255u)) / 255.0; }
fn pack_color(v: vec4<f32>) -> u32 { return (u32(clamp(v.a * 255.0, 0.0, 255.0)) << 24u) | (u32(clamp(v.b * 255.0, 0.0, 255.0)) << 16u) | (u32(clamp(v.g * 255.0, 0.0, 255.0)) << 8u) | u32(clamp(v.r * 255.0, 0.0, 255.0)); }
@group(0) @binding(2) var<uniform> params: vec4<f32>;
@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {

    if (global_id.x >= dimensions.x || global_id.y >= dimensions.y) { return; }
    let idx = global_id.y * dimensions.x + global_id.x;
    let color = unpack_color(pixels[idx]);
    let levels = max(2.0, 20.0 - (params.x * 16.0));
    let r = floor(color.r * levels) / levels;
    let g = floor(color.g * levels) / levels;
    let b = floor(color.b * levels) / levels;
    pixels[idx] = pack_color(vec4<f32>(r, g, b, color.a));

}
"#;

const EDGE_DETECTION_WGSL: &str = r#"
@group(0) @binding(0) var<storage, read_write> pixels: array<u32>;
@group(0) @binding(1) var<uniform> dimensions: vec2<u32>;
fn get_idx(x: u32, y: u32) -> u32 { return y * dimensions.x + x; }
fn unpack_color(c: u32) -> vec4<f32> { return vec4<f32>(f32(c & 255u), f32((c >> 8u) & 255u), f32((c >> 16u) & 255u), f32((c >> 24u) & 255u)) / 255.0; }
fn pack_color(v: vec4<f32>) -> u32 { return (u32(clamp(v.a * 255.0, 0.0, 255.0)) << 24u) | (u32(clamp(v.b * 255.0, 0.0, 255.0)) << 16u) | (u32(clamp(v.g * 255.0, 0.0, 255.0)) << 8u) | u32(clamp(v.r * 255.0, 0.0, 255.0)); }
fn luma(c: vec4<f32>) -> f32 { return dot(c.rgb, vec3<f32>(0.299, 0.587, 0.114)); }
@group(0) @binding(2) var<uniform> params: vec4<f32>;
@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {

    if (global_id.x == 0u || global_id.x >= dimensions.x - 1u || global_id.y == 0u || global_id.y >= dimensions.y - 1u) { return; }

    let tl = luma(unpack_color(pixels[get_idx(global_id.x - 1u, global_id.y - 1u)]));
    let t  = luma(unpack_color(pixels[get_idx(global_id.x, global_id.y - 1u)]));
    let tr = luma(unpack_color(pixels[get_idx(global_id.x + 1u, global_id.y - 1u)]));
    let l  = luma(unpack_color(pixels[get_idx(global_id.x - 1u, global_id.y)]));
    let r  = luma(unpack_color(pixels[get_idx(global_id.x + 1u, global_id.y)]));
    let bl = luma(unpack_color(pixels[get_idx(global_id.x - 1u, global_id.y + 1u)]));
    let b  = luma(unpack_color(pixels[get_idx(global_id.x, global_id.y + 1u)]));
    let br = luma(unpack_color(pixels[get_idx(global_id.x + 1u, global_id.y + 1u)]));

    let gx = -tl - 2.0*l - bl + tr + 2.0*r + br;
    let gy = -tl - 2.0*t - tr + bl + 2.0*b + br;

    let mag = sqrt(gx*gx + gy*gy) * params.x;
    let idx = get_idx(global_id.x, global_id.y);
    let a = unpack_color(pixels[idx]).a;
    let original = unpack_color(pixels[idx]).rgb;
    let final_color = mix(original, vec3<f32>(mag, mag, mag), clamp(params.x, 0.0, 1.0));
    pixels[idx] = pack_color(vec4<f32>(final_color, a));

}
"#;

const PIXELATE_WGSL: &str = r#"
@group(0) @binding(0) var<storage, read_write> pixels: array<u32>;
@group(0) @binding(1) var<uniform> dimensions: vec2<u32>;
fn get_idx(x: u32, y: u32) -> u32 { return y * dimensions.x + x; }
fn unpack_color(c: u32) -> vec4<f32> { return vec4<f32>(f32(c & 255u), f32((c >> 8u) & 255u), f32((c >> 16u) & 255u), f32((c >> 24u) & 255u)) / 255.0; }
fn pack_color(v: vec4<f32>) -> u32 { return (u32(clamp(v.a * 255.0, 0.0, 255.0)) << 24u) | (u32(clamp(v.b * 255.0, 0.0, 255.0)) << 16u) | (u32(clamp(v.g * 255.0, 0.0, 255.0)) << 8u) | u32(clamp(v.r * 255.0, 0.0, 255.0)); }
@group(0) @binding(2) var<uniform> params: vec4<f32>;
@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {

    if (global_id.x >= dimensions.x || global_id.y >= dimensions.y) { return; }
    let block_size = max(1u, u32(params.x * 10.0));
    let bx = (global_id.x / block_size) * block_size;
    let by = (global_id.y / block_size) * block_size;

    let b_idx = get_idx(bx, by);
    let color = unpack_color(pixels[b_idx]);

    let idx = get_idx(global_id.x, global_id.y);
    pixels[idx] = pack_color(color);

}
"#;

const VHS_GLITCH_WGSL: &str = r#"
@group(0) @binding(0) var<storage, read_write> pixels: array<u32>;
@group(0) @binding(1) var<uniform> dimensions: vec2<u32>;
fn get_idx(x: u32, y: u32) -> u32 { return y * dimensions.x + x; }
fn unpack_color(c: u32) -> vec4<f32> { return vec4<f32>(f32(c & 255u), f32((c >> 8u) & 255u), f32((c >> 16u) & 255u), f32((c >> 24u) & 255u)) / 255.0; }
fn pack_color(v: vec4<f32>) -> u32 { return (u32(clamp(v.a * 255.0, 0.0, 255.0)) << 24u) | (u32(clamp(v.b * 255.0, 0.0, 255.0)) << 16u) | (u32(clamp(v.g * 255.0, 0.0, 255.0)) << 8u) | u32(clamp(v.r * 255.0, 0.0, 255.0)); }
@group(0) @binding(2) var<uniform> params: vec4<f32>;
@compute @workgroup_size(16, 16)
fn main(@builtin(global_invocation_id) global_id: vec3<u32>) {

    if (global_id.x >= dimensions.x || global_id.y >= dimensions.y) { return; }

    var offset_x = global_id.x;
    if (global_id.y % 50u > 45u) {
        let shift = u32(10.0 * params.x);
        if (offset_x + shift < dimensions.x) {
            offset_x += shift;
        }
    }

    let src_idx = get_idx(offset_x, global_id.y);
    let idx = get_idx(global_id.x, global_id.y);
    let color = unpack_color(pixels[src_idx]);

    var mult = 1.0;
    if (global_id.y % 4u == 0u) { mult = 1.0 - clamp(0.2 * params.x, 0.0, 1.0); }

    pixels[idx] = pack_color(vec4<f32>(color.r, color.g * mult, color.b, color.a));

}
"#;
