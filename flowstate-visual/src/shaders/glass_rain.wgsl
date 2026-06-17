// GlassRain (render doc §5.1): screen-space refraction with running droplets on
// an implied pane of glass. Reads the composited-so-far backdrop, offsets the
// sample UV per droplet (refraction), and adds a frosted haze + faint droplet
// highlights. Paired with FULLSCREEN_VS (modules/mod.rs).

struct U {
    params: vec4<f32>,  // rain_density, refraction_strength, glass_fog, time
};
@group(0) @binding(0) var backdrop: texture_2d<f32>;
@group(0) @binding(1) var smp: sampler;
@group(0) @binding(2) var<uniform> u: U;

fn hash2(p: vec2<f32>) -> f32 {
    return fract(sin(dot(p, vec2<f32>(127.1, 311.7))) * 43758.5453);
}

fn vnoise(p: vec2<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let w = f * f * (3.0 - 2.0 * f);
    let a = hash2(i);
    let b = hash2(i + vec2<f32>(1.0, 0.0));
    let c = hash2(i + vec2<f32>(0.0, 1.0));
    let d = hash2(i + vec2<f32>(1.0, 1.0));
    return mix(mix(a, b, w.x), mix(c, d, w.x), w.y);
}

@fragment
fn fs(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let density = u.params.x;
    let refr = u.params.y;
    let fog = u.params.z;
    let t = u.params.w;

    // Droplet field scrolling downward (rain running down the glass).
    let p = vec2<f32>(uv.x * 22.0, uv.y * 14.0 + t * 0.2);
    let n = vnoise(p);
    let nx = vnoise(p + vec2<f32>(0.6, 0.0));
    let ny = vnoise(p + vec2<f32>(0.0, 0.6));
    let droplet = smoothstep(0.55, 0.85, n) * density;

    // Refraction: offset the backdrop sample along the noise gradient where
    // droplets sit.
    let grad = vec2<f32>(nx - n, ny - n);
    let offset = grad * refr * 0.06 * droplet;
    var col = textureSample(backdrop, smp, uv + offset).rgb;

    // Frosted fog (stronger over droplets) and a faint highlight.
    col = mix(col, col * 0.92 + vec3<f32>(0.06), fog * (0.4 + 0.6 * droplet));
    col = col + vec3<f32>(droplet * 0.04);

    return vec4<f32>(col, 1.0);
}
