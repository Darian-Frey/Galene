# flowstate_render_scene.md — Scene Rendering Architecture

> **Status**: Active — v0.1 design specification  
> **Provenance**: Shane Hartley (vision and domain expertise), Claude web (rendering architecture)  
> **Last reviewed**: 2026-05-03  
> **Why**: The definitive guide to how FlowState renders its environments. Hand this to Claude Code when building the `flowstate-visual` crate's scene rendering. Read the main FlowState design document first for the product context (the Richness system, work/break states, the twelve environments, the EnvironmentDriver). This document covers specifically *how a scene is rendered* — the compositing model, the layer system, the new primitives, and the scene definition format.

---

## Clarification protocol

If anything here is ambiguous or requires a design decision not covered by this document or the main FlowState design document, stop and present a question in this format:

```
QUESTION FOR CLAUDE (WEB):
[specific question, with the relevant file/component/decision named]
```

Shane will relay it to Claude web (which holds the full design context) and paste back the answer. A short question is cheaper than a wrong implementation of the rendering core.

---

## 1. The core rendering decision

**FlowState environments are rendered as composited 2.5D layers — a living painting with depth — not as modelled 3D scenes.**

Do not reach for a game-engine approach (modelled rooms, placed lights, mesh furniture). It is the wrong answer on two grounds:

**Technical**: full 3D scenes need art assets. Art assets either loop (which feels dead over a multi-hour focus session) or require enormous volume to avoid looping. They are heavy, slow to author, and hard to make *evolve* organically over a session.

**Cognitive** (the more important reason): FlowState exists to *feed the simulation engine enough that it can rest, without demanding the working mind's attention*. A photorealistic environment demands attention — it competes with the work. An impressionistic, atmospheric *suggestion* of a place is correct: it is a scaffold the hyperphantasic user's imagination fills in at full fidelity. The impressionism is not a performance compromise. It is the right aesthetic for the product's purpose. Render warm lamplight, the soft suggestion of shelves in the dark, rain on glass — and let the mind build the rest.

Implication for every rendering decision: **prefer atmosphere over detail, light over geometry, suggestion over representation.** When in doubt, blur it, dim it, and let it recede.

---

## 2. The layer stack model

An environment is a back-to-front stack of independent layers. Each layer is rendered to its own offscreen texture, then all layers are composited in order, then a post-processing chain runs on the result.

Each layer:
- Names a **module** that draws it (a shared Synaesthesia module, or one of the three new primitives)
- Carries **base parameters** (module-specific)
- Carries an **evolution envelope** (how its parameters drift over the cycle)
- Carries **work-state** and **break-state** parameter values
- Has a **depth/blur level** (further-back layers get more depth-of-field blur)
- Has a **blend mode** for compositing (normal alpha, additive for light, etc.)

### Worked example — the Rainy Library layer stack (back to front)

| # | Layer | Module | Blend | Notes |
|---|---|---|---|---|
| 1 | Base atmosphere | Shader Canvas | normal | Warm dark gradient. Colour temperature driven by time-of-day. |
| 2 | Far shelves | Geometric Field | normal | Low-detail shelf silhouettes. Heavy DOF blur. |
| 3 | Window + storm beyond | Geometric Field + sky shader | normal | Tall window frames; night/storm visible through them. |
| 4 | Reading table + lamp | Geometric Field + point light | normal | Foreground anchor object. Lamp is a VolumetricLight source. |
| 5 | Rain on glass | **GlassRain** (new) | normal (refraction) | Screen-space refraction + droplet trails over everything behind. |
| 6 | Lamplight pools | **VolumetricLight** (new) | additive | Soft bloom pools from the lamp. |
| 7 | Drifting dust | Particle System | additive | Sparse motes catching the light. |
| 8 | Post | Post chain | — | Vignette, film grain, colour grade, depth of field. |

The two warm layers (4, 6) and the two cold layers (3, 5) carry the entire emotional register of the environment — warm interior against cold storm. Most of an environment's character lives in two or three layers; the rest is support.

---

## 3. The compositing pipeline in wgpu

This reuses Synaesthesia's wgpu pipeline. The difference is the layer compositing stage and the parameter source (EnvironmentDriver, not audio analysis).

### Render flow per frame

