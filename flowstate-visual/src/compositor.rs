//! Multi-target layer compositing + post (render doc §3).
//!
//! Each layer is rendered to its own RGBA16F offscreen target, optionally
//! depth-of-field blurred, then composited back-to-front into an HDR (RGBA16F)
//! accumulation target with the layer's blend mode. The [`PostStage`] then runs
//! bloom → grade → vignette → grain → tone-map from the accumulation to the
//! output. This is render-doc §11 **steps 1–4**.

use crate::dof::DofBlur;
use crate::layer::{BlendMode, ResolvedParams};
use crate::modules::{FrameCtx, VisualModule};
use crate::post::{ColourGrade, PostChain, PostStage};

/// HDR format for all layer targets and the accumulation (render doc §3: light
/// layers exceed 1.0).
pub const LAYER_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba16Float;

/// Maximum DOF blur radius in pixels, reached at `depth_blur == 1.0`.
const MAX_BLUR_PX: f32 = 8.0;

/// Blur radius (px) for the frosted-glass backdrop handed to refraction layers.
const FRAME_BLUR_RADIUS: f32 = 7.0;

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
    width: u32,
    height: u32,
    layers: Vec<LayerBuf>,
    dof: DofBlur,
    composite_normal: wgpu::RenderPipeline,
    composite_additive: wgpu::RenderPipeline,
    accum_tex: wgpu::Texture,
    accum_view: wgpu::TextureView,
    /// A copy of the accumulation-so-far, for refraction layers to sample (§5.1).
    backdrop_tex: wgpu::Texture,
    backdrop_view: wgpu::TextureView,
    /// A strongly-blurred copy of the backdrop (frosted glass), and its scratch.
    backdrop_blur_view: wgpu::TextureView,
    blur_temp_view: wgpu::TextureView,
    post: PostStage,
    post_chain: PostChain,
    grade: ColourGrade,
}

impl Compositor {
    /// Allocate per-layer targets + the HDR accumulation, and build the DOF,
    /// composite, and post pipelines. `post_chain` / `grade` come from the scene.
    pub fn new(
        device: &wgpu::Device,
        width: u32,
        height: u32,
        output_format: wgpu::TextureFormat,
        specs: &[CompositeLayer],
        post_chain: PostChain,
        grade: ColourGrade,
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

        // Layers composite into the HDR accumulation, not the final output.
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
                        format: LAYER_FORMAT,
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

        let layers: Vec<LayerBuf> = specs
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

        let accum_tex = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("compositor.accum"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: LAYER_FORMAT,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_SRC,
            view_formats: &[],
        });
        let accum_view = accum_tex.create_view(&wgpu::TextureViewDescriptor::default());

        let backdrop_tex = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("compositor.backdrop"),
            size: wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: LAYER_FORMAT,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });
        let backdrop_view = backdrop_tex.create_view(&wgpu::TextureViewDescriptor::default());

        // Frosted-glass backdrop (and its separable-blur scratch).
        let make_blur_target = || {
            device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some("compositor.backdrop_blur"),
                    size: wgpu::Extent3d {
                        width,
                        height,
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
        let backdrop_blur_view = make_blur_target();
        let blur_temp_view = make_blur_target();

        let post = PostStage::new(device, width, height, output_format, &accum_view);

        Self {
            width,
            height,
            layers,
            dof: DofBlur::new(device, LAYER_FORMAT),
            composite_normal,
            composite_additive,
            accum_tex,
            accum_view,
            backdrop_tex,
            backdrop_view,
            backdrop_blur_view,
            blur_temp_view,
            post,
            post_chain,
            grade,
        }
    }

    /// Render one frame: each module → its layer target (+ DOF), composite
    /// back-to-front into the HDR accumulation, then run the post chain to
    /// `output_view`. `modules` / `params` are parallel to the layer specs.
    pub fn render_frame(
        &self,
        gpu: &crate::gpu::GpuContext,
        modules: &mut [&mut (dyn VisualModule + '_)],
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

        // 1. Each non-refraction layer → its target, then optional DOF blur.
        //    Refraction layers (GlassRain) are skipped here — they read the
        //    composited backdrop during step 2.
        for (i, layer) in self.layers.iter().enumerate() {
            if layer.blend == BlendMode::Refraction {
                continue;
            }
            let ctx = FrameCtx {
                device,
                queue,
                resolution: layer.size,
                time_secs,
                seed,
                params: &params[i],
                backdrop: None,
                backdrop_blur: None,
            };
            modules[i].render(&ctx, &layer.primary, &mut encoder);

            if layer.depth_blur > 0.0 {
                let radius = layer.depth_blur * MAX_BLUR_PX;
                self.dof.blur_in_place(
                    device,
                    &mut encoder,
                    &layer.primary,
                    &layer.temp,
                    layer.size,
                    radius,
                );
            }
        }

        // 2. Composite back-to-front into the HDR accumulation (layer 0 clears).
        for (i, layer) in self.layers.iter().enumerate() {
            if layer.blend == BlendMode::Refraction {
                // Copy the accumulation-so-far into the backdrop, then let the
                // module refract it back into the accumulation (render doc §5.1).
                encoder.copy_texture_to_texture(
                    wgpu::TexelCopyTextureInfo {
                        texture: &self.accum_tex,
                        mip_level: 0,
                        origin: wgpu::Origin3d::ZERO,
                        aspect: wgpu::TextureAspect::All,
                    },
                    wgpu::TexelCopyTextureInfo {
                        texture: &self.backdrop_tex,
                        mip_level: 0,
                        origin: wgpu::Origin3d::ZERO,
                        aspect: wgpu::TextureAspect::All,
                    },
                    wgpu::Extent3d {
                        width: self.width,
                        height: self.height,
                        depth_or_array_layers: 1,
                    },
                );

                // Frost the backdrop (backdrop → blur_temp → backdrop_blur) so the
                // refraction module can sample foggy glass cheaply.
                let hb = self.dof.pass_bind_group(
                    device,
                    &self.backdrop_view,
                    [1.0 / self.width as f32, 0.0],
                    FRAME_BLUR_RADIUS,
                );
                self.dof.record(&mut encoder, &hb, &self.blur_temp_view);
                let vb = self.dof.pass_bind_group(
                    device,
                    &self.blur_temp_view,
                    [0.0, 1.0 / self.height as f32],
                    FRAME_BLUR_RADIUS,
                );
                self.dof.record(&mut encoder, &vb, &self.backdrop_blur_view);

                let ctx = FrameCtx {
                    device,
                    queue,
                    resolution: (self.width, self.height),
                    time_secs,
                    seed,
                    params: &params[i],
                    backdrop: Some(&self.backdrop_view),
                    backdrop_blur: Some(&self.backdrop_blur_view),
                };
                modules[i].render(&ctx, &self.accum_view, &mut encoder);
                continue;
            }

            let load = if i == 0 {
                wgpu::LoadOp::Clear(wgpu::Color::BLACK)
            } else {
                wgpu::LoadOp::Load
            };
            let pipeline = match layer.blend {
                BlendMode::Additive => &self.composite_additive,
                BlendMode::Normal | BlendMode::Refraction => &self.composite_normal,
            };
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("composite.pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &self.accum_view,
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

        // 3. Post chain: accumulation → output.
        self.post.run(
            device,
            queue,
            &mut encoder,
            output_view,
            &self.post_chain,
            &self.grade,
            seed,
            (self.width, self.height),
        );

        queue.submit(std::iter::once(encoder.finish()));
    }
}
