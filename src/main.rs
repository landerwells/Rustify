#![warn(clippy::all, rust_2018_idioms)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

// When compiling natively:
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    use eframe::IconData;
    use image::io::Reader as ImageReader;
    use std::path::Path;

    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    // Load the image
    let img_path = Path::new("assets/rustify.png");
    let img = ImageReader::open(img_path)
        .expect("Failed to open icon path")
        .decode()
        .expect("Failed to decode icon")
        .to_rgba8();

    let (width, height) = img.dimensions();
    let rgba = img.into_raw();

    let native_options = eframe::NativeOptions {
        icon_data: Some(IconData {
            rgba,
            width,
            height,
        }),
        initial_window_size: Some([400.0, 300.0].into()),
        min_window_size: Some([300.0, 220.0].into()),
        ..Default::default()
    };
    eframe::run_native(
        "Rustify",
        native_options,
        Box::new(|cc| Box::new(Rustify::TemplateApp::new(cc))),
    )
}

// When compiling to web using trunk:
#[cfg(target_arch = "wasm32")]
fn main() {
    // Redirect `log` message to `console.log` and friends:
    eframe::WebLogger::init(log::LevelFilter::Debug).ok();

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "the_canvas_id", // hardcode it
                web_options,
                Box::new(|cc| Box::new(eframe_template::TemplateApp::new(cc))),
            )
            .await
            .expect("failed to start eframe");
    });
}
