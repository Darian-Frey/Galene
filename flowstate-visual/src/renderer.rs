//! The wgpu pipeline — device/queue/surface setup, the module trait wiring, and
//! the WGSL shaders, reused from Synaesthesia (render doc §3, FlowState §13).
//!
//! What FlowState adds on top lives in [`crate::compositor`] (multi-target
//! compositing), [`crate::dof`] (per-layer blur), [`crate::driver`] (parameter
//! source), and [`crate::modules`] (the new primitives).

// TODO: wgpu device/queue/surface setup and the shared module render path.
// Deferred pending the open questions in render doc §12:
//   §12.1 — is the shared module set a separate crate, or extract a `nyx-vis`?
//   §12.2 — does Synaesthesia render offscreen or to the swapchain?
//   §12.3 — is there an existing bloom effect to lift?
