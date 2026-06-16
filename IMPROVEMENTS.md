# Improvements

Catalogue of code-quality improvements, refactors, and architectural
changes proposed during development. Per Maintenance Rule 8, improvements
are logged here when noticed, not silently applied. The author decides
whether to apply, defer, or decline.

This is the dual of BUGS.md: bugs are broken; improvements work but
could be better.

Status vocabulary: suggested | applied | declined | deferred.
Effort vocabulary: trivial | small | medium | large.

## Suggested

### IMP-001: Blend effective richness across the work↔break transition
**Status:** suggested
**Found:** 2026-06-16 (while implementing the driver/richness logic)
**Location:** [flowstate-visual/src/driver.rs](flowstate-visual/src/driver.rs) `context()`, [flowstate-visual/src/layer.rs](flowstate-visual/src/layer.rs) `resolve_layer_params`
**Effort:** small
**Description.** `resolve_layer_params` derives the `RichnessMapping` from
`effective_richness(user_richness, ctx.state)`, where `state` is the discrete
target (Work or Break). So the richness mapping *steps* at the instant
`set_state` is called, while the authored work/break params blend smoothly over
the 60–90s transition via `state_blend`. The mismatch means richness-scaled
params (rain, particles, light) jump at transition start instead of easing.
**Proposal.** Pass a continuous effective richness into the resolver: lerp the
work and break effective-richness values by `state_blend`
(`lerp(user*0.5, 0.5+user*0.5, state_blend)`), and build the mapping from that.
**Trade-offs.** Adds a second, blend-aware richness path alongside the discrete
`effective_richness`; marginally more complex. The step may be visually
negligible once authored params already blend — worth confirming against the
real renderer before committing, which is why this is logged rather than applied.
**Notes.** Related to DECISIONS D-010 (richness resolution). Defer until the
renderer makes the transition visible.

<!--
### IMP-NNN: {short title}
**Status:** suggested
**Found:** YYYY-MM-DD ({session/commit context})
**Location:** {path/to/file.rs:line, or "cross-cutting"}
**Effort:** {trivial | small | medium | large}
**Description.** {What could be improved and why.}
**Proposal.** {How to do it.}
**Trade-offs.** {What we'd give up or risk. Required.}
**Notes.** {Related context, dependencies on other work.}
-->

## Applied

_None yet._

## Declined

_None yet._

## Deferred

_None yet._