```
for each layer in stack (back to front):
    1. Bind the layer's offscreen render target (RGBA16F texture)
    2. Clear to transparent
    3. Invoke the layer's module to draw into the target,
       using the layer's CURRENT parameters (resolved from
       base + evolution + richness + work/break — see §6)
    4. If the layer's depth/blur level > 0:
         apply a separable Gaussian blur pass to the target
         (blur radius proportional to depth level)

composite pass:
    5. Bind the final HDR accumulation target
    6. For each layer target, back to front:
         draw a fullscreen quad sampling the layer texture,
         applying the layer's blend mode
         (normal alpha for scene layers, additive for light/dust)

post pass:
    7. Run the post chain on the accumulation target:
       depth-of-field composite (if using per-layer DOF this is
       already baked in; a global far-haze can be added here),
       bloom extraction + blur + recombine,
       colour grade (LUT or curve),
       vignette,
       film grain,
       tone-map HDR -> sRGB
    8. Present
```

### Key technical points

- **Render targets are RGBA16F** (half-float HDR) so light layers can exceed 1.0 and bloom works correctly. Tone-map to sRGB only in the final post pass.
- **Per-layer blur is what creates the painterly depth.** Far layers (shelves, distant features) are blurred heavily; near layers (foreground objects, rain) are sharp. This depth-of-field separation does most of the "this is a real place" work. Do not skip it.
- **Layer textures can be lower resolution for heavily-blurred far layers.** A layer that will be blurred to mush does not need full resolution — render it at half or quarter res to save fill rate. This is a key performance lever (see §10).
- **Blend modes**: scene layers use standard alpha (`src_alpha, one_minus_src_alpha`). Light and dust layers use additive (`one, one`) so they brighten what is behind them. The blend mode is a property of the layer in the scene definition.
- **The composite quad is trivial** — a single fullscreen triangle per layer with the layer texture bound. The cost is fill rate, not geometry.
- **Layer count is bounded** — most environments are 6–9 layers. Cap at 12. If an environment seems to need more, it is probably over-detailed for FlowState's purpose.

### Reusing Synaesthesia's pipeline

Synaesthesia already has: the wgpu device/queue/surface setup, the module trait, the WGSL module shaders, and (probably) a bloom post-effect. What `flowstate-visual` adds on top:

- The **multi-target layer compositing stage** (steps 1–6 above). Synaesthesia renders modules directly to the swapchain; FlowState renders each to an offscreen target and composites.
- The **per-layer DOF blur** (step 4).
- The **EnvironmentDriver** as the parameter source replacing Synaesthesia's `MappingGraph`/`MappingState`.
- The **three new primitives** (§5).

The module trait itself should be shared. A module does not know or care whether its parameters came from an audio transient (Synaesthesia) or an evolution function (FlowState). Keep the module interface identical between the two crates so modules are written once.

---

## 4. Module reuse map

Almost every layer is drawn by a module that already exists in Synaesthesia. Reuse, do not rewrite.

| Module (shared) | FlowState uses it for |
|---|---|
| Particle System | rain streaks, snow, dust motes, drifting embers, falling leaves, starfields, bubbles |
| Fluid Field | fog, nebulae, steam, mist, slow cloud churn, geothermal haze |
| Geometric Field | architectural silhouettes — shelves, window frames, tables, bulkheads, station modules, machinery (all low-detail, DOF-blurred) |
| Terrain | distant landscape silhouettes, sea horizon, ice shelf, dune line |
| Voronoi Field | water caustics (lake, cave lava-light), cracked ice (arctic) |
| Cellular Automata | rare — subtle texture evolution on surfaces if needed |
| Waveform Ribbon | rare in FlowState — possibly aurora ribbons |
| Shader Canvas | every base-atmosphere gradient; custom per-environment background shaders |

---

## 5. The three new primitives

These are the only genuinely new rendering pieces FlowState needs beyond Synaesthesia's modules. Build them as modules implementing the shared module trait so they slot into the layer system like any other.

### 5.1 GlassRain — `src/modules/glass_rain.rs`

The most reused new primitive. Appears in Library, Workshop, Greenhouse, Chart Room, Midnight City. Worth building well.

A screen-space refraction layer that distorts everything behind it with running droplet trails on an implied pane of glass.

