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
                            if let Some(seed) = &mut config.seed {
                                ui.add(egui::DragValue::new(seed));
                            } else {
                                ui.label("N/A");
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.label("Game Mode:");
                            let game_mode = config.game_mode.get_or_insert("Adventure".to_string());
                             egui::ComboBox::from_id_salt(format!("wgm_{}", name))
                                .selected_text(&*game_mode)
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(game_mode, "Adventure".to_string(), "Adventure");
                                    ui.selectable_value(game_mode, "Creative".to_string(), "Creative");
                                });
                        });
                        ui.horizontal(|ui| {
                             ui.label("Game Time:");
                             if let Some(time) = &mut config.game_time {
                                 ui.text_edit_singleline(time);
                             } else {
                                 ui.label("N/A");
                             }
                        });
                        ui.horizontal(|ui| {
                            ui.label("Gameplay Config:");
                            if let Some(gc) = &mut config.gameplay_config {
                                ui.text_edit_singleline(gc);
                            } else {
                                ui.label("N/A");
                            }
                        });
                        if let Some(v) = config.version {
                             ui.label(format!("Version: {}", v));
                        }
                        if let Some(uuid) = &config.uuid {
                            ui.label(format!("UUID: {} (Type: {})", uuid.binary, uuid.type_));
                        }
                    });

                    // --- Spawn Section ---
                    ui.group(|ui| {
                        ui.heading("Spawn");
                        if let Some(provider) = &mut config.spawn_provider {
                            ui.horizontal(|ui| {
                                ui.label("Provider ID:");
                                ui.text_edit_singleline(&mut provider.id);
                            });
                            ui.label("Spawn Point:");
                            ui.horizontal(|ui| {
                                ui.label("X:"); ui.add(egui::DragValue::new(&mut provider.spawn_point.x));
                                ui.label("Y:"); ui.add(egui::DragValue::new(&mut provider.spawn_point.y));
                                ui.label("Z:"); ui.add(egui::DragValue::new(&mut provider.spawn_point.z));
                            });
                             ui.horizontal(|ui| {
                                ui.label("Pitch:"); ui.add(egui::DragValue::new(&mut provider.spawn_point.pitch));
                                ui.label("Yaw:"); ui.add(egui::DragValue::new(&mut provider.spawn_point.yaw));
                                ui.label("Roll:"); ui.add(egui::DragValue::new(&mut provider.spawn_point.roll));
                            });
                        } else {
                            ui.label("No Spawn Provider Data");
                        }
                    });

                    // --- Generator Config ---
                    ui.group(|ui| {
                        ui.heading("World Generation");
                        ui.horizontal(|ui| {
                             ui.label("Gen Type:");
                             if let Some(wg) = &mut config.world_gen {
                                 ui.text_edit_singleline(&mut wg.type_);
                             } else { ui.label("N/A"); }
                        });
                        ui.horizontal(|ui| {
                             ui.label("Structure:");
                             if let Some(wg) = &mut config.world_gen {
                                ui.text_edit_singleline(&mut wg.world_structure);
                             } else { ui.label("N/A"); }
                        });
                        ui.horizontal(|ui| {
                             ui.label("Map Type:");
                             if let Some(wm) = &mut config.world_map {
                                ui.text_edit_singleline(&mut wm.type_);
                             } else { ui.label("N/A"); }
                        });
                        ui.horizontal(|ui| {
                             ui.label("Chunk Storage:");
                             if let Some(cs) = &mut config.chunk_storage {
                                ui.text_edit_singleline(&mut cs.type_);
                             } else { ui.label("N/A"); }
                        });
                        ui.horizontal(|ui| {
                             ui.label("Resource Storage:");
                             if let Some(rs) = &mut config.resource_storage {
                                ui.text_edit_singleline(&mut rs.type_);
                             } else { ui.label("N/A"); }
                        });
                    });

                    // --- Game Rules & Toggles ---
                    ui.group(|ui| {
                        ui.heading("Game Rules & Toggles");
                        egui::Grid::new(format!("rules_{}", name)).num_columns(2).show(ui, |ui| {
                            if let Some(v) = &mut config.is_pvp_enabled { ui.checkbox(v, "PVP Enabled"); }
                            if let Some(v) = &mut config.is_fall_damage_enabled { ui.checkbox(v, "Fall Damage"); }
                            ui.end_row();
                            
                            if let Some(v) = &mut config.is_ticking { ui.checkbox(v, "Is Ticking"); }
                            if let Some(v) = &mut config.is_block_ticking { ui.checkbox(v, "Block Ticking"); }
                            ui.end_row();

                            if let Some(v) = &mut config.is_game_time_paused { ui.checkbox(v, "Pause Time"); }
                            if let Some(v) = &mut config.is_compass_updating { ui.checkbox(v, "Compass Update"); }
                            ui.end_row();
                            
                            if let Some(v) = &mut config.is_spawning_npc { ui.checkbox(v, "Spawn NPCs"); }
                            if let Some(v) = &mut config.is_all_npc_frozen { ui.checkbox(v, "Freeze NPCs"); }
                            ui.end_row();

                            if let Some(v) = &mut config.is_spawn_markers_enabled { ui.checkbox(v, "Spawn Markers"); }
                            if let Some(v) = &mut config.is_objective_markers_enabled { ui.checkbox(v, "Obj. Markers"); }
                            ui.end_row();

                            if let Some(v) = &mut config.is_saving_players { ui.checkbox(v, "Save Players"); }
                            if let Some(v) = &mut config.is_saving_chunks { ui.checkbox(v, "Save Chunks"); }
                            ui.end_row();
                            
                            if let Some(v) = &mut config.save_new_chunks { ui.checkbox(v, "Save New Chunks"); }
                            if let Some(v) = &mut config.is_unloading_chunks { ui.checkbox(v, "Unload Chunks"); }
                            ui.end_row();
                            
                            if let Some(v) = &mut config.delete_on_universe_start { ui.checkbox(v, "Delete on Start"); }
                            if let Some(v) = &mut config.delete_on_remove { ui.checkbox(v, "Delete on Remove"); }
                            ui.end_row();
                        });
                    });

                    // --- Client Effects ---
                    ui.group(|ui| {
                        ui.heading("Client Effects");
                        if let Some(fx) = &mut config.client_effects {
                            egui::Grid::new(format!("fx_{}", name)).num_columns(2).show(ui, |ui| {
                                ui.label("Sun Height %:");
                                ui.add(egui::Slider::new(&mut fx.sun_height_percent, 0.0..=100.0));
                                ui.end_row();
                                
                                ui.label("Sun Angle:");
                                ui.add(egui::Slider::new(&mut fx.sun_angle_degrees, 0.0..=360.0));
                                ui.end_row();
                                
                                ui.label("Sun Intensity:");
                                ui.add(egui::Slider::new(&mut fx.sun_intensity, 0.0..=10.0));
                                ui.end_row();

                                ui.label("Sunshaft Intensity:");
                                ui.add(egui::Slider::new(&mut fx.sunshaft_intensity, 0.0..=10.0));
                                ui.end_row();

                                ui.label("Sunshaft Scale:");
                                ui.add(egui::Slider::new(&mut fx.sunshaft_scale_factor, 0.0..=10.0));
                                ui.end_row();

                                ui.label("Bloom Intensity:");
                                ui.add(egui::Slider::new(&mut fx.bloom_intensity, 0.0..=10.0));
                                ui.end_row();

                                ui.label("Bloom Power:");
                                ui.add(egui::Slider::new(&mut fx.bloom_power, 0.0..=100.0));
                                ui.end_row();
                            });
                        } else {
                            ui.label("No Client Effects Data");
                        }
                    });
                });
            }
        });
    } else {
        ui.label("No worlds loaded.");
    }
}
