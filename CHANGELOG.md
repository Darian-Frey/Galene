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

- `FocusSession::tick` — work/break interval logic for Pomodoro, Custom, and
  Free Flow sessions, emitting `SessionEvent` boundaries (F-005).
- `EnvironmentDriver::advance` — ticks the evolution cycle and animates the
  work↔break `state_blend` over the transition duration (F-004).
- `resolve_layer_params` richness-scaling table — the user dial now scales
  density/intensity/motion params by name (D-010, F-003).
- `EvolutionConfig::active_events` — resolves `Always` evolution-event windows.
- `flowstate-app` headless logic demo — loads the Rainy Library, prints the
  richness-dial→rain table, and runs a simulated Pomodoro driving the session +
  environment driver together, recording the result into `AnalyticsStore`.

### Changed
- Project renamed to **Galene** (branding only — D-009). Crate names
  (`flowstate-*`), the binary, the `.flowenv` format, and the design docs retain
  the original working name FlowState.
