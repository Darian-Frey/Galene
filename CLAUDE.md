# CLAUDE.md

## Project

Galene is a sensory-immersion focus tool for hyperphantasic minds: a Rust
workspace (core / audio / visual / Tauri app) that renders slowly-evolving
ambient "environments" to feed the imagination just enough that the working mind
can focus. See [README.md](README.md); the authoritative design lives in
[docs/FlowState.md](docs/FlowState.md) and [docs/flowstate_render_scene.md](docs/flowstate_render_scene.md).

**Naming:** the project is **Galene**; the crates (`flowstate-*`), binary,
`.flowenv` format, and design docs keep the original working name **FlowState**.
Use "Galene" for the product in prose; leave `flowstate-` identifiers as-is
(D-009).

## Current state

- `flowstate-core`: **implemented + tested** — `RichnessMapping::from_richness`,
  `effective_richness` (work 0–0.5 / break 0.5–1.0), `EvolutionState`. Types for
  `Environment`, `SessionType`/`FocusSession`, `SessionRecord`/`AnalyticsStore`,
  RON (de)serialisation. `FocusSession::tick` interval logic for all four session
  types (Pomodoro / Custom / Free Flow / Deep Work, the last emitting
  `SessionEvent::Completed`) and `EvolutionConfig::active_events` (`Always` only)
  implemented + tested. The insights engine and SQLite persistence remain
  stubbed (`TODO`).
- `flowstate-visual`: scene model + RON loader, `EnvironmentDriver` (advance +
  transition blend), `resolve_layer_params` richness scaling (D-010) — all tested.
  **Renderer is live (greenfield, D-011):** the canonical `VisualModule` trait
  (`ModuleInit`/`FrameCtx`), a headless `GpuContext` (wgpu 29), the `ShaderCanvas`
  module, the offscreen `Compositor` (multi-layer, blend modes), per-layer
  `DofBlur`, and the scene→GPU wiring (`build_module` factory + `SceneRenderer`
  driven by the `EnvironmentDriver`) — render-doc §11 steps 1–3, verified by
  headless readback + unit tests and the `render_frame` / `dof_layers` /
  `scene_render` examples. Non-ShaderCanvas layers render as tinted
  `PlaceholderModule` fills for now. **Still stubbed:** `post/*`, the real
  GlassRain/VolumetricLight/GeometricField/… modules, and the windowed surface loop.
- `flowstate-audio`: richness→patch-parameter mapping implemented; records
  params into a map. **Nyx synthesis not wired.**
- `flowstate-app`: runs a **headless logic demo** — loads the Rainy Library
  scene, prints the richness-dial→rain table, then simulates a 25/5 Pomodoro
  driving `FocusSession` + `EnvironmentDriver` together (transitions, blend,
  evolution) and records the session into `AnalyticsStore`. **Tauri shell + TS
  frontend not started.**

## Active task

Phase 0 GPU work (see [ROADMAP.md](ROADMAP.md)), render-doc §11 build order:
design the canonical `VisualModule` render-into-target trait → wgpu device/surface
setup → compositor with one trivial layer → per-layer DOF → post chain → Rainy
Library + GlassRain/VolumetricLight.

The render-doc §12 questions are **resolved** (D-011); steps 1–3 (compositor,
per-layer DOF, scene→GPU wiring) are done — the Rainy Library scene renders and
the dial drives it. **Next:** render-doc §11 step 4 — the post chain. This
introduces an HDR (RGBA16F) accumulation target (layers currently composite into
the LDR output), then bloom → colour grade (the scene's `LiftGammaGain`, D-008) →
vignette → film grain → tone-map. Then steps 5–6: the real GlassRain /
VolumetricLight / GeometricField modules. Features: F-010, F-001, F-003, F-009.

## Invariants

- Work-state richness must never exceed break-state richness (AV-003).
- 60fps on target hardware is non-negotiable; simplify the environment rather
  than drop frames (AV-001).
- Environments must never audibly/visibly loop over a multi-hour session (AV-004).
- No wall-clock or RNG inside the renderer's per-frame path — seeds come from the
  host (so frames stay reproducible; grain/flicker are host-seeded).
- Environments are **data, not code**: a new environment is a `.ron` file, not a
  new module (D-002).
- The visual module trait stays identical to Synaesthesia's so modules are
  written once (D-003).

## Build & test

```bash
cargo build
cargo test
cargo clippy --workspace
cargo run -p flowstate-app
```

## Conventions

- British spelling in prose and identifiers where the design docs use it
  (`colour`, `serialiser`).
- Each source file opens with a doc comment citing the relevant design-doc
  section. `TODO(phase-N)` / `TODO(nyx)` mark deferred work.
- Commit messages end with the `Co-Authored-By: Claude` trailer.
- Stable doc IDs: F- (features), D- (decisions), AV- (attack vectors),
  BUG-, IMP-. Append-only.

## Pitfalls

- See [ATTACK_VECTORS.md](ATTACK_VECTORS.md) for the canonical failure-mode list.
- Naming: the *rendering* scene type is `flowstate_visual::Scene`; the
  *product* type is `flowstate_core::Environment`. They are distinct (D-004) —
  don't conflate them. `core::Environment::visual.scene` names the `.ron` file.
- When extracting the shared visual-module crate, do **not** name it `nyx-vis` —
  that name is reserved for the audio→visual data bridge (D-011). Use a distinct
  name (`vis-modules`, `nyx-modules`, …).
- `Cargo.lock` is committed (workspace has a binary).

## Out of scope (do not change without asking)

- The 2.5D-layers-not-3D rendering decision (D-001) — settled.
- Adding Tauri, rusqlite, or Nyx ahead of their phase (D-005) — keep the
  workspace building offline until then. (wgpu is now **in-phase** — render-doc
  §12 resolved, D-011 — and may be added for the renderer.)
- **Log, don't silently fix:** per Maintenance Rule 8, when you spot a bug or
  improvement while doing something else, add it to [BUGS.md](BUGS.md) /
  [IMPROVEMENTS.md](IMPROVEMENTS.md) and let the user decide — don't fix inline.
