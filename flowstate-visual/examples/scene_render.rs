//! Render the actual Rainy Library scene (render-doc §11 step 3): the
//! `EnvironmentDriver` resolves every layer's parameters from the richness dial,
//! work/break state, and evolution cycle, and `SceneRenderer` composites the
//! whole `.ron`-defined layer stack. ShaderCanvas is real; the other layers are
//! tinted placeholders until their modules land.
//!
//!     cargo run -p flowstate-visual --example scene_render
//!     # writes scene_work.ppm and scene_break.ppm (open with any image viewer)

use std::io::Write;

use flowstate_core::WorkBreakState;
use flowstate_visual::{render_scene_to_rgba8, EnvironmentDriver, GpuContext, Scene, SceneRenderer};

const RAINY_LIBRARY: &str = include_str!("../../environments/rainy_library.ron");

fn write_ppm(path: &str, w: u32, h: u32, rgba: &[u8]) {
    let mut file = std::fs::File::create(path).expect("create output file");
    write!(file, "P6\n{w} {h}\n255\n").unwrap();
    let mut rgb = Vec::with_capacity((w * h * 3) as usize);
    for px in rgba.chunks_exact(4) {
        rgb.extend_from_slice(&px[0..3]);
    }
    file.write_all(&rgb).unwrap();
}

fn main() {
    let Some(gpu) = GpuContext::new_headless() else {
        eprintln!("No GPU adapter available — cannot render.");
        std::process::exit(1);
    };
    println!("Rendering on: {} [{:?}]", gpu.adapter_info.name, gpu.adapter_info.backend);

    let scene = Scene::from_ron(RAINY_LIBRARY).expect("scene parses");
    println!("Scene: {} ({} layers)", scene.name, scene.layers.len());

    let (w, h) = (512u32, 288u32);
    let mut driver = EnvironmentDriver::new(scene);
    let mut renderer =
        SceneRenderer::new(&gpu, w, h, wgpu::TextureFormat::Rgba8UnormSrgb, &driver.scene);

    // Work state at the viewer's default richness, mid-animation (t = 5s).
    driver.richness = 0.45;
    driver.state = WorkBreakState::Work;
    driver.state_blend = 0.0;
    let work = render_scene_to_rgba8(&gpu, w, h, &mut renderer, &driver, 5.0, 0);
    write_ppm("scene_work.ppm", w, h, &work);

    // Break state, high richness (0.9).
    driver.richness = 0.9;
    driver.state = WorkBreakState::Break;
    driver.state_blend = 1.0;
    let brk = render_scene_to_rgba8(&gpu, w, h, &mut renderer, &driver, 5.0, 0);
    write_ppm("scene_break.ppm", w, h, &brk);

    println!("Wrote scene_work.ppm and scene_break.ppm — the dial/state drive the difference.");
}
