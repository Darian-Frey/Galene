// Separable Gaussian blur fragment (render doc §3 step 4). Run twice per layer:
// once horizontally (src → temp), once vertically (temp → dst). Paired with
// FULLSCREEN_VS (modules/mod.rs). A 9-tap kernel; `radius` scales the tap step
// (in pixels) so heavier `depth_blur` reads as deeper defocus.

struct Blur {
    dir: vec2<f32>,   // texel step along the blur axis (1/size, 0) or (0, 1/size)
    radius: f32,      // tap spacing in pixels
    _pad: f32,
};
@group(0) @binding(0) var tex: texture_2d<f32>;
@group(0) @binding(1) var smp: sampler;
@group(0) @binding(2) var<uniform> b: Blur;

@fragment
fn fs(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    // Normalised 9-tap Gaussian weights (centre + 4 each side).
    let w = array<f32, 5>(0.227027, 0.1945946, 0.1216216, 0.054054, 0.016216);

    var col = textureSample(tex, smp, uv) * w[0];
    for (var i = 1; i < 5; i = i + 1) {
        let off = b.dir * b.radius * f32(i);
        col = col + textureSample(tex, smp, uv + off) * w[i];
        col = col + textureSample(tex, smp, uv - off) * w[i];
    }
    return col;
}
