# Decisions

Append-only log of significant design decisions.
Each entry: D-NNN, with Decided and Recorded dates (ISO 8601), status, context,
options, decision, consequences, and reversal conditions.
Status vocabulary: Proposed | Accepted | Superseded by D-NNN | Deprecated.

### D-001 Composited 2.5D layers, not modelled 3D
**Decided:** 2026-05-03 (from docs/flowstate_render_scene.md §1)
**Recorded:** 2026-06-16
**Status:** Accepted
**Authors:** Shane Hartley, Claude (web)
**Related:** F-001, F-010, ARCHITECTURE.md

**Context.** Environments must evolve organically over multi-hour sessions and
must not *demand* attention. Two rendering approaches were considered.

**Options.**
- **A. Modelled 3D scenes** (rooms, placed lights, meshes). Rejected: needs art
  assets that either loop (feels dead) or require enormous volume; heavy to
  author; hard to evolve; and photorealism *competes* with the work.
- **B. Composited 2.5D layers — a living painting with depth.** Chosen.

**Decision.** Render each environment as a back-to-front stack of impressionistic
layers, composited with per-layer depth-of-field. Prefer atmosphere over detail,
light over geometry, suggestion over representation.

**Consequences.** The impressionism is the correct aesthetic (a scaffold the
imagination fills in), and it is cheap to author and evolve. Drives the entire
`flowstate-visual` design.

**Reversal conditions.** Revisit only if user testing shows the impressionistic
style fails to satisfy the simulation engine — a product-thesis-level change.

### D-002 Environments are declarative RON data, not code
**Decided:** 2026-05-03 (from docs/flowstate_render_scene.md §7)
**Recorded:** 2026-06-16
**Status:** Accepted
**Authors:** Shane Hartley, Claude (web)
**Related:** F-002, F-007

**Context.** Twelve launch environments, "two new per year" post-launch, and
sub-5KB shareable community environments are all goals.

**Options.**
- **A. Each environment is a Rust module.** Rejected: new environments need code
  changes and can't be shared as small files.
- **B. The renderer is general; each environment is a `.ron` file.** Chosen.

**Decision.** An environment is a data file (`environments/*.ron`) describing the
layer stack, grade, post chain, and audio patch. Custom environments export as
`.flowenv` JSON.

**Consequences.** New environments are cheap; sharing is a small file; the
renderer must be fully data-driven. Validated by the `rainy_library.ron` parse test.

**Reversal conditions.** Revisit if an environment genuinely needs bespoke
rendering code that can't be expressed as a module + parameters.

### D-003 Share rendering infrastructure and the module trait with Synaesthesia
**Decided:** 2026-05-02 (from docs/FlowState.md §13)
**Recorded:** 2026-06-16
**Status:** Accepted
**Authors:** Shane Hartley, Claude (web)
**Related:** F-010, ARCHITECTURE.md, AV-001

**Context.** Galene and Synaesthesia both render the same visual modules; only
the parameter source differs (audio analysis vs evolution/richness).

**Options.**
- **A. Independent renderers.** Rejected: duplicates the wgpu pipeline, the
  module shaders, and the post effects.
- **B. Shared module trait + pipeline; Galene adds compositing, DOF, driver,
  and the new primitives on top.** Chosen.

**Decision.** Keep the visual-module trait identical between the two products so
modules are written once. Galene's `EnvironmentDriver` replaces Synaesthesia's
`MappingGraph` as the parameter source.

**Consequences.** Less code, shared improvements — but introduces a cross-repo
contract and four open questions (render-doc §12) that **block the GPU work**:
module-crate arrangement, offscreen vs swapchain, bloom reuse, grade representation.

**Reversal conditions.** Revisit if the Synaesthesia coupling proves more costly
to coordinate than maintaining a separate renderer would be.

### D-004 Distinct `Scene` (rendering) vs `Environment` (product) types
**Decided:** 2026-06-16
**Recorded:** 2026-06-16
**Status:** Accepted
**Authors:** Claude, Shane Hartley
**Related:** D-002, ARCHITECTURE.md

**Context.** The render doc's RON root is named `Environment(...)`, but
`flowstate-core` already has an `Environment` struct (work/break states, audio
config, session metadata). They are different concepts sharing a name.

