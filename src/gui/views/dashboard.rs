use eframe::egui;
use crate::gui::app::HytaleSaveEditor;

pub fn show(ui: &mut egui::Ui, app: &mut HytaleSaveEditor) {
    ui.heading("Dashboard");
    
    if let Some(path) = &app.current_path {
        ui.label(format!("Save Path: {:?}", path));
    } else {
        ui.label("Please open a save folder.");
    }

    if let Some(texture) = &app.preview_image {
        ui.image(texture);
    } else {
        ui.label("No preview image available.");
    }
}