- It is a fullscreen fragment shader that samples the **already-composited layers behind it** (so it needs the accumulation-so-far as an input texture — this means GlassRain composites slightly differently: it reads the back buffer rather than drawing to its own isolated target). Implement as: composite layers 1..n-1, bind that result as a texture, run GlassRain reading from it with refraction offset, output to the accumulation target.
- Droplets are procedural: a scrolling noise field generates droplet positions; each droplet offsets the sample UV (refraction) and leaves a vertical trail.
- Parameters: `rain_density` (0–1), `droplet_size`, `trail_length`, `run_speed`, `refraction_strength`, `glass_fog` (0–1, adds a frosted haze).
- Driven by richness (density, refraction strength) and evolution (density breathing) and work/break (work: light drizzle on glass; break: heavy run-off).

### 5.2 VolumetricLight — `src/modules/volumetric_light.rs`

Soft bloom pools and light shafts for lamps, fires, station lights, geothermal glow.

- A placeable light source rendered as a soft additive radial falloff, optionally with directional shafts (god rays) via radial blur from the source point.
- Parameters: `position` (screen-space), `colour` (with HDR intensity > 1.0), `radius`, `falloff`, `shaft_strength` (0 = pool only, >0 = god rays), `flicker` (0–1 amount).
- Composited additively. Multiple instances per environment allowed (e.g. two lamps).
- Driven by richness (intensity, radius) and evolution (flicker, slow intensity drift) and work/break (work: dim steady; break: brighter, more flicker).

### 5.3 Post chain — `src/post/mod.rs`

The painterly look depends heavily on the grade and grain. Synaesthesia may already have bloom; extend into a full chain.

Order: DOF far-haze (optional global) → bloom (extract bright, blur, recombine) → colour grade → vignette → film grain → tone-map.

- **Colour grade**: per-environment. Either a 3D LUT texture or a lift/gamma/gain curve in the shader. This is what makes the Library warm-amber and the Arctic cold-blue from the same renderer. Parameter: `grade` (LUT handle or curve params).
- **Vignette**: `vignette_amount`, `vignette_softness`. Darkens edges, focuses the eye.
- **Film grain**: `grain_amount` (subtle — 0.02–0.06 typical). Animated per frame. Critical for the painterly feel; a clean digital image looks wrong.
- **Bloom**: `bloom_threshold`, `bloom_intensity`. Light layers already exceed 1.0 in HDR; bloom catches them.
- **Tone-map**: ACES or Reinhard. HDR → sRGB.

All post parameters are driven by richness (bloom intensity, grain, vignette can scale slightly) but the colour grade is mostly fixed per environment.

---

## 6. EnvironmentDriver integration — how layers get their parameters

This is what makes it FlowState rather than a screensaver. Each layer resolves its current parameters every frame from four inputs:

```
current_param = resolve(
    base_value,          // from the scene definition
    evolution_offset,    // from the cycle position (slow drift)
    richness_scaling,    // from the user's richness dial
    work_break_state     // work baseline vs break baseline
)
```

The resolution order (matches the main FlowState design doc):

```rust
fn effective_richness(user_richness: f32, state: WorkBreakState) -> f32 {
    match state {
        WorkBreakState::Work  => user_richness * 0.5,        // 0.0..0.5
        WorkBreakState::Break => 0.5 + user_richness * 0.5,  // 0.5..1.0
    }
}

// Per layer, per frame:
fn resolve_layer_params(layer: &Layer, ctx: &DriverContext) -> ResolvedParams {
    let r   = effective_richness(ctx.user_richness, ctx.state);
    let m   = RichnessMapping::from_richness(r);     // the curve from the main doc
    let evo = layer.evolution.current_offsets(ctx.cycle_position);

    let mut p = layer.base_params.clone();
    p.apply_evolution(&evo);          // slow drift, per-param phase offsets
    p.apply_richness(&m);             // scale intensity/density/etc.
    p.lerp_toward_state(&layer.work_state, &layer.break_state, ctx.state_blend);
    p
}
```

- `ctx.cycle_position` advances over the environment's `cycle_minutes` (from EvolutionConfig). Each parameter drifts on its own phase offset so the scene never returns to exactly the same state — this is what stops a 2-hour session feeling looped.
- `ctx.state_blend` is 0.0 in work, 1.0 in break, and interpolates over the 90-second work→break transition (60-second break→work). The whole environment shift is just this blend moving.
- `RichnessMapping::from_richness` is the non-linear curve already defined in the main FlowState document (particle density quadratic, event frequency quadratic, etc.). Reuse it.

**The driver replaces Synaesthesia's MappingGraph.** Where Synaesthesia feeds modules from `VisFrame` audio-analysis data, FlowState feeds them from `resolve_layer_params`. Same modules, different parameter source.

