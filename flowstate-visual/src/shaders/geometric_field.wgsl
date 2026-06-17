// GeometricField (render doc §4): low-detail architectural silhouettes —
// shelves, window frames, table. Impressionistic by design (most uses are
// DOF-blurred); per-preset procedural shapes. Paired with FULLSCREEN_VS.

struct U {
    params: vec4<f32>,   // density, frame_darkness, storm_visibility, lamp_on
    info: vec4<f32>,     // preset_id, aspect, _, _
};
@group(0) @binding(0) var<uniform> u: U;

// Far bookshelves: a soft dark mass with gentle low-frequency vertical
// variation. This layer is heavily DOF-blurred, so the shapes must stay
// low-frequency — high frequencies alias into a moiré at the layer's reduced
// resolution. A faint shelf banding, kept soft to avoid hard-edge aliasing.
fn bookshelf(uv: vec2<f32>, density: f32) -> vec4<f32> {
    let book = 0.6 + 0.4 * sin(uv.x * 16.0 + sin(uv.x * 4.3) * 2.0);
    let shelf = 0.85 + 0.15 * sin(uv.y * 18.0);
    let mass = density * book * shelf;
    return vec4<f32>(vec3<f32>(0.06, 0.05, 0.04), clamp(mass, 0.0, 1.0));
}

// Tall windows: three panes with dark mullions; panes glow with the cool storm.
fn tall_windows(uv: vec2<f32>, frame: f32, storm: f32) -> vec4<f32> {
    let wx = fract(uv.x * 3.0);
    let pane = step(0.14, wx) * step(wx, 0.86) * step(0.08, uv.y) * step(uv.y, 0.82);
    let sky = vec3<f32>(0.10, 0.13, 0.20) * storm;
    let frame_col = vec3<f32>(0.03, 0.03, 0.04);
    let col = mix(frame_col, sky, pane);
    let a = mix(frame, max(frame, storm * 0.85), pane);
    return vec4<f32>(col, a);
}

// Reading table: a dark foreground mass across the lower portion.
fn reading_table(uv: vec2<f32>, lamp_on: f32) -> vec4<f32> {
    let table = smoothstep(0.70, 0.74, uv.y);
    return vec4<f32>(vec3<f32>(0.05, 0.035, 0.02), table * 0.92);
}

@fragment
fn fs(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    let id = i32(u.info.x);
    if (id == 0) {
        return bookshelf(uv, u.params.x);
    } else if (id == 1) {
        return tall_windows(uv, u.params.y, u.params.z);
    } else {
        return reading_table(uv, u.params.w);
    }
}
