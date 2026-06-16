# Architecture

Descriptive view of the system as it currently is. Rationale for the structural
choices lives in [DECISIONS.md](DECISIONS.md); deep technical detail lives in the
design docs under [docs/](docs/) (the project's domain spec, per D-007).

## System overview

```
                    ┌─────────────────────────────────────────────┐
                    │                flowstate-app                 │
                    │   (Tauri shell — placeholder binary for now) │
                    │   commands/{session,environment,analytics}   │
                    └───────┬───────────────┬───────────────┬──────┘
                            │               │               │
              ┌─────────────▼──┐  ┌─────────▼────────┐  ┌────▼───────────────┐
              │ flowstate-core │  │ flowstate-visual │  │  flowstate-audio   │
              │  richness      │  │  scene + driver  │  │  richness → Nyx    │
              │  evolution     │◄─┤  compositor/DOF  │  │  patch params      │
              │  session       │  │  post chain      │  │  evolution_audio   │
              │  analytics     │  │  new primitives  │  │                    │
              │  environment   │  └──────────────────┘  └────────────────────┘
              └────────────────┘
                     ▲                      ▲                      ▲
                     │                      │                      │
              richness/evolution     scene .ron files        Nyx patches
              types are the shared    (environments/)        (deferred)
              vocabulary all three    one file per env
              crates resolve against
```

Data flow per frame (when rendering is wired): the `EnvironmentDriver` reads the
loaded `Scene`, the user richness dial, the work/break state, and the evolution
cycle position, and resolves each layer's parameters
(`base + evolution + richness + work/break`). The compositor renders each layer
to an offscreen HDR target, depth-of-field-blurs it, composites back-to-front,
and runs the post chain. The same resolved richness drives the audio engine's
Nyx patch parameters, so visual and audio stay in lockstep.

## Module responsibilities

- **flowstate-core** — the non-rendering heart. Owns the `Environment` product
  model, the `RichnessMapping` curve and work/break resolution, the
  `EvolutionState` cycle, the `FocusSession` engine, and `AnalyticsStore`. It
  defines the shared vocabulary (richness, evolution, work/break) the other
  crates resolve against, and depends on nothing internal.
- **flowstate-visual** — turns a `Scene` (loaded from a `.ron` file) into frames.
  Adds, on top of Synaesthesia's reused pipeline: the multi-target layer
  compositor, per-layer DOF blur, the `EnvironmentDriver` (replacing
  Synaesthesia's audio-driven mapping graph), the post chain, and the two new
  primitives (`GlassRain`, `VolumetricLight`). Depends on `flowstate-core`.
- **flowstate-audio** — drives the ambient soundscape. Maps the resolved richness
  onto Nyx patch parameters and applies slow per-parameter evolution drift.
  Depends on `flowstate-core`.
- **flowstate-app** — the Tauri desktop shell and command surface; owns
  `AppState` and the work-mode integration. Depends on all three library crates.

## Key invariants

- **Work richness ≤ break richness** for any dial value (AV-003).
- **No loop** over a multi-hour session — every parameter drifts on its own phase
  offset (AV-004).
- **60fps** on target hardware at all richness levels; the lever is reducing
  resolution of blurred far layers and capping additive light layers (AV-001/002).
- **Determinism in the render path** — no wall-clock/RNG inside per-frame code;
  seeds are supplied by the host.
- **Environments are data** — a new environment is a `.ron` file, not code (D-002).
- **Shared module trait** is byte-for-byte the same as Synaesthesia's (D-003).

## Cross-cutting concerns

- **Parameter resolution** is the central abstraction: `resolve_layer_params`
  (visual) and `update_richness` (audio) both consume `RichnessMapping` from core.
- **Serialisation**: RON for scene/environment data; `.flowenv` (JSON) for shared
  custom environments; SQLite for session analytics (Phase 2).
- **Error handling / concurrency**: not yet established — to be defined when the
  Tauri shell and the render loop land. (Tracked as a gap; revisit in Phase 0.)
- **Sibling contract**: the module trait, Nyx patches, and the `SensoryProfile`
  vocabulary are shared with Synaesthesia / StoryEngine. A `VOCABULARY.md` will be
  added if/when that contract needs pinning across repos (currently a render-doc
  §12 open question).
