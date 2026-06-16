//! Galene desktop application entry point.
//!
//! Until the Tauri shell and the wgpu renderer land (gated on the render-doc §12
//! questions), this binary runs a **headless logic demo**: it loads the Rainy
//! Library scene, shows how the richness dial scales an authored parameter, then
//! drives a `FocusSession` and an `EnvironmentDriver` together through a
//! simulated Pomodoro — exercising the whole non-GPU pipeline end to end.
//!
//! The eventual command handlers are stubbed under [`commands`].

mod commands;
mod state;

use flowstate_core::analytics::SessionRecord;
use flowstate_core::{effective_richness, FocusSession, SessionEvent, SessionType, WorkBreakState};
use flowstate_visual::{EnvironmentDriver, Layer, Scene};

use state::AppState;

/// The flagship scene, bundled at compile time. Real environments load from disk
/// once the app has a data directory (Phase 1).
const RAINY_LIBRARY: &str = include_str!("../../environments/rainy_library.ron");

fn main() {
    println!("Galene {} — headless logic demo\n", env!("CARGO_PKG_VERSION"));

    let scene = Scene::from_ron(RAINY_LIBRARY).expect("bundled scene should parse");
    println!(
        "Environment: {} — {} layers, {:.0}-minute cycle",
        scene.name,
        scene.layers.len(),
        scene.cycle_minutes,
    );

    let mut app = AppState::new();
    let mut driver = EnvironmentDriver::new(scene);

    demo_richness_dial(&mut driver);
    demo_session(&mut driver, &mut app);
    demo_deep_work();

    println!(
        "\nAnalytics store now holds {} session(s).",
        app.analytics.records().len(),
    );
}

/// Show how the user's richness dial scales an authored work/break parameter —
/// `rain_on_glass.rain_density` (authored work 0.35, break 0.90). This is the
/// F-003 "the dial adjusts the rain" behaviour, in numbers.
fn demo_richness_dial(driver: &mut EnvironmentDriver) {
    let rain = rain_layer(driver);

    println!("\nRichness dial → rain_on_glass.rain_density:");
    println!("  dial    work    break");
    for &dial in &[0.0_f32, 0.45, 1.0] {
        driver.richness = dial;

        driver.state = WorkBreakState::Work;
        driver.state_blend = 0.0;
        let work = driver.resolve(&rain)["rain_density"];

        driver.state = WorkBreakState::Break;
        driver.state_blend = 1.0;
        let brk = driver.resolve(&rain)["rain_density"];

        println!("  {dial:.2}   {work:.3}   {brk:.3}");
    }
}

/// Drive a session and the environment together through a simulated Pomodoro,
/// printing each work↔break transition and a periodic status sample, then record
/// the session into the analytics store.
fn demo_session(driver: &mut EnvironmentDriver, app: &mut AppState) {
    driver.richness = 0.45; // FlowState §8.3 default
    driver.state = WorkBreakState::Work;
    driver.state_blend = 0.0;

    let mut session = FocusSession::new(
        SessionType::Pomodoro {
            work_min: 25.0,
            break_min: 5.0,
            long_break_min: 20.0,
            intervals_until_long: 4,
        },
        driver.scene.id.clone(),
        Some("the API integration".into()),
    );

    println!("\nSession: 25/5 Pomodoro · richness 0.45 · intention \"the API integration\"");
    println!("Simulating 70 minutes (Δt = 30s); transitions and 5-minute samples:\n");

    let rain = rain_layer(driver);
    let dt = 30.0_f32;
    let total = 70.0 * 60.0;
    let mut t = 0.0_f32;
    let mut next_sample = 0.0_f32;

    while t < total {
        match session.tick(dt) {
            SessionEvent::EnteredBreak { long } => {
                driver.set_state(WorkBreakState::Break);
                let kind = if long { "LONG" } else { "short" };
                println!(
                    "[{}] ✓ interval {} complete → {kind} BREAK",
                    fmt(t),
                    session.intervals_completed,
                );
            }
            SessionEvent::EnteredWork => {
                driver.set_state(WorkBreakState::Work);
                println!("[{}] ↻ break over → WORK resumes", fmt(t));
            }
            // A Pomodoro loops indefinitely; it never completes on its own.
            SessionEvent::None | SessionEvent::Completed => {}
        }

        driver.advance(dt);

        if t >= next_sample {
            let eff = effective_richness(driver.richness, driver.state);
            let rain_density = driver.resolve(&rain)["rain_density"];
            println!(
                "    {}  state={:<5} blend={:.2}  eff_richness={:.2}  rain={:.3}",
                fmt(t),
                format!("{:?}", driver.state),
                driver.state_blend,
                eff,
                rain_density,
            );
            next_sample += 300.0;
        }

        t += dt;
    }

    // Record the completed session (no wall clock in core; the app stamps time —
    // fixed here for a deterministic demo).
    app.analytics.record(SessionRecord {
        started_at: 0,
        duration_secs: total,
        environment_id: session.environment_id.clone(),
        session_type: "Pomodoro 25/5".into(),
        avg_richness: 0.45,
        richness_min: 0.45,
        richness_max: 0.45,
        intention: session.intention.clone(),
        intervals_completed: session.intervals_completed,
        intervals_total: session.intervals_completed,
        interruptions: 0,
        quality: None,
    });
}

/// Run a Deep Work session — one long block with a single midpoint break — and
/// print its boundaries through to completion.
fn demo_deep_work() {
    println!("\nDeep Work: 90-minute block · 5-minute midpoint break · intention \"chapter 3\"");
    let mut session = FocusSession::new(
        SessionType::DeepWork {
            total_min: 90.0,
            break_min: 5.0,
        },
        "rainy_library".into(),
        Some("chapter 3".into()),
    );

    let dt = 30.0_f32;
    let guard = 200.0 * 60.0; // safety bound
    let mut t = 0.0_f32;
    while t < guard {
        match session.tick(dt) {
            SessionEvent::EnteredBreak { .. } => println!("  [{}] midpoint → BREAK", fmt(t)),
            SessionEvent::EnteredWork => println!("  [{}] break over → second half", fmt(t)),
            SessionEvent::Completed => {
                println!("  [{}] ■ session COMPLETE", fmt(t));
                break;
            }
            SessionEvent::None => {}
        }
        t += dt;
    }
}

/// Clone the `rain_on_glass` layer out of the driver's scene for repeated resolution.
fn rain_layer(driver: &EnvironmentDriver) -> Layer {
    driver
        .scene
        .layers
        .iter()
        .find(|l| l.name == "rain_on_glass")
        .cloned()
        .expect("rainy_library has a rain_on_glass layer")
}

/// Format a simulated-seconds value as `mm:ss`.
fn fmt(secs: f32) -> String {
    let s = secs as u32;
    format!("{:02}:{:02}", s / 60, s % 60)
}
