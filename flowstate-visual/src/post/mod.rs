//! The post-processing chain (render doc §5.3).
//!
//! Order: DOF far-haze → bloom → colour grade → vignette → film grain → tone-map.
//! The grade and grain do most of the painterly work; a clean digital image
//! looks wrong for this product.

pub mod bloom;
pub mod grade;
pub mod grain;
pub mod vignette;

use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};

pub use grade::ColourGrade;

use crate::dof::DofBlur;
use crate::modules::FULLSCREEN_VS;

/// Per-environment post-chain parameters, as stored in the scene definition.
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PostChain {
    pub bloom_threshold: f32,
    pub bloom_intensity: f32,
    pub vignette_amount: f32,
    pub vignette_softness: f32,
    /// Subtle — 0.02–0.06 typical.
    pub grain_amount: f32,
    /// Optional global far-haze depth-of-field.
    #[serde(default)]
    pub dof_far_haze: f32,
}

impl Default for PostChain {
    /// A no-op chain (no bloom / vignette / grain) for the lower-level render
    /// paths that have no scene post settings.
    fn default() -> Self {
        Self {
            bloom_threshold: 1.0,
            bloom_intensity: 0.0,
            vignette_amount: 0.0,
            vignette_softness: 0.5,
            grain_amount: 0.0,
            dof_far_haze: 0.0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct BloomUniform {
    params: [f32; 4], // x = threshold
}

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct PostUniform {
    lift: [f32; 4],
    gamma: [f32; 4],
    gain: [f32; 4],
    p0: [f32; 4], // bloom_intensity, vignette_amount, vignette_softness, grain_amount
    p1: [f32; 4], // seed, aspect, _, _
}

/// The runtime post-processing stage: HDR accumulation → bloom → grade →
/// vignette → grain → tone-map → output (render doc §3 step 7, §5.3).
pub struct PostStage {
    bloom_size: (u32, u32),
    bloom_view: wgpu::TextureView,
    bloom_temp_view: wgpu::TextureView,
    bloom_dof: DofBlur,
    extract_pipeline: wgpu::RenderPipeline,
    extract_bind_group: wgpu::BindGroup,
    extract_uniform: wgpu::Buffer,
    combine_pipeline: wgpu::RenderPipeline,
    combine_bind_group: wgpu::BindGroup,
    combine_uniform: wgpu::Buffer,
}

impl PostStage {
    /// Build the post stage reading the HDR `accum_view` and writing to
    /// `output_format`.
    pub fn new(
        device: &wgpu::Device,
        width: u32,
        height: u32,
        output_format: wgpu::TextureFormat,
        accum_view: &wgpu::TextureView,
    ) -> Self {
        let hdr = wgpu::TextureFormat::Rgba16Float;
        let bloom_size = ((width / 2).max(1), (height / 2).max(1));

        let make_target = || {
            device
                .create_texture(&wgpu::TextureDescriptor {
                    label: Some("post.bloom"),
                    size: wgpu::Extent3d {
                        width: bloom_size.0,
                        height: bloom_size.1,
                        depth_or_array_layers: 1,
                    },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format: hdr,
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                        | wgpu::TextureUsages::TEXTURE_BINDING,
                    view_formats: &[],
                })
                .create_view(&wgpu::TextureViewDescriptor::default())
        };
        let bloom_view = make_target();
        let bloom_temp_view = make_target();

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("post.sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        // --- Bloom bright-pass pipeline (accum → bloom) ---
        let extract_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("bloom_extract"),
            source: wgpu::ShaderSource::Wgsl(
                format!("{}\n{}", FULLSCREEN_VS, include_str!("../shaders/post/bloom_extract.wgsl"))
                    .into(),
            ),
        });
        let extract_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("bloom_extract.bgl"),
            entries: &[
                tex_entry(0),
                sampler_entry(1),
                uniform_entry(2),
            ],
        });
        let extract_uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("bloom_extract.uniform"),
            size: std::mem::size_of::<BloomUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let extract_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("bloom_extract.bg"),
            layout: &extract_bgl,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(accum_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: extract_uniform.as_entire_binding(),
                },
            ],
        });
        let extract_pipeline =
            fullscreen_pipeline(device, &extract_shader, &[&extract_bgl], hdr, "bloom_extract");

        // --- Combine pipeline (accum + bloom → output) ---
        let combine_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("post"),
            source: wgpu::ShaderSource::Wgsl(
                format!("{}\n{}", FULLSCREEN_VS, include_str!("../shaders/post/post.wgsl")).into(),
            ),
        });
        let combine_bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("post.bgl"),
            entries: &[tex_entry(0), sampler_entry(1), tex_entry(2), uniform_entry(3)],
        });
        let combine_uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("post.uniform"),
            size: std::mem::size_of::<PostUniform>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let combine_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("post.bg"),
            layout: &combine_bgl,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(accum_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&bloom_view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: combine_uniform.as_entire_binding(),
                },
            ],
        });
        let combine_pipeline = fullscreen_pipeline(
            device,
            &combine_shader,
            &[&combine_bgl],
            output_format,
            "post",
        );

        Self {
            bloom_size,
            bloom_view,
            bloom_temp_view,
            bloom_dof: DofBlur::new(device, hdr),
            extract_pipeline,
            extract_bind_group,
            extract_uniform,
            combine_pipeline,
            combine_bind_group,
            combine_uniform,
        }
    }

    /// Record the post passes: bloom bright-pass + blur, then the combine into
    /// `output_view`.
    #[allow(clippy::too_many_arguments)]
    pub fn run(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        encoder: &mut wgpu::CommandEncoder,
        output_view: &wgpu::TextureView,
        chain: &PostChain,
        grade: &ColourGrade,
        seed: u32,
        resolution: (u32, u32),
    ) {
        queue.write_buffer(
            &self.extract_uniform,
            0,
            bytemuck::bytes_of(&BloomUniform {
                params: [chain.bloom_threshold, 0.0, 0.0, 0.0],
            }),
        );
        render_fullscreen(encoder, &self.extract_pipeline, &self.extract_bind_group, &self.bloom_view);
        self.bloom_dof.blur_in_place(
            device,
            encoder,
            &self.bloom_view,
            &self.bloom_temp_view,
            self.bloom_size,
            4.0,
        );

        let (lift, gamma, gain) = grade.components();
        let aspect = resolution.0 as f32 / resolution.1.max(1) as f32;
        queue.write_buffer(
            &self.combine_uniform,
            0,
            bytemuck::bytes_of(&PostUniform {
                lift: [lift[0], lift[1], lift[2], 0.0],
                gamma: [gamma[0], gamma[1], gamma[2], 0.0],
                gain: [gain[0], gain[1], gain[2], 0.0],
                p0: [
                    chain.bloom_intensity,
                    chain.vignette_amount,
                    chain.vignette_softness,
                    chain.grain_amount,
                ],
                p1: [seed as f32, aspect, 0.0, 0.0],
            }),
        );
        render_fullscreen(encoder, &self.combine_pipeline, &self.combine_bind_group, output_view);
    }
}