---

## 7. The scene definition format

Because every environment is the same layer stack with different module instances and parameters, **an environment is a declarative data file, not code.** The renderer is general; the twelve environments are twelve data files. This makes the "two new environments a year" post-launch plan cheap, and makes a community environment a shareable sub-5KB file.

Use RON (Rusty Object Notation) to match the house style (Eigenspace and FlowState already use `.ron` for data files).

### Example — `environments/rainy_library.ron`

```ron
Environment(
    id: "rainy_library",
    name: "The Rainy Library",
    description: "A large reading room in an old library, rain on tall windows.",
    tags: ["interior", "warm", "rain", "calm", "study"],

    cycle_minutes: 20.0,

    // Colour grade applied in the post chain — this is what makes
    // the whole scene read as warm amber.
    grade: LiftGammaGain(
        lift:  (0.04, 0.02, 0.00),
        gamma: (1.00, 0.96, 0.88),
        gain:  (1.05, 1.00, 0.90),
    ),

    layers: [
        Layer(
            name: "base_atmosphere",
            module: ShaderCanvas(shader: "warm_interior_gradient"),
            blend: Normal,
            depth_blur: 0.0,
            base: { "warmth": 0.7, "darkness": 0.6 },
            evolution: { "warmth": Drift(amount: 0.05, phase: 0.0) },
            work:  { "darkness": 0.65 },
            break: { "darkness": 0.45 },
        ),
        Layer(
            name: "far_shelves",
            module: GeometricField(preset: "bookshelf_silhouette"),
            blend: Normal,
            depth_blur: 0.85,            // heavily blurred — far back
            resolution_scale: 0.5,       // render at half res, it's blurred anyway
            base: { "density": 0.6 },
            evolution: {},
            work:  {}, break: {},
        ),
        Layer(
            name: "window_storm",
            module: GeometricField(preset: "tall_windows"),
            blend: Normal,
            depth_blur: 0.3,
            base: { "frame_darkness": 0.8, "storm_visibility": 0.5 },
            evolution: { "storm_visibility": Drift(amount: 0.15, phase: 1.7) },
            work:  { "storm_visibility": 0.4 },
            break: { "storm_visibility": 0.8 },   // lightning visible in break
        ),
        Layer(
            name: "table_lamp",
            module: GeometricField(preset: "reading_table"),
            blend: Normal,
            depth_blur: 0.0,             // foreground, sharp
            base: { "lamp_on": 1.0 },
            evolution: {}, work: {}, break: {},
        ),
        Layer(
            name: "rain_on_glass",
            module: GlassRain,
            blend: Refraction,           // reads back buffer, see §5.1
            depth_blur: 0.0,
            base: { "rain_density": 0.5, "refraction_strength": 0.3, "glass_fog": 0.2 },
            evolution: { "rain_density": Sine(amount: 0.2, period_frac: 0.15) },
            work:  { "rain_density": 0.35 },
            break: { "rain_density": 0.9 },
        ),
        Layer(
            name: "lamplight_pools",
            module: VolumetricLight(sources: [
                (pos: (0.32, 0.55), colour: (1.0, 0.75, 0.45), radius: 0.25),
            ]),
            blend: Additive,
            depth_blur: 0.0,
            base: { "intensity": 0.7, "flicker": 0.05 },
            evolution: { "intensity": Drift(amount: 0.1, phase: 0.5) },
            work:  { "intensity": 0.6 },
            break: { "intensity": 0.85, "flicker": 0.12 },
        ),
        Layer(
            name: "dust",
            module: ParticleSystem(preset: "drifting_dust"),
            blend: Additive,
            depth_blur: 0.1,
            base: { "density": 0.3, "drift_speed": 0.05 },
            evolution: {},
            work:  { "density": 0.2 },
            break: { "density": 0.4 },
        ),
    ],

    post: PostChain(
        bloom_threshold: 0.8,
        bloom_intensity: 0.5,
        vignette_amount: 0.35,
        vignette_softness: 0.6,
        grain_amount: 0.04,
        dof_far_haze: 0.3,
    ),

    audio_patch: "interior_rain.nyx",   // from flowstate-audio, see main doc
)
```

The `Drift`, `Sine` evolution functions map to the evolution envelope system. `period_frac` is a fraction of the full cycle; `phase` offsets so different params drift independently.

---

## 8. Two more worked examples (the stress cases)

