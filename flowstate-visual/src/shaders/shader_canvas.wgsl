// Fragment stage for the Shader Canvas base-atmosphere gradient (render doc §7).
// Paired with FULLSCREEN_VS (modules/mod.rs) to form a complete shader.

struct Params {
    // x = warmth, y = darkness, zw = padding
    data: vec4<f32>,
};
@group(0) @binding(0) var<uniform> p: Params;

@fragment
fn fs(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let warm = vec3<f32>(1.0, 0.55, 0.25);
    let cool = vec3<f32>(0.05, 0.07, 0.12);
    let g = uv.y;                            // warmer toward the bottom
    var col = mix(cool, warm, g * p.data.x); // warmth scales the warm mix
    col = col * (1.0 - 0.7 * p.data.y);      // darkness dims
    return vec4<f32>(col, 1.0);
}
