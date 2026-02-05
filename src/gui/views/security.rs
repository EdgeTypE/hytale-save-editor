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
            let mut user_to_remove = None;
            for (i, user) in whitelist.list.iter_mut().enumerate() {
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(user);
                    if ui.button("X").clicked() {
                        user_to_remove = Some(i);
                    }
                });
            }
            if let Some(i) = user_to_remove {
                whitelist.list.remove(i);
            }
            if ui.button("Add User").clicked() {
                whitelist.list.push("New User ID".to_string());
            }
        });
    }

    if let Some(bans) = &mut app.bans {
        ui.group(|ui| {
            ui.heading("Bans");
            let mut ban_to_remove = None;
            for (i, ban) in bans.iter_mut().enumerate() {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                         ui.label("Target:");
                         ui.text_edit_singleline(&mut ban.target);
                    });
                    ui.horizontal(|ui| {
                         ui.label("Reason:");
                         ui.text_edit_singleline(&mut ban.reason);
                    });
                    // TODO: Date editing if needed
                    if ui.button("Remove Ban").clicked() {
                        ban_to_remove = Some(i);
                    }
                });
            }
            if let Some(i) = ban_to_remove {
                bans.remove(i);
            }
            if ui.button("Add Ban").clicked() {
                bans.push(crate::models::bans::BanEntry {
                    ban_type: "infinite".to_string(),
                    target: "Target UUID".to_string(),
                    by: "Admin".to_string(),
                    timestamp: 0,
                    reason: "Reason".to_string(),
                });
            }
        });
    }
}
