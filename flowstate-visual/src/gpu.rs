//! The wgpu context — Galene's first GPU code.
//!
//! The render-doc §12 questions are resolved (DECISIONS D-011): there is no
//! Synaesthesia pipeline to adopt, so this is greenfield. The context is
//! headless-capable (no surface) so the compositor can be exercised in tests and
//! offscreen renders; a windowed surface path is added later.

/// Owns the wgpu device and queue (and the adapter info, for diagnostics).
pub struct GpuContext {
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub adapter_info: wgpu::AdapterInfo,
}

impl GpuContext {
    /// Create a headless context. Returns `None` if no GPU adapter is available,
    /// so tests skip gracefully on machines without a usable GPU.
    pub fn new_headless() -> Option<Self> {
        let instance =
            wgpu::Instance::new(wgpu::InstanceDescriptor::new_without_display_handle_from_env());

        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: None,
        }))
        .ok()?;

        let adapter_info = adapter.get_info();

        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: Some("galene-gpu"),
            required_features: wgpu::Features::empty(),
            required_limits: wgpu::Limits::downlevel_defaults(),
            memory_hints: wgpu::MemoryHints::Performance,
            experimental_features: wgpu::ExperimentalFeatures::disabled(),
            trace: wgpu::Trace::Off,
        }))
        .ok()?;

        Some(Self {
            device,
            queue,
            adapter_info,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn headless_context_initialises_or_skips() {
        match GpuContext::new_headless() {
            Some(gpu) => {
                println!(
                    "GPU adapter: {} [{:?}, {:?}]",
                    gpu.adapter_info.name, gpu.adapter_info.backend, gpu.adapter_info.device_type,
                );
            }
            None => eprintln!("no GPU adapter available — skipping GPU tests"),
        }
    }
}
