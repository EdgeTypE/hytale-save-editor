mod models;
mod io;
mod app;

use eframe::egui;

fn main() -> eframe::Result<()> {
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0])
            .with_min_inner_size([400.0, 300.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Hytale Save Editor",
        native_options,
        Box::new(|cc| Box::new(app::HytaleEditorApp::new(cc))),
    )
}
