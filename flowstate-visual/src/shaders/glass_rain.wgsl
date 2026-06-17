// GlassRain (render doc §5.1): procedural rain on a pane of glass.
//
// Builds a water height-field from a dense field of static droplets, sliding
// drops + trails, and vertical runnels (after Steinrucken's "Heartfelt" and
// Lagarde's "Water drop"). The height-field gradient refracts the backdrop;
// the glass is frosted everywhere and clear (sharp, refracted) through the
// water, with a specular rim on the drop edges. Paired with FULLSCREEN_VS.

struct U {
    params: vec4<f32>,  // rain_density, refraction_strength, glass_fog, time
    info: vec4<f32>,    // aspect, texel_x, texel_y, _
};
@group(0) @binding(0) var backdrop: texture_2d<f32>;
@group(0) @binding(1) var smp: sampler;
@group(0) @binding(2) var<uniform> u: U;

fn n13(p: f32) -> vec3<f32> {
    var p3 = fract(vec3<f32>(p) * vec3<f32>(0.1031, 0.11369, 0.13787));
    p3 = p3 + dot(p3, p3.yzx + 19.19);
    return fract(vec3<f32>(
        (p3.x + p3.y) * p3.z,
        (p3.x + p3.z) * p3.y,
        (p3.y + p3.z) * p3.x,
    ));
}

fn saw(b: f32, t: f32) -> f32 {
    return smoothstep(0.0, b, t) * smoothstep(1.0, b, t);
}

// One grid of static droplets (size- and position-varied). `coverage` gates how
// many cells actually hold a drop, so the *number* of drops scales with rain —
// light rain is a sparse scatter, heavy rain covers the glass. Each drop also
// has a slow per-cell life cycle (form → linger → erode away), after Toadstorm's
// alpha-erosion timing, so the field stays alive instead of frozen.
fn static_layer(uv: vec2<f32>, scale: f32, seed: f32, coverage: f32, t: f32) -> f32 {
    let g = uv * scale + seed;
    let id = floor(g);
    let f = fract(g) - 0.5;
    let n = n13(id.x * 127.1 + id.y * 311.7 + seed * 13.3);
    let present = step(1.0 - coverage, fract(n.x * 7.0 + n.y * 3.0));
    let life_t = fract(t * 0.04 + n.z);
    let life = smoothstep(0.0, 0.12, life_t) * smoothstep(1.0, 0.6, life_t);
    let pos = (n.xy - 0.5) * 0.7;
    let size = (0.14 + 0.32 * n.z) * (0.5 + 0.5 * life);
    let d = length(f - pos);
    return smoothstep(size, size * 0.2, d) * present * life;
}

// Static droplet field — three overlapping grids so it reads as a random
// scatter rather than a regular grid; coverage scales with rain density.
fn static_drops(uv: vec2<f32>, coverage: f32, t: f32) -> f32 {
    var h = static_layer(uv, 26.0, 0.0, coverage, t);
    h = max(h, static_layer(uv, 39.0, 7.3, coverage, t));
    h = max(h, static_layer(uv, 17.0, 23.1, coverage, t));
    return h;
}

// One grid layer of sliding drops + their trails → water height.
fn drop_layer(uv_in: vec2<f32>, t: f32) -> f32 {
    let base_uv = uv_in;
    var uv = uv_in;
    uv.y = uv.y + t * 0.75;

    let a = vec2<f32>(6.0, 1.0);
    let grid = a * 2.0;
    var id = floor(uv * grid);
    uv.y = uv.y + n13(id.x * 1.7).x;
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
    let trail = smoothstep(0.23 * r, 0.15 * r * r, cd) * smoothstep(-0.02, 0.02, st.y - y) * r * r;

    return main_drop + trail * 0.5;
}

