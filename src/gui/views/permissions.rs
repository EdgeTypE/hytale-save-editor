use eframe::egui;
use crate::gui::app::HytaleSaveEditor;

pub fn show(ui: &mut egui::Ui, app: &mut HytaleSaveEditor) {
    ui.heading("Permissions");
    if let Some(permissions) = &mut app.permissions {
        ui.group(|ui| {
             ui.heading("Users");
             let mut user_to_remove = None;
             for (uuid, user_perm) in &mut permissions.users {
                 ui.collapsing(uuid, |ui| {
                     ui.group(|ui| {
                         ui.label("Groups:");
                         let mut group_to_remove = None;
                         for (i, group) in user_perm.groups.iter_mut().enumerate() {
                             ui.horizontal(|ui| {
                                 ui.text_edit_singleline(group);
                                 if ui.button("X").clicked() {
                                     group_to_remove = Some(i);
                                 }
                             });
                         }
                         if let Some(i) = group_to_remove {
                             user_perm.groups.remove(i);
                         }
                         if ui.button("Add Group").clicked() {
                             user_perm.groups.push("Default".to_string());
                         }
                     });
                     if ui.button("Remove User").clicked() {
                         user_to_remove = Some(uuid.clone());
                     }
                 });
             }
             if let Some(uuid) = user_to_remove {
                 permissions.users.remove(&uuid);
             }
             
             ui.separator();
             ui.horizontal(|ui| {
                 if ui.button("Add User").clicked() {
                     permissions.users.insert("START-UUID".to_string(), crate::models::permissions::UserPermission { groups: vec!["Default".to_string()] });
                 }
             });
        });
        
        ui.group(|ui| {
            ui.heading("Groups");
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
             // TODO: Add new group logic
        });
    } else {
        ui.label("No permissions data loaded.");
    }
}
