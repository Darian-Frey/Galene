# Changelog

Format follows [Keep a Changelog](https://keepachangelog.com).
References F-, D-, AV-, BUG-, and IMP- IDs for traceability.

## [Unreleased]

### Added
- Initial Cargo workspace scaffold: `flowstate-core`, `flowstate-audio`,
  `flowstate-visual`, `flowstate-app`.
- `flowstate-core`: `RichnessMapping`, `effective_richness`, `EvolutionState`,
  and environment/session/analytics types (F-003, F-004, F-005).
- `flowstate-visual`: RON scene model + loader, `EnvironmentDriver`,
  per-layer parameter resolution, Drift/Sine evolution envelopes (F-001, F-010).
- `flowstate-audio`: richness→Nyx patch-parameter mapping (F-009).
- `environments/rainy_library.ron`: the flagship scene, validated by a parse test (F-002).
- Development documentation set per the documentation standard: README status
  header, FEATURES, ROADMAP, ARCHITECTURE, DECISIONS (D-001…D-009), BUILD,
  ATTACK_VECTORS (AV-001…AV-005), BUGS, IMPROVEMENTS, CLAUDE, dual LICENSE.

- `FocusSession::tick` — work/break interval logic for Pomodoro, Custom, Free
  Flow, and Deep Work (single block with a midpoint break, emitting
  `SessionEvent::Completed`), emitting `SessionEvent` boundaries (F-005).
- `EnvironmentDriver::advance` — ticks the evolution cycle and animates the
  work↔break `state_blend` over the transition duration (F-004).
- `resolve_layer_params` richness-scaling table — the user dial now scales
  density/intensity/motion params by name (D-010, F-003).
- `EvolutionConfig::active_events` — resolves `Always` evolution-event windows.
- `flowstate-app` headless logic demo — loads the Rainy Library, prints the
  richness-dial→rain table, and runs a simulated Pomodoro driving the session +
  environment driver together, recording the result into `AnalyticsStore`.
- **Renderer (greenfield, D-011)** — the canonical `VisualModule` GPU trait
  (`ModuleInit` / `FrameCtx`), a wgpu `GpuContext` (headless-capable), the
  `ShaderCanvas` module, and the offscreen `Compositor` (render-doc §11 step 1):
  layer → RGBA16F target → composite → output. Verified headlessly by pixel
  readback and a `render_frame` example that writes a viewable image (F-010).
- **Per-layer DOF + multi-layer compositing (render-doc §11 step 2)** — a
  separable Gaussian `DofBlur` (radius ∝ `depth_blur`), N layers each rendered to
  its own target at its `resolution_scale`, blurred, then composited back-to-front
  with Normal/Additive blend modes. Verified by a hard-edge-softening unit test
  and a `dof_layers` example (sharp foreground over a blurred background).
- **Scene wired to the screen (render-doc §11 step 3)** — a `ModuleSpec`→module
  factory (`build_module`; ShaderCanvas real, others tinted `PlaceholderModule`),
  and a `SceneRenderer` that drives the whole `.ron` layer stack from an
  `EnvironmentDriver`. The Rainy Library scene now renders, and the richness dial
  + work/break state visibly change it. Verified by a test and a `scene_render`
  example (subdued work vs rich break). (F-001, F-003, F-010)
- **Post chain (render-doc §11 step 4)** — layers now composite into an HDR
  (RGBA16F) accumulation, then `PostStage` runs bloom (bright-pass → blur →
  recombine) → colour grade (the scene's `LiftGammaGain`) → vignette → host-seeded
  film grain → tone-map, to an sRGB output. The Library reads warm-amber with a
  vignette and grain. Tone-map is `saturate` for now (IMP-002). (F-010)
- **Real visual modules (render-doc §11 steps 5–6)** — replaced placeholders
  with `VolumetricLight` (additive radial lamp pools), `GeometricField`
  (procedural shelf / window / table silhouette presets), and `GlassRain`
  (screen-space refraction). GlassRain reads the composited backdrop, so the
  compositor gained a backdrop path: refraction layers are skipped in the
  per-layer pass, then composite the accumulation-so-far into a backdrop texture
  and refract it back (render doc §5.1). The Rainy Library now reads as a
  warm-lit room with three windows, a lamp, and rain on the glass. ParticleSystem
  (dust) and the other shared modules remain placeholder. (F-001, F-010)

### Changed
- Resolved the render-doc §12 rendering questions (D-011). Repo inspection found
  Synaesthesia is not built (no repo/crate; the Nyx workspace is audio-only), so
  Galene is the **canonical origin** of the shared rendering stack: the
  `VisualModule` trait, the offscreen compositor, and bloom are greenfield here,
  not reuse. Q4 (lift/gamma/gain grade) confirmed. The wgpu gate (D-005) is
  lifted; D-003's direction is amended (Galene upstream, Synaesthesia conforms).
- Project renamed to **Galene** (branding only — D-009). Crate names
  (`flowstate-*`), the binary, the `.flowenv` format, and the design docs retain
  the original working name FlowState.
