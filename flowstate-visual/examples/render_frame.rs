//! Headless render of the Rainy Library base-atmosphere layer to a PPM image.
//!
//! Proves render-doc §11 step 1 visibly: module → offscreen RGBA16F target →
//! composite → output, read back and written to a file you can open. A windowed
//! surface path (on-screen, animated) comes later.
//!
//!     cargo run -p flowstate-visual --example render_frame
//!     # writes first_frame.ppm (open with any image viewer)

use std::io::Write;

use flowstate_visual::{module_init, render_module_to_rgba8, GpuContext, ShaderCanvasModule};

fn main() {
    let Some(gpu) = GpuContext::new_headless() else {
        eprintln!("No GPU adapter available — cannot render.");
        std::process::exit(1);
    };
    println!(
        "Rendering on: {} [{:?}]",
        gpu.adapter_info.name, gpu.adapter_info.backend,
    );

    let (w, h) = (512u32, 288u32); // 16:9

    let mut module = ShaderCanvasModule::new(&module_init(&gpu), "warm_interior_gradient");

    // The Rainy Library base_atmosphere layer, work state (FlowState §7.4).
    let mut params = std::collections::HashMap::new();
    params.insert("warmth".to_string(), 0.7);
    params.insert("darkness".to_string(), 0.65);

    let rgba = render_module_to_rgba8(&gpu, w, h, &mut module, &params);

    let path = "first_frame.ppm";
    let mut file = std::fs::File::create(path).expect("create output file");
    write!(file, "P6\n{w} {h}\n255\n").unwrap();
    let mut rgb = Vec::with_capacity((w * h * 3) as usize);
    for px in rgba.chunks_exact(4) {
        rgb.extend_from_slice(&px[0..3]); // drop alpha
    }
    file.write_all(&rgb).unwrap();

    println!("Wrote {path} ({w}×{h}). Open it with any image viewer.");
}
