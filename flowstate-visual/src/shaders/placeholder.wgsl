// Placeholder fill for modules not yet implemented (GeometricField, GlassRain,
// VolumetricLight, ParticleSystem, …). A flat tint whose alpha tracks the
// layer's resolved parameters, so the full stack composites and the richness
// dial visibly drives every layer until the real module lands. Paired with
// FULLSCREEN_VS (modules/mod.rs).

struct P {
    tint: vec4<f32>,   // rgb tint, a = coverage
};
@group(0) @binding(0) var<uniform> p: P;

@fragment
fn fs(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    // Premultiplied output: works for both alpha-over and additive blending.
    return vec4<f32>(p.tint.rgb * p.tint.a, p.tint.a);
}
