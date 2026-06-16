# FlowState — A Sensory Immersion Focus Tool

> **Status**: Concept  
> **Provenance**: Claude (primary architect), Shane Hartley (ideation and domain expertise)  
> **Last reviewed**: 2026-05-02  
> **Why**: Ninth of ten Hyperphantasia_ideas to be developed in detail. FlowState is a focus and flow environment built on a single insight that every other productivity tool gets wrong: for hyperphantasic minds, the antidote to distraction is not emptiness — it is a controlled, beautiful, slowly-evolving world that satisfies the inner simulation engine just enough to free the working mind. Shares rendering infrastructure with Synaesthesia and audio infrastructure with both Synaesthesia and StoryEngine.

---

## Table of Contents

1. [Vision and Purpose](#1-vision-and-purpose)
2. [The Core Insight](#2-the-core-insight)
3. [The Problem It Solves](#3-the-problem-it-solves)
4. [Who It Is For](#4-who-it-is-for)
5. [Core Feature Set](#5-core-feature-set)
6. [The Environment System](#6-the-environment-system)
7. [Built-in Environments](#7-built-in-environments)
8. [The Richness System](#8-the-richness-system)
9. [The Focus Session Engine](#9-the-focus-session-engine)
10. [Session Analytics](#10-session-analytics)
11. [The Environment Builder](#11-the-environment-builder)
12. [Nyx Audio Integration](#12-nyx-audio-integration)
13. [Shared Rendering Infrastructure](#13-shared-rendering-infrastructure)
14. [User Experience Design](#14-user-experience-design)
15. [Technical Architecture](#15-technical-architecture)
16. [Development Roadmap](#16-development-roadmap)
17. [Monetisation Strategy](#17-monetisation-strategy)
18. [Itch.io Launch Strategy](#18-itchio-launch-strategy)
19. [Future Directions](#19-future-directions)

---

## 1. Vision and Purpose

Every productivity tool is built on the same false premise: that the ideal working environment is as close to nothing as possible. White walls. White noise. Blank backgrounds. Minimal interfaces. The wisdom of distraction-free writing, deep work theory, and digital minimalism all point in the same direction — strip the environment down, remove the stimulus, let the mind focus on the task.

This is correct for most people. For hyperphantasic people it is precisely wrong.

A hyperphantasic mind in an empty environment does not focus. It generates. Within seconds of a blank screen and silence it has populated its inner world with a full sensory environment — a place it finds more interesting than the task at hand. The emptier the external environment, the richer the internal one becomes. Minimalism, for this cognitive profile, is not a focus aid — it is an invitation to disappear entirely into the inner cinema.

FlowState is built on the opposite principle. It provides the hyperphantasic mind with a slowly evolving, sensorially rich external world — one beautiful and engaging enough to satisfy the simulation engine's appetite for sensory input, but never demanding enough to compete with the work. The inner cinema is not starved into submission. It is fed just enough that it can rest, allowing the working mind to do its job.

---

## 2. The Core Insight

The hyperphantasic focus problem has a specific neurological character. The mind's simulation engine — the system responsible for constructing vivid inner experience — does not have an off switch. It is always running, always seeking material. In a rich environment it processes external sensory input. In an empty environment it generates internal content, which is to say: it daydreams, with the full photorealistic quality of a hyperphantasic imagination.

The solution is not to fight this tendency. It is to manage it. Provide the simulation engine with a sufficiently rich external feed — a feed that is slow-changing (so it does not demand active attention), beautiful (so it is satisfying), and ambient (so it stays in the periphery of awareness rather than the centre). The engine processes this feed continuously in the background, leaving the foreground — the working mind — free.

This is not a novel idea in isolation. Lofi hip-hop study streams, café ambience apps, and natural soundscape tools are all doing something similar for auditory attention management. FlowState extends this to the full sensory experience — visual, auditory, and spatial — and designs it specifically for the depth of inner world that hyperphantasic people actually inhabit.

The word "feeds" is deliberate. FlowState is not a background. It is food for the inner simulation engine. It must be rich enough to be satisfying, varied enough to be interesting over hours of work, and slow enough never to demand the working mind's attention.

---

## 3. The Problem It Solves

### The minimalism failure for hyperphantasic workers

Standard focus tools assume that every added stimulus is a distraction risk. This is true for neurotypical attentional profiles where external stimulus competes with task attention. For hyperphantasic profiles the primary distraction risk is not external — it is internal. Removing external stimulus makes the internal world more compelling, not less.

A hyperphantasic person who opens a blank document in a silent room is not entering a focused state. They are entering an environment that strongly favours inner world generation over task engagement. Every standard productivity tool is optimised for a failure mode they don't have while making their actual failure mode worse.

### The lofi problem

Lofi study music streams and café ambience apps are the closest existing solution. They work, to a degree — ambient audio helps many hyperphantasic people maintain working focus. But they address only the auditory channel. The visual channel — a static wallpaper, a blank editor, a motionless background — remains empty. The simulation engine processes the audio but the visual system has nothing to do, and visual generation is where the hyperphantasic mind is most active and most powerful. A static visual background is not a feed — it is, after a few minutes, nothing.

FlowState fills both channels. Audio and visual together, slowly evolving, perfectly matched in sensory character, derived from a single coherent environment.

### The work-environment mismatch problem

Even among people who understand the value of ambient environments, choosing the right one for a given type of work is difficult and personal. Generative background tools (most music visualisers, most ambient video streams) cannot be calibrated to the user. FlowState tracks which environments produce the best focus sessions and learns the user's preferences over time. Over weeks of use, it becomes a personalised focus instrument rather than a generic tool.

---

## 4. Who It Is For

### Primary audience — Hyperphantasic knowledge workers

People doing extended creative, intellectual, or technical work — writing, coding, designing, researching, composing — who have noticed that their focus is better with ambient stimulus than without, but have never had a tool designed around this insight. The recognition moment is strong: "someone finally understood that blank is bad for me."

### Secondary audiences

- **Anyone with a rich inner world who struggles with standard focus tools** — the profile is broader than diagnosed hyperphantasia. Many people with vivid imaginations, high creativity, or ADHD-adjacent attentional patterns find that ambient richness helps focus. FlowState works for this broader group even without the hyperphantasia framing
- **Deep work practitioners** who want an environment with genuine sensory integrity rather than a YouTube lofi stream
- **Remote workers and home office users** who miss the ambient richness of a café or office environment and want something more immersive than background music
- **Writers and creative professionals** who build their working environment deliberately as part of their creative process
- **Students** doing extended study sessions who want an environment that matches their work's emotional register

---

## 5. Core Feature Set

### 5.1 Immersive Generative Environments

A library of slowly-evolving, fully rendered generative environments displayed full-screen or as a desktop overlay. Each environment is a coherent sensory world — visual, auditory, and atmospheric — that evolves organically over hours without repeating. See sections 6 and 7 for full detail.

### 5.2 Richness Control

A single master control — the Richness dial — that adjusts how much sensory detail the environment presents, from near-minimal to fully immersive. The user finds their personal calibration between "enough to feed the engine" and "too much to ignore." See section 8.

### 5.3 Focus Session Engine

Structured work sessions with configurable intervals, gentle environmental transitions between work and rest states, and session logging. Built on Pomodoro principles but extended for hyperphantasic needs. See section 9.

### 5.4 Session Analytics

Long-term tracking of focus sessions, environment usage, and self-reported quality. Over time surfaces patterns: which environments produce the user's deepest focus, which times of day work best, whether richness level affects quality. See section 10.

### 5.5 Environment Builder

An advanced tool for users who want to compose custom environments from modular components. Combine visual modules (from Synaesthesia's module library), Nyx audio patches, and atmospheric parameters into a personalised environment. See section 11.

### 5.6 Work Mode Integration

- **Window manager integration**: FlowState can display behind other applications as a desktop background (Linux, Windows)
- **Dual monitor**: environment on secondary display, work on primary
- **Overlay mode**: a floating panel showing only the session timer, hotkeys, and break indicator, sitting over the working application

### 5.7 Break Mode

When a break begins, the environment shifts state — it becomes richer, more dynamic, more engaging. During work, the environment is deliberately subdued: beautiful but not demanding. During break, it becomes a reward: a moment of full sensory immersion before returning to work. This reinforces the work/break rhythm psychologically as well as practically.

---

## 6. The Environment System

An environment in FlowState is a complete sensory world definition. It has:

- A **Visual Layer** — one or more generative visual modules composited together
- An **Audio Layer** — one or more Nyx ambient patches generating a soundscape
- A **Sensory Profile** — the atmospheric character described in sensory terms
- An **Evolution System** — rules governing how the environment changes over time
- **Work and Break states** — two configurations of the same environment, one subdued and one rich

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Environment {
    pub id:           EnvironmentId,
    pub name:         String,
    pub description:  String,           // short sensory description
    pub tags:         Vec<String>,      // e.g. ["space", "warm", "industrial", "nature"]
    pub visual:       VisualConfig,
    pub audio:        AudioConfig,
    pub sensory:      SensoryProfile,   // shared vocabulary with DreamForge/StoryEngine
    pub evolution:    EvolutionConfig,
    pub work_state:   EnvironmentState, // subdued configuration
    pub break_state:  EnvironmentState, // rich configuration
    pub transition:   TransitionConfig, // work↔break transition timing and character
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentState {
    pub richness:       f32,            // 0.0–1.0 base richness for this state
    pub visual_params:  HashMap<String, f32>,  // module-specific overrides
    pub audio_params:   HashMap<String, f32>,  // patch-specific overrides
    pub evolution_rate: f32,            // how fast the environment evolves
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionConfig {
    pub cycle_minutes:      f32,        // duration of one environmental cycle
    pub variation_amount:   f32,        // how much the environment varies over a cycle
    pub events:             Vec<EvolutionEvent>,  // discrete scheduled changes
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionEvent {
    pub name:           String,
    pub cycle_position: f32,           // 0.0–1.0, position in cycle when this occurs
    pub duration_secs:  f32,
    pub visual_change:  HashMap<String, f32>,
    pub audio_change:   HashMap<String, f32>,
    pub recurrence:     EventRecurrence,  // always, random_chance(p), once_per_session
}
```

---

## 7. Built-in Environments

FlowState ships with twelve environments at launch, each representing a distinct sensory world with a coherent character designed for specific types of work and personal temperament.

### 7.1 Deep Space Observatory

**Visual character**: A vast spherical observation deck. Stars drift infinitely slowly across the curved glass. Distant nebulae pulse with the slowest possible breathing — change visible only over ten minutes of watching. Occasionally a distant transit or flare brightens and fades over thirty seconds.

**Audio character**: Near-total silence. Faint deep subsonic throb of the ship's drives, sensed more as pressure than sound. Occasional soft electronic tone — a system check, distant and unhurried. Long intervals of nothing.

**Sensory profile**: Vast enclosure. Cool neutral air. Slight cabin pressure sensation. The particular silence of deep vacuum.

**Work state**: Stars drift barely. The nebulae are still. Sub-bass is present but barely perceptible. Deep, focused, removed from everything.

**Break state**: The view rotates slowly to face a particularly spectacular nebula. Stars increase in density and motion slightly. The audio gains texture — distant pulsar rhythm, slow and even.

**Best for**: Deep technical work, mathematical thinking, writing that requires sustained concentration. The absence of Earth references removes temporal anxiety. Particularly effective for late-night work sessions.

**Evolution events**:
- Every 45 minutes: a meteor briefly streaks across the view
- Every 90 minutes: a distant binary star system gradually rises above the rim
- Once per session: a slow gravitational lens effect bends starlight across the glass for 2 minutes

---

### 7.2 The Workshop at Night

**Visual character**: A craftsman's workshop lit by two pendant lamps and the glow of instruments. Tools hang precisely on shadow boards. A half-finished project sits on the bench. Dust motes drift in the lamplight. The window shows darkness and occasional rain on glass.

**Audio character**: The fundamental working environment. Occasional rain intensifying and fading on the roof. The remote creak of building structure in the wind. An electric hum from some instrument, very soft. Periodically a distant train, heard and gone in thirty seconds.

**Sensory profile**: Medium interior. Warm lamplight, cool periphery. Smell of machine oil, wood shavings, and dust. Hard floor, smooth bench surface.

**Work state**: Lamps dim slightly. Dust motes slow. Rain is present but very soft. The room is still.

**Break state**: Rain intensifies to a comfortable patter. A lamp flickers briefly in a gust. The window fogs slightly at the corners. The room feels fully alive.

**Best for**: Practical, making-oriented, hands-on work. Writing technical documentation. Any work that benefits from the feeling of a craftsperson's relationship to their tools. Strong overlap with the maker community.

**Evolution events**:
- Every 30 minutes: a brief intensification of rain (90 seconds), then fade
- Every hour: a distant train passes, 45 seconds
- Twice per session: wind gusts, the lamp sways, shadows shift

---

### 7.3 Orbital Station — Earth Watch

**Visual character**: A wide habitat window looking down on the slowly rotating Earth. The terminator — the day/night boundary — moves visibly over a 90-minute work session, echoing the real orbital period. Weather systems are visible from above, evolving at an appropriate scale. The station's structure is visible at the frame edges — a hint of hull and distant solar panels.

**Audio character**: Station ambient hum, very low. Climate control cycling. Distant structural sounds. Occasionally a quiet announcement tone — unintelligible but present.

**Sensory profile**: Enclosed spacecraft. Cool, slightly pressurised air. The particular stillness of microgravity hinted at rather than represented.

**Work state**: Earth rotates at actual orbital speed. Cloud systems evolve slowly. The station sounds are present but very low. The terminator advances measurably over a 90-minute session — a natural visual timer.

**Break state**: The view tilts slightly toward a specific feature — a coastline, a storm system, an aurora visible from above. The station sounds come up slightly. The experience of looking down becomes more present.

**Best for**: Work that benefits from global perspective. Strategic thinking. Any work where time pressure is present and a visual timer (the terminator advancing) is useful without being intrusive.

**Evolution events**:
- Terminator progression: continuous, real-time
- Every 25 minutes: weather system develops or dissipates over a specific region
- Twice per session: brief auroral activity visible at the poles
- Once per session: the station's shadow crosses its own solar panels — a brief darkening

---

### 7.4 The Rainy Library

**Visual character**: A large reading room in an old library. Rain on tall windows, streaking and pooling. Lamplight warm against the dark stone. Bookshelves floor to ceiling, slightly blurred in the middle distance. A reading table in the foreground. No people — the library is closed, or simply empty at this hour.

**Audio character**: Rain on glass, consistent and unhurried. The particular sound of a large empty room — slightly reverberant, absorptive. Very occasional creak of floor above. Paper turning, once, distantly.

**Sensory profile**: Large interior. Warm lamplight in the foreground, cool stone periphery. Smell of old paper, dust, wood polish, rain-damp air where the window isn't quite sealed.

**Work state**: Rain is constant and soft. The lamps are warm but not bright. The room is deeply still.

**Break state**: Rain intensifies to a genuine downpour. Lightning briefly illuminates the tall windows, followed by distant thunder. The lamps flicker. The storm makes the interior warmth more present.

**Best for**: Writing, reading-adjacent work, research, any intellectual work that benefits from the specific combination of warmth, enclosure, and the sound of rain. The most broadly popular environment in user testing.

**Evolution events**:
- Every 20 minutes: rain intensity varies slowly over a 3-minute cycle
- Every 45 minutes: thunder, distant and low
- Once per session: a brief storm peak (2 minutes) followed by quieter rain

---

### 7.5 Tide Pools at Low Water

**Visual character**: A rocky coastal shelf at low tide. Tide pools still and mirror-clear. Distant surf breaking at the water's edge. Afternoon light at a low angle, specific and golden. Sea birds moving distantly. The horizon is wide.

**Audio character**: Distant surf rhythm — the fundamental pulse. Wind, moderate and directional. Occasional gull cry, distant. The quiet of a large open space with a single persistent sound source.

**Sensory profile**: Open air. Cool salt wind. Smell of sea wrack, salt, cold stone. Hard uneven surface underfoot. Wide sky.

**Work state**: Surf continues at natural rhythm. Wind is low. Birds are rare. The light is steady.

**Break state**: The surf builds slightly. The light shifts toward golden hour. A closer wave runs over the shelf and fills a pool. More birds, briefly.

**Best for**: Creative work, brainstorming, lateral thinking, any work that benefits from the specific spaciousness of coastal open air. The rhythm of the distant surf is subtly regulating — it provides a natural cadence without imposing a beat.

**Evolution events**:
- Tidal progression: the water level very slowly rises over a 3-hour cycle
- Every 35 minutes: a wave runs slightly closer to the observation point
- Hourly: the light angle shifts noticeably, changing the character of the reflections in the pools

---

### 7.6 The Greenhouse in Rain

**Visual character**: The interior of a large Victorian greenhouse. Iron frame and glass. Tropical plants dense in the middle distance. Rain hammering the glass roof in waves — sound as much as visual. Condensation on the panes. A single gas lamp burning somewhere.

**Audio character**: Rain on glass — but glass of the roof, which is different: more reverberant, more enclosing, a specific indoor-outdoor relationship. The acoustic is the most distinctive element of this environment. Water dripping from leaves after a wave of rain passes. Steam heat ticking.

**Sensory profile**: Large glass-roofed interior. Warm, humid. Smell of damp earth, green growing things, tropical density, faint mould. The smell of wet glass.

**Work state**: Rain is moderate. The lamp is warm. The plants are still.

**Break state**: Rain becomes intense. The glass rattles slightly. Steam rises from somewhere. The environment becomes very present — loud, warm, enclosed.

**Best for**: Creative writing, especially writing that benefits from warmth and density. Works well for writers who like the feeling of being contained within the weather rather than exposed to it.

**Evolution events**:
- Rain intensity pulses on a 12-minute cycle
- Every 50 minutes: a brief very heavy downpour (3 minutes) then steady
- Once per session: a rare winter sun appears briefly through the rain — a specific quality of cold bright light through wet glass

---

### 7.7 Arctic Research Station — Winter

**Visual character**: The interior of a research station through a frosted double-pane window, looking out at the ice shelf. The aurora visible far above — not dramatic, but present. Blowing snow catching the station's exterior lights. A thermometer on the outside wall reading extreme cold.

**Audio character**: Wind across the structure — not howling, but persistent and directional. The station itself: heating system running, a computer fan distantly. The specific quality of cold silence when the wind briefly drops.

**Sensory profile**: Medium enclosed interior looking at vast exterior. Warm inside, extreme cold implied outside. Smell of recycled air, machine heat, synthetic materials.

**Work state**: Wind is steady and low. Aurora is faint and slow. The station sounds are very quiet. Deep isolation.

**Break state**: Wind intensifies. Aurora brightens and flows. The contrast between the warm interior and the visible extreme cold becomes vivid.

**Best for**: Long intense work sessions. The environment communicates productive isolation — the feeling of being far from distraction, working on something that matters in difficult conditions. Effective for coding, complex writing, any work requiring extended focus without social interruption.

**Evolution events**:
- Every 40 minutes: wind intensifies briefly (2 minutes), then drops
- Hourly: aurora behaviour changes — different colour, different movement
- Once per session: a brief whiteout (complete exterior white for 90 seconds) then clearing

---

### 7.8 The Ship's Chart Room

**Visual character**: A small navigational cabin on a sailing ship. Charts on the table under a brass lamp. The porthole shows a grey-green sea with moderate swell. The ship moves gently — a subtle roll and pitch. Rain occasionally across the glass.

**Audio character**: The sea: wave frequency low, rhythmic. The ship: creak of hull and rigging, the specific sound of a wooden boat working in a moderate sea. Wind outside. Rain occasionally.

**Sensory profile**: Small enclosed interior. The specific motion of being at sea — not nauseating, just present. Smell of salt, damp charts, old wood, brass polish, lantern oil.

**Work state**: Sea is steady with a regular rhythm. Ship sounds are low. The roll is very gentle.

**Break state**: Sea state increases slightly. The ship's motion becomes more present. Rain appears. The porthole spray picks up.

**Best for**: Writing that involves problem-solving or navigation metaphors. Works well for anyone who finds the rhythm of water naturally regulating. The subtle motion cue (though not actually present) is suggested strongly enough by the audio to have a calming effect.

**Evolution events**:
- Sea state gradually builds and falls on a 90-minute cycle
- Every 45 minutes: a wave larger than the pattern strikes the hull — a single resonant impact
- Once per session: a ship light visible briefly through the porthole — another vessel, passing

---

### 7.9 The Geothermal Cave

**Visual character**: A vast underground cave lit by geothermal glow from a lava lake in the far distance. Mineral formations. Dripping water. The rock formations are stable — the light source pulses on a geological timescale: detectable but not distracting.

**Audio character**: Drips — irregular, resonant, deeply acoustic. Distant subsonic rumble of the earth. The cave's characteristic reverb: long tail, reflective, every sound is present for a long time. Occasional large drip into a still pool.

**Sensory profile**: Vast interior. Cool-to-warm gradient depending on proximity to the lava. Mineral smell, damp stone, sulphur trace. The particular silence of deep rock — not empty silence, but filled silence.

**Work state**: Geothermal glow is low and stable. Drips are infrequent. Subsonic is barely perceptible.

**Break state**: The glow brightens and pulses slightly. More frequent drips. The reverb becomes more present.

**Best for**: Night-time work sessions. Work requiring deep internal states — mathematical thinking, introspective writing, meditation-adjacent focus. The most unusual environment in the library; strongly preferred by a subset of users who find it intensely focusing.

**Evolution events**:
- Every 25 minutes: a geological event — distant low rumble, the glow shifts
- Hourly: a large mineral deposit catches the light differently
- Once per session: a brief very bright geothermal event — the cave floods with warm orange light for 3 minutes then dims

---

### 7.10 Autumn Forest Path

**Visual character**: A forest path in late autumn. Leaves on the ground and still falling. Trees in their last colour — amber, ochre, deep red, with grey branches visible through. Light filtered and golden. Wind moves through the canopy slowly.

**Audio character**: Wind in autumn leaves — a specific dry rattling. Leaves falling, individually audible when close. Distant birds. A stream somewhere, heard rather than seen. Underfoot: the sound of walking through leaves, implied but not intrusive.

**Sensory profile**: Open forest. Cool air. Smell of leaf mould, cold earth, the specific dry-sweet smell of dead leaves, distant fungus.

**Work state**: Wind is low. The light is still. Leaves fall occasionally. The forest is very quiet.

**Break state**: Wind picks up. More leaves fall, some moving across the path. The light shifts as clouds pass. The forest becomes actively autumnal.

**Best for**: Writing with strong natural imagery. Any work benefiting from the specific emotional register of autumn — reflective, ending-aware, beautiful but aware of ending. Strongly seasonal: users often return to this environment in their own autumn.

**Evolution events**:
- Wind pulses every 15 minutes, lasting 2-3 minutes
- Hourly: a group of birds moves through the canopy — audible passage
- Once per session: brief heavy leaf fall, as if a wind gust has stripped a branch

---

### 7.11 The River Mill Interior

**Visual character**: The working floor of a water mill. Stone grinding mechanism turning slowly. Water wheel visible through a gap in the wall, driving everything. Flour dust in the air, caught in beams of light. The mechanism is old, complex, and entirely functional.

**Audio character**: The mill: rhythmic, mechanical, continuous. The wheel turning, the stones grinding (low, felt as much as heard), the creak of wooden gearing. Water outside — present but subordinate to the mechanism. Everything in a slow, functional rhythm.

**Sensory profile**: Medium interior. Warm from the machinery and grain. Smell of flour, old wood, water, stone dust, oil. Vibration underfoot from the mechanism.

**Work state**: Mechanism continues at its normal pace. Light is steady. The rhythm is constant but not intrusive.

**Break state**: The mill increases pace slightly — a change in water level driving it harder. The light shifts. The rhythm becomes more present.

**Best for**: Systematic, process-oriented work. Data analysis, structured writing, coding with a clear plan. The mechanical rhythm of the mill is overtly productive — the environment communicates "this is how work gets done," slowly and continuously.

**Evolution events**:
- Mill rhythm occasionally shifts slightly (water level variation) — subtle
- Every 50 minutes: the miller adjusts something — a physical sound, footsteps, a mechanism change
- Once per session: a brief stop and restart of the mechanism — a moment of silence, then the rhythm resumes

---

### 7.12 Midnight City Rain (Rooftop)

**Visual character**: A city rooftop at midnight in rain. City lights below and across, blurred by rain. The immediate space is the rooftop: wet concrete, a skylight with warm light from below, ventilation units. Rain falling consistently, visible in the streetlight cone below. The city moves distantly.

**Audio character**: Urban rain — the layered complexity of rain in a city. Rain on concrete, rain on metal, distant traffic and sirens, all filtered through rain and height. The city is present as a continuous murmur beneath the rain.

**Sensory profile**: Open air, elevated, urban. Cool, wet. Smell of rain, warm city air rising from below, petrichor on concrete. Wide sky but bounded by buildings.

**Work state**: Rain is steady. City sounds are low. The rooftop is still.

**Break state**: Rain intensifies. A distant siren. More city movement. A taxi's light briefly brightens on a street below.

**Best for**: Work that benefits from urban energy without urban distraction. Writers and creatives who miss cities while working from home. Late-night sessions. Work with an urban setting.

**Evolution events**:
- Rain varies on a 20-minute cycle
- Every 35 minutes: a distant siren, fading
- Hourly: the quality of city light below shifts — a bar's doors opening briefly, a bus passing
- Once per session: a brief gap in the rain — relative silence — then return

---

## 8. The Richness System

The Richness dial is FlowState's most important single control. It mediates the relationship between the user's sensory appetite and the environment's output.

### 8.1 What richness controls

Richness is a master parameter (0.0–1.0) that scales multiple environment properties simultaneously:

```rust
pub struct RichnessMapping {
    // Visual properties scaled by richness
    pub visual_detail:      f32,   // number of active visual elements
    pub motion_amount:      f32,   // rate of visual change
    pub particle_density:   f32,   // particles in particle systems
    pub effect_intensity:   f32,   // bloom, glow, atmospheric effects
    pub colour_saturation:  f32,   // colour vibrancy
    
    // Audio properties scaled by richness
    pub audio_volume:       f32,   // master volume
    pub audio_complexity:   f32,   // number of concurrent audio layers
    pub event_frequency:    f32,   // rate of discrete sound events (rain intensities, birds)
    pub reverb_presence:    f32,   // wet/dry mix on reverb effects
    
    // Evolution properties scaled by richness
    pub evolution_speed:    f32,   // how fast the environment changes over time
    pub event_probability:  f32,   // likelihood of evolution events triggering
}

impl RichnessMapping {
    pub fn from_richness(r: f32) -> Self {
        // Non-linear mapping: sensitive in the middle range,
        // compressed at extremes
        let r_curved = smooth_step(r);
        
        Self {
            visual_detail:    0.1 + r_curved * 0.9,
            motion_amount:    0.05 + r_curved * 0.5,
            particle_density: r_curved * r_curved,  // quadratic: sparse at low richness
            effect_intensity: 0.2 + r_curved * 0.7,
            colour_saturation:0.4 + r_curved * 0.6,
            audio_volume:     0.15 + r_curved * 0.7,
            audio_complexity: r_curved,
            event_frequency:  r_curved * r_curved,  // events are rare at low richness
            reverb_presence:  0.3 + r_curved * 0.6,
            evolution_speed:  0.1 + r_curved * 0.6,
            event_probability:r_curved * 0.8,
        }
    }
}
```

### 8.2 Work richness vs break richness

Each environment has two richness baselines: work state richness (always lower) and break state richness (always higher). The user's Richness dial scales within each baseline:

```
User richness dial:  0.0 ─────────────────── 1.0
                     
Work state:     applies as:  0.1 to 0.5  (range compressed)
Break state:    applies as:  0.5 to 1.0  (range elevated)
```

This ensures that even at maximum user richness, the work environment is never as engaging as the break environment. The dial personalises within the appropriate range for each state without breaking the work/break distinction.

### 8.3 Personal calibration

The correct richness level is different for every person and every type of work. FlowState makes it easy to find:

- Richness dial is always visible and accessible during a session
- Default: 0.45 (moderate — suitable for most users to start)
- Adjustment is immediate and smooth: no restart required
- Session analytics track richness level against session quality (see section 10)
- Over time, FlowState suggests a personalised richness range: "Your deepest focus sessions use richness 0.35–0.55 in this environment"

### 8.4 Richness presets

Quick-access preset buttons:
- **Still** (0.15): barely alive, almost minimal but not quite
- **Gentle** (0.35): the default working richness for most users
- **Full** (0.65): rich enough for a sensitive environment; good for breaks
- **Maximum** (1.0): everything the environment has; break mode or exploration

---

## 9. The Focus Session Engine

### 9.1 Session types

**Pomodoro (default)**
- Configurable work intervals (default: 25 minutes) and break intervals (default: 5 minutes)
- Long break after configurable number of work intervals (default: 4 intervals → 20-minute long break)
- Classic Pomodoro discipline: commit to the interval, do nothing else

**Deep Work**
- Single long work block (45–180 minutes, configurable)
- One break at the midpoint
- For work requiring sustained deep engagement without interruption

**Free Flow**
- No timer structure
- Session begins when the user starts it and ends when they stop
- Duration tracked but not constrained
- For users who find timers disruptive

**Custom**
- User-defined intervals, break lengths, and long break thresholds
- Save as named session type

### 9.2 Session setup

Session setup is minimal — three interactions at most:

1. Select or accept the current environment (previously used environment is the default)
2. Select session type (previously used type is the default)
3. Set today's work intention (optional text: "finishing chapter 3" / "the API integration")

One button: **Begin**.

The work intention is stored with the session data and appears in the analytics. It is optional but nudges the user to make an explicit commitment to the work before starting, which is a known focus booster.

### 9.3 Environmental transitions

The transition between work and break states is not an alarm. It is an environmental event:

**Work → break**: the environment gradually shifts to its break state over 90 seconds. The richness rises slowly. The visual layer becomes more present. The audio gains texture. By the time the break begins, the environment has already told the user it is time to stop.

**Break → work**: the environment shifts back over 60 seconds. Visual detail reduces. Audio quietens. The environment becomes the focused, subdued working space. The user is not startled back to work — they are guided.

No alarm sound. No notification. No pop-up. The world itself tells the user when to stop and when to continue.

### 9.4 Session interruption

If the user needs to pause mid-interval:
- Single hotkey pauses the timer and freezes the environment
- Environment continues displaying (frozen in current state)
- Resume with the same hotkey
- Interruptions are logged (time and duration) without judgement

### 9.5 Session end

At session end (all intervals complete, or user ends a free flow session):
- A brief end-of-session card appears: duration, intervals completed, optional quality rating
- Quality rating: 1–5 stars, one tap. No required comment
- The environment shifts to a gentle neutral state — not fully dimmed, just resting
- Session data saved to local database

---

## 10. Session Analytics

### 10.1 What is tracked

All tracking is local. No data leaves the device. The analytics exist to serve the user, not to report to anyone.

**Per session:**
- Date, time, duration
- Session type used
- Environment used
- Richness level (average over session, and range)
- Work intention (user text)
- Intervals completed vs total
- Interruptions: count and duration
- Quality rating (1–5, if provided)

**Derived over time:**
- Preferred environments: most-used, highest-rated
- Optimal richness range: richness levels correlating with highest quality ratings
- Best focus times: time-of-day patterns in session quality
- Focus streak: consecutive days with at least one completed session
- Weekly session volume: total focused hours per week trend

### 10.2 Analytics views

**Daily view**: today's sessions as a timeline. Work blocks and break blocks visible. Interruptions marked. Total focused time.

**Weekly view**: seven-day calendar of session activity. Colour-coded by quality rating. Pattern view: which day-of-week and time-of-day is most productive.

**Environment insights**: for each environment used more than 5 times:
- Average quality rating
- Average richness used
- Average session duration
- Trend: is quality improving as the user becomes familiar with this environment?

**Focus health**: a simple overview panel on the main screen:
- Streak: consecutive days with sessions
- This week: hours focused vs personal goal (if set)
- Best recent environment: the environment with the highest average rating in the last 14 days
- Suggested richness: based on patterns, FlowState suggests a starting richness for today

### 10.3 Insights engine

After sufficient data (approximately 20 sessions), FlowState begins generating insights:

```
"Your focus quality is consistently higher between 9am and 12pm."

"Deep Space Observatory gives you an average rating of 4.2 — your highest.
 Rainy Library gives you 3.1."

"Your best sessions use richness between 0.35 and 0.5.
 At richness above 0.6, your quality ratings drop slightly."

"You complete 85% of intervals in Deep Work sessions
 but only 60% in Pomodoro sessions.
 Consider switching to Deep Work as your default."
```

These insights are displayed as brief, non-prescriptive observations — FlowState does not tell users what to do. It shows them what it has noticed.

---

## 11. The Environment Builder

### 11.1 Purpose and audience

The environment builder is for users who want to compose their own environment beyond the built-in library. It is an advanced feature — accessible but not the default experience.

The builder shares its visual module system with Synaesthesia. The same modules (Fluid Field, Particle System, Geometric Field, Terrain, Waveform Ribbon, Voronoi, Cellular Automata) are available as visual layers in FlowState environments. This is a deliberate architectural decision: Synaesthesia users who discover FlowState (or vice versa) find a familiar visual vocabulary.

### 11.2 Builder workflow

**Step 1: Visual layers**
- Select up to 3 visual modules from the Synaesthesia module library
- Configure each module: the same parameters available in Synaesthesia, but with "static" values rather than audio-driven values
- Set blend modes between layers
- Adjust the evolution envelope: how much each layer's parameters drift over the cycle period

**Step 2: Audio layers**
- Select one or two Nyx ambient patches from the library
- Configure patch parameters at work and break state values
- Set how parameters evolve over the cycle

**Step 3: Sensory profile**
- Name the environment
- Write a short sensory description (for the library)
- Tag with sensory vocabulary
- Set the evolution cycle length

**Step 4: Work and break states**
- Set richness baselines for work and break
- Preview the environment in work state, then break state
- Adjust parameters

**Step 5: Save**
- Save to personal library
- Optional: export as a shareable `.flowenv` file

### 11.3 Environment sharing

Shared environments are `.flowenv` files — small JSON files describing the visual and audio configuration. No assets are bundled: all environments use the built-in Synaesthesia modules and Nyx patches. A `.flowenv` file is typically under 5KB.

Sharing workflow:
- Export `.flowenv` from the builder
- Share via itch.io community forum, Reddit, Discord, or directly
- Recipient imports into FlowState — environment appears in their library immediately

---

## 12. Nyx Audio Integration

FlowState shares its Nyx integration architecture with Synaesthesia and StoryEngine. The audio engine is parameterised ambient synthesis, not recorded audio.

### 12.1 Ambient patches

Each built-in environment uses one or two Nyx patches. The patches run continuously, generating a living soundscape that never loops, never repeats, never has the characteristic "seam" of a looped audio file.

**Patches used by built-in environments:**

| Environment | Primary Patch | Secondary Patch |
|---|---|---|
| Deep Space Observatory | `deep_space.nyx` | — |
| Workshop at Night | `interior_rain.nyx` | `workshop_ambient.nyx` |
| Orbital Station | `station_ambient.nyx` | — |
| Rainy Library | `interior_rain.nyx` | `large_reverb.nyx` |
| Tide Pools | `coastal_open.nyx` | — |
| Greenhouse | `greenhouse_rain.nyx` | — |
| Arctic Station | `arctic_wind.nyx` | `station_interior.nyx` |
| Chart Room | `maritime.nyx` | — |
| Geothermal Cave | `cave_ambient.nyx` | — |
| Autumn Forest | `forest_autumn.nyx` | — |
| River Mill | `mill_mechanism.nyx` | `water_wheel.nyx` |
| Midnight City Rain | `urban_rain.nyx` | — |

### 12.2 Audio parameters driven by richness

The Nyx patch parameters are driven by the resolved richness value (from the user's dial plus work/break state baseline):

```rust
impl AmbientAudioEngine {
    pub fn update_richness(&mut self, richness: f32, patch: &PatchType) {
        let m = RichnessMapping::from_richness(richness);
        
        match patch {
            PatchType::InteriorRain => {
                self.nyx.set_param("rain_density",  0.1 + m.audio_complexity * 0.8);
                self.nyx.set_param("rain_volume",   m.audio_volume);
                self.nyx.set_param("reverb_wet",    0.2 + m.reverb_presence * 0.6);
                self.nyx.set_param("event_rate",    m.event_frequency);
            }
            PatchType::DeepSpace => {
                self.nyx.set_param("sub_level",     m.audio_volume * 0.3);
                self.nyx.set_param("system_tone_rate", m.event_frequency * 0.2);
                self.nyx.set_param("silence_depth", 1.0 - m.audio_complexity * 0.4);
            }
            // ... etc
        }
    }
}
```

### 12.3 Evolution-driven audio variation

The evolution system drives slow, organic variation in audio parameters over the environment's cycle:

```rust
pub struct AudioEvolution {
    cycle_position: f32,     // 0.0–1.0, current position in evolution cycle
    base_params:    HashMap<String, f32>,
    variation:      HashMap<String, f32>,   // max variation from base
    phase_offsets:  HashMap<String, f32>,   // different params drift at different rates
}

impl AudioEvolution {
    pub fn current_params(&self) -> HashMap<String, f32> {
        self.base_params.iter().map(|(k, base)| {
            let variation = self.variation.get(k).copied().unwrap_or(0.0);
            let phase     = self.phase_offsets.get(k).copied().unwrap_or(0.0);
            let drift     = variation * (self.cycle_position * 2.0 * PI + phase).sin();
            (k.clone(), (base + drift).clamp(0.0, 1.0))
        }).collect()
    }
}
```

This ensures that even in a two-hour session, the audio environment is never exactly the same as it was an hour ago. The changes are slow enough that they are felt rather than noticed — the subconscious awareness that the world is alive, without the conscious awareness of what changed.

---

## 13. Shared Rendering Infrastructure

FlowState's visual layer shares its rendering infrastructure with Synaesthesia. This is the most important architectural connection in the product family.

### 13.1 Shared components

**Visual modules**: the same eight visual modules (Fluid Field, Particle System, Geometric Field, Terrain, Waveform Ribbon, Cellular Automata, Voronoi Field, Shader Canvas) are available in both products. In Synaesthesia, they are audio-reactive — their parameters are driven by `VisFrame` data. In FlowState, they run autonomously — their parameters are driven by the `EvolutionConfig` and the current richness level.

**Rendering pipeline**: the wgpu pipeline (frame structure, compositing, post-processing) is shared. The difference is the parameter source: Synaesthesia uses `MappingState` (from audio analysis), FlowState uses `EnvironmentState` (from evolution and richness).

**WGSL shaders**: identical. The Fluid Field shader does not know or care whether its injection velocity came from a bass transient or a slow evolution function.

### 13.2 Workspace structure

```
FlowState/
├── Cargo.toml                          — workspace manifest
├── CLAUDE.md
├── CHANGELOG.md
├── flowstate-core/                     — environment definitions, session engine, analytics
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── environment.rs              — Environment struct, EnvironmentState
│       ├── richness.rs                 — RichnessMapping, work/break resolution
│       ├── evolution.rs                — EvolutionConfig, cycle progression
│       ├── session.rs                  — FocusSession, SessionType, intervals
│       ├── analytics.rs                — session recording, insights engine
│       └── serialiser.rs              — .flowenv format, session data (SQLite)
├── flowstate-audio/                    — Nyx integration for ambient audio
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── ambient_engine.rs           — richness → Nyx param mapping
│       ├── patches.rs                  — built-in patch library
│       └── evolution_audio.rs          — audio evolution cycle
├── flowstate-visual/                   — visual rendering (shares with Synaesthesia)
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── renderer.rs                 — wgpu pipeline (reused from synaesthesia)
│       ├── module_driver.rs            — drives visual modules from EnvironmentState
│       ├── evolution_visual.rs         — visual evolution cycle
│       └── transition.rs              — work↔break visual transitions
└── flowstate-app/                      — Tauri desktop application
    ├── Cargo.toml
    ├── src/
    │   ├── main.rs
    │   ├── commands/
    │   │   ├── session.rs
    │   │   ├── environment.rs
    │   │   └── analytics.rs
    │   └── state.rs
    └── frontend/
        ├── src/
        │   ├── main.ts
        │   ├── MainView.ts             — full-screen environment + overlay
        │   ├── SessionOverlay.ts       — minimal timer and status overlay
        │   ├── EnvironmentPicker.ts    — library and selection UI
        │   ├── SessionSetup.ts         — session type and intention entry
        │   ├── Analytics.ts            — analytics dashboard
        │   ├── RichnessControl.ts      — richness dial component
        │   └── Builder.ts             — environment builder
        └── styles/
            ├── main.css
            └── overlay.css
```

### 13.3 The environment driver

FlowState's environment driver replaces Synaesthesia's mapping graph as the source of visual module parameters:

```rust
pub struct EnvironmentDriver {
    environment:     Environment,
    evolution:       EvolutionState,
    richness:        f32,
    state:           WorkBreakState,  // Work or Break
}

impl EnvironmentDriver {
    /// Called each render frame. Returns the current visual module parameters.
    pub fn current_visual_params(&self) -> VisualModuleParams {
        let m   = RichnessMapping::from_richness(self.effective_richness());
        let evo = self.evolution.current_visual_params();
        
        // Base params from environment definition
        let mut params = self.current_state_params().visual_params.clone();
        
        // Apply evolution variation
        for (k, v) in &evo {
            params.entry(k.clone()).and_modify(|p| *p += v);
        }
        
        // Scale by richness mapping
        apply_richness_scaling(&mut params, &m);
        
        params
    }
    
    fn effective_richness(&self) -> f32 {
        match self.state {
            WorkBreakState::Work  => self.richness * 0.5,
            WorkBreakState::Break => 0.5 + self.richness * 0.5,
        }
    }
}
```

---

## 14. Development Roadmap

### Phase 0 — Core infrastructure and one environment (3–4 weeks)

**Goal**: FlowState opens, one environment displays and sounds correct, a simple focus timer runs.

Deliverables:
- Tauri project scaffold with shared workspace entries
- `flowstate-core`: `Environment`, `EnvironmentState`, `RichnessMapping`
- `flowstate-audio`: Nyx integration, one ambient patch (`interior_rain.nyx`)
- `flowstate-visual`: wgpu pipeline (adapted from Synaesthesia Phase 1 infrastructure), one visual module (Fluid Field)
- One complete environment: The Rainy Library (the broadest appeal, best for testing)
- Richness dial: full range, immediate visual and audio response
- Simple focus timer: work interval, break interval, transition
- Transition: work→break environment shift and back
- Full-screen display mode
- Session end card (no analytics yet — just the display)

Milestone: Open FlowState. The Rainy Library appears. Rain plays through Nyx. Richness dial adjusts the rain and visual depth. Start a 25-minute Pomodoro. The environment shifts at interval end.

---

### Phase 1 — Full environment library (5–6 weeks)

**Goal**: All twelve built-in environments are complete and polished.

Deliverables, two environments per sprint (six sprints):
- All twelve environments with their visual configurations
- All Nyx ambient patches for each environment
- Work and break states for each
- Evolution events for each
- Environment picker UI: thumbnail gallery with name and description
- Environment preview: hover to see a 10-second preview before selecting
- All environments tested at all richness levels

Milestone: All twelve environments available and selectable. Each is visually and aurally distinct. Each has a compelling work and break state.

---

### Phase 2 — Session engine and analytics (3–4 weeks)

**Goal**: The full session system works. Analytics are collected and displayed.

Deliverables:
- All session types: Pomodoro, Deep Work, Free Flow, Custom
- Session setup flow: environment selection, session type, work intention
- Interruption handling: pause/resume hotkey
- Session end card with quality rating
- Local SQLite database for session data
- Daily, weekly, and environment analytics views
- Focus streak display
- Insights engine: generates observations after 20+ sessions
- Suggested richness based on historical data

Milestone: After ten sessions, the analytics view shows a pattern. The insights engine generates one accurate observation. The streak is tracked correctly.

---

### Phase 3 — Work mode integration (2–3 weeks)

**Goal**: FlowState works alongside other applications without friction.

Deliverables:
- Desktop background mode (Linux, Windows): environment renders behind other application windows
- Dual monitor mode: environment on secondary display, work on primary
- Overlay mode: minimal floating timer overlay sits above working application
- System tray integration: session status visible in system tray without opening full window
- Auto-start option: FlowState launches minimised on login
- Hotkeys configurable: default hotkeys for start session, pause, richness up/down, skip to break

Milestone: Open a code editor. FlowState runs as desktop background. The environment is visible but not intrusive. Session timer is in the system tray. Coding session begins without switching windows.

---

### Phase 4 — Environment builder (3–4 weeks)

**Goal**: Users can create and share custom environments.

Deliverables:
- Environment builder UI: visual layer selection and configuration
- Synaesthesia module library exposed to builder (all 8 modules)
- Nyx patch library exposed to builder
- Evolution configuration panel
- Work and break state preview
- Save to personal library
- Export as `.flowenv` file
- Import `.flowenv` file
- Share widget: copies a share link or file path

Milestone: Build a custom environment from two visual modules and one audio patch. Save it. Export it. Import it in a fresh install and it appears correctly.

---

### Phase 5 — Polish and shipping (2–3 weeks)

**Goal**: FlowState is complete and shippable.

Deliverables:
- UI polish: typography, spacing, transitions, colour palette
- Onboarding: first-launch guided setup — choose first environment, set richness, run a short trial session
- Settings: session defaults, hotkeys, audio device selection, display mode
- Performance: 60fps on all environments on a mid-range GPU
- Linux AppImage, Windows installer, macOS DMG
- Itch.io page assets: screenshots, trailer, description

Milestone: FlowState is on itch.io. A first-time user completes onboarding, chooses an environment, and runs a 25-minute session.

---

### Phase 6 — Post-launch additions (ongoing)

- **Mobile companion**: iOS/Android app showing session timer and break notifications, synced to desktop. The visual environment is simplified for mobile but audio continues
- **Calendar integration**: pull today's tasks from Google Calendar to populate the work intention field
- **Community environment library**: browse and import environments created by other users, hosted on a simple server
- **Nature sounds expansion**: additional Nyx patches (tropical rain, desert wind, mountain stream, ocean swell)
- **Soundscape layering**: mix two ambient patches with independent volume controls (rain + fire simultaneously)
- **Heart rate integration**: optional pairing with a fitness tracker to detect actual focus depth and adapt richness accordingly

---

## 15. Monetisation Strategy

### Model: Freemium with subscription or one-time unlock

**Free tier**
- Three environments (Rainy Library, Deep Space Observatory, Autumn Forest)
- Pomodoro session type only
- Basic session log (count and duration, no analytics)
- Richness dial (full range)
- No environment builder

**Premium — two options:**

**One-time purchase (~£18)**: all twelve environments, all session types, full analytics and insights engine, environment builder, import/export, community environments

**Annual subscription (~£8/year)**: same as one-time, plus new environments as they are added (target: 2 new environments per year post-launch), access to future expansion packs

### Rationale

Offering both a one-time option and a subscription respects different user preferences. The subscription is only meaningfully better value if the user plans to use FlowState long-term and values new environment additions — which the core audience will. The one-time purchase suits users who want to own the tool outright.

The free tier must demonstrate the core value: one full session in the Rainy Library, with the richness dial, is enough for a hyperphantasic user to have the recognition experience. Three environments and the Pomodoro timer is genuinely useful — not artificially crippled.

---

## 16. Itch.io Launch Strategy

### Positioning
*"For minds that need a world to work in."*

The itch.io page opens with the insight, not the features. "Most focus apps give you nothing. For some people, nothing is the problem." Then: what happens when a hyperphantasic mind has nothing to look at. Then: what FlowState does instead.

A short video shows a work session in the Rainy Library. No UI visible. Rain plays. The timer counts quietly in a corner. The environment breathes. At interval end, the rain intensifies, the light shifts. Break. Then back to work.

No feature list. Just the experience.

### Trailer structure

- 5 seconds: rain on library windows, warm light, timer counting. No text.
- 5 seconds: the environment shifts. Rain intensifies. Break.
- 5 seconds: back to work state. Rain quietens. The environment knows.
- 5 seconds: switch environment — Space Observatory. Stars drift. Deep silence.
- 5 seconds: the richness dial being adjusted. Environment responds.
- 10 seconds: analytics view — streak, environment patterns, insights.
- 5 seconds: title. "FlowState. For minds that need a world to work in."

### Community seeding

- **ADHD and neurodivergent productivity communities**: Reddit r/ADHD, r/productivity — the "minimalism doesn't work for me" experience resonates powerfully here. Many ADHD users have exactly the hyperphantasic focus pattern
- **Hyperphantasia communities**: aphantasia.com, r/hyperphantasia — direct target audience
- **Remote work and home office communities**: r/digitalnomad, r/remotework — people who miss having a rich environment to work in
- **Deep work and productivity tool enthusiasts**: r/productivity, r/getdisciplined — the unusual "richness helps focus" insight is counterintuitive enough to generate discussion
- **Lofi and study music communities**: Reddit r/weddingplanning (the lofi-beats community is huge and engaged) — positioning FlowState as "lofi, but visual too"

### Launch timing

- Beta at Phase 1 completion (all environments, basic timer)
- Free beta to gather environment quality feedback before full analytics launch
- Full launch at Phase 5 with the complete product

---

## 17. Future Directions

### Integration with Synaesthesia

The most natural product connection. A Synaesthesia preset — a personal visual language built around a piece of music — could run as a FlowState environment. The user works to their own music, sees it rendered in their personal visual language, and the Nyx synthesis is the audio layer. FlowState and Synaesthesia share a renderer, share Nyx, and share the module vocabulary. A combined mode — "use my Synaesthesia preset as a FlowState environment with this ambient music" — is architecturally trivial and powerfully personal.

### Integration with StoryEngine

A StoryEngine story's sensory state could drive a FlowState environment. When writing a scene in a rainy library, the writer's working environment could mirror the scene's physical reality — rain on windows, warm lamplight, the same acoustic character as the setting. The writer inhabits the world they are building. This is achievable with a simple bridge: export the current `SensoryState` from StoryEngine's author mode, import it into FlowState as a live environment configuration.

### Biometric adaptation

With access to a heart rate variability (HRV) sensor or similar biometric feed, FlowState could adapt richness in real time based on detected focus depth. When HRV indicates high focus, richness decreases slightly to minimise distraction risk. When HRV indicates distraction or fatigue, richness increases slightly to re-engage the simulation engine and reset attention. This closes the feedback loop between the environment and the user's cognitive state.

### Focus research contribution

The question of whether ambient visual richness improves focus for hyperphantasic individuals has not been studied directly. FlowState, with opt-in data contribution, could be a research instrument: anonymised session data (environment richness, session quality ratings, session completion rates) contributed to researchers studying attentional differences. The Aphantasia Network and cognitive psychology research groups at universities would be natural partners.

---

*FlowState is built on the recognition that the right environment for focused work is not nothing. For some minds it is a world — one that breathes, that changes, that never demands attention, and that tells the inner simulation engine: you are not starving, you do not need to generate, there is enough here. Rest, and let the work be done.*
