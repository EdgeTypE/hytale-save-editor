use eframe::egui;
use std::path::PathBuf;
use crate::models::*;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum Language {
    English,
    Turkish,
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

// New Struct for Save Summary
pub struct SaveSummary {
    pub folder_name: String,
    pub path: PathBuf,
    pub display_name: String, // From config or folder name
    // pub preview_path: Option<PathBuf>, // Removed to fix warning
    pub last_modified: std::time::SystemTime,
    pub preview_image_data: Option<egui::ColorImage>,
    pub texture_handle: Option<egui::TextureHandle>,
}

// Removed derive(Default) because Sender doesn't implement it
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
    
    // API Cache
    pub profile_cache: crate::api::ProfileCache,
    pub avatar_textures: std::collections::HashMap<String, egui::TextureHandle>,
    
    // UI State
    pub new_op_input: String,
    pub new_whitelist_input: String,
    pub new_ban_input: String,
    
    // Mods manifests
    pub manifests: std::collections::HashMap<String, manifest::ModManifest>,
    
    // Async Events
    pub api_tx: std::sync::mpsc::Sender<crate::api::ApiEvent>,
    pub api_rx: std::sync::mpsc::Receiver<crate::api::ApiEvent>,
    
    // Asset Manager
    pub asset_manager: crate::assets::AssetManager,
    
    // Auto-detected saves
    // Auto-detected saves
    pub available_saves: Vec<SaveSummary>,

    // Localization
    pub language: Language,
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
    Settings,
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

        let (api_tx, api_rx) = std::sync::mpsc::channel();

        Self {
            current_path: None,
            active_tab: Tab::Dashboard,
            permissions: None,
            mods_config: None,
            whitelist: None,
            bans: None,
            client_metadata: None,
            memories: None,
            players: None,
            worlds: None,
            preview_image: None,
            preview_image_data: None,
            profile_cache: crate::api::ProfileCache::default(),
            avatar_textures: std::collections::HashMap::new(),
            new_op_input: String::new(),
            new_whitelist_input: String::new(),
            new_ban_input: String::new(),
            manifests: std::collections::HashMap::new(),
            api_tx,
            api_rx,
            asset_manager: crate::assets::AssetManager::new(),

            available_saves: Self::detect_saves(),
            language: Language::English,
        }
    }

    fn detect_saves() -> Vec<SaveSummary> {
        let mut saves = Vec::new();
        if let Some(roaming) = dirs::config_dir() {
            let hytale_saves = roaming.join("Hytale").join("UserData").join("Saves");
            if hytale_saves.exists() {
                 if let Ok(entries) = std::fs::read_dir(hytale_saves) {
                     for entry in entries.flatten() {
                         if entry.path().is_dir() {
                             if let Some(name) = entry.file_name().to_str() {
                                 let path = entry.path();
                                 let mut display_name = name.to_string();
                                 // let mut preview_path = None;
                                 let mut last_modified = std::time::SystemTime::UNIX_EPOCH;

                                 // Get modification time
                                 if let Ok(metadata) = std::fs::metadata(&path) {
                                     if let Ok(modified) = metadata.modified() {
                                         last_modified = modified;
                                     }
                                 }

                                 // Try to find display Name in universe/worlds/*/config.json
                                 let worlds_path = path.join("universe").join("worlds");
                                 if let Ok(world_entries) = std::fs::read_dir(worlds_path) {
                                     for w_entry in world_entries.flatten() {
                                         if w_entry.path().is_dir() {
                                              let config_path = w_entry.path().join("config.json");
                                              if let Some(first_world_config) = load_json::<world::WorldConfig>(config_path) {
                                                  display_name = first_world_config.display_name;
                                                  break; // Found one
                                              }
                                         }
                                     }
                                 }

                                 // Check for preview.png at root
                                 let p_path = path.join("preview.png");
                                 let mut preview_image_data = None;
                                 if p_path.exists() {
                                     preview_image_data = Self::load_image_data(&p_path);
                                     // preview_path = Some(p_path);
                                 }

                                 saves.push(SaveSummary {
                                     folder_name: name.to_string(),
                                     path,
                                     display_name,
                                     // preview_path,
                                     last_modified,
                                     preview_image_data,
                                     texture_handle: None,
                                 });
                             }
                         }
                     }
                 }
            }
        }
        // Sort by last modified (descending)
        saves.sort_by(|a, b| b.last_modified.cmp(&a.last_modified));
        saves
    }

    fn load_image_data(path: &PathBuf) -> Option<egui::ColorImage> {
        if let Ok(image_reader) = image::ImageReader::open(path) {
            if let Ok(image) = image_reader.decode() {
                let size = [image.width() as usize, image.height() as usize];
                let image_buffer = image.to_rgba8();
                let pixels = image_buffer.as_flat_samples();
                return Some(egui::ColorImage::from_rgba_unmultiplied(
                    size,
                    pixels.as_slice(),
                ));
            }
        }
        None
    }

    pub fn open_folder_dialog(&mut self) {
        if let Some(path) = rfd::FileDialog::new().pick_folder() {
             self.current_path = Some(path.clone());
             self.load_data(path);
        }
    }

    pub fn load_data(&mut self, path: PathBuf) {
        self.permissions = load_json(path.join("permissions.json"));
        self.mods_config = load_json(path.join("config.json"));
        self.whitelist = load_json(path.join("whitelist.json")).or(Some(Default::default()));
        self.bans = load_json(path.join("bans.json")).or(Some(Default::default()));
        self.bans = load_json(path.join("bans.json")).or(Some(Default::default()));
        self.client_metadata = load_json(path.join("client_metadata.json"));
        self.memories = load_json(path.join("universe").join("memories.json"));

        // Load Players
        let players_path = path.join("universe").join("players"); 
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
        
        // Load Mod Manifests
        self.load_manifests(path.clone());
    }
    
    fn load_manifests(&mut self, save_path: PathBuf) {
        // Assume structure: .../Hytale/UserData/Saves/SaveName -> save_path
        // Mods should be at: .../Hytale/UserData/Mods
        
        // This logic is a bit heuristic. We go up twice.
        // If the structure is strictly maintained:
        if let Some(user_data) = save_path.parent().and_then(|p| p.parent()) {
            let mods_path = user_data.join("Mods");
            if mods_path.exists() && mods_path.is_dir() {
                if let Ok(entries) = std::fs::read_dir(mods_path) {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.extension().and_then(|s| s.to_str()) == Some("jar") {
                            // Open jar
                            if let Ok(file) = std::fs::File::open(&path) {
                                if let Ok(mut archive) = zip::ZipArchive::new(file) {
                                    // Try manifest.json first, then hytale.json
                                    let mut content = None;
                                    
                                    // Check manifest.json first
                                    if let Ok(mut file) = archive.by_name("manifest.json") {
                                        let mut s = String::new();
                                        if std::io::Read::read_to_string(&mut file, &mut s).is_ok() {
                                            content = Some(s);
                                        }
                                    }
                                    
                                    // If not found, check hytale.json
                                    if content.is_none() {
                                         if let Ok(mut file) = archive.by_name("hytale.json") {
                                             let mut s = String::new();
                                             if std::io::Read::read_to_string(&mut file, &mut s).is_ok() {
                                                 content = Some(s);
                                             }
                                         }
                                    }
                                    
                                    if let Some(content) = content {
                                        if let Ok(manifest) = serde_json::from_str::<manifest::ModManifest>(&content) {
                                            self.manifests.insert(manifest.id(), manifest);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
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
    pub fn save_data(&self) {
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

        // Load Save Summary Textures
        for save in &mut self.available_saves {
             if save.texture_handle.is_none() {
                 if let Some(data) = save.preview_image_data.take() {
                      save.texture_handle = Some(ctx.load_texture(&save.folder_name, data, Default::default()));
                 }
             }
        }

        // Handle Async Events
        while let Ok(event) = self.api_rx.try_recv() {
            match event {
                crate::api::ApiEvent::AddToWhitelist(uuid) => {
                    if let Some(whitelist) = &mut self.whitelist {
                        if !whitelist.list.contains(&uuid) {
                            whitelist.list.push(uuid);
                        }
                    }
                }
                crate::api::ApiEvent::AddToBans(uuid) => {
                     if let Some(bans) = &mut self.bans {
                         // Check if already banned?
                         if !bans.iter().any(|b| b.target == uuid) {
                             bans.push(crate::models::bans::BanEntry {
                                 target: uuid,
                                 ban_type: "Global".to_string(), // Default
                                 reason: "Banned by Editor".to_string(),
                                 by: "Editor".to_string(),
                                 timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis() as i64
                             });
                         }
                     }
                }
                crate::api::ApiEvent::Error(msg) => {
                    eprintln!("API Error: {}", msg);
                    // Optionally show a toast or error message in UI
                }
            }
        }
        
        // Process loaded avatars
        let mut avatars_to_load = Vec::new();
        if let Ok(mut avatars) = self.profile_cache.avatars.lock() {
            for (uuid, data) in avatars.iter_mut() {
                if let Some(image) = data.take() {
                    avatars_to_load.push((uuid.clone(), image));
                }
            }
        }
        for (uuid, image) in avatars_to_load {
            self.avatar_textures.insert(uuid.clone(), ctx.load_texture(&uuid, image, Default::default()));
        }

        // Update Asset Manager (process background loads)
        self.asset_manager.update(ctx);

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

        // Global Footer (Moved from Dashboard to here for consistency and layout safe)
        egui::TopBottomPanel::bottom("bottom_panel").show(ctx, |ui| {
            egui::Frame::none()
                .fill(ui.visuals().window_fill()) 
                .inner_margin(5.0)
                .show(ui, |ui| {
                    ui.set_width(ui.available_width());
                    ui.horizontal(|ui| {
                         ui.label(egui::RichText::new("Hytale Save Editor").strong().size(12.0));
                         ui.label(egui::RichText::new("by EdgeTypE").weak().size(10.0));
                         
                         ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.hyperlink_to(
                                egui::RichText::new("GitHub").color(egui::Color32::from_rgb(100, 149, 237)), 
                                "https://github.com/EdgeTypE/hytale-save-editor"
                            );
                        });
                    });
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
             ui.separator();
             if ui.selectable_value(&mut self.active_tab, Tab::Settings, "Settings").clicked() {};
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.active_tab {
                Tab::Dashboard => crate::gui::views::dashboard::show(ui, self),
                Tab::Permissions => crate::gui::views::permissions::show(ui, self),
                Tab::Mods => crate::gui::views::mods::show(ui, self),
                Tab::Security => crate::gui::views::security::show(ui, self),
                Tab::Memories => crate::gui::views::memories::show(ui, self),
                Tab::Players => {
                    if let Some(players) = &mut self.players {
                        crate::gui::views::players::show(ui, players, &mut self.asset_manager);
                    } else {
                        ui.label("No players loaded.");
                    }
                },
                Tab::Worlds => crate::gui::views::worlds::show(ui, self),
                Tab::Settings => crate::gui::views::settings::show(ui, self),
            }
        });
    }
}
