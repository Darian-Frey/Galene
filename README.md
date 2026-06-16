# FlowState

> *For minds that need a world to work in.*

A sensory-immersion focus tool for **hyperphantasic** minds — and anyone with a
rich inner world who finds that blank, minimal workspaces make focus *harder*,
not easier.

## The idea

Every focus tool is built on the same premise: strip the environment down to
nothing. White walls, white noise, blank backgrounds. For most attentional
profiles that is correct. For a hyperphantasic mind it is precisely wrong — an
empty environment doesn't quiet the mind, it invites it to *generate* a richer
inner world than the task at hand.

FlowState does the opposite. It provides a slowly-evolving, sensorially rich
external world — beautiful enough to satisfy the imagination's appetite for
input, but slow and ambient enough never to compete with the work. The inner
simulation engine is fed just enough that it can rest, leaving the working mind
free.

Each **environment** is a coherent sensory world — composited visual layers plus
a generative ambient soundscape — with a subdued **work** state and a richer
**break** state. A single **Richness** dial calibrates how much the environment
presents, and the world shifts between work and break states as an *environmental
event* rather than an alarm.

See [`docs/FlowState.md`](docs/FlowState.md) for the full product design and
[`docs/flowstate_render_scene.md`](docs/flowstate_render_scene.md) for the
rendering architecture.

## Status

**Early scaffold.** The workspace builds, the core systems (richness mapping,
evolution cycle, scene format) are implemented with tests, and the rendering and
app layers are stubbed pending the open questions below.

```
cargo build      # builds the workspace
cargo test       # richness/evolution logic + scene-format parse test
cargo run -p flowstate-app
```

## Workspace layout

| Crate | Role |
|---|---|
| [`flowstate-core`](flowstate-core/) | Environment definitions, the Richness system, evolution cycle, session engine, analytics |
| [`flowstate-audio`](flowstate-audio/) | Nyx ambient-audio integration — richness → patch-parameter mapping |
| [`flowstate-visual`](flowstate-visual/) | Scene rendering: layer compositing, the `EnvironmentDriver`, and the new primitives |
| [`flowstate-app`](flowstate-app/) | Desktop application (Tauri shell — currently a placeholder binary) |

```
Galene/
├── docs/                       design documents
├── environments/               scene definitions (.ron) — one file per environment
├── flowstate-core/
├── flowstate-audio/
├── flowstate-visual/
└── flowstate-app/
```

## Rendering model

Environments are rendered as **composited 2.5D layers** — a living painting with
depth — not modelled 3D scenes. The aesthetic is impressionistic by design: a
scaffold the imagination fills in, not a photorealistic scene that competes for
attention. Per frame, each layer is drawn to its own HDR offscreen target,
depth-of-field blurred, composited back-to-front, then finished with a post chain
(bloom → colour grade → vignette → film grain → tone-map).

An **environment is declarative data, not code** — the renderer is general, and
each environment is a `.ron` file under [`environments/`](environments/). See
[`environments/rainy_library.ron`](environments/rainy_library.ron) for the
flagship example.

## What's implemented vs. deferred

**Implemented (with tests):**

- `RichnessMapping` and work/break richness resolution (`flowstate-core::richness`)
- The evolution cycle and Drift/Sine parameter envelopes
- Per-frame layer parameter resolution (`flowstate-visual::layer`)
- The RON scene format and loader — validated against `rainy_library.ron`

**Deferred (stubbed, `TODO`-tagged):**

- All wgpu GPU wiring in `flowstate-visual` (`renderer`, `compositor`, `dof`, `post`)
- Live Nyx synthesis (the ambient engine currently records resolved parameters)
- SQLite session persistence and the insights engine
- The Tauri shell and TypeScript frontend

### Open questions blocking the GPU work

These depend on the **Synaesthesia** codebase, with which FlowState shares its
renderer (render doc §12):

1. Are Synaesthesia's visual modules a separate crate FlowState can depend on, or
   do they need extracting into a shared `nyx-vis`-style crate?
2. Does Synaesthesia's pipeline render to offscreen targets or directly to the
   swapchain?
3. Is there an existing bloom post-effect to lift into the FlowState post chain?
4. Colour grade as LUT textures or lift/gamma/gain curves? (Scaffold uses curves.)

## Roadmap

| Phase | Goal |
|---|---|
| **0** | Core infrastructure + one environment (Rainy Library) + a simple focus timer |
| **1** | The full twelve-environment library |
| **2** | Session engine + analytics |
| **3** | Work-mode integration (desktop background, dual monitor, overlay) |
| **4** | Environment builder + sharing (`.flowenv`) |
| **5** | Polish and shipping (itch.io) |

## License

MIT OR Apache-2.0.
