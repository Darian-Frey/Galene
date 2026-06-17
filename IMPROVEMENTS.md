# Improvements

Catalogue of code-quality improvements, refactors, and architectural
changes proposed during development. Per Maintenance Rule 8, improvements
are logged here when noticed, not silently applied. The author decides
whether to apply, defer, or decline.

This is the dual of BUGS.md: bugs are broken; improvements work but
could be better.

Status vocabulary: suggested | applied | declined | deferred.
Effort vocabulary: trivial | small | medium | large.

## Suggested

### IMP-001: Blend effective richness across the work↔break transition
**Status:** suggested
**Found:** 2026-06-16 (while implementing the driver/richness logic)
**Location:** [flowstate-visual/src/driver.rs](flowstate-visual/src/driver.rs) `context()`, [flowstate-visual/src/layer.rs](flowstate-visual/src/layer.rs) `resolve_layer_params`
**Effort:** small
**Description.** `resolve_layer_params` derives the `RichnessMapping` from
`effective_richness(user_richness, ctx.state)`, where `state` is the discrete
target (Work or Break). So the richness mapping *steps* at the instant
`set_state` is called, while the authored work/break params blend smoothly over
the 60–90s transition via `state_blend`. The mismatch means richness-scaled
params (rain, particles, light) jump at transition start instead of easing.
**Proposal.** Pass a continuous effective richness into the resolver: lerp the
work and break effective-richness values by `state_blend`
(`lerp(user*0.5, 0.5+user*0.5, state_blend)`), and build the mapping from that.
**Trade-offs.** Adds a second, blend-aware richness path alongside the discrete
`effective_richness`; marginally more complex. The step may be visually
negligible once authored params already blend — worth confirming against the
real renderer before committing, which is why this is logged rather than applied.
**Notes.** Related to DECISIONS D-010 (richness resolution). Defer until the
renderer makes the transition visible.

<!--
### IMP-NNN: {short title}
**Status:** suggested
**Found:** YYYY-MM-DD ({session/commit context})
**Location:** {path/to/file.rs:line, or "cross-cutting"}
**Effort:** {trivial | small | medium | large}
**Description.** {What could be improved and why.}
**Proposal.** {How to do it.}
**Trade-offs.** {What we'd give up or risk. Required.}
**Notes.** {Related context, dependencies on other work.}
-->

### IMP-002: Filmic tone-map (ACES) once HDR light layers exist
**Status:** suggested
**Found:** 2026-06-16 (building the post chain, render-doc §11 step 4)
**Location:** [flowstate-visual/src/shaders/post/post.wgsl](flowstate-visual/src/shaders/post/post.wgsl)
**Effort:** small
**Description.** The post chain's tone-map is currently `clamp(c, 0, 1)`
(saturate). The render doc (§5.3) calls for ACES or Reinhard. Saturate is correct
*now* because all content is roughly LDR (the gradient + translucent placeholder
fills), but once `VolumetricLight` and additive layers push values > 1.0,
saturate will hard-clip highlights instead of rolling them off.
**Proposal.** Replace the clamp with an ACES filmic approximation (optionally an
exposure multiplier) in `post.wgsl`.
**Trade-offs.** ACES darkens mid-tones slightly, so applying it now — before any
true-HDR content exists — would dim the scene for no benefit and make the current
output look worse. It also wants the real light layers present to tune the knee.
Hence deferred, not applied.
**Notes.** Flagged inline in `post.wgsl`. Revisit with render-doc §11 steps 5–6
(real VolumetricLight). Relates to AV-001 (the post chain is in the frame budget).

### IMP-003: GlassRain — cheaper gradient + break the static-drop lattice
**Status:** suggested
**Found:** 2026-06-17 (the rain rewrite)
**Location:** [flowstate-visual/src/shaders/glass_rain.wgsl](flowstate-visual/src/shaders/glass_rain.wgsl)
**Effort:** medium
**Description.** The refraction normal is the gradient of the drop field,
computed by evaluating `drops()` three times per pixel (~27 drop-layer
evals/pixel) — the dominant cost and a 60fps risk (AV-001), especially with
GlassRain reused across five environments. (The static-drop *lattice* sub-item is
now addressed — the layer is gated/sparser — so only the perf item remains.)
**Proposal.** Evaluate `drops()` once and derive the normal more cheaply — an
analytic gradient, or render the drop height-field to a small offscreen target
once per frame and sample it (also lets bloom/DOF reuse it).
**Trade-offs.** An analytic gradient is more shader algebra and easy to get
subtly wrong; the height-field-to-texture route adds a pass + target.
**Notes.** Relates to AV-001 (frame budget). The 3× evaluation mirrors the
reference shader, which is also heavy; worth profiling on the ThinkPad P15 first.

## Applied

_None yet._

## Declined

_None yet._

## Deferred

_None yet._
