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

impl ColourGrade {
    /// A no-op grade (lift 0, gamma 1, gain 1) — used by the lower-level render
    /// paths that have no scene grade.
    pub fn identity() -> Self {
        ColourGrade::LiftGammaGain {
            lift: (0.0, 0.0, 0.0),
            gamma: (1.0, 1.0, 1.0),
            gain: (1.0, 1.0, 1.0),
        }
    }

    /// `(lift, gamma, gain)` as RGB triples, for packing into the post uniform.
    pub fn components(&self) -> ([f32; 3], [f32; 3], [f32; 3]) {
        match *self {
            ColourGrade::LiftGammaGain { lift, gamma, gain } => (
                [lift.0, lift.1, lift.2],
                [gamma.0, gamma.1, gamma.2],
                [gain.0, gain.1, gain.2],
            ),
        }
    }
}
