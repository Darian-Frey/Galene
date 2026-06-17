//! Per-layer depth-of-field blur (render doc §3 step 4).
//!
//! A separable Gaussian: a horizontal pass then a vertical pass. The blur radius
//! is proportional to a layer's `depth_blur`, so far layers defocus while near
//! layers stay sharp — the separation that does most of the "real place" work.
//! Heavily-blurred layers can render at reduced resolution (`resolution_scale`),
//! the biggest performance lever (render doc §10).

use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

use crate::modules::FULLSCREEN_VS;

#[repr(C)]
#[derive(Clone, Copy, Pod, Zeroable)]
struct BlurUniform {
    dir: [f32; 2],
    radius: f32,
    _pad: f32,
}

/// A reusable separable-Gaussian blur pipeline for a given texture format.
pub struct DofBlur {
    pipeline: wgpu::RenderPipeline,
    bgl: wgpu::BindGroupLayout,
    sampler: wgpu::Sampler,
}

impl DofBlur {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        let source = format!("{}\n{}", FULLSCREEN_VS, include_str!("shaders/dof.wgsl"));
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("dof"),
            source: wgpu::ShaderSource::Wgsl(source.into()),
        });

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("dof.sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::MipmapFilterMode::Nearest,
            ..Default::default()
        });

        let bgl = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("dof.bgl"),
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
            label: Some("dof.layout"),
            bind_group_layouts: &[Some(&bgl)],
            immediate_size: 0,
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("dof.pipeline"),
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
        });

        Self {
            pipeline,
            bgl,
            sampler,
        }
    }

    /// Build a bind group for one blur pass sampling `src` along `dir` with the
    /// given pixel `radius`. (The uniform buffer is kept alive by the bind group.)
    pub fn pass_bind_group(
        &self,
        device: &wgpu::Device,
        src: &wgpu::TextureView,
        dir: [f32; 2],
        radius: f32,
    ) -> wgpu::BindGroup {
        let uniform = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("dof.uniform"),
            contents: bytemuck::bytes_of(&BlurUniform {
                dir,
                radius,
                _pad: 0.0,
            }),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("dof.bg"),
            layout: &self.bgl,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(src),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: uniform.as_entire_binding(),
                },
            ],
        })
    }

    /// Record one blur pass: draw the fullscreen triangle sampling `bind` into `dst`.
    pub fn record(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        bind: &wgpu::BindGroup,
        dst: &wgpu::TextureView,
    ) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("dof.pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: dst,
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
        pass.set_bind_group(0, bind, &[]);
        pass.draw(0..3, 0..1);
    }

    /// Blur `primary` in place using `temp` as scratch (horizontal → vertical).
    pub fn blur_in_place(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        primary: &wgpu::TextureView,
        temp: &wgpu::TextureView,
        size: (u32, u32),
        radius: f32,
    ) {
        let (w, h) = size;
        let h_bind = self.pass_bind_group(device, primary, [1.0 / w as f32, 0.0], radius);
        self.record(encoder, &h_bind, temp);
        let v_bind = self.pass_bind_group(device, temp, [0.0, 1.0 / h as f32], radius);
        self.record(encoder, &v_bind, primary);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gpu::GpuContext;

    #[test]
    fn gaussian_softens_a_hard_edge() {
        let Some(gpu) = GpuContext::new_headless() else {
            eprintln!("no GPU adapter — skipping DOF test");
            return;
        };
        let device = &gpu.device;
        let queue = &gpu.queue;
        let (w, h) = (64u32, 8u32);
        let format = wgpu::TextureFormat::Rgba8Unorm;

        let make = |usage| {
            device.create_texture(&wgpu::TextureDescriptor {
                label: Some("dof.test"),
                size: wgpu::Extent3d {
                    width: w,
                    height: h,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format,
                usage,
                view_formats: &[],
            })
        };
        let src = make(
            wgpu::TextureUsages::RENDER_ATTACHMENT
                | wgpu::TextureUsages::TEXTURE_BINDING
                | wgpu::TextureUsages::COPY_DST
                | wgpu::TextureUsages::COPY_SRC,
        );
        let temp = make(
            wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
        );
        let src_view = src.create_view(&wgpu::TextureViewDescriptor::default());
        let temp_view = temp.create_view(&wgpu::TextureViewDescriptor::default());

        // Hard vertical edge: left half black, right half white.
        let mut data = vec![0u8; (w * h * 4) as usize];
        for y in 0..h {
            for x in 0..w {
                let i = ((y * w + x) * 4) as usize;
                let v = if x >= w / 2 { 255 } else { 0 };
                data[i] = v;
                data[i + 1] = v;
                data[i + 2] = v;
                data[i + 3] = 255;
            }
        }
        queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &src,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &data,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(w * 4),
                rows_per_image: Some(h),
            },
            wgpu::Extent3d {
                width: w,
                height: h,
                depth_or_array_layers: 1,
            },
        );

        let dof = DofBlur::new(device, format);
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("dof.test.encoder"),
        });
        dof.blur_in_place(device, &mut encoder, &src_view, &temp_view, (w, h), 4.0);
        queue.submit(std::iter::once(encoder.finish()));

        let px = crate::renderer::read_texture_rgba8(&gpu, &src, w, h);
        let row = h / 2;
        let red = |x: u32| px[((row * w + x) * 4) as usize];

        // The hard edge at the centre is now a ramp: the black side just left of
        // centre picked up light, the white side just right lost some.
        assert!(red(w / 2 - 1) > 0, "edge did not soften on the dark side");
        assert!(red(w / 2) < 255, "edge did not soften on the bright side");
        // The overall dark-left / bright-right structure is preserved.
        assert!(red(2) < red(w - 3), "gradient direction lost");
    }
}
