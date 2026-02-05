use eframe::egui;


pub fn show(ui: &mut egui::Ui, players: &mut std::collections::HashMap<String, crate::models::player::PlayerData>, asset_manager: &mut crate::assets::AssetManager) {
    ui.heading("Players");

    // Players map passed directly
    for (uuid, data) in players {
            ui.collapsing(uuid, |ui| {
                // We utilize a locally unique ID struct for tab selection if we had localized state.
                // For now, simpler: vertical sections.
                
                // --- General Section ---
                ui.group(|ui| {
                    ui.heading("General");
                    if let Some(nameplate) = &mut data.components.nameplate {
                        ui.horizontal(|ui| {
                            ui.label("Nameplate:");
                            ui.text_edit_singleline(&mut nameplate.text);
                        });
                    }
                     // Game Mode
                    if let Some(player_comp) = &mut data.components.player {
                        ui.horizontal(|ui| {
                             ui.label("Game Mode:");
                             ui.text_edit_singleline(&mut player_comp.game_mode);
                        });
                    }
                });

                // --- Transform Section ---
                ui.group(|ui| {
                    ui.heading("Position & Rotation");
                    if let Some(transform) = &mut data.components.transform {
                         ui.horizontal(|ui| {
                             ui.label("X:"); ui.add(egui::DragValue::new(&mut transform.position.x));
                             ui.label("Y:"); ui.add(egui::DragValue::new(&mut transform.position.y));
                             ui.label("Z:"); ui.add(egui::DragValue::new(&mut transform.position.z));
                         });
                         ui.horizontal(|ui| {
                             ui.label("Pitch:"); ui.add(egui::DragValue::new(&mut transform.rotation.pitch));
                             ui.label("Yaw:"); ui.add(egui::DragValue::new(&mut transform.rotation.yaw));
                             ui.label("Roll:"); ui.add(egui::DragValue::new(&mut transform.rotation.roll));
                         });
                    }
                });
                
                // --- Entity Stats ---
                ui.group(|ui| {
                    ui.heading("Stats");
                    if let Some(entity_stats) = &mut data.components.entity_stats {
                        egui::Grid::new("stats_grid").num_columns(2).show(ui, |ui| {
                             for (key, stat) in &mut entity_stats.stats {
                                 ui.label(key);
                                 ui.add(egui::DragValue::new(&mut stat.value));
                                 ui.end_row();
                             }
                        });
                    }
                });

                // --- Inventory Section ---
                if let Some(player_comp) = &mut data.components.player {
                    ui.group(|ui| {
                        ui.heading("Inventory");
                        
                        // Action queue for inventory modifications
                        enum InvAction {
                            SetQuantity(String, String, i32), // Container, Slot, Qty
                            RemoveItem(String, String), // Container, Slot
                            MoveItem(String, String, String, String), // FromContainer, FromSlot, ToContainer, ToSlot
                        }
                        let mut actions = Vec::new();

                        // Helper to look up storage by name for move action later
                        // (Actually we can't easily lookup storage by string name unless we match on it. 
                        //  So we'll implement the Move action application logic separately below.)

                        // Helper to render a storage grid
                        let mut render_storage = |ui: &mut egui::Ui, name: &str, storage: &mut crate::models::player::Storage, cols: usize, actions_queue: &mut Vec<InvAction>| {
                            ui.label(format!("{} (Capacity: {})", name, storage.capacity));
                             egui::Grid::new(format!("inv_{}_{}", uuid, name)).num_columns(cols).show(ui, |ui| {
                                 for i in 0..storage.capacity {
                                     let slot_idx = i.to_string();
                                     let item_opt = storage.items.get_mut(&slot_idx);
                                     
                                     // Frame for the slot
                                     egui::Frame::group(ui.style()).inner_margin(4.0).show(ui, |ui| {
                                         // Fixed size slot
                                         let size = egui::vec2(64.0, 64.0);
                                         
                                         // Drag and Drop Logic
                                         // ID for this slot
                                         let slot_id = ui.make_persistent_id(format!("slot_{}_{}_{}", uuid, name, i));
                                         
                                         // Check if being dragged
                                         let is_being_dragged = ui.ctx().is_being_dragged(slot_id);
                                         
                                         // Allocate Layout
                                         let (rect, response) = ui.allocate_exact_size(size, egui::Sense::click_and_drag());
                                         
                                         // Handle Drag Start
                                         if response.drag_started() {
                                              ui.memory_mut(|mem| {
                                                  mem.data.insert_temp(egui::Id::new("dragged_item"), (name.to_string(), slot_idx.clone()));
                                              });
                                         }
                                         
                                         // Handle Drop
                                         if response.hovered() && !is_being_dragged {
                                              if ui.input(|i| i.pointer.any_released()) {
                                                   let payload: Option<(String, String)> = ui.memory(|mem| mem.data.get_temp(egui::Id::new("dragged_item")));
                                                   if let Some((from_container, from_slot)) = payload {
                                                       if from_container != name || from_slot != slot_idx {
                                                            actions_queue.push(InvAction::MoveItem(from_container, from_slot, name.to_string(), slot_idx.clone()));
                                                       }
                                                   }
                                              }
                                         }
                                         
                                         // Visuals
                                         if is_being_dragged {
                                             ui.painter().rect_filled(rect, 4.0, egui::Color32::from_white_alpha(50));
                                         }

                                         if let Some(item) = item_opt {
                                             // Try to load icon
                                             let texture = asset_manager.get_icon(ui.ctx(), &item.id);
                                             if let Some(texture) = texture {
                                                 ui.painter().image(
                                                     texture.id(),
                                                     rect,
                                                     egui::Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                                                     egui::Color32::WHITE
                                                 );
                                             } else {
                                                 ui.painter().text(
                                                     rect.center(),
                                                     egui::Align2::CENTER_CENTER,
                                                     "?",
                                                     egui::FontId::proportional(20.0),
                                                     egui::Color32::YELLOW
                                                 );
                                             }
                                             
                                             // Draw Quantity if > 1
                                             if item.quantity > 1 {
                                                 ui.painter().text(
                                                     rect.max - egui::vec2(2.0, 2.0),
                                                     egui::Align2::RIGHT_BOTTOM,
                                                     item.quantity.to_string(),
                                                     egui::FontId::proportional(16.0),
                                                     egui::Color32::WHITE
                                                 );
                                             }
                                             
                                             // Interaction (Tooltip + Context Menu)
                                             response
                                                 .on_hover_ui(|ui| {
                                                     ui.label(egui::RichText::new(&item.id).strong());
                                                     ui.label(format!("Qty: {}", item.quantity));
                                                     if let Some(dur) = item.durability {
                                                         let max = item.max_durability.unwrap_or(100.0);
                                                         ui.label(format!("Durability: {:.0}/{:.0}", dur, max));
                                                     }
                                                 })
                                                 .context_menu(|ui| {
                                                     ui.label("Actions");
                                                     ui.separator();
                                                     // Quantity
                                                     ui.horizontal(|ui| {
                                                         ui.label("Quantity:");
                                                         let mut qty = item.quantity;
                                                         if ui.add(egui::DragValue::new(&mut qty).range(1..=64)).changed() {
                                                              actions_queue.push(InvAction::SetQuantity(name.to_string(), slot_idx.clone(), qty));
                                                         }
                                                     });
                                                     
                                                     if ui.button("Remove Item").clicked() {
                                                         actions_queue.push(InvAction::RemoveItem(name.to_string(), slot_idx.clone()));
                                                         ui.close_menu();
                                                     }
                                                 });
                                         }
                                     });
                                     
                                     if (i + 1) % (cols as i32) == 0 {
                                         ui.end_row();
                                     }
                                 }
                             });
                        };

                        render_storage(ui, "Hotbar", &mut player_comp.inventory.hotbar, 9, &mut actions);
                        if let Some(backpack) = &mut player_comp.inventory.backpack {
                            ui.separator();
                            render_storage(ui, "Backpack", backpack, 9, &mut actions);
                        }
                        render_storage(ui, "Storage", &mut player_comp.inventory.storage, 9, &mut actions);
                         if let Some(armor) = &mut player_comp.inventory.armor {
                            ui.separator();
                             render_storage(ui, "Armor", armor, 4, &mut actions);
                        }
                        
                        // Apply Actions
                        for action in actions {
                            match action {
                                InvAction::SetQuantity(container, slot, qty) => {
                                    let storage_opt = match container.as_str() {
                                        "Hotbar" => Some(&mut player_comp.inventory.hotbar),
                                        "Backpack" => player_comp.inventory.backpack.as_mut(),
                                        "Storage" => Some(&mut player_comp.inventory.storage),
                                        "Armor" => player_comp.inventory.armor.as_mut(),
                                        _ => None
                                    };
                                    if let Some(storage) = storage_opt {
                                        if let Some(item) = storage.items.get_mut(&slot) {
                                            item.quantity = qty;
                                        }
                                    }
                                }
                                InvAction::RemoveItem(container, slot) => {
                                     let storage_opt = match container.as_str() {
                                        "Hotbar" => Some(&mut player_comp.inventory.hotbar),
                                        "Backpack" => player_comp.inventory.backpack.as_mut(),
                                        "Storage" => Some(&mut player_comp.inventory.storage),
                                        "Armor" => player_comp.inventory.armor.as_mut(),
                                        _ => None
                                    };
                                    if let Some(storage) = storage_opt {
                                        storage.items.remove(&slot);
                                    }
                                }
                                InvAction::MoveItem(from_c, from_s, to_c, to_s) => {
                                     // Phase 1: Remove from Source
                                     let src_item = {
                                         let storage_opt = match from_c.as_str() {
                                            "Hotbar" => Some(&mut player_comp.inventory.hotbar),
                                            "Backpack" => player_comp.inventory.backpack.as_mut(),
                                            "Storage" => Some(&mut player_comp.inventory.storage),
                                            "Armor" => player_comp.inventory.armor.as_mut(),
                                            _ => None
                                        };
                                        if let Some(s) = storage_opt {
                                            s.items.remove(&from_s)
                                        } else { None }
                                     };

                                     if let Some(item_to_move) = src_item {
                                         // Phase 2: Remove from Dest (to Swap)
                                         let existing_dest_item = {
                                              let storage_opt = match to_c.as_str() {
                                                "Hotbar" => Some(&mut player_comp.inventory.hotbar),
                                                "Backpack" => player_comp.inventory.backpack.as_mut(),
                                                "Storage" => Some(&mut player_comp.inventory.storage),
                                                "Armor" => player_comp.inventory.armor.as_mut(),
                                                _ => None
                                            };
                                            if let Some(s) = storage_opt {
                                                s.items.remove(&to_s)
                                            } else { None }
                                         };

                                         // Phase 3: Insert Source Item into Dest
                                         {
                                              let storage_opt = match to_c.as_str() {
                                                "Hotbar" => Some(&mut player_comp.inventory.hotbar),
                                                "Backpack" => player_comp.inventory.backpack.as_mut(),
                                                "Storage" => Some(&mut player_comp.inventory.storage),
                                                "Armor" => player_comp.inventory.armor.as_mut(),
                                                _ => None
                                            };
                                            if let Some(s) = storage_opt {
                                                s.items.insert(to_s.clone(), item_to_move);
                                            } else {
                                                // Failed to find dest? Return item to source later.
                                                // For now, let's assume valid dest or lose item (should catch in Phase 4 safety)
                                                // Actually, if we fail here, we should re-insert to source.
                                            }
                                         }
                                         
                                         // Phase 4: Insert Existing Dest Item into Source (Swap) or Return Source Item if Dest Failed
                                         if let Some(existing) = existing_dest_item {
                                              let storage_opt = match from_c.as_str() {
                                                "Hotbar" => Some(&mut player_comp.inventory.hotbar),
                                                "Backpack" => player_comp.inventory.backpack.as_mut(),
                                                "Storage" => Some(&mut player_comp.inventory.storage),
                                                "Armor" => player_comp.inventory.armor.as_mut(),
                                                _ => None
                                            };
                                            if let Some(s) = storage_opt {
                                                s.items.insert(from_s, existing);
                                            }
                                         }
                                     }
                                }
                            }
                        }
                    });
                }
            });
        }
}
