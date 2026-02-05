use eframe::egui;
use crate::models::{SaveData, PermissionsConfig, ModsConfig, Whitelist, BanList, BanEntry, WorldConfig, MemoriesConfig, PlayerFile, Storage, Item};
use crate::io;
use std::path::PathBuf;
use std::collections::HashMap;

#[derive(PartialEq)]
pub enum Tab {
    General,
    Worlds,
    Memories,
    Players,
}

impl Default for Tab {
    fn default() -> Self {
        Tab::General
    }
}

pub struct HytaleEditorApp {
    pub save_data: Option<SaveData>,
    pub current_tab: Tab,
    pub texture: Option<egui::TextureHandle>,
    pub error_message: Option<String>,

    // UI State
    new_whitelist_input: String,
    new_ban_target_input: String,
    selected_world: Option<String>,
    selected_player: Option<String>,

    last_save_time: Option<std::time::Instant>,
}

impl Default for HytaleEditorApp {
    fn default() -> Self {
        Self {
            save_data: None,
            current_tab: Tab::General,
            texture: None,
            error_message: None,
            new_whitelist_input: String::new(),
            new_ban_target_input: String::new(),
            selected_world: None,
            selected_player: None,
            last_save_time: None,
        }
    }
}

impl HytaleEditorApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        egui_extras::install_image_loaders(&cc.egui_ctx);
        Default::default()
    }

    fn load_save(&mut self, path: PathBuf, ctx: &egui::Context) {
        match io::load_save_data(path.clone()) {
            Ok(data) => {
                self.save_data = Some(data);
                self.error_message = None;
                self.load_preview_image(path, ctx);
            }
            Err(e) => {
                self.error_message = Some(e);
                self.save_data = None;
                self.texture = None;
            }
        }
    }

    fn load_preview_image(&mut self, root: PathBuf, ctx: &egui::Context) {
        let img_path = root.join("preview.png");
        if img_path.exists() {
            if let Ok(reader) = image::ImageReader::open(&img_path) {
                if let Ok(img) = reader.decode() {
                    let size = [img.width() as _, img.height() as _];
                    let image_buffer = img.to_rgba8();
                    let pixels = image_buffer.as_flat_samples();
                    let color_image = egui::ColorImage::from_rgba_unmultiplied(
                        size,
                        pixels.as_slice(),
                    );
                    self.texture = Some(ctx.load_texture(
                        "preview",
                        color_image,
                        egui::TextureOptions::default()
                    ));
                    return;
                }
            }
        }
        self.texture = None;
    }

    fn ui_general_tab(&mut self, ui: &mut egui::Ui) {
        let texture = self.texture.clone();
        let new_whitelist_input = &mut self.new_whitelist_input;
        let new_ban_target_input = &mut self.new_ban_target_input;

        if let Some(data) = &mut self.save_data {
            egui::ScrollArea::vertical().show(ui, |ui| {
                if let Some(texture) = &texture {
                    ui.image(texture);
                    ui.separator();
                }

                ui.heading("Permissions");
                if let Some(perms) = &mut data.permissions {
                    Self::ui_permissions(ui, perms);
                } else {
                    ui.label("permissions.json not found.");
                }
                ui.separator();

                ui.heading("Mods Config");
                if let Some(mods) = &mut data.mods {
                    Self::ui_mods(ui, mods);
                } else {
                    ui.label("config.json (Mods) not found.");
                }
                ui.separator();

                ui.heading("Whitelist");
                if let Some(whitelist) = &mut data.whitelist {
                    Self::ui_whitelist(ui, whitelist, new_whitelist_input);
                } else {
                    ui.label("whitelist.json not found.");
                }
                ui.separator();

                ui.heading("Bans");
                if let Some(bans) = &mut data.bans {
                    Self::ui_bans(ui, bans, new_ban_target_input);
                } else {
                    ui.label("bans.json not found.");
                }
            });
        }
    }

    fn ui_permissions(ui: &mut egui::Ui, perms: &mut PermissionsConfig) {
        ui.group(|ui| {
            let mut user_ids: Vec<String> = perms.users.keys().cloned().collect();
            user_ids.sort();

            for uuid in user_ids {
                if let Some(user) = perms.users.get_mut(&uuid) {
                    ui.horizontal(|ui| {
                        ui.label(format!("User: {}", uuid));
                        ui.label(format!("Groups: {:?}", user.groups));

                        if ui.button("+ OP").clicked() {
                            if !user.groups.contains(&"OP".to_string()) {
                                user.groups.push("OP".to_string());
                            }
                        }
                        if ui.button("- OP").clicked() {
                            user.groups.retain(|g| g != "OP");
                        }

                        egui::ComboBox::from_id_source(&uuid)
                            .selected_text("Set Group...")
                            .show_ui(ui, |ui| {
                                if ui.selectable_label(false, "Adventure").clicked() {
                                    if !user.groups.contains(&"Adventure".to_string()) {
                                        user.groups.push("Adventure".to_string());
                                    }
                                }
                                if ui.selectable_label(false, "Creative").clicked() {
                                     if !user.groups.contains(&"Creative".to_string()) {
                                        user.groups.push("Creative".to_string());
                                    }
                                }
                                if ui.selectable_label(false, "Spectator").clicked() {
                                     if !user.groups.contains(&"Spectator".to_string()) {
                                        user.groups.push("Spectator".to_string());
                                    }
                                }
                            });
                    });
                }
            }
        });
    }

    fn ui_mods(ui: &mut egui::Ui, mods: &mut ModsConfig) {
        ui.group(|ui| {
            let mut mod_names: Vec<String> = mods.mods.keys().cloned().collect();
            mod_names.sort();

            for name in mod_names {
                if let Some(entry) = mods.mods.get_mut(&name) {
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut entry.enabled, &name);
                    });
                }
            }
        });
    }

    fn ui_whitelist(ui: &mut egui::Ui, whitelist: &mut Whitelist, new_input: &mut String) {
        ui.group(|ui| {
            ui.checkbox(&mut whitelist.enabled, "Whitelist Enabled");

            ui.horizontal(|ui| {
                ui.text_edit_singleline(new_input);
                if ui.button("Add User (UUID)").clicked() {
                    if !new_input.is_empty() {
                        whitelist.list.push(new_input.clone());
                        new_input.clear();
                    }
                }
            });

            let mut to_remove = None;
            for (i, entry) in whitelist.list.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(entry);
                    if ui.button("Remove").clicked() {
                        to_remove = Some(i);
                    }
                });
            }
            if let Some(i) = to_remove {
                whitelist.list.remove(i);
            }
        });
    }

    fn ui_bans(ui: &mut egui::Ui, bans: &mut BanList, new_input: &mut String) {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.text_edit_singleline(new_input);
                if ui.button("Ban User (UUID)").clicked() {
                     if !new_input.is_empty() {
                         bans.push(BanEntry {
                             ban_type: "infinite".to_string(),
                             target: new_input.clone(),
                             by: "Console".to_string(),
                             timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as i64,
                             reason: "Banned via Editor".to_string()
                         });
                         new_input.clear();
                     }
                }
            });

            let mut to_remove = None;
            for (i, entry) in bans.iter().enumerate() {
                ui.horizontal(|ui| {
                    ui.label(format!("{} (Reason: {})", entry.target, entry.reason));
                    if ui.button("Unban").clicked() {
                        to_remove = Some(i);
                    }
                });
            }
            if let Some(i) = to_remove {
                bans.remove(i);
            }
        });
    }

    fn ui_worlds_tab(ui: &mut egui::Ui, worlds: &mut HashMap<String, WorldConfig>, selected_world: &mut Option<String>) {
         ui.horizontal(|ui| {
             ui.label("Select World:");
             egui::ComboBox::from_id_source("world_selector")
                 .selected_text(selected_world.as_deref().unwrap_or("None"))
                 .show_ui(ui, |ui| {
                     for name in worlds.keys() {
                         ui.selectable_value(selected_world, Some(name.clone()), name);
                     }
                 });
         });

         if let Some(name) = selected_world {
             if let Some(config) = worlds.get_mut(name) {
                 ui.separator();
                 ui.heading(format!("Editing World: {}", name));

                 egui::Grid::new("world_grid").striped(true).show(ui, |ui| {
                     // Display Name
                     if let Some(dn) = &mut config.display_name {
                         ui.label("Display Name:");
                         ui.text_edit_singleline(dn);
                         ui.end_row();
                     }

                     // Seed
                     if let Some(seed) = &mut config.seed {
                          ui.label("Seed:");
                          ui.add(egui::DragValue::new(seed));
                          ui.end_row();
                     }

                     // GameMode
                     if let Some(gm) = &mut config.game_mode {
                          ui.label("GameMode:");
                          egui::ComboBox::from_id_source("world_gm")
                            .selected_text(gm.as_str())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(gm, "Creative".to_string(), "Creative");
                                ui.selectable_value(gm, "Adventure".to_string(), "Adventure");
                                ui.selectable_value(gm, "Survival".to_string(), "Survival");
                            });
                          ui.end_row();
                     }

                     // PvP
                     if let Some(pvp) = &mut config.is_pvp_enabled {
                         ui.label("PvP Enabled:");
                         ui.checkbox(pvp, "");
                         ui.end_row();
                     }

                     // Fall Damage
                     if let Some(fd) = &mut config.is_fall_damage_enabled {
                         ui.label("Fall Damage Enabled:");
                         ui.checkbox(fd, "");
                         ui.end_row();
                     }

                     // Time
                     if let Some(gt) = &mut config.game_time {
                          ui.label("Game Time (ISO):");
                          ui.text_edit_singleline(gt);
                          ui.end_row();
                     }
                 });
             }
         }
    }

    fn ui_memories_tab(ui: &mut egui::Ui, memories_config: &mut MemoriesConfig) {
        if ui.button("Add New Memory").clicked() {
             memories_config.memories.push(crate::models::Memory {
                 id: "NPC".to_string(),
                 npc_role: "New_Role".to_string(),
                 translation_key: "server.npcRoles.New_Role.name".to_string(),
                 is_overridden: false,
                 captured_timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis() as i64,
                 found_location: "server.map.region.Zone1".to_string(),
             });
        }
        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
             let mut to_remove = None;
             for (i, mem) in memories_config.memories.iter_mut().enumerate() {
                 ui.group(|ui| {
                     ui.horizontal(|ui| {
                         ui.label(format!("Memory #{}", i));
                         if ui.button("Remove").clicked() {
                             to_remove = Some(i);
                         }
                     });

                     egui::Grid::new(format!("mem_grid_{}", i)).striped(true).show(ui, |ui| {
                         ui.label("NPC Role:");
                         ui.text_edit_singleline(&mut mem.npc_role);
                         ui.end_row();

                         ui.label("Translation Key:");
                         ui.text_edit_singleline(&mut mem.translation_key);
                         ui.end_row();

                         ui.label("Location Key:");
                         ui.text_edit_singleline(&mut mem.found_location);
                         ui.end_row();

                         ui.label("Timestamp:");
                         ui.add(egui::DragValue::new(&mut mem.captured_timestamp));
                         ui.end_row();

                         ui.label("Name Overridden:");
                         ui.checkbox(&mut mem.is_overridden, "");
                         ui.end_row();
                     });
                 });
             }

             if let Some(i) = to_remove {
                 memories_config.memories.remove(i);
             }
        });
    }

    fn ui_players_tab(ui: &mut egui::Ui, players: &mut HashMap<String, PlayerFile>, selected_player: &mut Option<String>) {
         ui.horizontal(|ui| {
             ui.label("Select Player:");
             egui::ComboBox::from_id_source("player_selector")
                 .selected_text(selected_player.as_deref().unwrap_or("None"))
                 .show_ui(ui, |ui| {
                     for name in players.keys() {
                         ui.selectable_value(selected_player, Some(name.clone()), name);
                     }
                 });
         });

         if let Some(pid) = selected_player {
             if let Some(pfile) = players.get_mut(pid) {
                 let components = &mut pfile.components;

                 egui::ScrollArea::vertical().show(ui, |ui| {
                     ui.heading(format!("Editing Player: {}", pid));

                     ui.collapsing("General", |ui| {
                         if let Some(transform) = &mut components.transform {
                             ui.label("Position:");
                             ui.horizontal(|ui| {
                                 ui.label("X:"); ui.add(egui::DragValue::new(&mut transform.position.x));
                                 ui.label("Y:"); ui.add(egui::DragValue::new(&mut transform.position.y));
                                 ui.label("Z:"); ui.add(egui::DragValue::new(&mut transform.position.z));
                             });
                         }

                         if let Some(player_comp) = &mut components.player {
                             if let Some(gm) = &mut player_comp.game_mode {
                                  ui.horizontal(|ui| {
                                      ui.label("GameMode:");
                                      egui::ComboBox::from_id_source("p_gm")
                                        .selected_text(gm.as_str())
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(gm, "Creative".to_string(), "Creative");
                                            ui.selectable_value(gm, "Adventure".to_string(), "Adventure");
                                            ui.selectable_value(gm, "Survival".to_string(), "Survival");
                                        });
                                  });
                             }
                         }
                     });

                     ui.collapsing("Stats", |ui| {
                         if let Some(estats) = &mut components.entity_stats {
                             for (_key, stat) in &mut estats.stats {
                                 ui.horizontal(|ui| {
                                     ui.label(&stat.id);
                                     ui.add(egui::DragValue::new(&mut stat.value));
                                 });
                             }
                         } else {
                             ui.label("No EntityStats found.");
                         }
                     });

                     ui.collapsing("Inventory", |ui| {
                         if let Some(player_comp) = &mut components.player {
                             if let Some(inv) = &mut player_comp.inventory {
                                 if let Some(storage) = &mut inv.storage {
                                     ui.collapsing("Main Storage", |ui| {
                                         Self::ui_inventory_storage(ui, storage);
                                     });
                                 }
                                 if let Some(hotbar) = &mut inv.hot_bar {
                                     ui.collapsing("Hotbar", |ui| {
                                         Self::ui_inventory_storage(ui, hotbar);
                                     });
                                 }
                                 if let Some(armor) = &mut inv.armor {
                                      ui.collapsing("Armor", |ui| {
                                         Self::ui_inventory_storage(ui, armor);
                                     });
                                 }
                                 if let Some(bp) = &mut inv.backpack {
                                      ui.collapsing("Backpack", |ui| {
                                         Self::ui_inventory_storage(ui, bp);
                                     });
                                 }
                             } else {
                                 ui.label("No Inventory found.");
                             }
                         }
                     });
                 });
             }
         }
    }

    fn ui_inventory_storage(ui: &mut egui::Ui, storage: &mut Storage) {
        let mut slots: Vec<String> = storage.items.keys().cloned().collect();
        // Try parsing as int for sorting
        slots.sort_by(|a, b| {
            let ai = a.parse::<i32>().unwrap_or(9999);
            let bi = b.parse::<i32>().unwrap_or(9999);
            ai.cmp(&bi)
        });

        for slot in slots {
            if let Some(item) = storage.items.get_mut(&slot) {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label(format!("Slot {}: ", slot));
                        ui.text_edit_singleline(&mut item.id);
                        ui.label("Qty:");
                        ui.add(egui::DragValue::new(&mut item.quantity));
                        if let Some(dur) = &mut item.durability {
                            ui.label("Dur:");
                            ui.add(egui::DragValue::new(dur));
                        }
                    });
                });
            }
        }

        if ui.button("Add Item (Find Free Slot)").clicked() {
            for i in 0..storage.capacity {
                let key = i.to_string();
                if !storage.items.contains_key(&key) {
                    storage.items.insert(key, Item {
                        id: "New_Item".to_string(),
                        quantity: 1,
                        durability: None,
                        max_durability: None,
                        other: HashMap::new(),
                    });
                    break;
                }
            }
        }
    }
}