These two stress the approach in opposite directions and are worth building early to validate the layer system handles both extremes.

### 8.1 Deep Space Observatory — the sparse extreme

Almost empty. Tests whether the layer system is comfortable with very low richness and near-stillness without looking broken or dead.

| # | Layer | Module | Blend | Notes |
|---|---|---|---|---|
| 1 | Deep space gradient | Shader Canvas | normal | Near-black, faint colour at horizon of the glass dome. |
| 2 | Distant nebulae | Fluid Field | additive | Extremely slow churn — change visible only over ~10 min. Heavily blurred. |
| 3 | Starfield | Particle System | additive | Thousands of near-static points. Parallax drift barely perceptible. |
| 4 | Dome glass frame | Geometric Field | normal | Curved frame suggesting the observation dome. |
| 5 | Occasional flare | Particle System (event) | additive | Event-driven: a distant transit/flare brightens over 30s, every ~45 min. |
| 6 | Post | Post chain | — | Heavy vignette, minimal grain, cool grade, subtle bloom on stars. |

Key point: at low richness this scene approaches genuine stillness. The evolution events (meteor every 45 min, binary star rising every 90 min) are what keep it from being a static image. The work/break difference is subtle — break rotates the view slightly toward a nebula and lifts star density.

### 8.2 Geothermal Cave — the volumetric-heavy extreme

Tests VolumetricLight, Voronoi caustics, and additive blending under load.

| # | Layer | Module | Blend | Notes |
|---|---|---|---|---|
| 1 | Cave darkness | Shader Canvas | normal | Near-black base with faint warm bleed from below. |
| 2 | Rock formations | Geometric Field | normal | Stable mineral silhouettes. Mid blur. |
| 3 | Geothermal glow | VolumetricLight | additive | Large warm source from the lava lake direction. Pulses on geological timescale. |
| 4 | Lava caustics | Voronoi Field | additive | Rippling warm light patterns cast on the rock. |
| 5 | Water seep / drips | Particle System | additive | Sparse drips, resonant. |
| 6 | Mineral haze | Fluid Field | normal | Faint volumetric haze near the glow. |
| 7 | Post | Post chain | — | Warm grade, strong bloom on the glow, heavy vignette, long reverb pairs with audio. |

Key point: two additive light layers (3, 4) plus bloom is the fill-rate stress case. Validate 60fps here. If it struggles, the lever is rendering the Voronoi caustics layer at reduced resolution (it is soft, low-frequency light — half res is invisible).

---

## 9. Repo placement

Within the FlowState workspace (see main design doc for the full tree), this work lands in `flowstate-visual`:

```
flowstate-visual/
├── Cargo.toml
└── src/
    ├── lib.rs
    ├── renderer.rs            — wgpu pipeline (reused from synaesthesia)
    ├── compositor.rs          — NEW: multi-target layer compositing (§3)
    ├── layer.rs               — NEW: Layer struct, resolve_layer_params (§6)
    ├── scene.rs               — NEW: Environment/scene definition loading (§7)
    ├── driver.rs              — EnvironmentDriver (replaces Synaesthesia MappingGraph)
    ├── evolution_visual.rs    — evolution envelope functions (Drift, Sine, events)
    ├── dof.rs                 — NEW: per-layer separable Gaussian blur
    ├── modules/
    │   ├── mod.rs             — shared module trait (same as Synaesthesia)
    │   ├── glass_rain.rs      — NEW primitive (§5.1)
    │   ├── volumetric_light.rs— NEW primitive (§5.2)
    │   └── (others re-exported / shared from synaesthesia)
    ├── post/
    │   ├── mod.rs             — NEW: post chain (§5.3)
    │   ├── bloom.rs
    │   ├── grade.rs
    │   ├── vignette.rs
    │   └── grain.rs
    └── shaders/               — WGSL
        ├── glass_rain.wgsl
        ├── volumetric_light.wgsl
        ├── composite.wgsl
        ├── dof_blur.wgsl
        └── post/*.wgsl

environments/                  — scene definition data files (RON)
├── rainy_library.ron
├── deep_space_observatory.ron
├── geothermal_cave.ron
└── ... (12 total)
```

The shared modules (Particle System, Fluid Field, Geometric Field, Terrain, Voronoi, etc.) should live in a place both Synaesthesia and FlowState can depend on — either a shared `nyx-vis` style crate or a path dependency on Synaesthesia's module crate. Confirm with a question to Claude web which arrangement the Synaesthesia workspace currently uses before deciding.

