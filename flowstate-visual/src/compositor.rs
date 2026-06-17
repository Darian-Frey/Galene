//! Multi-target layer compositing (render doc §3).
//!
//! Each layer is rendered to its own RGBA16F offscreen target, optionally
//! depth-of-field blurred (proportional to `depth_blur`), then composited
//! back-to-front into the output with the layer's blend mode. This is render-doc
//! §11 **steps 1–2**: offscreen targets, per-layer DOF, and the composite.
//!
//! Layers currently composite straight into the (LDR) output. The HDR
//! accumulation target + tone-mapping post pass arrive with the post chain
//! (step 4), when additive light layers and bloom need headroom > 1.0.

use crate::dof::DofBlur;
use crate::layer::{BlendMode, ResolvedParams};
use crate::modules::{FrameCtx, VisualModule};

/// HDR format for all layer targets (render doc §3: light layers exceed 1.0).
pub const LAYER_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;

/// Maximum DOF blur radius in pixels, reached at `depth_blur == 1.0`.
const MAX_BLUR_PX: f32 = 8.0;

/// The compositing properties of one layer (the subset the compositor needs to
/// allocate targets and choose blend/blur). Drawn from the scene's `Layer`.
pub struct CompositeLayer {
    /// Render-target scale (blurred far layers can use < 1.0).
    pub resolution_scale: f32,
    /// 0.0 = sharp, 1.0 = maximum defocus.
    pub depth_blur: f32,
    pub blend: BlendMode,
}

struct LayerBuf {
    size: (u32, u32),
    primary: wgpu::TextureView,
    temp: wgpu::TextureView,
    depth_blur: f32,
    blend: BlendMode,
    composite_bg: wgpu::BindGroup,
}

pub struct Compositor {
    layers: Vec<LayerBuf>,
    dof: DofBlur,
    composite_normal: wgpu::RenderPipeline,
    composite_additive: wgpu::RenderPipeline,
}

impl Compositor {
    /// Allocate per-layer targets and build the DOF + composite pipelines that
    /// write to `output_format`.
    pub fn new(
        device: &wgpu::Device,
        width: u32,
        height: u32,
        output_format: wgpu::TextureFormat,
        specs: &[CompositeLayer],
    ) -> Self {
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("compositor.sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        let source = format!(
            "{}\n{}",
            crate::modules::FULLSCREEN_VS,
            include_str!("shaders/composite.wgsl"),
        );
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("composite"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });

        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("composite.bgl"),
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
            ],
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("composite.layout"),
            bind_group_layouts: &[Some(&bgl)],
            immediate_size: 0,
        });

        let make_pipeline = |blend: Option<wgpu::BlendState>| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("composite.pipeline"),
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
                        format: output_format,
                        blend,
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                    compilation_options: wgpu::PipelineCompilationOptions::default(),
                }),
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                multiview_mask: None,
                cache: None,
            })
        };

        let composite_normal = make_pipeline(Some(wgpu::BlendState::ALPHA_BLENDING));
        let composite_additive = make_pipeline(Some(wgpu::BlendState {
            color: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            },
            alpha: wgpu::BlendComponent {
                src_factor: wgpu::BlendFactor::One,
                dst_factor: wgpu::BlendFactor::One,
                operation: wgpu::BlendOperation::Add,
            },
        }));

        let layers = specs
            .iter()
            .map(|spec| {
                let scale = spec.resolution_scale.clamp(0.05, 1.0);
                let size = (
                    ((width as f32 * scale) as u32).max(1),
                    ((height as f32 * scale) as u32).max(1),
                );
                let make_target = || {
                    device
                        .create_texture(&wgpu::TextureDescriptor {
                            label: Some("compositor.layer"),
                            size: wgpu::Extent3d {
                                width: size.0,
                                height: size.1,
                                depth_or_array_layers: 1,
                            },
                            mip_level_count: 1,
                            sample_count: 1,
                            dimension: wgpu::TextureDimension::D2,
                            format: LAYER_FORMAT,
                            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                                | wgpu::TextureUsages::TEXTURE_BINDING,
                            view_formats: &[],
                        })
                        .create_view(&wgpu::TextureViewDescriptor::default())
                };
                let primary = make_target();
                let temp = make_target();
                let composite_bg = device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("compositor.layer.bg"),
                    layout: &bgl,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&primary),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&sampler),
                        },
                    ],
                });
                LayerBuf {
                    size,
                    primary,
                    temp,
                    depth_blur: spec.depth_blur,
                    blend: spec.blend,
                    composite_bg,
                }
            })
            .collect();

        Self {
            layers,
            dof: DofBlur::new(device, LAYER_FORMAT),
            composite_normal,
            composite_additive,
        }
    }

    /// Render one frame: draw each module into its layer target, blur layers with
    /// `depth_blur > 0`, then composite back-to-front into `output_view`.
    /// `modules` and `params` are parallel to the layer specs (back-to-front).
    pub fn render_frame(
        &self,
        gpu: &crate::gpu::GpuContext,
        modules: &mut [&mut dyn VisualModule],
        params: &[ResolvedParams],
        time_secs: f32,
        seed: u32,
        output_view: &wgpu::TextureView,
    ) {
        assert_eq!(modules.len(), self.layers.len(), "modules vs layers");
        assert_eq!(params.len(), self.layers.len(), "params vs layers");

        let device = &gpu.device;
        let queue = &gpu.queue;
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("compositor.encoder"),
        });

        // 1. Each layer → its target, then optional DOF blur.
        for (i, layer) in self.layers.iter().enumerate() {
            let ctx = FrameCtx {
                device,
                queue,
                resolution: layer.size,
                time_secs,
                seed,
                params: &params[i],
                backdrop: None,
            };
            modules[i].render(&ctx, &layer.primary, &mut encoder);

            if layer.depth_blur > 0.0 {
                let radius = layer.depth_blur * MAX_BLUR_PX;
                self.dof
                    .blur_in_place(device, &mut encoder, &layer.primary, &layer.temp, layer.size, radius);
            }
        }

        // 2. Composite back-to-front into the output (first layer clears it).
        for (i, layer) in self.layers.iter().enumerate() {
            let load = if i == 0 {
                wgpu::LoadOp::Clear(wgpu::Color::BLACK)
            } else {
                wgpu::LoadOp::Load
            };
            let pipeline = match layer.blend {
                BlendMode::Additive => &self.composite_additive,
                // Refraction is handled specially later (render doc §5.1); treat
                // as normal alpha for now.
                BlendMode::Normal | BlendMode::Refraction => &self.composite_normal,
            };
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("composite.pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: output_view,
                    resolve_target: None,
                    depth_slice: None,
                    ops: wgpu::Operations {
                        load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            pass.set_pipeline(pipeline);
            pass.set_bind_group(0, &layer.composite_bg, &[]);
            pass.draw(0..3, 0..1);
        }

        queue.submit(std::iter::once(encoder.finish()));
    }
}
