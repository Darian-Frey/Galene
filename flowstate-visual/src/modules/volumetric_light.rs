//! VolumetricLight — soft additive bloom pools and optional light shafts for
//! lamps, fires, station lights, and geothermal glow (render doc §5.2).

use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};

use super::{draw_fullscreen, fullscreen_uniform, FrameCtx, FullscreenUniform, ModuleInit, VisualModule};

/// A single placeable light source within a VolumetricLight layer.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct LightSource {
    /// Screen-space position, 0–1 on each axis.
    pub pos: (f32, f32),
    /// HDR colour; intensity may exceed 1.0 so bloom catches it.
    pub colour: (f32, f32, f32),
    pub radius: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumetricLight {
    pub sources: Vec<LightSource>,
    pub falloff: f32,
    /// 0 = pool only, >0 = directional god rays.
    pub shaft_strength: f32,
    /// 0–1 flicker amount.
    pub flicker: f32,
}

const MAX_SOURCES: usize = 4;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct GpuSource {
    pos_radius: [f32; 4],
    colour: [f32; 4],
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Uniforms {
    head: [f32; 4], // count, intensity, flicker, aspect
    srcs: [GpuSource; MAX_SOURCES],
}

/// Renders the soft additive light pools for a VolumetricLight layer. God rays
/// (`shaft_strength`) are not yet implemented.
pub struct VolumetricLightModule {
    fs: FullscreenUniform,
    count: usize,
    srcs: [GpuSource; MAX_SOURCES],
}

impl VolumetricLightModule {
    pub fn new(init: &ModuleInit, sources: &[LightSource]) -> Self {
        let mut srcs = [GpuSource {
            pos_radius: [0.0; 4],
            colour: [0.0; 4],
        }; MAX_SOURCES];
        let count = sources.len().min(MAX_SOURCES);
        for (dst, s) in srcs.iter_mut().zip(sources.iter()).take(count) {
            dst.pos_radius = [s.pos.0, s.pos.1, s.radius, 0.0];
            dst.colour = [s.colour.0, s.colour.1, s.colour.2, 0.0];
        }
        let fs = fullscreen_uniform(
            init,
            "volumetric_light",
            include_str!("../shaders/volumetric_light.wgsl"),
            std::mem::size_of::<Uniforms>() as u64,
        );
        Self { fs, count, srcs }
    }
}

impl VisualModule for VolumetricLightModule {
    fn name(&self) -> &str {
        "VolumetricLight"
    }

    fn render(
        &mut self,
        ctx: &FrameCtx,
        target: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        let intensity = ctx.params.get("intensity").copied().unwrap_or(0.7);
        let flicker = ctx.params.get("flicker").copied().unwrap_or(0.0);
        let aspect = ctx.resolution.0 as f32 / ctx.resolution.1.max(1) as f32;
        let u = Uniforms {
            head: [self.count as f32, intensity, flicker, aspect],
            srcs: self.srcs,
        };
        ctx.queue.write_buffer(&self.fs.uniform, 0, bytemuck::bytes_of(&u));
        draw_fullscreen(encoder, &self.fs, target);
    }
}

// TODO(phase-1): directional god rays (radial blur from the source) when
// `shaft_strength > 0`, and flicker driven by the host time/seed.
