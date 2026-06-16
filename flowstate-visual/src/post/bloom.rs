//! Bloom — extract bright (HDR > 1.0), blur, recombine. Light layers already
//! exceed 1.0 so bloom catches them (render doc §5.3).
//!
//! If Synaesthesia already has a bloom effect it should be lifted here rather
//! than rewritten — open question §12.3.

// TODO(phase-0): bloom passes (extract → separable blur → recombine) in
// shaders/post/bloom.wgsl, driven by PostChain::{bloom_threshold, bloom_intensity}.
