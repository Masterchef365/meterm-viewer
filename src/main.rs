#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

const DEFAULT_SERVER: &str = "ws://127.0.0.1:5000";

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 300.0])
            .with_min_inner_size([300.0, 220.0])
            .with_icon(
                // NOTE: Adding an icon is optional
                eframe::icon_data::from_png_bytes(&include_bytes!("../assets/icon-256.png")[..])
                    .expect("Failed to load icon"),
            ),
        ..Default::default()
    };

    let server = std::env::args().skip(1).next()
        .unwrap_or_else(|| DEFAULT_SERVER.to_string());

    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Ok(Box::new(meterm_viewer::TemplateApp::new(cc, server)))),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    // Get the server from the URL
    let server: String = web_sys::window()
        .unwrap()
        .location()
        .href()
        .unwrap()
        .split("?srv=")
        .skip(1)
        .next()
        .map(|s| s.to_string())
        .unwrap_or_else(|| DEFAULT_SERVER.to_string());

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Ok(Box::new(meterm_viewer::TemplateApp::new(cc, server)))),
            )
            .await
            .expect("failed to start eframe");
    });
}
