# Roadmap

Phases mirror the development roadmap in [docs/FlowState.md](docs/FlowState.md) §16.
Phases are append-only; mark Complete with an ISO date.

## Phase 0 — Core infrastructure and one environment
**Goal:** Galene opens, one environment displays and sounds correct, a simple focus timer runs.
**Status:** In progress
**Features delivered:** F-001 (partial), F-003 (partial), F-009 (partial)
**Deliverables:**
- [x] Cargo workspace scaffold (`flowstate-core`, `-audio`, `-visual`, `-app`)
- [x] `flowstate-core`: `Environment`, `EnvironmentState`, `RichnessMapping` (+ tests)
- [x] RON scene format + loader, validated on `environments/rainy_library.ron`
- [x] Canonical `VisualModule` GPU trait + wgpu `GpuContext` (D-011)
- [x] `flowstate-visual`: offscreen compositor + ShaderCanvas layer (render-doc §11
      step 1) — headless render verified by readback + `--example render_frame`
- [x] Per-layer DOF blur + multi-layer compositing with blend modes (step 2) —
      verified by unit test + `--example dof_layers`
- [x] EnvironmentDriver + richness wired to visible output via a
      `ModuleSpec`→module factory + `SceneRenderer` (step 3) — the `rainy_library.ron`
      scene renders and the dial/state change it (`--example scene_render`)
- [x] Post chain (step 4): HDR accumulation + bloom + colour grade + vignette +
      grain + tone-map — the Library reads warm-amber (`--example scene_render`)
- [x] Real modules (steps 5–6): VolumetricLight (additive lamp glow),
      GeometricField (shelf/window/table presets), GlassRain (backdrop-reading
      refraction + the compositor backdrop path). ParticleSystem (dust) and the
      other shared modules remain placeholder.
- [ ] Nyx audio (`interior_rain.nyx`) + the windowed surface loop + session timer
- [ ] Post chain (vignette, grain, grade, bloom, tone-map)
- [ ] GlassRain and VolumetricLight primitives
- [ ] `flowstate-audio`: Nyx integration with `interior_rain.nyx`
- [ ] The Rainy Library, complete (visual + audio + work/break)
- [ ] Simple Pomodoro timer with work↔break environmental transition
- [ ] Full-screen display + session-end card
**Acceptance:** Open Galene → Rainy Library appears, rain plays through Nyx, the
richness dial adjusts rain and visual depth, a 25-minute Pomodoro shifts the
environment at interval end.
**Blockers:** none external — render-doc §12 resolved (D-011): Synaesthesia isn't
built, so the renderer is greenfield in Galene. Internal prerequisite: design the
canonical `VisualModule` render-into-target trait before wgpu code.

## Phase 1 — Full environment library
**Goal:** All twelve built-in environments complete and polished.
**Status:** Not started
**Features delivered:** F-002, F-001
**Deliverables:**
- [ ] All twelve environments as `.ron` + Nyx patches + work/break + evolution events
- [ ] Environment picker UI (thumbnail gallery, 10s hover preview)
- [ ] Validated at all richness levels; stress cases (Deep Space, Geothermal Cave) verified
**Acceptance:** All twelve selectable, each visually/aurally distinct with compelling work and break states.

## Phase 2 — Session engine and analytics
**Goal:** Full session system; analytics collected and displayed.
**Status:** Not started
**Features delivered:** F-005, F-006, F-004
**Deliverables:**
- [ ] All session types; setup flow; interruption handling; quality rating
- [ ] Local SQLite session store
- [ ] Daily / weekly / environment views; focus streak; insights engine (20+ sessions)
**Acceptance:** After ten sessions the analytics view shows a pattern and the insights engine produces one accurate observation.

## Phase 3 — Work-mode integration
**Goal:** Galene works alongside other applications without friction.
**Status:** Not started
**Features delivered:** F-008
**Deliverables:**
- [ ] Desktop-background mode (Linux, Windows); dual-monitor; overlay; system tray; hotkeys
**Acceptance:** Run as a desktop background behind a code editor with the timer in the system tray.

## Phase 4 — Environment builder
**Goal:** Users can create and share custom environments.
**Status:** Not started
**Features delivered:** F-007
**Deliverables:**
- [ ] Builder UI (visual layers, Nyx patches, evolution config, work/break preview)
- [ ] `.flowenv` export/import + share widget
**Acceptance:** Build from two modules + one patch, export, import in a fresh install correctly.

## Phase 5 — Polish and shipping
**Goal:** Galene is complete and shippable.
**Status:** Not started
**Deliverables:**
- [ ] UI polish; onboarding; settings; 60fps on mid-range GPU; Linux/Windows/macOS packages; itch.io assets
**Acceptance:** Live on itch.io; a first-time user completes onboarding and runs a 25-minute session.

## Phase 6 — Post-launch (ongoing)
**Status:** Not started
See FEATURES candidate list: mobile companion, calendar integration, community library,
soundscape layering, HRV adaptation, Synaesthesia/StoryEngine bridges.