// Long vertical runnels: water running down in wobbling streaks, in a subset of
// columns, with moving blobs sliding down each streak.
fn runnels(uv: vec2<f32>, t: f32) -> f32 {
    let cols = 14.0;
    let id = floor(uv.x * cols);
    let n = n13(id * 71.3);
    if (n.x < 0.62) {
        return 0.0;
    }
    let centre = (id + 0.5 + (n.y - 0.5) * 0.6) / cols;
    let wob = sin(uv.y * 5.0 + n.z * 6.2832) * (0.6 / cols);
    let dx = abs(uv.x - centre - wob);
    let width = (0.12 / cols) * (0.6 + 0.8 * n.z);
    let streak = smoothstep(width, 0.0, dx);
    // Two staggered running blobs so the stream stays continuous (no pop on
    // reset — the flow-map "dual rest field" idea).
    let phase = uv.y * 1.5 + t * (0.25 + n.z * 0.4) + n.y * 5.0;
    let blob = max(
        smoothstep(0.4, 0.0, abs(fract(phase) - 0.5)),
        smoothstep(0.4, 0.0, abs(fract(phase * 0.6 + 0.5) - 0.5)) * 0.7,
    );
    return streak * (0.3 + 0.7 * blob);
}

// Combined water height at `uv` (used for the value and its gradient).
fn water_height(uv: vec2<f32>, t: f32) -> f32 {
    // Flip to y-up: the drop/runnel motion is authored for an upward y axis, so
    // this makes the water actually run *down* the screen.
    let p = vec2<f32>(uv.x, 1.0 - uv.y);
    let density = max(u.params.x, 0.0);
    let coverage = clamp(density * 1.2, 0.0, 1.0);  // drop count (light → heavy)
    let l_slide = smoothstep(0.0, 0.55, density);
    let l_run = smoothstep(0.25, 0.9, density);

    var h = static_drops(p, coverage, t);
    h = max(h, drop_layer(p, t) * l_slide);
    h = max(h, drop_layer(p * 1.7 + vec2<f32>(3.1, 1.7), t) * l_slide * 0.8);
    h = max(h, runnels(p, t) * l_run);
    return clamp(h, 0.0, 1.0);
}

// Frosted-glass blur (5x5, explicit LOD — safe in any control flow).
fn frost_sample(uv: vec2<f32>, step: vec2<f32>) -> vec3<f32> {
    var col = vec3<f32>(0.0);
    for (var i = -2; i <= 2; i = i + 1) {
        for (var j = -2; j <= 2; j = j + 1) {
            let o = vec2<f32>(f32(i), f32(j)) * step;
            col = col + textureSampleLevel(backdrop, smp, uv + o, 0.0).rgb;
        }
    }
    return col / 25.0;
}

@fragment
fn fs(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let strength = u.params.y;
    let fog = u.params.z;
    let t = u.params.w;
    let aspect = u.info.x;
    let texel = vec2<f32>(u.info.y, u.info.z);

    // Water height + gradient (drops are computed in aspect-corrected space).
    let uv_a = vec2<f32>(uv.x * aspect, uv.y);
    let e = 0.003;
    let h = water_height(uv_a, t);
    let hx = water_height(uv_a + vec2<f32>(e, 0.0), t);
    let hy = water_height(uv_a + vec2<f32>(0.0, e), t);
    let normal = vec2<f32>(hx - h, hy - h);

    // Refraction: a water drop is a lens — sample the backdrop displaced along
    // the surface slope (and clear, not frosted, so you see *through* the drop).
    let offset = vec2<f32>(normal.x / aspect, normal.y) * strength * 5.0;
    let sharp = textureSampleLevel(backdrop, smp, uv + offset, 0.0).rgb;

    // Frosted glass everywhere; water reveals the clear refracted view.
    let frost = frost_sample(uv, texel * (2.0 + fog * 10.0));
    let glass = mix(textureSampleLevel(backdrop, smp, uv, 0.0).rgb, frost, clamp(fog * 1.4, 0.0, 0.9));
    let wet = smoothstep(0.03, 0.4, h);
    var col = mix(glass, sharp, wet);

    // Subtle dark meniscus at the drop edge gives the water body.
    let edge = smoothstep(0.004, 0.03, length(normal));
    col = mix(col, col * 0.78, edge * wet);

    // Glassy specular glint — a small bright highlight where the curved surface
    // faces the light (upper-left), which is what reads as "wet" in the photos.
    let n3 = normalize(vec3<f32>(-normal.x / aspect, -normal.y, 0.05));
    let light = normalize(vec3<f32>(-0.4, -0.5, 0.6));
    let spec = pow(max(dot(n3, light), 0.0), 22.0);
    col = col + vec3<f32>(spec) * wet * 0.8;

    return vec4<f32>(col, 1.0);
}
