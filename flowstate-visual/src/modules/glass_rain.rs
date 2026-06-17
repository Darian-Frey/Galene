//! GlassRain — screen-space refraction with running droplet trails on an implied
//! pane of glass (render doc §5.1). The most reused new primitive: Library,
//! Workshop, Greenhouse, Chart Room, Midnight City.
//!
//! Unlike other layers it reads the already-composited back buffer and offsets
//! sample UVs for refraction, so it composites with [`crate::layer::BlendMode::Refraction`].

use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};

use super::{FrameCtx, ModuleInit, VisualModule, FULLSCREEN_VS};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct GlassRain {
    pub rain_density: f32,
    pub droplet_size: f32,
    pub trail_length: f32,
    pub run_speed: f32,
    pub refraction_strength: f32,
    /// Frosted-haze amount, 0–1.
    pub glass_fog: f32,
}

impl Default for GlassRain {
    fn default() -> Self {
        Self {
            rain_density: 0.5,
            droplet_size: 0.5,
            trail_length: 0.5,
            run_speed: 0.5,
            refraction_strength: 0.3,
            glass_fog: 0.2,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Uniforms {
    params: [f32; 4], // rain_density, refraction_strength, glass_fog, time
}

/// Renders the rain-on-glass refraction layer. Unlike ordinary modules it reads
/// the composited backdrop (`FrameCtx::backdrop`) supplied by the compositor and
/// refracts it (render doc §5.1); [`VisualModule::reads_backdrop`] is `true`.
pub struct GlassRainModule {
    pipeline: wgpu::RenderPipeline,
    bgl: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
    uniform: wgpu::Buffer,
}

impl GlassRainModule {
    pub fn new(init: &ModuleInit) -> Self {
        let device = init.device;
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("glass_rain"),
            source: wgpu::ShaderSource::Wgsl(
                format!("{}\n{}", FULLSCREEN_VS, include_str!("../shaders/glass_rain.wgsl")).into(),
            ),
        });
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("glass_rain.sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });
        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("glass_rain.uniform"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("glass_rain.bgl"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("glass_rain.layout"),
            bind_group_layouts: &[Some(&bgl)],
            immediate_size: 0,
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("glass_rain.pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs"),
                buffers: &[],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs"),
                targets: &[Some(wgpu::ColorTargetState {
                    format: init.target_format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview_mask: None,
            cache: None,
        });
        Self {
            pipeline,
            bgl,
            sampler,
            uniform,
        }
    }
}

impl VisualModule for GlassRainModule {
    fn name(&self) -> &str {
        "GlassRain"
    }

    fn reads_backdrop(&self) -> bool {
        true
    }

    fn render(
        &mut self,
        ctx: &FrameCtx,
        target: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        let backdrop = ctx
            .backdrop
            .expect("GlassRain requires a backdrop (compositor refraction path)");

        let u = Uniforms {
            params: [
                ctx.params.get("rain_density").copied().unwrap_or(0.5),
                ctx.params.get("refraction_strength").copied().unwrap_or(0.3),
                ctx.params.get("glass_fog").copied().unwrap_or(0.2),
                ctx.time_secs,
            ],
        };
        ctx.queue.write_buffer(&self.uniform, 0, bytemuck::bytes_of(&u));

        let bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("glass_rain.bg"),
            layout: &self.bgl,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(backdrop),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: self.uniform.as_entire_binding(),
                },
            ],
        });

        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("glass_rain.pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                depth_slice: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &bind_group, &[]);
        pass.draw(0..3, 0..1);
    }
}
