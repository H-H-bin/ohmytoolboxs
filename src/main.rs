/*!
 * OhMyToolboxs - A comprehensive desktop toolbox application
 * 
 * This application provides various utility tools in a single, easy-to-use GUI.
 */

mod app;
mod config;
mod tools;
mod ui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native(
        "OhMyToolboxs",
        options,
        Box::new(|cc| {
            // This gives us image support:
            egui_extras::install_image_loaders(&cc.egui_ctx);
            
            Ok(Box::new(app::OhMyToolboxsApp::new(cc)))
        }),
    )
}
