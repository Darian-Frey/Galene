//! GeometricField — low-detail architectural silhouettes (shelves, window
//! frames, tables, bulkheads, machinery), all impressionistic and usually
//! DOF-blurred (render doc §4). Selected by preset name.

use bytemuck::{Pod, Zeroable};

use super::{draw_fullscreen, fullscreen_uniform, FrameCtx, FullscreenUniform, ModuleInit, VisualModule};

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Uniforms {
    params: [f32; 4], // density, frame_darkness, storm_visibility, lamp_on
    info: [f32; 4],   // preset_id, aspect, _, _
}

pub struct GeometricFieldModule {
    fs: FullscreenUniform,
    preset_id: f32,
}

impl GeometricFieldModule {
    pub fn new(init: &ModuleInit, preset: &str) -> Self {
        let preset_id = match preset {
            "bookshelf_silhouette" => 0.0,
            "tall_windows" => 1.0,
            "reading_table" => 2.0,
            // Default to the bookshelf mass for unknown presets.
            _ => 0.0,
        };
        let fs = fullscreen_uniform(
            init,
            "geometric_field",
            include_str!("../shaders/geometric_field.wgsl"),
            std::mem::size_of::<Uniforms>() as u64,
        );
        Self { fs, preset_id }
    }
}

impl VisualModule for GeometricFieldModule {
    fn name(&self) -> &str {
        "GeometricField"
    }

    fn render(
        &mut self,
        ctx: &FrameCtx,
        target: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        let p = |k: &str, d: f32| ctx.params.get(k).copied().unwrap_or(d);
        let aspect = ctx.resolution.0 as f32 / ctx.resolution.1.max(1) as f32;
        let u = Uniforms {
            params: [
                p("density", 0.5),
                p("frame_darkness", 0.8),
                p("storm_visibility", 0.5),
                p("lamp_on", 1.0),
            ],
            info: [self.preset_id, aspect, 0.0, 0.0],
        };
        ctx.queue.write_buffer(&self.fs.uniform, 0, bytemuck::bytes_of(&u));
        draw_fullscreen(encoder, &self.fs, target);
    }
}
