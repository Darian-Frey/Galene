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
  RON (de)serialisation. `FocusSession::tick` interval logic (Pomodoro / Custom /
  Free Flow) and `EvolutionConfig::active_events` (`Always` only) implemented +
  tested. Deep Work ticking, the insights engine, and SQLite persistence remain
  stubbed (`TODO`).
- `flowstate-visual`: scene model + RON loader (`Scene`, `Layer`, Drift/Sine
  evolution) implemented; `rainy_library.ron` parse-tested. `EnvironmentDriver`
  now ticks (`advance`: evolution + work↔break `state_blend` over the transition)
  and `resolve_layer_params` applies the richness-scaling table (D-010) — both
  tested. **All GPU code is stubbed** — `renderer`, `compositor`, `dof`,
  `post/*`, and the WGSL shaders are doc-only `TODO`s.
- `flowstate-audio`: richness→patch-parameter mapping implemented; records
  params into a map. **Nyx synthesis not wired.**
- `flowstate-app`: runs a **headless logic demo** — loads the Rainy Library
  scene, prints the richness-dial→rain table, then simulates a 25/5 Pomodoro
  driving `FocusSession` + `EnvironmentDriver` together (transitions, blend,
  evolution) and records the session into `AnalyticsStore`. **Tauri shell + TS
  frontend not started.**

## Active task

Phase 0 (see [ROADMAP.md](ROADMAP.md)), following render-doc §11 build order:
compositor with one trivial layer → per-layer DOF → driver+richness wired to
visible output → post chain → Rainy Library + GlassRain/VolumetricLight.
**Blocked** on the render-doc §12 open questions about the Synaesthesia pipeline
(DECISIONS D-003/D-005) — resolve those before writing wgpu code. Relevant
features: F-010, F-001, F-003, F-009.

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
- `Cargo.lock` is committed (workspace has a binary).

## Out of scope (do not change without asking)

- The 2.5D-layers-not-3D rendering decision (D-001) — settled.
- Adding heavy dependencies (wgpu, Tauri, rusqlite, Nyx) ahead of their phase
  (D-005) — keep the workspace building offline until then.
- **Log, don't silently fix:** per Maintenance Rule 8, when you spot a bug or
  improvement while doing something else, add it to [BUGS.md](BUGS.md) /
  [IMPROVEMENTS.md](IMPROVEMENTS.md) and let the user decide — don't fix inline.
