use eframe::egui;
use crate::gui::app::HytaleSaveEditor;
use std::collections::BTreeSet;

pub fn show(ui: &mut egui::Ui, app: &mut HytaleSaveEditor) {
    ui.heading("Permissions & Players");

    // 1. Collect all unique UUIDs from Players and Permissions
    let mut all_uuids = BTreeSet::new();
    if let Some(players) = &app.players {
        for k in players.keys() {
            all_uuids.insert(k.clone());
        }
    }
    if let Some(permissions) = &app.permissions {
        for k in permissions.users.keys() {
            all_uuids.insert(k.clone());
        }
    }

    // 2. Manual Add UUID Interface
    ui.horizontal(|ui| {
        ui.label("Add Manual UUID:");
        ui.text_edit_singleline(&mut app.new_op_input); // Reusing this field for generic manual add
        if ui.button("Add").clicked() {
            if !app.new_op_input.is_empty() {
                // Determine if we need to add to permissions or just refresh list (which happens next frame)
                // If we want to ensure it shows up, we might need to add a dummy entry to permissions if it doesn't exist in players
                if let Some(permissions) = &mut app.permissions {
                    if !permissions.users.contains_key(&app.new_op_input) {
                        permissions.users.insert(app.new_op_input.clone(), crate::models::permissions::UserPermission {
                            groups: vec!["Default".to_string()]
                        });
                    }
                }
                app.new_op_input = String::new();
            }
        }
    });

    ui.separator();

    // 3. Player List
    egui::ScrollArea::vertical().id_source("permissions_scroll").max_height(400.0).show(ui, |ui| {
        for uuid in all_uuids {
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    // --- Avatar & Name ---
                    let texture = app.avatar_textures.get(&uuid);
                    if let Some(texture) = texture {
                        ui.image(texture);
                    } else {
                        // Fetch if missing
                         let cache = app.profile_cache.clone();
                         if !cache.avatars.lock().unwrap().contains_key(&uuid) {
                             cache.avatars.lock().unwrap().insert(uuid.clone(), None);
                             crate::api::fetch_avatar(uuid.clone(), cache, ui.ctx().clone());
                         }
                        ui.label("[?]");
                    }

                    // Resolve Name
                    let mut display_name = uuid.clone();
                     let cache = app.profile_cache.clone();
                     {
                         let profiles = cache.profiles.lock().unwrap();
                         if let Some(Some(profile)) = profiles.get(&uuid) {
                             display_name = format!("{} ({})", profile.name, uuid);
                         } else if !profiles.contains_key(&uuid) {
                             drop(profiles);
                             app.profile_cache.profiles.lock().unwrap().insert(uuid.clone(), None);
                             crate::api::fetch_profile(uuid.clone(), cache, ui.ctx().clone());
                         }
                     }
                    
                    ui.vertical(|ui| {
                        ui.strong(display_name);
                        
                        // --- OP Toggle ---
                        if let Some(permissions) = &mut app.permissions {
                            let mut is_op = false;
                            let mut has_entry = false;
                            
                            if let Some(user) = permissions.users.get(&uuid) {
                                has_entry = true;
                                is_op = user.groups.contains(&"OP".to_string());
                            }

                            if ui.checkbox(&mut is_op, "is OP").changed() {
                                if has_entry {
                                    if let Some(user) = permissions.users.get_mut(&uuid) {
                                        if is_op {
                                            if !user.groups.contains(&"OP".to_string()) {
                                                user.groups.push("OP".to_string());
                                            }
                                        } else {
                                            user.groups.retain(|g| g != "OP");
                                        }
                                    }
                                } else {
                                    // create entry
                                    let groups = if is_op { vec!["Default".to_string(), "OP".to_string()] } else { vec!["Default".to_string()] };
                                    permissions.users.insert(uuid.clone(), crate::models::permissions::UserPermission { groups });
                                }
                            }
                        }
                    });
                });
                
                ui.separator();
                
                // --- Groups Management ---
                if let Some(permissions) = &mut app.permissions {
                    ui.horizontal_wrapped(|ui| {
                        ui.label("Groups:");
                        
                        let mut group_to_remove = None;
                        let mut groups_lists = Vec::new();
                         if let Some(user) = permissions.users.get(&uuid) {
                            groups_lists = user.groups.clone();
                         }

                        // Display existing groups
                         for (i, group) in groups_lists.iter().enumerate() {
                             ui.push_id(format!("g_{}_{}", uuid, i), |ui| {
                                 // Highlight Adventure vs Creative
                                 let text = if group == "Adventure" || group == "Creative" {
                                     egui::RichText::new(format!(" [{}]", group)).color(egui::Color32::LIGHT_BLUE)
                                 } else {
                                     egui::RichText::new(format!(" [{}]", group))
                                 };

                                 ui.label(text);
                                 if ui.small_button("x").clicked() {
                                     group_to_remove = Some(i);
                                 }
                             });
                         }
                         
                         // Remove group logic
                         if let Some(idx) = group_to_remove {
                             if let Some(user) = permissions.users.get_mut(&uuid) {
                                 if idx < user.groups.len() {
                                     user.groups.remove(idx);
                                 }
                             }
                         }

                        // Add Group Logic
                         ui.push_id(format!("add_g_{}", uuid), |ui| {
                            ui.menu_button("+ Add", |ui| {
                                // Default option
                                if ui.button("Default").clicked() {
                                     if let Some(user) = permissions.users.get_mut(&uuid) {
                                         if !user.groups.contains(&"Default".to_string()) {
                                              user.groups.push("Default".to_string());
                                         }
                                     }
                                     ui.close_menu();
                                }
                                
                                ui.separator();
                                
                                // Exclusive GameMode Options
                                if ui.button("Adventure").clicked() {
                                     if let Some(user) = permissions.users.get_mut(&uuid) {
                                         // Remove Creative if present
                                         user.groups.retain(|g| g != "Creative");
                                         // Add Adventure if not present
                                         if !user.groups.contains(&"Adventure".to_string()) {
                                              user.groups.push("Adventure".to_string());
                                         }
                                     } else {
                                          permissions.users.insert(uuid.clone(), crate::models::permissions::UserPermission {
                                              groups: vec!["Default".to_string(), "Adventure".to_string()]
                                          });
                                     }
                                     ui.close_menu();
                                }
                                
                                if ui.button("Creative").clicked() {
                                     if let Some(user) = permissions.users.get_mut(&uuid) {
                                         // Remove Adventure if present
                                         user.groups.retain(|g| g != "Adventure");
                                          // Add Creative if not present
                                         if !user.groups.contains(&"Creative".to_string()) {
                                              user.groups.push("Creative".to_string());
                                         }
                                     } else {
                                          permissions.users.insert(uuid.clone(), crate::models::permissions::UserPermission {
                                              groups: vec!["Default".to_string(), "Creative".to_string()]
                                          });
                                     }
                                     ui.close_menu();
                                }
                            });
                         });
                    });
                     // Allow editing the last group name added? Or just text fields?
                     // Existing implementation had text fields for every group. merging that logic.
                     /* 
                     // Commented out raw text edit to enforce the rule via UI actions better. 
                     // If user wants to edit raw, they can't easily break the rule here unless they type it manually.
                     if let Some(user) = permissions.users.get_mut(&uuid) {
                         for group in user.groups.iter_mut() {
                              ui.text_edit_singleline(group);
                         }
                     }
                     */
                }
            });
        }
    });

    ui.separator();
    
    // 4. Group Definitions (Collapsible)
    if let Some(permissions) = &mut app.permissions {
        ui.collapsing("Group Definitions", |ui| {
            for (group_name, perms) in &mut permissions.groups {
                 ui.collapsing(group_name, |ui| {
                     ui.label("Permissions (Strings):");
                     let mut perm_to_remove = None;
                     for (i, p) in perms.iter_mut().enumerate() {
                         ui.horizontal(|ui| {
                             ui.text_edit_singleline(p);
                             if ui.button("X").clicked() {
                                 perm_to_remove = Some(i);
                             }
                         });
                     }
                     if let Some(i) = perm_to_remove {
                         perms.remove(i);
                     }
                     if ui.button("Add Permission").clicked() {
                         perms.push("new.permission".to_string());
                     }
                 });
            }
             // TODO: Add new group definition
        });
    }
}
