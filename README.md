> **Status:** Active
> **Provenance:** Shane Hartley (vision, domain expertise); Claude (architecture, implementation)
> **Last reviewed:** 2026-06-16
> **Why this status:** Phase 0 scaffold in progress — workspace builds, core systems implemented with tests, rendering/app layers stubbed.

# Galene

> *For minds that need a world to work in.*

Galene is a sensory-immersion focus tool for **hyperphantasic** minds — and
anyone with a rich inner world who finds that blank, minimal workspaces make
focus *harder*, not easier.

> **Naming:** Galene is the project name. The workspace crates (`flowstate-*`),
> the binary, the `.flowenv` file format, and the design documents under
> [docs/](docs/) retain the original working name **FlowState** (see
> [DECISIONS.md](DECISIONS.md) D-009).

Every focus tool is built on the same premise: strip the environment down to
nothing. For most attentional profiles that is correct. For a hyperphantasic
mind it is precisely wrong — an empty environment doesn't quiet the mind, it
invites it to *generate* a richer inner world than the task at hand. Galene
does the opposite: it provides a slowly-evolving, sensorially rich external
world — beautiful enough to satisfy the imagination's appetite for input, but
slow and ambient enough never to compete with the work. Each **environment** is
a coherent sensory world (composited visual layers + a generative ambient
soundscape) with a subdued **work** state and a richer **break** state; a single
**Richness** dial calibrates how much it presents.

## Quick Start

```bash
cargo build                 # build the workspace
cargo test                  # core logic + scene-format parse test
cargo run -p flowstate-app  # placeholder binary (Tauri shell not yet wired)
```

## Build requirements

- Rust 1.95+ (stable). No other toolchain required at this stage.
- Heavy dependencies (wgpu, Tauri, SQLite, Nyx) are deferred per phase — see
  [BUILD.md](BUILD.md).

## Project structure

```
Galene/
├── docs/                  design documents (the authoritative domain spec)
├── environments/          scene definitions (.ron) — one file per environment
├── flowstate-core/        environment/richness/evolution/session/analytics types
├── flowstate-audio/       Nyx ambient-audio integration
├── flowstate-visual/      scene rendering: compositing, EnvironmentDriver, primitives
└── flowstate-app/         desktop application (Tauri shell — placeholder binary)
```

## Documentation

- [Features](FEATURES.md) — capabilities, priorities, acceptance criteria
- [Roadmap](ROADMAP.md) — phased development plan
- [Architecture](ARCHITECTURE.md) — module boundaries, data flow, invariants
- [Decisions](DECISIONS.md) — design-decision log with rationale
- [Build instructions](BUILD.md)
- [Attack vectors](ATTACK_VECTORS.md) — project-specific failure modes
- [Bugs](BUGS.md) · [Improvements](IMPROVEMENTS.md)
- [Changelog](CHANGELOG.md)
- [CLAUDE.md](CLAUDE.md) — handoff for AI-assisted sessions
- **Domain spec:** [docs/FlowState.md](docs/FlowState.md) (product design) and
  [docs/flowstate_render_scene.md](docs/flowstate_render_scene.md) (rendering
  architecture) — these serve as the project's `SPEC` (see DECISIONS D-007).

## License

Dual-licensed under MIT or Apache-2.0 — see [LICENSE-MIT](LICENSE-MIT) and
[LICENSE-APACHE](LICENSE-APACHE).
