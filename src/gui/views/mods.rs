use eframe::egui;
use crate::gui::app::HytaleSaveEditor;

pub fn show(ui: &mut egui::Ui, app: &mut HytaleSaveEditor) {
    ui.heading("Mods Configuration");
    if let Some(config) = &mut app.mods_config {
        egui::ScrollArea::vertical().show(ui, |ui| {
            for (mod_name, settings) in &mut config.mods {
                ui.group(|ui| {
                     ui.horizontal(|ui| {
                        ui.checkbox(&mut settings.enabled, "");
                        ui.strong(mod_name);
                    });
                    
                    if let Some(manifest) = app.manifests.get(mod_name) {
                        ui.indent("mod_info", |ui| {
                             ui.horizontal(|ui| {
                                 ui.label(format!("v{}", manifest.version));
                                 ui.separator();
                                 if let Some(authors) = &manifest.authors {
                                     let author_names: Vec<String> = authors.iter().map(|a| a.name.clone()).collect();
                                     ui.label(format!("By: {}", author_names.join(", ")));
                                 }
                             });
                             ui.label(&manifest.description);
                             if let Some(website) = &manifest.website {
                                 ui.hyperlink(website);
                             }
                        });
                    } else {
                        ui.indent("mod_missing", |ui| {
                            ui.label(egui::RichText::new("Manifest not found").italics().weak());
                        });
                    }
                });
            }
        });
    } else {
        ui.label("No mods config loaded.");
    }
}
