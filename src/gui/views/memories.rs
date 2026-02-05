use eframe::egui;
use crate::gui::app::HytaleSaveEditor;

pub fn show(ui: &mut egui::Ui, app: &mut HytaleSaveEditor) {
    ui.heading("Universe Memories");
    if let Some(memories) = &mut app.memories {
        egui::ScrollArea::vertical().show(ui, |ui| {
            let mut memory_to_remove = None;
            for (i, memory) in memories.memories.iter_mut().enumerate() {
                ui.group(|ui| {
                    ui.horizontal(|ui| {
                        ui.label("Role:");
                        ui.text_edit_singleline(&mut memory.npc_role);
                        if ui.button("X").clicked() {
                            memory_to_remove = Some(i);
                        }
                    });
                    ui.horizontal(|ui| {
                        ui.label("Trans Key:");
                        ui.text_edit_singleline(&mut memory.translation_key);
                    });
                    ui.checkbox(&mut memory.is_memories_name_overridden, "Name Overridden");
                });
            }
            if let Some(i) = memory_to_remove {
                memories.memories.remove(i);
            }
            
            if ui.button("Add Memory").clicked() {
                memories.memories.push(crate::models::memories::MemoryEntry {
                    id: "NPC".to_string(),
                    npc_role: "New_Role".to_string(),
                    translation_key: "key".to_string(),
                    is_memories_name_overridden: false,
                    captured_timestamp: 0,
                    found_location_name_key: "Location".to_string(),
                });
            }
        });
    } else {
        ui.label("No memories loaded.");
    }
}
