//! Colour grade — what makes the Library warm-amber and the Arctic cold-blue
//! from the same renderer (render doc §5.3, §7).
//!
//! v1 uses lift/gamma/gain curves (no asset pipeline). A 3D LUT variant may be
//! added later if art direction needs it — open question §12.4.

use serde::{Deserialize, Serialize};

/// Per-environment colour grade applied in the post chain.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum ColourGrade {
    /// Per-channel lift (shadows), gamma (mids), gain (highlights).
    LiftGammaGain {
        lift: (f32, f32, f32),
        gamma: (f32, f32, f32),
        gain: (f32, f32, f32),
    },
    // TODO(post-v1): Lut { handle: String } once an asset pipeline exists.
}

// TODO(phase-0): WGSL implementation in shaders/post/grade.wgsl.
