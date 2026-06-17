// Post combine (render doc §3 step 7, §5.3): bloom add → colour grade →
// vignette → film grain → tone-map. Reads the HDR accumulation and the blurred
// bloom; writes the final image. Paired with FULLSCREEN_VS (modules/mod.rs).

@group(0) @binding(0) var accum: texture_2d<f32>;
@group(0) @binding(1) var smp: sampler;
@group(0) @binding(2) var bloom: texture_2d<f32>;

struct PostU {
    lift: vec4<f32>,
    gamma: vec4<f32>,
    gain: vec4<f32>,
    p0: vec4<f32>,   // bloom_intensity, vignette_amount, vignette_softness, grain_amount
    p1: vec4<f32>,   // seed, aspect, _, _
};
@group(0) @binding(3) var<uniform> u: PostU;

fn hash21(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(127.1, 311.7))) * 43758.5453);
}

@fragment
fn fs(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    var c = textureSample(accum, smp, uv).rgb;

    // Bloom.
    let b = textureSample(bloom, smp, uv).rgb;
    c = c + b * u.p0.x;

    // Colour grade (lift / gamma / gain) — the per-environment look.
    c = c * u.gain.rgb + u.lift.rgb;
    c = pow(max(c, vec3<f32>(0.0)), vec3<f32>(1.0) / max(u.gamma.rgb, vec3<f32>(1e-3)));

    // Vignette.
    let d = length((uv - vec2<f32>(0.5)) * vec2<f32>(u.p1.y, 1.0));
    let vig = 1.0 - u.p0.y * smoothstep(u.p0.z, 1.0, d * 1.3);
    c = c * vig;

    // Film grain (host-seeded — reproducible per frame, AV-006).
    let n = hash21(uv * 1024.0 + vec2<f32>(u.p1.x, u.p1.x * 1.7)) - 0.5;
    c = c + vec3<f32>(n * u.p0.w);

    // Tone-map: saturate for now. A filmic curve (ACES) lands once HDR light
    // layers (VolumetricLight) actually push values > 1 — see IMPROVEMENTS.
    c = clamp(c, vec3<f32>(0.0), vec3<f32>(1.0));
    return vec4<f32>(c, 1.0);
}
