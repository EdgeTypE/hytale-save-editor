use eframe::egui;
use std::path::PathBuf;
use crate::models::*;

#[derive(Default)]
pub struct HytaleSaveEditor {
    pub current_path: Option<PathBuf>,
    pub active_tab: Tab,
    // We will hold the data here. For now using Option.
    // In a real app we might want a Result to handle errors.
    pub permissions: Option<permissions::Permissions>,
    pub mods_config: Option<config::ModsConfig>,
    pub whitelist: Option<whitelist::Whitelist>,
    pub bans: Option<bans::Bans>,
    pub client_metadata: Option<client_metadata::ClientMetadata>,
    pub memories: Option<memories::Memories>,
    // Players are a map of UUID -> PlayerData
    // We'll need to iterate the players directory
    pub players: Option<std::collections::HashMap<String, player::PlayerData>>,
    pub worlds: Option<std::collections::HashMap<String, world::WorldConfig>>,
    
    // Preview image
    pub preview_image: Option<egui::TextureHandle>,
    pub preview_image_data: Option<egui::ColorImage>,
}

#[derive(PartialEq, Clone, Copy)]
pub enum Tab {
    Dashboard,
    Permissions,
    Mods,
    Security, // Whitelist & Bans
    Memories,
    Players,
    Worlds,
}

impl Default for Tab {
    fn default() -> Self {
        Tab::Dashboard
    }
}

impl HytaleSaveEditor {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_visuals and cc.egui_ctx.set_fonts
        // Load fonts or images if needed
        
        // Ensure image support
        egui_extras::install_image_loaders(&_cc.egui_ctx);

        Self::default()
    }

    fn open_folder_dialog(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_folder() {
             self.current_path = Some(path.clone());
             self.load_data(path);
        }
    }

    fn load_data(&mut self, path: PathBuf) {
        self.permissions = load_json(path.join("permissions.json"));
        self.mods_config = load_json(path.join("config.json"));
        self.whitelist = load_json(path.join("whitelist.json"));
        self.bans = load_json(path.join("bans.json"));
        self.client_metadata = load_json(path.join("client_metadata.json"));
        self.memories = load_json(path.join("universe").join("memories.json"));

        // Load Players
        let players_path = path.join("universe").join("players"); // Correct path based on prompt: x/universe/players/uuid.json (wait, prompt says universe/players/uuid.json, usually it's universe/players/<uuid>.json. Ah, prompt says x/universe/players/uuid.json but then listing shows 60ba....json. So it's a directory of jsons)
        // Adjusting logic to scan directory.
        if let Ok(entries) = std::fs::read_dir(&players_path) {
            let mut players = std::collections::HashMap::new();
            for entry in entries.flatten() {
                 let path = entry.path();
                 if path.extension().and_then(|s| s.to_str()) == Some("json") {
                     if let Some(file_name) = path.file_stem().and_then(|s| s.to_str()) {
                         if let Some(player_data) = load_json::<player::PlayerData>(path.clone()) {
                             players.insert(file_name.to_string(), player_data);
                         }
                     }
                 }
            }
            self.players = Some(players);
        } else {
            self.players = None;
        }

        // Load Worlds
        let worlds_path = path.join("universe").join("worlds");
        if let Ok(entries) = std::fs::read_dir(&worlds_path) {
            let mut worlds = std::collections::HashMap::new();
             for entry in entries.flatten() {
                 if entry.path().is_dir() {
                     if let Some(world_name) = entry.file_name().to_str() {
                         let config_path = entry.path().join("config.json");
                         if let Some(world_config) = load_json::<world::WorldConfig>(config_path) {
                             worlds.insert(world_name.to_string(), world_config);
                         }
                     }
                 }
            }
            self.worlds = Some(worlds);
        } else {
            self.worlds = None;
        }
        
        // Load Preview Image
        self.load_preview_image(path.join("preview.png"));
    }

    fn load_preview_image(&mut self, path: PathBuf) {
        if let Ok(image_reader) = image::ImageReader::open(&path) {
            if let Ok(image) = image_reader.decode() {
                let size = [image.width() as usize, image.height() as usize];
                let image_buffer = image.to_rgba8();
                let pixels = image_buffer.as_flat_samples();
                let color_image = egui::ColorImage::from_rgba_unmultiplied(
                    size,
                    pixels.as_slice(),
                );
                self.preview_image_data = Some(color_image);
            }
        }
    }
    fn save_data(&self) {
        if let Some(path) = &self.current_path {
            if let Some(permissions) = &self.permissions {
                save_json(path.join("permissions.json"), permissions);
            }
            if let Some(mods) = &self.mods_config {
                save_json(path.join("config.json"), mods);
            }
            if let Some(whitelist) = &self.whitelist {
                save_json(path.join("whitelist.json"), whitelist);
            }
            if let Some(bans) = &self.bans {
                save_json(path.join("bans.json"), bans);
            }
            if let Some(meta) = &self.client_metadata {
                save_json(path.join("client_metadata.json"), meta);
            }
            if let Some(memories) = &self.memories {
                save_json(path.join("universe").join("memories.json"), memories);
            }
            
            // Save Players
             if let Some(players) = &self.players {
                 for (uuid, data) in players {
                     let player_path = path.join("universe").join("players").join(format!("{}.json", uuid));
                     save_json(player_path, data);
                 }
             }

            // Save Worlds
            if let Some(worlds) = &self.worlds {
                for (name, config) in worlds {
                    let world_path = path.join("universe").join("worlds").join(name).join("config.json");
                    save_json(world_path, config);
                }
            }
            
            println!("Saved all data.");
        }
    }
}

