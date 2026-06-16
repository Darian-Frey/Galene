//! Per-layer depth-of-field blur (render doc §3, step 4).
//!
//! A separable Gaussian blur whose radius is proportional to a layer's
//! `depth_blur`. This depth separation does most of the "this is a real place"
//! work and must not be skipped. Heavily-blurred layers can render at reduced
//! resolution (`Layer::resolution_scale`) — the biggest performance lever (§10).

// TODO(phase-0, build step 2): separable Gaussian blur pass in shaders/dof_blur.wgsl.
