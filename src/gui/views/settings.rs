use eframe::egui;
use crate::gui::app::{HytaleSaveEditor, Language};

pub fn show(ui: &mut egui::Ui, app: &mut HytaleSaveEditor) {
    ui.heading("Settings");
    ui.add_space(20.0);

    // --- Language Settings ---
    egui::Frame::group(ui.style())
        .fill(ui.visuals().faint_bg_color)
        .inner_margin(15.0)
        .show(ui, |ui| {
            ui.heading("Language / Dil");
            ui.add_space(10.0);
            
            ui.vertical(|ui| {
                ui.radio_value(&mut app.language, Language::English, "English");
                ui.radio_value(&mut app.language, Language::Turkish, "Türkçe");
            });
            
            ui.add_space(5.0);
            ui.label(egui::RichText::new(match app.language {
                Language::English => "Restart might be required for changes to fully apply.",
                Language::Turkish => "Değişikliklerin tam uygulanması için yeniden başlatma gerekebilir.",
            }).weak().small());
        });

    ui.add_space(20.0);

    // --- About Section ---
    egui::Frame::group(ui.style())
        .fill(ui.visuals().faint_bg_color)
        .inner_margin(15.0)
        .show(ui, |ui| {
            ui.heading("About");
            ui.add_space(10.0);
            
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Hytale Save Editor").strong());
                ui.label("v0.1.0");
            });
            
            ui.add_space(5.0);
            ui.label("A community-made tool for editing Hytale save files.");
            ui.add_space(10.0);
            ui.hyperlink_to("GitHub Repository", "https://github.com/EdgeTypE/hytale-save-editor");
            ui.add_space(5.0);
            ui.label(egui::RichText::new("Created by EdgeTypE").italics().weak());
            
            ui.separator();
            ui.label(egui::RichText::new("This tool is not affiliated with Hypixel Studios.").small().weak());
        });
}
