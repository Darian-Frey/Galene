// GlassRain (render doc §5.1): procedural rain on a pane of glass.
//
// Technique after Steinrucken's "Heartfelt" and Lagarde's "Water drop" work
// (see IMPROVEMENTS / commit notes): drops are placed on a grid of cells with
// per-cell hashed positions; a main drop slides down each cell (sawtooth) with
// a sine-of-sine wobble and leaves a trail of static droplets; the drop
// height-field's gradient refracts the backdrop; the glass is sampled foggy
// (blurred) everywhere and clear through the drops/trails. Paired with
// FULLSCREEN_VS (modules/mod.rs).

struct U {
    params: vec4<f32>,  // rain_density, refraction_strength, glass_fog, time
    info: vec4<f32>,    // aspect, texel_x, texel_y, _
};
@group(0) @binding(0) var backdrop: texture_2d<f32>;
@group(0) @binding(1) var smp: sampler;
@group(0) @binding(2) var<uniform> u: U;

fn n1(p: f32) -> f32 {
    return fract(sin(p * 12345.564) * 7658.76);
}

fn n13(p: f32) -> vec3<f32> {
    var p3 = fract(vec3<f32>(p) * vec3<f32>(0.1031, 0.11369, 0.13787));
    p3 = p3 + dot(p3, p3.yzx + 19.19);
    return fract(vec3<f32>(
        (p3.x + p3.y) * p3.z,
        (p3.x + p3.z) * p3.y,
        (p3.y + p3.z) * p3.x,
    ));
}

// A brief triangular pulse: 0 → 1 at `b` → 0.
fn saw(b: f32, t: f32) -> f32 {
    return smoothstep(0.0, b, t) * smoothstep(1.0, b, t);
}

// Persistent specks of water clinging to the glass.
fn static_drops(uv_in: vec2<f32>, t: f32) -> f32 {
    let uv = uv_in * 40.0;
    let id = floor(uv);
    let f = fract(uv) - 0.5;
    let n = n13(id.x * 107.45 + id.y * 3543.654);
    let p = vec2<f32>(n.x, n.y) - 0.5;
    let d = length(f - p);
    return smoothstep(0.32, 0.0, d) * n.z;
}

// One grid layer of sliding drops + their trails.
// Returns (drop mask, trail mask).
fn drop_layer(uv_in: vec2<f32>, t: f32) -> vec2<f32> {
    let base_uv = uv_in;
    var uv = uv_in;
    uv.y = uv.y + t * 0.75;            // slide down

    let a = vec2<f32>(6.0, 1.0);
    let grid = a * 2.0;
    var id = floor(uv * grid);
    let col_shift = n1(id.x);
    uv.y = uv.y + col_shift;           // stagger columns
    id = floor(uv * grid);
    let n = n13(id.x * 35.2 + id.y * 2376.1);

    let st = fract(uv * grid) - vec2<f32>(0.5, 0.0);

    var x = n.x - 0.5;
    var y = base_uv.y * 20.0;
    let wiggle = sin(y + sin(y));
    x = x + wiggle * (0.5 - abs(x)) * (n.z - 0.5);
    x = x * 0.7;

    let ti = fract(t + n.z);
    y = (saw(0.85, ti) - 0.5) * 0.9 + 0.5;

    let p = vec2<f32>(x, y);
    let d = length((st - p) * a.yx);
    let main_drop = smoothstep(0.4, 0.0, d);

    let r = sqrt(smoothstep(1.0, y, st.y));
    let cd = abs(st.x - x);
    var trail = smoothstep(0.23 * r, 0.15 * r * r, cd);
    let trail_front = smoothstep(-0.02, 0.02, st.y - y);
    trail = trail * trail_front * r * r;

    // static droplets left in the trail
    let yfront = fract(base_uv.y * 10.0) + (st.y - 0.5);
    let dd = length(st - vec2<f32>(x, yfront));
    let droplets = smoothstep(0.3, 0.0, dd) * trail_front;
    let trail2 = smoothstep(0.2 * r, 0.0, cd);

    let m = main_drop + droplets * r * trail2;
    return vec2<f32>(m, trail);
}

// Combine static specks + two sliding layers at different scales.
fn drops(uv: vec2<f32>, t: f32, l0: f32, l1: f32, l2: f32) -> vec2<f32> {
    let s = static_drops(uv, t) * l0;
    let m1 = drop_layer(uv, t) * l1;
    let m2 = drop_layer(uv * 1.85, t) * l2;
    var c = s + m1.x + m2.x;
    c = smoothstep(0.3, 1.0, c);
    return vec2<f32>(c, max(m1.y * l1, m2.y * l2));
}

// 3x3 frosted-glass blur (explicit LOD — safe in any control flow).
fn frost_sample(uv: vec2<f32>, step: vec2<f32>) -> vec3<f32> {
    var col = vec3<f32>(0.0);
    for (var i = -1; i <= 1; i = i + 1) {
        for (var j = -1; j <= 1; j = j + 1) {
            let o = vec2<f32>(f32(i), f32(j)) * step;
            col = col + textureSampleLevel(backdrop, smp, uv + o, 0.0).rgb;
        }
    }
    return col / 9.0;
}

@fragment
fn fs(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let density = u.params.x;
    let strength = u.params.y;
    let fog = u.params.z;
    let t = u.params.w;
    let aspect = u.info.x;
    let texel = vec2<f32>(u.info.y, u.info.z);

    // Layer weights from rain density (sparse → busy).
    let l0 = smoothstep(0.0, 0.3, density);
    let l1 = smoothstep(0.05, 0.6, density);
    let l2 = smoothstep(0.30, 1.0, density);

    // Drops are computed in aspect-corrected space so they stay round.
    let uv_a = vec2<f32>(uv.x * aspect, uv.y);
    let e = vec2<f32>(0.0015, 0.0);
    let c = drops(uv_a, t, l0, l1, l2);
    let cx = drops(uv_a + e.xy, t, l0, l1, l2).x;
    let cy = drops(uv_a + e.yx, t, l0, l1, l2).x;
    let grad = vec2<f32>(cx - c.x, cy - c.x);

    // Refraction: displace the backdrop sample along the drop gradient (mapped
    // back out of aspect space), localized to where drops sit.
    let offset = vec2<f32>(grad.x / aspect, grad.y) * strength * 2.0;
    let sharp = textureSampleLevel(backdrop, smp, uv + offset, 0.0).rgb;

    // Foggy glass everywhere; drops and trails reveal the clear refracted view.
    let frost = frost_sample(uv, texel * (1.0 + fog * 6.0));
    let glass = mix(textureSampleLevel(backdrop, smp, uv, 0.0).rgb, frost, fog);
    let reveal = clamp(c.x + c.y * 0.5, 0.0, 1.0);
    var col = mix(glass, sharp, reveal);

    // Faint specular glint on the drops.
    col = col + vec3<f32>(c.x * 0.04);

    return vec4<f32>(col, 1.0);
}