---

## 10. Performance constraints

| Metric | Target |
|---|---|
| Frame rate (all environments, all richness levels) | 60fps on ThinkPad P15 Gen 2i (mobile RTX-class GPU) |
| Layer count per environment | 6–9 typical, 12 hard cap |
| Render target format | RGBA16F |
| Memory (all layer targets at 1080p) | < 200MB GPU |

Performance levers, in order of preference:
1. **Reduce resolution of blurred layers.** A layer with `depth_blur > 0.6` can render at half or quarter resolution invisibly. This is the biggest win and should be applied per-layer via `resolution_scale` in the scene definition.
2. **Reduce particle counts at low richness.** The richness mapping already scales these quadratically — verify it is actually reducing draw cost, not just visual density.
3. **Cap additive light layers.** Two VolumetricLight sources plus bloom is the realistic ceiling. The Geothermal Cave is the stress test.
4. **Lower the cycle update rate for far layers.** A heavily-blurred far layer drifting on a 20-minute cycle does not need per-frame parameter resolution — update it every few frames.

60fps is non-negotiable (consistent with the whole product family). A focus environment that stutters defeats its own purpose. If an environment cannot hit 60fps, simplify the environment (fewer layers, lower-res blurred layers) rather than accepting a lower frame rate.

---

## 11. Build order (suggested)

1. **Compositor first, with one trivial layer.** Get the multi-target render → composite → present loop working with a single Shader Canvas gradient layer. Prove the offscreen-target compositing path before adding complexity.
2. **Add per-layer DOF blur.** Two layers (sharp foreground gradient + blurred background gradient) to prove the depth separation reads correctly.
3. **Wire in the EnvironmentDriver + richness.** A richness dial that visibly changes the two test layers.
4. **Build the post chain.** Vignette, grain, grade, bloom, tone-map. The test scene should immediately look more "real" once the grade and grain land.
5. **Build the Rainy Library** using shared modules + the first new primitive needed (GlassRain). This is the broadest-appeal environment and the best first real target (it is the milestone environment in the main FlowState roadmap Phase 0).
6. **Build GlassRain and VolumetricLight** properly as part of completing the Library.
7. **Validate the two stress cases** (Deep Space sparse, Geothermal volumetric) before building the remaining nine environments.
8. **Build the remaining environments** as data files — by this point each is a `.ron` file plus any environment-specific Shader Canvas background, not new rendering code.

---

## 12. Open questions to confirm before starting

> **Resolved 2026-06-16 — see Galene `DECISIONS.md` D-011.** Repo inspection found
> Synaesthesia is not yet built (no repo; the Nyx workspace is audio-only), so
> there is no upstream to reuse. **Q1:** extract a *new* shared module crate from
> Galene — do **not** name it `nyx-vis` (reserved for the audio→visual data
> bridge); Galene defines the canonical `VisualModule` trait. **Q2:** the
> per-layer RGBA16F offscreen compositor is greenfield here, not an adaptation.
> **Q3:** build bloom new (the `bloom_threshold` / `bloom_intensity` knobs are
> already correct). **Q4:** lift/gamma/gain curves confirmed (Galene D-008); no
> LUT for v1. The questions below are retained as the original plan.

These need a question to Claude web (or a decision from Shane) before implementation:

1. **Shared module crate arrangement** — does Synaesthesia currently expose its modules as a separate crate FlowState can depend on, or do they need extracting into a shared `nyx-vis`-style crate? (§9)
2. **Does Synaesthesia's pipeline already render to offscreen targets, or directly to the swapchain?** If the former, the compositor is a smaller change; if the latter, the offscreen-target path is new work. (§3)
3. **Bloom reuse** — does Synaesthesia already have a bloom post-effect that can be lifted into the FlowState post chain, or is it new? (§5.3)
4. **Colour grade representation** — LUT textures or lift/gamma/gain curves? LUTs are more flexible for art direction; curves are simpler and need no asset pipeline. Recommend curves for v1 (no asset pipeline), LUTs later if needed. Confirm. (§5.3, §7)

---

*This document covers scene rendering only. For the Richness system, work/break states, session engine, analytics, the twelve environment designs, and Nyx audio integration, see the main FlowState design document. For the EnvironmentDriver and RichnessMapping curve specifics, the main document is authoritative — this document assumes them.*
