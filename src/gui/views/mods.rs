use eframe::egui;
use crate::gui::app::HytaleSaveEditor;

pub fn show(ui: &mut egui::Ui, app: &mut HytaleSaveEditor) {
    ui.heading("Mods Configuration");
    if let Some(config) = &mut app.mods_config {
        egui::ScrollArea::vertical().show(ui, |ui| {
            for (mod_name, settings) in &mut config.mods {
                ui.horizontal(|ui| {
                    ui.checkbox(&mut settings.enabled, "");
                    ui.label(mod_name);
                });
            }
        });
    } else {
        ui.label("No mods config loaded.");
    }
}