// Helper generic function
fn load_json<T: serde::de::DeserializeOwned>(path: PathBuf) -> Option<T> {
    if let Ok(file) = std::fs::File::open(&path) {
        let reader = std::io::BufReader::new(file);
        match serde_json::from_reader(reader) {
            Ok(data) => Some(data),
            Err(e) => {
                eprintln!("Failed to parse JSON {:?}: {}", path, e);
                None
            }
        }
    } else {
        None
    }
}

fn save_json<T: serde::Serialize>(path: PathBuf, data: &T) {
    if let Ok(file) = std::fs::File::create(&path) {
        let writer = std::io::BufWriter::new(file);
        if let Err(e) = serde_json::to_writer_pretty(writer, data) {
            eprintln!("Failed to save JSON {:?}: {}", path, e);
        }
    } else {
        eprintln!("Failed to create file {:?}", path);
    }
}


impl eframe::App for HytaleSaveEditor {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Load texture if data is pending
        if let Some(data) = self.preview_image_data.take() {
            self.preview_image = Some(ctx.load_texture("preview", data, Default::default()));
        }

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Open Folder").clicked() {
                    self.open_folder_dialog();
                }
                if let Some(path) = &self.current_path {
                    ui.label(format!("Current: {:?}", path));
                } else {
                    ui.label("No folder selected");
                }
                
                if ui.button("Save All").clicked() {
                    self.save_data();
                }
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
             ui.heading("Menu");
             ui.separator();
             if ui.selectable_value(&mut self.active_tab, Tab::Dashboard, "Dashboard").clicked() {};
             if ui.selectable_value(&mut self.active_tab, Tab::Permissions, "Permissions").clicked() {};
             if ui.selectable_value(&mut self.active_tab, Tab::Mods, "Mods").clicked() {};
             if ui.selectable_value(&mut self.active_tab, Tab::Security, "Whitelist & Bans").clicked() {};
             if ui.selectable_value(&mut self.active_tab, Tab::Memories, "Memories").clicked() {};
             if ui.selectable_value(&mut self.active_tab, Tab::Players, "Players").clicked() {};
             if ui.selectable_value(&mut self.active_tab, Tab::Worlds, "Worlds").clicked() {};
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.active_tab {
                Tab::Dashboard => crate::gui::views::dashboard::show(ui, self),
                Tab::Permissions => crate::gui::views::permissions::show(ui, self),
                Tab::Mods => crate::gui::views::mods::show(ui, self),
                Tab::Security => crate::gui::views::security::show(ui, self),
                Tab::Memories => crate::gui::views::memories::show(ui, self),
                Tab::Players => crate::gui::views::players::show(ui, self),
                Tab::Worlds => crate::gui::views::worlds::show(ui, self),
            }
        });
    }
}
