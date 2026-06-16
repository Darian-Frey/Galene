//! Multi-target layer compositing (render doc §3) — the stage FlowState adds on
//! top of Synaesthesia's pipeline.
//!
//! Per frame: render each layer to its own RGBA16F offscreen target, apply
//! per-layer DOF blur, then composite back-to-front with each layer's blend mode
//! into an HDR accumulation target, then run the post chain.
//!
//! GlassRain is the exception: it reads the accumulation-so-far as a texture and
//! refracts it, so it composites after the layers behind it are already merged
//! (render doc §5.1).

// TODO(phase-0, build step 1): get the multi-target render → composite → present
// loop working with a single Shader Canvas gradient layer before adding DOF, the
// driver, and the post chain. Requires the Synaesthesia wgpu setup (open
// questions §12.1–§12.2).
