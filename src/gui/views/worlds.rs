use eframe::egui;
use crate::gui::app::HytaleSaveEditor;

pub fn show(ui: &mut egui::Ui, app: &mut HytaleSaveEditor) {
    ui.heading("Worlds");
    if let Some(worlds) = &mut app.worlds {
        for (name, config) in worlds {
             ui.collapsing(name, |ui| {
                 ui.group(|ui| {
                     ui.label("General:");
                     ui.horizontal(|ui| {
                         ui.label("Display Name:");
                         ui.text_edit_singleline(&mut config.display_name);
                     });
                     ui.horizontal(|ui| {
                         ui.label("Seed:");
                         ui.add(egui::DragValue::new(&mut config.seed));
                     });
                      ui.horizontal(|ui| {
                         ui.label("Game Mode:");
                         ui.text_edit_singleline(&mut config.game_mode);
                     });
                     ui.checkbox(&mut config.is_pvp_enabled, "PVP Enabled");
                 });

                 ui.group(|ui| {
                    ui.label("Client Effects:");
                    ui.add(egui::Slider::new(&mut config.client_effects.sun_height_percent, 0.0..=100.0).text("Sun Height %"));
                    ui.add(egui::Slider::new(&mut config.client_effects.sun_angle_degrees, 0.0..=360.0).text("Sun Angle"));
                    ui.add(egui::Slider::new(&mut config.client_effects.sun_intensity, 0.0..=10.0).text("Sun Intensity"));
                 });
             });
        }
    } else {
        ui.label("No worlds loaded.");
    }
}
