//! Film grain — animated per frame, subtle (0.02–0.06). Critical for the
//! painterly feel; a clean digital image looks wrong (render doc §5.3).
//! Driven by PostChain::grain_amount.

// TODO(phase-0): shaders/post/grain.wgsl, animated by a per-frame seed supplied
// by the host (no Math.random / wall-clock inside the renderer).
