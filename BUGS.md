# Bugs

Catalogue of bugs discovered during development. Per Maintenance Rule 8,
bugs are logged here when found, not silently fixed. The author decides
whether to fix immediately, defer, or leave alone.

Status vocabulary: open | fixed | wontfix | deferred.
Severity vocabulary: low | medium | high.

## Open

_None yet._

<!--
### BUG-001: {short title}
**Status:** open
**Found:** YYYY-MM-DD ({session/commit context})
**Location:** {path/to/file.rs:line, or "cross-cutting"}
**Severity:** {low | medium | high}
**Description.** {What's wrong and why it matters.}
**Reproduction.** {Minimum steps to trigger.}
**Notes.** {Related context, suggested fix, links to BUG/IMP/D/AV entries.}
-->

## Fixed

### BUG-001: Film grain `sin`-hash aliased into a full-screen cross-hatch weave
**Status:** fixed (2026-06-17)
**Found:** 2026-06-17 (reported from live-window screenshots — looked like the rain was broken)
**Location:** [flowstate-visual/src/shaders/post/post.wgsl](flowstate-visual/src/shaders/post/post.wgsl) (grain)
**Severity:** medium (pervasive visual artifact; no crash)
**Description.** The post-chain film grain used `fract(sin(dot(uv*1024, k)) * 43758.5)`.
At grain frequency the `sin` argument changes by hundreds of radians per pixel, so
it aliases into a structured diagonal cross-hatch *weave* over the entire frame
instead of white noise. Being a full-screen post effect it covered everything —
windows, walls, rain — and read as "the rain is broken."
**Reproduction.** Run the viewer (or `--example scene_render`); a regular weave
covers the whole frame, strongest in the dark areas. Static between frames in
character (so not obviously animated grain), which is what made it look structural.
**Notes.** Diagnosed by disabling grain (weave vanished entirely). Fixed by
replacing the hash with Dave Hoskins' `fract`/`dot` `hash12` (no `sin`), which
produces real white-noise grain. Same session also improved GlassRain
incidentally: a `pow(d,0.4)` density response so light rain is visible in the work
state, gated/sparser static drops (no lattice), and lower-frequency far-shelf
silhouettes (no moiré). See IMP-003.

## Won't Fix

_None yet._

## Deferred

_None yet._
