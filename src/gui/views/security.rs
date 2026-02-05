use eframe::egui;
use crate::gui::app::HytaleSaveEditor;

pub fn show(ui: &mut egui::Ui, app: &mut HytaleSaveEditor) {
    ui.heading("Security");
    
    if let Some(whitelist) = &mut app.whitelist {
        ui.group(|ui| {
            ui.heading("Whitelist");
            ui.checkbox(&mut whitelist.enabled, "Enabled");
            
            ui.separator();
            ui.label("Whitelisted Users:");
            
            egui::ScrollArea::vertical().id_salt("whitelist_scroll").max_height(300.0).show(ui, |ui| {
                let mut user_to_remove = None;
                for (i, uuid) in whitelist.list.iter().enumerate() {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            // --- Avatar & Name Display (Reused logic) ---
                            let texture = app.avatar_textures.get(uuid);
                            if let Some(texture) = texture {
                                ui.image(texture);
                            } else {
                                 let cache = app.profile_cache.clone();
                                 if !cache.avatars.lock().unwrap().contains_key(uuid) {
                                     cache.avatars.lock().unwrap().insert(uuid.clone(), None);
                                     crate::api::fetch_avatar(uuid.clone(), cache, ui.ctx().clone());
                                 }
                                ui.label("[?]");
                            }
                            
                            let mut display_name = uuid.clone();
                            let cache = app.profile_cache.clone();
                             {
                                 let profiles = cache.profiles.lock().unwrap();
                                 if let Some(Some(profile)) = profiles.get(uuid) {
                                     display_name = format!("{} ({})", profile.name, uuid);
                                 } else if !profiles.contains_key(uuid) {
                                     drop(profiles);
                                     app.profile_cache.profiles.lock().unwrap().insert(uuid.clone(), None);
                                     crate::api::fetch_profile(uuid.clone(), cache, ui.ctx().clone());
                                 }
                             }
                             
                             ui.label(display_name);
                             
                             if ui.button("X").clicked() {
                                user_to_remove = Some(i);
                             }
                        });
                    });
                }
                if let Some(i) = user_to_remove {
                    whitelist.list.remove(i);
                }
            });

            ui.separator();
            ui.horizontal(|ui| {
                ui.label("Add User (Name or UUID):");
                ui.text_edit_singleline(&mut app.new_whitelist_input);
                if ui.button("Add").clicked() {
                    if !app.new_whitelist_input.is_empty() {
                        crate::api::resolve_profile_add(
                            app.new_whitelist_input.clone(), 
                            app.api_tx.clone(), 
                            ui.ctx().clone(),
                            crate::api::ResolveTarget::Whitelist
                        );
                        app.new_whitelist_input.clear();
                    }
                }
            });
        });
    }

    if let Some(bans) = &mut app.bans {
        ui.group(|ui| {
            ui.heading("Bans");
            
            egui::ScrollArea::vertical().id_salt("bans_scroll").max_height(300.0).show(ui, |ui| {
                let mut ban_to_remove = None;
                for (i, ban) in bans.iter_mut().enumerate() {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                             // --- Avatar & Name Display ---
                            let uuid = &ban.target;
                            let texture = app.avatar_textures.get(uuid);
                            if let Some(texture) = texture {
                                ui.image(texture);
                            } else {
                                 let cache = app.profile_cache.clone();
                                 if !cache.avatars.lock().unwrap().contains_key(uuid) {
                                     cache.avatars.lock().unwrap().insert(uuid.clone(), None);
                                     crate::api::fetch_avatar(uuid.clone(), cache, ui.ctx().clone());
                                 }
                                ui.label("[?]");
                            }
                            
                            let mut display_name = uuid.clone();
                            let cache = app.profile_cache.clone();
                             {
                                 let profiles = cache.profiles.lock().unwrap();
                                 if let Some(Some(profile)) = profiles.get(uuid) {
                                     display_name = format!("{} ({})", profile.name, uuid);
                                 } else if !profiles.contains_key(uuid) {
                                     drop(profiles);
                                     app.profile_cache.profiles.lock().unwrap().insert(uuid.clone(), None);
                                     crate::api::fetch_profile(uuid.clone(), cache, ui.ctx().clone());
                                 }
                             }
                             ui.label(display_name);

                             ui.separator();
                             ui.vertical(|ui| {
                                 ui.horizontal(|ui| {
                                     ui.label("Reason:");
                                     ui.text_edit_singleline(&mut ban.reason);
                                 });
                                 ui.horizontal(|ui| {
                                    ui.label("By:");
                                    ui.label(&ban.by);
                                 });
                             });
                             
                             if ui.button("Remove Ban").clicked() {
                                ban_to_remove = Some(i);
                             }
                        });
                    });
                }
                if let Some(i) = ban_to_remove {
                    bans.remove(i);
                }
            });
            
             ui.separator();
            ui.horizontal(|ui| {
                ui.label("Ban User (Name or UUID):");
                ui.text_edit_singleline(&mut app.new_ban_input);
                if ui.button("Ban").clicked() {
                    if !app.new_ban_input.is_empty() {
                        crate::api::resolve_profile_add(
                            app.new_ban_input.clone(), 
                            app.api_tx.clone(), 
                            ui.ctx().clone(),
                            crate::api::ResolveTarget::Ban
                        );
                        app.new_ban_input.clear();
                    }
                }
            });
        });
    }
}
