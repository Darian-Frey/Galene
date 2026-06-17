//! # flowstate-visual
//!
//! Renders FlowState environments as composited 2.5D layers — a living painting
//! with depth, not a modelled 3D scene (render doc §1). Each layer is drawn to
//! an offscreen target, depth-of-field blurred, then composited back-to-front;
//! a post chain (bloom → grade → vignette → grain → tone-map) finishes the frame.
//!
//! Parameters come from the [`driver::EnvironmentDriver`], which replaces
//! Synaesthesia's audio-driven mapping graph.
//!
//! GPU wiring (wgpu) is deferred pending the render-doc §12 open questions; the
//! modules below currently define the data model, scene format, and parameter
//! resolution.

pub mod compositor;
pub mod dof;
pub mod driver;
pub mod gpu;
pub mod evolution_visual;
pub mod layer;
pub mod modules;
pub mod post;
pub mod renderer;
pub mod scene;

pub use compositor::{CompositeLayer, Compositor};
pub use dof::DofBlur;
pub use driver::EnvironmentDriver;
pub use gpu::GpuContext;
pub use layer::{BlendMode, DriverContext, Layer};
pub use modules::shader_canvas::ShaderCanvasModule;
pub use modules::{build_module, FrameCtx, ModuleInit, PlaceholderModule, VisualModule, FULLSCREEN_VS};
pub use renderer::{
    module_init, render_layers_to_rgba8, render_module_to_rgba8, render_scene_to_rgba8,
    SceneRenderer,
};
pub use scene::Scene;
