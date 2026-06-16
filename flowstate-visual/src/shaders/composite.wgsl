// Composite fragment: sample a layer target and write it to the output.
// Paired with FULLSCREEN_VS (modules/mod.rs).
//
// For now this is a straight sample of a single layer. Multi-layer blend modes
// (render doc §3) and tone-mapping (render doc §5.3) arrive with later build steps.

@group(0) @binding(0) var src_tex: texture_2d<f32>;
@group(0) @binding(1) var src_smp: sampler;

@fragment
fn fs(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    return textureSample(src_tex, src_smp, uv);
}