fn tex_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Texture {
            sample_type: wgpu::TextureSampleType::Float { filterable: true },
            view_dimension: wgpu::TextureViewDimension::D2,
            multisampled: false,
        },
        count: None,
    }
}

fn sampler_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
        count: None,
    }
}

fn uniform_entry(binding: u32) -> wgpu::BindGroupLayoutEntry {
    wgpu::BindGroupLayoutEntry {
        binding,
        visibility: wgpu::ShaderStages::FRAGMENT,
        ty: wgpu::BindingType::Buffer {
            ty: wgpu::BufferBindingType::Uniform,
            has_dynamic_offset: false,
            min_binding_size: None,
        },
        count: None,
    }
}

fn fullscreen_pipeline(
    device: &wgpu::Device,
    shader: &wgpu::ShaderModule,
    bind_group_layouts: &[&wgpu::BindGroupLayout],
    format: wgpu::TextureFormat,
    label: &str,
) -> wgpu::RenderPipeline {
    let bgls: Vec<Option<&wgpu::BindGroupLayout>> =
        bind_group_layouts.iter().map(|b| Some(*b)).collect();
    let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some(label),
        bind_group_layouts: &bgls,
        immediate_size: 0,
    });
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(label),
        layout: Some(&layout),
        vertex: wgpu::VertexState {
            module: shader,
            entry_point: Some("vs"),
            buffers: &[],
            compilation_options: wgpu::PipelineCompilationOptions::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: Some("fs"),
            targets: &[Some(wgpu::ColorTargetState {
                format,
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
    })
}

fn render_fullscreen(
    encoder: &mut wgpu::CommandEncoder,
    pipeline: &wgpu::RenderPipeline,
    bind_group: &wgpu::BindGroup,
    target: &wgpu::TextureView,
) {
    let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label: Some("post.pass"),
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
    pass.set_pipeline(pipeline);
    pass.set_bind_group(0, bind_group, &[]);
    pass.draw(0..3, 0..1);
}