**Options.**
- **A. One `Environment` type spanning product + rendering.** Rejected: conflates
  the product model with the layer stack; bloats core with rendering concerns.
- **B. `flowstate_visual::Scene` for rendering, `flowstate_core::Environment` for
  the product model, linked by name.** Chosen.

**Decision.** The `.ron` files define `Scene`; `core::Environment::visual.scene`
holds the scene file name. The scaffold's RON uses `Scene(` accordingly.

**Consequences.** Clear separation of concerns. Mild divergence from the design
doc's `Environment(` token in the RON example (cosmetic; RON ignores the name).

**Reversal conditions.** Revisit if the indirection proves more confusing than a
single merged type.

### D-005 Defer heavy dependencies until their phase
**Decided:** 2026-06-16
**Recorded:** 2026-06-16
**Status:** Accepted
**Authors:** Claude, Shane Hartley
**Related:** F-010, BUILD.md, D-003

**Context.** wgpu, Tauri, rusqlite, and Nyx are all needed eventually, but adding
them now would either fail offline builds or commit to designs that depend on the
unresolved render-doc §12 questions.

**Options.**
- **A. Add all intended dependencies up front.** Rejected: heavy, may not build
  offline, premature given open questions.
- **B. Scaffold with `serde` + `ron` only; add each heavy dep at its phase.** Chosen.

**Decision.** Keep the workspace building offline with minimal deps. The root
`Cargo.toml` documents the deferred set and its triggers.

**Consequences.** Fast, reliable builds now; GPU/audio/persistence work gated on
the relevant phase. Stubs carry `TODO(phase-N)` markers.

**Reversal conditions.** Add a dependency as soon as its phase begins and (for
wgpu) the §12 questions are resolved.

### D-006 Dual-license MIT OR Apache-2.0
**Decided:** 2026-06-16
**Recorded:** 2026-06-16
**Status:** Accepted
**Authors:** Shane Hartley

**Context.** A license is required before first public commit; the Rust ecosystem
convention is dual MIT/Apache-2.0.

**Options.**
- **A. MIT only.** **B. Apache-2.0 only.** **C. Dual MIT OR Apache-2.0** — chosen
  (matches the Cargo metadata and ecosystem norm; Apache adds a patent grant,
  MIT maximises compatibility).

**Decision.** Dual-license under MIT OR Apache-2.0; ship `LICENSE-MIT` and
`LICENSE-APACHE`.

**Consequences.** Standard, permissive, contributor-friendly.

**Reversal conditions.** Revisit only if a commercial-licensing or copyleft
strategy is later required (would affect all contributions retroactively).

### D-007 The design docs serve as SPEC; no separate SPEC.md
**Decided:** 2026-06-16
**Recorded:** 2026-06-16
**Status:** Accepted
**Authors:** Claude, Shane Hartley
**Related:** ARCHITECTURE.md, README.md

**Context.** The documentation standard recommends a `SPEC.md` (Tier 2). Galene
already has two authoritative technical specifications:
`docs/FlowState.md` (richness curve, session model, the twelve environments) and
`docs/flowstate_render_scene.md` (compositing pipeline, primitives, scene format).

**Options.**
- **A. Create a `SPEC.md`.** Rejected: would duplicate the design docs and risk
  divergence (violates "one source of truth per fact").
- **B. Treat the two `docs/` files as the project's domain spec.** Chosen.

**Decision.** No separate `SPEC.md`. README and ARCHITECTURE point to the `docs/`
files as the authoritative spec. (Tier 2 exemption recorded per the standard's
cost-note friction test.)

**Consequences.** Single source of truth. If a concise implementation-constants
reference is later wanted, add `SPEC.md` then.

**Reversal conditions.** Add `SPEC.md` if implementation-level constants diverge
from the prose design docs and need their own authoritative home.

### D-008 Colour grade via lift/gamma/gain curves for v1, not LUTs
**Decided:** 2026-06-16
**Recorded:** 2026-06-16
**Status:** Accepted
**Authors:** Claude (per render-doc §12.4 recommendation)
**Related:** F-010

