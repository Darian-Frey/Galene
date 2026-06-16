//! Top-level render entry points.
//!
//! The windowed/surface render loop (driven by the app) is added later. For now
//! this provides a **headless** single-frame render to an RGBA8 buffer, which is
//! how the offscreen→composite path is exercised and tested without a window
//! (render-doc §11 step 1). The render-doc §12 questions are resolved (D-011):
//! this stack is greenfield in Galene.

use crate::compositor::{Compositor, LAYER_FORMAT};
use crate::gpu::GpuContext;
use crate::layer::ResolvedParams;
use crate::modules::{ModuleInit, VisualModule};

/// Build a `ModuleInit` for constructing modules that render into the layer
/// target (RGBA16F).
pub fn module_init<'a>(gpu: &'a GpuContext) -> ModuleInit<'a> {
    ModuleInit {
        device: &gpu.device,
        queue: &gpu.queue,
        target_format: LAYER_FORMAT,
    }
}

/// Render a single module through the compositor at `width`×`height` and read
/// the result back as tightly-packed RGBA8 (`width*height*4` bytes).
pub fn render_module_to_rgba8(
    gpu: &GpuContext,
    width: u32,
    height: u32,
    module: &mut dyn VisualModule,
    params: &ResolvedParams,
) -> Vec<u8> {
    let device = &gpu.device;
    let queue = &gpu.queue;
    let output_format = wgpu::TextureFormat::Rgba8Unorm;

    let output_tex = device.create_texture(&wgpu::TextureDescriptor {
        label: Some("render.output"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: output_format,
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
        view_formats: &[],
    });
    let output_view = output_tex.create_view(&wgpu::TextureViewDescriptor::default());

    let compositor = Compositor::new(device, width, height, output_format);
    compositor.render_frame(gpu, module, params, 0.0, 0, &output_view);

    // Copy the output texture into a mappable buffer (rows padded to 256 bytes).
    let unpadded_bytes_per_row = width * 4;
    let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
    let padded_bytes_per_row = unpadded_bytes_per_row.div_ceil(align) * align;

    let buffer = device.create_buffer(&wgpu::BufferDescriptor {
        label: Some("render.readback"),
        size: (padded_bytes_per_row * height) as u64,
        usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("render.copy"),
    });
    encoder.copy_texture_to_buffer(
        wgpu::TexelCopyTextureInfo {
            texture: &output_tex,
            mip_level: 0,
            origin: wgpu::Origin3d::ZERO,
            aspect: wgpu::TextureAspect::All,
        },
        wgpu::TexelCopyBufferInfo {
            buffer: &buffer,
            layout: wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(padded_bytes_per_row),
                rows_per_image: Some(height),
            },
        },
        wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
    );
    queue.submit(std::iter::once(encoder.finish()));

    // Map and read back.
    let slice = buffer.slice(..);
    let (tx, rx) = std::sync::mpsc::channel();
    slice.map_async(wgpu::MapMode::Read, move |r| {
        let _ = tx.send(r);
    });
    let _ = device.poll(wgpu::PollType::wait_indefinitely());
    rx.recv().expect("map callback").expect("buffer map");

    let mapped = slice.get_mapped_range();
    let mut out = Vec::with_capacity((unpadded_bytes_per_row * height) as usize);
    for row in 0..height {
        let start = (row * padded_bytes_per_row) as usize;
        let end = start + unpadded_bytes_per_row as usize;
        out.extend_from_slice(&mapped[start..end]);
    }
    drop(mapped);
    buffer.unmap();

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::shader_canvas::ShaderCanvasModule;

    #[test]
    fn shader_canvas_renders_a_vertical_gradient() {
        let Some(gpu) = GpuContext::new_headless() else {
            eprintln!("no GPU adapter — skipping render test");
            return;
        };

        let mut module = ShaderCanvasModule::new(&module_init(&gpu), "warm_interior_gradient");

        let mut params = ResolvedParams::new();
        params.insert("warmth".into(), 0.8);
        params.insert("darkness".into(), 0.2);

        let (w, h) = (64u32, 64u32);
        let px = render_module_to_rgba8(&gpu, w, h, &mut module, &params);
        assert_eq!(px.len() as u32, w * h * 4);

        // Sample the red channel of the centre column at the top vs the bottom.
        let red_at = |row: u32| -> u8 {
            let x = w / 2;
            px[((row * w + x) * 4) as usize]
        };
        let top = red_at(1);
        let bottom = red_at(h - 2);

        // The gradient warms toward the bottom → more red there than at the top.
        assert!(
            bottom > top,
            "expected a vertical gradient (bottom red {bottom} > top red {top})",
        );
        // And something actually rendered (not an all-black frame).
        assert!(px.iter().any(|&b| b > 0), "frame is entirely black");
    }
}
