//! Maps the resolved richness value onto Nyx patch parameters.
//!
//! Until Nyx is wired (see crate docs), `update_richness` records the resolved
//! parameters into `params` so the mapping logic can be developed and tested in
//! isolation. The `// TODO(nyx)` lines mark where `self.nyx.set_param(...)` calls
//! will go, matching `docs/FlowState.md` §12.2.

use std::collections::HashMap;

use flowstate_core::RichnessMapping;

use crate::patches::PatchType;

/// Generates a living ambient soundscape for one environment.
#[derive(Debug, Default)]
pub struct AmbientAudioEngine {
    /// Last-resolved patch parameters. Replaced by live Nyx state once wired.
    pub params: HashMap<String, f32>,
}

impl AmbientAudioEngine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Resolve patch parameters from a richness mapping for the given patch.
    pub fn update_richness(&mut self, richness: f32, patch: PatchType) {
        let m = RichnessMapping::from_richness(richness);

        match patch {
            PatchType::InteriorRain => {
                self.set("rain_density", 0.1 + m.audio_complexity * 0.8);
                self.set("rain_volume", m.audio_volume);
                self.set("reverb_wet", 0.2 + m.reverb_presence * 0.6);
                self.set("event_rate", m.event_frequency);
            }
            PatchType::DeepSpace => {
                self.set("sub_level", m.audio_volume * 0.3);
                self.set("system_tone_rate", m.event_frequency * 0.2);
                self.set("silence_depth", 1.0 - m.audio_complexity * 0.4);
            }
            // TODO(phase-1): the remaining patches, two environments per sprint.
            _ => {
                self.set("volume", m.audio_volume);
                self.set("complexity", m.audio_complexity);
            }
        }
    }

    fn set(&mut self, key: &str, value: f32) {
        // TODO(nyx): self.nyx.set_param(key, value);
        self.params.insert(key.to_string(), value.clamp(0.0, 1.0));
    }
}
