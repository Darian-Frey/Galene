# Features

## Target users

Hyperphantasic knowledge workers — and the broader group of people with vivid
inner worlds (high-imagination, ADHD-adjacent, deep-work practitioners) — who do
extended creative, intellectual, or technical work and find that ambient sensory
richness helps focus where minimalism fails.

## Out of scope

- Photorealistic / modelled 3D environments (see [DECISIONS.md](DECISIONS.md) D-001).
- Recorded-audio loops (audio is parameterised Nyx synthesis, never a looped file).
- Cloud accounts or telemetry — all session data is local (see F-006).
- A task manager / to-do system — Galene is the environment, not the work tracker.
- Mobile as a primary platform (a companion app is a post-launch candidate).

## Features

### F-001 Immersive generative environments
**Priority:** Must
**Acceptance:**
- A selected environment renders full-screen as composited 2.5D layers with a
  generative ambient soundscape.
- The environment evolves continuously over a multi-hour session without an
  audible or visible loop.
**Status:** In progress (scene format + driver implemented; GPU rendering stubbed)
**Notes:** See [docs/flowstate_render_scene.md](docs/flowstate_render_scene.md).

### F-002 Twelve built-in environments
**Priority:** Must
**Acceptance:**
- All twelve environments (FlowState.md §7) ship as `.ron` scene definitions with
  work and break states and evolution events.
- Each is visually and aurally distinct and parses against the renderer types.
**Status:** In progress (1/12 — Rainy Library scene authored and parse-tested)

### F-003 Richness dial and presets
**Priority:** Must
**Acceptance:**
- A single 0.0–1.0 dial scales visual/audio/evolution detail in real time with
  no restart.
- Presets: Still (0.15), Gentle (0.35), Full (0.65), Maximum (1.0). Default 0.45.
**Status:** In progress (`RichnessMapping` + work/break resolution implemented and tested)

### F-004 Work/break states and environmental transitions
**Priority:** Must
**Acceptance:**
- Each environment has a subdued work state and a richer break state; work
  richness never exceeds break richness ([ATTACK_VECTORS.md](ATTACK_VECTORS.md) AV-003).
- Transitions are gradual environmental shifts (90s work→break, 60s break→work),
  not alarms.
**Status:** In progress (resolution logic implemented; transition timing stubbed in driver)

### F-005 Focus session engine
**Priority:** Must (Pomodoro) / Should (Deep Work, Free Flow, Custom)
**Acceptance:**
- Pomodoro with configurable work/break/long-break intervals (default 25/5/20, 4).
- Deep Work (single long block, midpoint break), Free Flow (untimed), Custom (saved).
- Pause/resume via hotkey; interruptions logged without judgement.
- End-of-session card with optional 1–5 quality rating.
**Status:** In progress (interval ticking for all four session types implemented +
tested; pause/resume done; setup flow, interruption logging, and the end-of-session
card are Phase 2 UI)

### F-006 Session analytics and insights engine
**Priority:** Should
**Acceptance:**
- Local-only per-session recording (no data leaves the device).
- Daily / weekly / per-environment views; focus streak.
- After ~20 sessions, generates non-prescriptive observations and a suggested
  richness range.
**Status:** Not started (record types + in-memory store stubbed)

### F-007 Environment builder and sharing
**Priority:** Could
**Acceptance:**
- Compose an environment from ≤3 visual modules + 1–2 Nyx patches with
  work/break previews.
- Export/import `.flowenv` (sub-5KB JSON, no bundled assets).
**Status:** Not started

### F-008 Work-mode integration
**Priority:** Could
**Acceptance:**
- Desktop-background mode (Linux, Windows), dual-monitor mode, floating timer
  overlay, system-tray status, configurable hotkeys.
**Status:** Not started

### F-009 Nyx ambient audio
**Priority:** Must
**Acceptance:**
- Each environment drives one or two Nyx patches whose parameters track the
  resolved richness and a slow evolution cycle; soundscape never loops.
**Status:** In progress (richness→param mapping implemented; Nyx synthesis not yet wired)

### F-010 Scene rendering pipeline
**Priority:** Must
**Acceptance:**
- Multi-target layer compositing with per-layer depth-of-field blur, HDR
  (RGBA16F) targets, and a post chain (bloom → grade → vignette → grain → tone-map).
- Holds 60fps on target hardware at all richness levels (AV-001).
**Status:** Not started (wgpu wiring deferred — see DECISIONS D-005 and render-doc §12)

## Candidate features (uncommitted)

- Mobile companion app (timer + break notifications, audio continues).
- Calendar integration to populate the work-intention field.
- Community environment library (browse/import shared environments).
- Soundscape layering (mix two patches with independent volume).
- Heart-rate / HRV adaptation of richness in real time.
- Synaesthesia preset as a Galene environment; StoryEngine sensory-state bridge.