impl eframe::App for HytaleEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Open Save Folder").clicked() {
                    if let Some(path) = rfd::FileDialog::new().pick_folder() {
                        self.load_save(path, ctx);
                    }
                }
                if let Some(data) = &self.save_data {
                    ui.label(format!("Loaded: {:?}", data.root_path));

                    if ui.button("💾 Save All Changes").clicked() {
                         let path = data.root_path.clone();
                         let mut errors = Vec::new();

                         if let Some(p) = &data.permissions {
                             if let Err(e) = io::save_json(&path.join("permissions.json"), p) { errors.push(format!("Permissions: {}", e)); }
                         }
                         if let Some(m) = &data.mods {
                             if let Err(e) = io::save_json(&path.join("config.json"), m) { errors.push(format!("Mods: {}", e)); }
                         }
                         if let Some(w) = &data.whitelist {
                             if let Err(e) = io::save_json(&path.join("whitelist.json"), w) { errors.push(format!("Whitelist: {}", e)); }
                         }
                         if let Some(b) = &data.bans {
                             if let Err(e) = io::save_json(&path.join("bans.json"), b) { errors.push(format!("Bans: {}", e)); }
                         }
                         if let Some(mem) = &data.memories {
                             if let Err(e) = io::save_json(&path.join("universe").join("memories.json"), mem) { errors.push(format!("Memories: {}", e)); }
                         }

                         for (name, world_config) in &data.worlds {
                             let world_path = path.join("universe").join("worlds").join(name).join("config.json");
                             if let Err(e) = io::save_json(&world_path, world_config) { errors.push(format!("World {}: {}", name, e)); }
                         }

                         for (uuid, pfile) in &data.players {
                             let p_path = path.join("universe").join("players").join(format!("{}.json", uuid));
                             if let Err(e) = io::save_json(&p_path, pfile) { errors.push(format!("Player {}: {}", uuid, e)); }
                         }

                         if !errors.is_empty() {
                             self.error_message = Some(errors.join("\n"));
                         } else {
                             self.error_message = None;
                             self.last_save_time = Some(std::time::Instant::now());
                         }
                    }

                    if let Some(t) = self.last_save_time {
                        if t.elapsed().as_secs() < 3 {
                            ui.label(egui::RichText::new("Saved!").color(egui::Color32::GREEN));
                        }
                    }
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            if let Some(err) = &self.error_message {
                ui.colored_label(egui::Color32::RED, err);
            }

            let selected_world = &mut self.selected_world;
            let selected_player = &mut self.selected_player;

            if let Some(data) = &mut self.save_data {
                 ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.current_tab, Tab::General, "General");
                    ui.selectable_value(&mut self.current_tab, Tab::Worlds, "Worlds");
                    ui.selectable_value(&mut self.current_tab, Tab::Memories, "Memories");
                    ui.selectable_value(&mut self.current_tab, Tab::Players, "Players");
                });
                ui.separator();

                match self.current_tab {
                    Tab::General => self.ui_general_tab(ui),
                    Tab::Worlds => {
                        Self::ui_worlds_tab(ui, &mut data.worlds, selected_world);
                    },
                    Tab::Memories => {
                        if let Some(mem) = &mut data.memories {
                            Self::ui_memories_tab(ui, mem);
                        } else {
                            ui.label("memories.json not found.");
                        }
                    },
                    Tab::Players => {
                         Self::ui_players_tab(ui, &mut data.players, selected_player);
                    },
                }
            } else {
                ui.centered_and_justified(|ui| {
                    ui.label("Please open a Hytale save folder.");
                });
            }
        });
    }
}
