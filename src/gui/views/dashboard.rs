use eframe::egui;
use crate::gui::app::HytaleSaveEditor;
use std::path::PathBuf;

// Define an enum to handle actions deferred until after borrowing ends
enum DashboardAction {
    None,
    CloseActive,
    Load(PathBuf),
    OpenDialog,
}

pub fn show(ui: &mut egui::Ui, app: &mut HytaleSaveEditor) {
    let mut action = DashboardAction::None;

    // --- Active Save View ---
    if let Some(path) = app.current_path.clone() {
        ui.horizontal(|ui| {
             ui.heading("Dashboard");
             ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                 if ui.button("Back").clicked() {
                     action = DashboardAction::CloseActive;
                 }
             });
        });
        ui.separator();
        
        egui::Frame::group(ui.style())
            .fill(ui.visuals().faint_bg_color)
            .stroke(egui::Stroke::new(1.0, egui::Color32::from_rgb(60, 180, 60)))
            .inner_margin(10.0)
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("✅ Loaded:").strong().color(egui::Color32::from_rgb(60, 180, 60)));
                    ui.label(format!("{:?}", path));
                });
            });
            
        if let Some(texture) = &app.preview_image {
            ui.add_space(20.0);
            ui.image(texture);
        }
        
    } else {
        // --- Main Split Layout ---
        let height = ui.available_height() - 30.0; // Reserve space for footer

        ui.columns(2, |columns| {
            // --- Left Column: Save List ---
            columns[0].vertical(|ui| {
                ui.heading("Detected Saves");
                ui.label(egui::RichText::new(format!("{} found", app.available_saves.len())).weak());
                ui.add_space(10.0);
                
                egui::ScrollArea::vertical()
                    .max_height(height)
                    .show(ui, |ui| {
                        for save in &app.available_saves {
                            egui::Frame::group(ui.style())
                                .rounding(5.0)
                                .inner_margin(10.0)
                                .fill(ui.visuals().faint_bg_color)
                                .show(ui, |ui| {
                                    ui.set_width(ui.available_width());
                                    
                                    ui.horizontal(|ui| {
                                        // Icon or Image
                                        if let Some(texture) = &save.texture_handle {
                                             ui.image((texture.id(), egui::vec2(64.0, 64.0))); // Square thumbnail
                                        } else {
                                             ui.label(egui::RichText::new("🌍").size(32.0));
                                        }
                                        
                                        ui.vertical(|ui| {
                                            ui.label(egui::RichText::new(&save.display_name).strong().size(16.0));
                                            ui.label(egui::RichText::new(&save.folder_name).monospace().weak());
                                            
                                            // Time
                                            let elapsed = std::time::SystemTime::now().duration_since(save.last_modified).unwrap_or_default();
                                            let time_str = if elapsed.as_secs() < 60 {
                                                "Just now".to_string()
                                            } else if elapsed.as_secs() < 3600 {
                                                format!("{}m ago", elapsed.as_secs() / 60)
                                            } else if elapsed.as_secs() < 86400 {
                                                format!("{}h ago", elapsed.as_secs() / 3600)
                                            } else {
                                                format!("{}d ago", elapsed.as_secs() / 86400)
                                            };
                                            ui.label(egui::RichText::new(time_str).small().weak());
                                        });
                                        
                                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                            if ui.button("Load").clicked() {
                                                action = DashboardAction::Load(save.path.clone());
                                            }
                                        });
                                    });
                                });
                            ui.add_space(5.0);
                        }
                    });
            });

            // --- Right Column: Manual Load ---
            columns[1].vertical_centered(|ui| {
                ui.add_space(height * 0.3); // Push down to center vertically
                
                ui.label(egui::RichText::new("📂").size(64.0));
                ui.add_space(20.0);
                ui.heading("Open Other Folder");
                ui.label("Select a Hytale save folder manually");
                ui.add_space(20.0);
                
                if ui.add(egui::Button::new(egui::RichText::new("Browse...").size(20.0)).min_size(egui::vec2(150.0, 50.0))).clicked() {
                    action = DashboardAction::OpenDialog;
                }
            });
        });
    }
    
    // --- Footer ---
    ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
        egui::Frame::none()
            .fill(ui.visuals().window_fill()) // Slight background distinction
            .inner_margin(10.0)
            .show(ui, |ui| {
                ui.set_width(ui.available_width()); // Force full width
                ui.horizontal(|ui| {
                     ui.label(egui::RichText::new("Hytale Save Editor").strong().size(14.0));
                     ui.label(egui::RichText::new("by EdgeTypE").weak());
                     
                     ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.hyperlink_to(
                            egui::RichText::new("GitHub Repository").color(egui::Color32::from_rgb(100, 149, 237)), // Cornflower Blue
                            "https://github.com/EdgeTypE/hytale-save-editor"
                        );
                    });
                });
            });
        ui.separator();
    });

    // Execute Deferred Actions
    match action {
        DashboardAction::CloseActive => {
            app.current_path = None;
        }
        DashboardAction::Load(path) => {
            app.current_path = Some(path.clone());
            app.load_data(path);
        }
        DashboardAction::OpenDialog => {
            app.open_folder_dialog();
        }
        DashboardAction::None => {}
    }
}