**Context.** Per-environment colour grade is what makes the Library warm-amber and
the Arctic cold-blue from one renderer. Render-doc §12.4 leaves the representation
open.

**Options.**
- **A. 3D LUT textures.** More flexible for art direction, but needs an asset pipeline.
- **B. Lift/gamma/gain curves in-shader.** Chosen for v1 — no asset pipeline.

**Decision.** Ship `ColourGrade::LiftGammaGain` for v1; reserve a `Lut` variant
for later. The scaffold encodes this in `flowstate-visual::post::grade`.

**Consequences.** No asset pipeline needed now; grades are authored numerically in
the `.ron` files.

**Reversal conditions.** Add the LUT path if art direction needs grades that
curves cannot express.

### D-009 Project named Galene; FlowState retained as the internal working name
**Decided:** 2026-06-16
**Recorded:** 2026-06-16
**Status:** Accepted
**Authors:** Shane Hartley
**Related:** README.md, CLAUDE.md

**Context.** The project is now called **Galene** (the repository name). The
existing codebase, design documents, and file format all use the original
working name **FlowState**. A full rebrand of identifiers would ripple through
crate names, the binary, paths, the `.flowenv` format, and the design-doc
contents.

**Options.**
- **A. Full rebrand** — rename crates (`galene-*`), binary, file format, and
  rewrite the design docs. Rejected for now: high churn touching code identifiers
  and large provided artifacts, for no functional gain.
- **B. Branding only** — present the project as Galene in the process docs; keep
  `flowstate-*` crate names, the `flowstate` binary, the `.flowenv` format, and
  the design docs (`docs/FlowState.md`, `docs/flowstate_render_scene.md`) under
  the FlowState name. Chosen.
- **C. No change** — rejected; the project name has changed and the docs should
  reflect it.

**Decision.** Option B. "Galene" is the product/project name in all prose; the
`flowstate-` prefix persists as the internal/working name in code and design
docs. README and CLAUDE carry a short naming note. Design-doc section citations
read as `FlowState.md §N`.

**Consequences.** Minimal churn; code is untouched and keeps building. A standing
FlowState↔Galene naming duality contributors must be aware of (documented in
README and CLAUDE).

**Reversal conditions.** Escalate to a full rebrand (Option A) if the dual naming
causes ongoing confusion, or before a public launch where a single consistent
name across code and branding is wanted. That would be a new DECISIONS entry.

### D-010 Layer richness scaling: work/break baseline × master richness, by name
**Decided:** 2026-06-16
**Recorded:** 2026-06-16
**Status:** Accepted
**Authors:** Claude
**Related:** F-003, F-010, render-doc §6

**Context.** `resolve_layer_params` had to commit to two things the render doc
leaves open: the order of operations, and which `RichnessMapping` field scales
which parameter ("module-specific" in render-doc §6). The doc's literal
pseudocode order is `base → evolution → richness → lerp(work,break)`, which would
make the work/break lerp *overwrite* the richness-scaled value — so the user's
richness dial would not affect any parameter that has authored work/break states
(e.g. `rain_density`). That contradicts F-003 and the Phase-0 milestone ("the
richness dial adjusts the rain").

**Options.**
- **A. Follow the doc's literal order.** Rejected: the dial cannot move authored
  params; the flagship "adjust the rain" demo would not work.
- **B. Reorder so work/break is the per-key baseline and richness is a master
  multiplier, with the mapping field chosen by parameter name.** Chosen.

**Decision.** Per key: baseline = work/break lerp if authored, else `base`; add
evolution drift; multiply by a richness factor selected by name
(`richness_scale_for` — density/particle/rain → `particle_density`,
intensity/glow/bloom → `effect_intensity`, motion/speed → `motion_amount`,
saturation/colour → `colour_saturation`, detail → `visual_detail`; everything
else unscaled). Result clamped to 0..1.

**Consequences.** The dial visibly scales authored params (rain, particles,
light). Structural params (darkness, warmth, storm visibility) are deliberately
left unscaled. The name heuristic is provisional and centralised in one function.

**Reversal conditions.** Replace the name heuristic with an explicit per-param
scaling declaration in the scene format if a module's parameters don't fit the
name buckets, or if the 0..1 clamp clips a legitimately-HDR parameter.
