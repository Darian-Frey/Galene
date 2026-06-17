// Bloom bright-pass: keep the portion of each pixel above the threshold
// (render doc §5.3). Paired with FULLSCREEN_VS (modules/mod.rs). The result is
// blurred and added back in the post combine.

@group(0) @binding(0) var src: texture_2d<f32>;
@group(0) @binding(1) var smp: sampler;
struct BU { params: vec4<f32> };   // x = threshold
@group(0) @binding(2) var<uniform> u: BU;

@fragment
fn fs(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let c = textureSample(src, smp, uv).rgb;
    let l = dot(c, vec3<f32>(0.2126, 0.7152, 0.0722));
    let over = max(l - u.params.x, 0.0);
    let factor = over / max(l, 1e-4);
    return vec4<f32>(c * factor, 1.0);
}
