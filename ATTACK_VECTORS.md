# Attack Vectors

Project-specific failure modes Galene must be resilient against.
Grouped by category. Severity: Critical (must hold) | Major (release blocker) | Minor (track only).

## Performance

### AV-001 Frame rate below 60fps
**Severity:** Critical
**Description.** Every environment, at every richness level, must hold 60fps on
target hardware (ThinkPad P15 Gen 2i / mobile RTX-class GPU). A focus environment
that stutters defeats its own purpose. The fill-rate stress case is the Geothermal
Cave (two additive light layers + bloom); the sparse stress case is Deep Space.
**Detection.** Not implemented (no renderer yet) — would require frame-time
profiling on target hardware once `flowstate-visual` is wired. Levers when it
fails: reduce resolution of blurred far layers (`resolution_scale`), cap additive
light layers, lower far-layer cycle update rate, or simplify the environment.
**Related decisions.** D-001 (2.5D layers), D-003 (shared pipeline).
**Related features.** F-010, F-001.
**History.** Identified from render-doc §10 ("60fps is non-negotiable").

### AV-002 GPU memory budget exceeded
**Severity:** Major
**Description.** All layer targets at 1080p must stay under ~200MB GPU memory.
RGBA16F targets are large; an over-detailed environment (>12 layers) or
full-resolution blurred layers blow the budget.
**Detection.** Not implemented — would require GPU memory instrumentation. Layer
count is capped at 12; blurred layers should use `resolution_scale < 1.0`.
**Related decisions.** D-001.
**Related features.** F-010.
**History.** From render-doc §10.

## Correctness

### AV-003 Work richness exceeds break richness
**Severity:** Critical
**Description.** For any user dial value, the resolved work-state richness must be
≤ break-state richness, or the work/break distinction collapses and the
environment competes with the work.
**Detection.** Implemented — `flowstate-core` test
`richness::tests::work_is_never_richer_than_break` asserts work ≤ 0.5 ≤ break
across the dial range.
**Related decisions.** —
**Related features.** F-003, F-004.
**History.** From FlowState.md §8.2.

### AV-005 Scene `.ron` fails to parse against the renderer types
**Severity:** Major
**Description.** A shipped environment whose `.ron` doesn't deserialise into
`Scene` is a broken environment. As the type model evolves, existing scene files
can silently fall out of sync.
**Detection.** Implemented (partial) — `flowstate-visual` test
`scene::tests::rainy_library_parses`. **Gap:** needs one parse test per shipped
environment as the library grows (Phase 1).
**Related decisions.** D-002, D-004.
**Related features.** F-002.
**History.** Established with the scaffold; flagged for per-environment coverage.

## Domain validity

### AV-004 Audible or visible looping over a long session
**Severity:** Critical
**Description.** The core product promise is a world that evolves and never
repeats. A perceptible loop — in the soundscape or the visuals — breaks the
illusion and re-engages the conscious mind. Each parameter must drift on its own
phase offset so the scene never returns to an identical state within a session.
**Detection.** Not implemented — would require long-run (multi-hour) inspection or
an automated state-recurrence check on the evolution outputs. The evolution
envelopes (Drift/Sine with per-parameter phase offsets) are designed to prevent
it; verification tooling does not yet exist.
**Related decisions.** D-001, D-002.
**Related features.** F-001, F-009.
**History.** From FlowState.md §12.3 and render-doc §6.

## Determinism

### AV-006 Wall-clock or RNG inside the renderer's per-frame path
**Severity:** Major
**Description.** Frames must be reproducible: grain, flicker, and any animation
seed come from the **host**, not from `std::time` or RNG called inside a module's
`render`. A module that reads the wall clock or rolls its own randomness makes
frames non-deterministic, breaks reproducibility, and risks per-frame jitter.
**Detection.** Partially structural — `FrameCtx` supplies `time_secs` and `seed`,
so modules have no reason to reach for the clock; enforced by review for now. Not
automated (would require a lint or an audit that no module imports `std::time` /
`rand` in its render path).
**Related decisions.** D-011 (the trait that carries the host-supplied frame inputs).
**Related features.** F-010.
**History.** Identified when the `VisualModule` trait / `FrameCtx` were defined
(2026-06-16); long-standing Galene invariant (see CLAUDE.md).
