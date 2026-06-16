//! Shader Canvas — the base-atmosphere gradient layer behind every environment
//! (render doc §4, §7). The first module wired to the GPU.

use bytemuck::{Pod, Zeroable};

use super::{FrameCtx, ModuleInit, VisualModule, FULLSCREEN_VS};

/// 16-byte uniform block: `data.x` = warmth, `data.y` = darkness.
#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct Uniforms {
    data: [f32; 4],
}

pub struct ShaderCanvasModule {
    name: String,
    pipeline: wgpu::RenderPipeline,
    uniform_buf: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
}

impl ShaderCanvasModule {
    /// Build the gradient pipeline targeting `init.target_format`. `shader_name`
    /// is the scene's `ShaderCanvas(shader: …)` identifier, kept for diagnostics
    /// (per-shader variation arrives when more backgrounds are authored).
    pub fn new(init: &ModuleInit, shader_name: &str) -> Self {
        let device = init.device;

        let source = format!(
            "{}\n{}",
            FULLSCREEN_VS,
            include_str!("../shaders/shader_canvas.wgsl"),
        );
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader_canvas"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });

        let uniform_buf = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("shader_canvas.uniforms"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("shader_canvas.bgl"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("shader_canvas.bg"),
            layout: &bgl,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buf.as_entire_binding(),
            }],
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("shader_canvas.layout"),
            bind_group_layouts: &[Some(&bgl)],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("shader_canvas.pipeline"),
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
            name: shader_name.to_string(),
            pipeline,
            uniform_buf,
            bind_group,
        }
    }
}

impl VisualModule for ShaderCanvasModule {
    fn name(&self) -> &str {
        &self.name
    }

    fn render(
        &mut self,
        ctx: &FrameCtx,
        target: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        let warmth = ctx.params.get("warmth").copied().unwrap_or(0.5);
        let darkness = ctx.params.get("darkness").copied().unwrap_or(0.5);
        let u = Uniforms {
            data: [warmth, darkness, 0.0, 0.0],
        };
        ctx.queue.write_buffer(&self.uniform_buf, 0, bytemuck::bytes_of(&u));

        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("shader_canvas.pass"),
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
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.draw(0..3, 0..1);
    }
}
