use eframe::egui;
use crate::gui::app::HytaleSaveEditor;

pub fn show(ui: &mut egui::Ui, app: &mut HytaleSaveEditor) {
    ui.heading("Players");
    if let Some(players) = &mut app.players {
        for (uuid, data) in players {
             ui.collapsing(uuid, |ui| {
                 ui.group(|ui| {
                    ui.label("Component: Nameplate");
                    if let Some(nameplate) = &mut data.components.nameplate {
                        ui.horizontal(|ui| {
                            ui.label("Text:");
                            ui.text_edit_singleline(&mut nameplate.text);
                        });
                    }
                 });

                 if let Some(player_comp) = &mut data.components.player {
                     ui.group(|ui| {
                         ui.label("Component: Player");
                         ui.horizontal(|ui| {
                             ui.label("Game Mode:");
                             ui.text_edit_singleline(&mut player_comp.game_mode);
                         });
                         
                         ui.separator();
                         ui.label("Inventory Storage:");
                         let storage = &mut player_comp.inventory.storage;
                         ui.label(format!("ID: {}, Capacity: {}", storage.id, storage.capacity));
                         
                         ui.collapsing("Items", |ui| {
                             let mut items_to_remove = Vec::new();
                             for (slot, item) in &mut storage.items {
                                 ui.horizontal(|ui| {
                                     ui.label(format!("Slot {}: ", slot));
                                     ui.text_edit_singleline(&mut item.id);
                                     ui.add(egui::DragValue::new(&mut item.quantity).prefix("Qty: "));
                                     if ui.button("X").clicked() {
                                         items_to_remove.push(slot.clone());
                                     }
                                 });
                             }
                             for slot in items_to_remove {
                                 storage.items.remove(&slot);
                             }
                             if ui.button("Add Item").clicked() {
                                 // Simple logic to find next empty slot or just add a placeholder
                                 let next_slot = (0..storage.capacity).find(|i| !storage.items.contains_key(&i.to_string())).unwrap_or(0);
                                 storage.items.insert(next_slot.to_string(), crate::models::player::Item {
                                     id: "New_Item".to_string(),
                                     quantity: 1,
                                     durability: None,
                                 });
                             }
                         });
                     });
                 }
             });
        }
    } else {
        ui.label("No players loaded.");
    }
}
