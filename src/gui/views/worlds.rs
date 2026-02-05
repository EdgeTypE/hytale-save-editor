use eframe::egui;
use crate::gui::app::HytaleSaveEditor;

pub fn show(ui: &mut egui::Ui, app: &mut HytaleSaveEditor) {
    ui.heading("Worlds");
    if let Some(worlds) = &mut app.worlds {
        egui::ScrollArea::vertical().show(ui, |ui| {
            for (name, config) in worlds {
                ui.collapsing(name, |ui| {
                    // --- General Section ---
                    ui.group(|ui| {
                        ui.heading("General");
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
                             egui::ComboBox::from_id_salt(format!("wgm_{}", name))
                                .selected_text(&config.game_mode)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(&mut config.game_mode, "Adventure".to_string(), "Adventure");
                                    ui.selectable_value(&mut config.game_mode, "Creative".to_string(), "Creative");
                                });
                        });
                        ui.horizontal(|ui| {
                             ui.label("Game Time:");
                             ui.text_edit_singleline(&mut config.game_time);
                        });
                        ui.horizontal(|ui| {
                            ui.label("Gameplay Config:");
                            ui.text_edit_singleline(&mut config.gameplay_config);
                        });
                        ui.label(format!("Version: {}", config.version));
                        ui.label(format!("UUID: {} (Type: {})", config.uuid.binary, config.uuid.type_));
                    });

                    // --- Spawn Section ---
                    ui.group(|ui| {
                        ui.heading("Spawn");
                        ui.horizontal(|ui| {
                            ui.label("Provider ID:");
                            ui.text_edit_singleline(&mut config.spawn_provider.id);
                        });
                        ui.label("Spawn Point:");
                        ui.horizontal(|ui| {
                            ui.label("X:"); ui.add(egui::DragValue::new(&mut config.spawn_provider.spawn_point.x));
                            ui.label("Y:"); ui.add(egui::DragValue::new(&mut config.spawn_provider.spawn_point.y));
                            ui.label("Z:"); ui.add(egui::DragValue::new(&mut config.spawn_provider.spawn_point.z));
                        });
                         ui.horizontal(|ui| {
                            ui.label("Pitch:"); ui.add(egui::DragValue::new(&mut config.spawn_provider.spawn_point.pitch));
                            ui.label("Yaw:"); ui.add(egui::DragValue::new(&mut config.spawn_provider.spawn_point.yaw));
                            ui.label("Roll:"); ui.add(egui::DragValue::new(&mut config.spawn_provider.spawn_point.roll));
                        });
                    });

                    // --- Generator Config ---
                    ui.group(|ui| {
                        ui.heading("World Generation");
                        ui.horizontal(|ui| {
                             ui.label("Gen Type:");
                             ui.text_edit_singleline(&mut config.world_gen.type_);
                        });
                        ui.horizontal(|ui| {
                             ui.label("Structure:");
                             ui.text_edit_singleline(&mut config.world_gen.world_structure);
                        });
                        ui.horizontal(|ui| {
                             ui.label("Map Type:");
                             ui.text_edit_singleline(&mut config.world_map.type_);
                        });
                        ui.horizontal(|ui| {
                             ui.label("Chunk Storage:");
                             ui.text_edit_singleline(&mut config.chunk_storage.type_);
                        });
                        ui.horizontal(|ui| {
                             ui.label("Resource Storage:");
                             ui.text_edit_singleline(&mut config.resource_storage.type_);
                        });
                    });

                    // --- Game Rules & Toggles ---
                    ui.group(|ui| {
                        ui.heading("Game Rules & Toggles");
                        egui::Grid::new(format!("rules_{}", name)).num_columns(2).show(ui, |ui| {
                            ui.checkbox(&mut config.is_pvp_enabled, "PVP Enabled");
                            ui.checkbox(&mut config.is_fall_damage_enabled, "Fall Damage");
                            ui.end_row();
                            
                            ui.checkbox(&mut config.is_ticking, "Is Ticking");
                            ui.checkbox(&mut config.is_block_ticking, "Block Ticking");
                            ui.end_row();

                            ui.checkbox(&mut config.is_game_time_paused, "Pause Time");
                            ui.checkbox(&mut config.is_compass_updating, "Compass Update");
                            ui.end_row();
                            
                            ui.checkbox(&mut config.is_spawning_npc, "Spawn NPCs");
                            ui.checkbox(&mut config.is_all_npc_frozen, "Freeze NPCs");
                            ui.end_row();

                            ui.checkbox(&mut config.is_spawn_markers_enabled, "Spawn Markers");
                            ui.checkbox(&mut config.is_objective_markers_enabled, "Obj. Markers");
                            ui.end_row();

                            ui.checkbox(&mut config.is_saving_players, "Save Players");
                            ui.checkbox(&mut config.is_saving_chunks, "Save Chunks");
                            ui.end_row();
                            
                            ui.checkbox(&mut config.save_new_chunks, "Save New Chunks");
                            ui.checkbox(&mut config.is_unloading_chunks, "Unload Chunks");
                            ui.end_row();
                            
                            ui.checkbox(&mut config.delete_on_universe_start, "Delete on Start");
                            ui.checkbox(&mut config.delete_on_remove, "Delete on Remove");
                            ui.end_row();
                        });
                    });

                    // --- Client Effects ---
                    ui.group(|ui| {
                        ui.heading("Client Effects");
                        egui::Grid::new(format!("fx_{}", name)).num_columns(2).show(ui, |ui| {
                            ui.label("Sun Height %:");
                            ui.add(egui::Slider::new(&mut config.client_effects.sun_height_percent, 0.0..=100.0));
                            ui.end_row();
                            
                            ui.label("Sun Angle:");
                            ui.add(egui::Slider::new(&mut config.client_effects.sun_angle_degrees, 0.0..=360.0));
                            ui.end_row();
                            
                            ui.label("Sun Intensity:");
                            ui.add(egui::Slider::new(&mut config.client_effects.sun_intensity, 0.0..=10.0));
                            ui.end_row();

                            ui.label("Sunshaft Intensity:");
                            ui.add(egui::Slider::new(&mut config.client_effects.sunshaft_intensity, 0.0..=10.0));
                            ui.end_row();

                            ui.label("Sunshaft Scale:");
                            ui.add(egui::Slider::new(&mut config.client_effects.sunshaft_scale_factor, 0.0..=10.0));
                            ui.end_row();

                            ui.label("Bloom Intensity:");
                            ui.add(egui::Slider::new(&mut config.client_effects.bloom_intensity, 0.0..=10.0));
                            ui.end_row();

                            ui.label("Bloom Power:");
                            ui.add(egui::Slider::new(&mut config.client_effects.bloom_power, 0.0..=100.0));
                            ui.end_row();
                        });
                    });
                });
            }
        });
    } else {
        ui.label("No worlds loaded.");
    }
}
