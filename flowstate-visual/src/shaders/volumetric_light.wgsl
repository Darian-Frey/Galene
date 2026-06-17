// VolumetricLight (render doc §5.2): soft additive radial glows for lamps,
// fires, station lights. Composited additively, so this just emits the glow
// colour. Paired with FULLSCREEN_VS (modules/mod.rs).

struct Src {
    pos_radius: vec4<f32>,  // xy = screen-space position, z = radius
    colour: vec4<f32>,      // rgb (HDR), a unused
};
struct U {
    head: vec4<f32>,        // x = source count, y = intensity, z = flicker, w = aspect
    srcs: array<Src, 4>,
};
@group(0) @binding(0) var<uniform> u: U;

@fragment
fn fs(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    var c = vec3<f32>(0.0);
    let n = i32(u.head.x);
    for (var i = 0; i < n; i = i + 1) {
        let s = u.srcs[i];
        let delta = (uv - s.pos_radius.xy) * vec2<f32>(u.head.w, 1.0);
        let d = length(delta);
        let r = max(s.pos_radius.z, 1e-3);
        let fall = exp(-(d * d) / (r * r) * 4.0);   // soft gaussian pool
        c = c + s.colour.rgb * fall;
    }
    c = c * u.head.y;       // intensity (richness/work-break driven)
    return vec4<f32>(c, 1.0);
}
