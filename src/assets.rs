use eframe::egui;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;
use std::io::Read;

pub struct AssetManager {
    pub hytale_assets_path: Option<PathBuf>,
    pub texture_cache: HashMap<String, Option<egui::TextureHandle>>, // None means loading/failed
    pub pending_requests: HashSet<String>,
    
    // Communication with background thread
    request_tx: Sender<String>,
    result_rx: Receiver<(String, Option<egui::ColorImage>)>,
}

impl Default for AssetManager {
    fn default() -> Self {
        let (request_tx, _) = channel();
        let (_, result_rx) = channel();
        Self {
            hytale_assets_path: None,
            texture_cache: HashMap::new(),
            pending_requests: HashSet::new(),
            request_tx,
            result_rx,
        }
    }
}

impl AssetManager {
    pub fn new() -> Self {
        let (request_tx, request_rx) = channel::<String>();
        let (result_tx, result_rx) = channel::<(String, Option<egui::ColorImage>)>();
        
        // Spawn background worker
        thread::spawn(move || {
            let mut hytale_user_data: Option<PathBuf> = None;
            
            // Try dirs crate first (Standard)
            if let Some(config_dir) = dirs::config_dir() {
                 let candidate = config_dir.join("Hytale");
                 if candidate.exists() {
                     hytale_user_data = Some(candidate);
                 }
            }
            
            // Fallback to Env var if dirs fails or path doesn't exist there
            if hytale_user_data.is_none() {
                if let Ok(appdata) = std::env::var("APPDATA") {
                    let candidate = PathBuf::from(appdata).join("Hytale");
                    if candidate.exists() {
                        hytale_user_data = Some(candidate);
                    }
                }
            }

            let assets_path = hytale_user_data.clone().map(|p| p.join("install/release/package/game/latest/Assets.zip"));
            let mods_path = hytale_user_data.clone().map(|p| p.join("UserData/Mods")); // Assuming this path
            
            // Cache Dir: "assets" folder in the current working directory (Project Root during dev, Exe dir during release)
            let cache_dir = std::env::current_dir().unwrap_or_default().join("assets");
                
            if !cache_dir.exists() {
                let _ = std::fs::create_dir_all(&cache_dir);
            }

            let mut archive_opt = None;
            if let Some(path) = &assets_path {
                 if let Ok(file) = std::fs::File::open(path) {
                     if let Ok(archive) = zip::ZipArchive::new(file) {
                         archive_opt = Some(archive);
                     }
                 }
            }

            while let Ok(item_id) = request_rx.recv() {
                let image_data = load_icon(&mut archive_opt, &item_id, &cache_dir, &mods_path);
                let _ = result_tx.send((item_id, image_data));
            }
        });

        let mut manager = Self {
            hytale_assets_path: None, 
            texture_cache: HashMap::new(),
            pending_requests: HashSet::new(),
            request_tx,
            result_rx,
        };
        
        manager.detect_hytale_assets();
        manager
    }

    pub fn detect_hytale_assets(&mut self) {
         if let Some(appdata) = dirs::config_dir().and_then(|p| p.parent().map(|p| p.join("Roaming"))) {
             let path = appdata.join("Hytale/install/release/package/game/latest/Assets.zip");
             if path.exists() {
                 self.hytale_assets_path = Some(path);
             }
        }
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        // Collect results
        while let Ok((item_id, image_opt)) = self.result_rx.try_recv() {
            self.pending_requests.remove(&item_id);
            if let Some(image) = image_opt {
                let texture = ctx.load_texture(&item_id, image, Default::default());
                self.texture_cache.insert(item_id, Some(texture));
            } else {
                // Failed to load
                self.texture_cache.insert(item_id, None);
            }
        }
    }

    pub fn get_icon(&mut self, _ctx: &egui::Context, item_id: &str) -> Option<egui::TextureHandle> {
        if let Some(texture_opt) = self.texture_cache.get(item_id) {
            return texture_opt.clone();
        }
        
        if !self.pending_requests.contains(item_id) {
            self.pending_requests.insert(item_id.to_string());
            let _ = self.request_tx.send(item_id.to_string());
        }
        
        None
    }
}

    // Helper function to extract icon - runs in background thread
    fn load_icon(
        archive: &mut Option<zip::ZipArchive<std::fs::File>>,
        item_id: &str,
        cache_dir: &std::path::Path,
        mods_dir: &Option<PathBuf>,
    ) -> Option<egui::ColorImage> {
        // 1. Check Cache
        let safe_id = item_id.replace(":", "_"); 
        let cache_path = cache_dir.join(format!("{}.png", safe_id));
        
        if cache_path.exists() {
             if let Ok(image) = image::open(&cache_path) {
                 let size = [image.width() as usize, image.height() as usize];
                 let image_buffer = image.to_rgba8();
                 let pixels = image_buffer.as_flat_samples();
                 return Some(egui::ColorImage::from_rgba_unmultiplied(
                     size,
                     pixels.as_slice(),
                 ));
             }
        }

        // 2. Search Mods (Priority over base game?)
        // Hytale mods: UserData/Mods/[ModName]/assets/...
        // We need to look for {item_id}.json to find the icon path.
        let mut icon_path_in_mod: Option<(PathBuf, String)> = None; // (ModRoot, RelativePath)
        
        if let Some(mods_path) = mods_dir {
             if let Ok(entries) = std::fs::read_dir(mods_path) {
                 for entry in entries.flatten() {
                     let mod_root = entry.path();
                     if mod_root.is_dir() {
                         // Naive search for definition json: {item_id}.json
                         // This is expensive if we do it for every item.
                         // Optimization: AssetManager should ideally index mods once. 
                         // For now, we search recursively for the JSON.
                         if let Some(json_path) = find_file_recursive(&mod_root, &format!("{}.json", item_id)) {
                             // Parse JSON
                             if let Ok(content) = std::fs::read_to_string(&json_path) {
                                  if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                                      if let Some(icon) = json.get("Icon").and_then(|v| v.as_str()) {
                                          icon_path_in_mod = Some((mod_root.clone(), icon.to_string()));
                                          break;
                                      }
                                  }
                             }
                         }
                     }
                 }
             }
        }
        
        // Load from Mod if found
        if let Some((mod_root, icon_relative_path)) = icon_path_in_mod {
             // Try to find the icon file in the mod
             // The icon path in JSON might be consistent "assets/textures/..." or just filename
             let target_name = std::path::Path::new(&icon_relative_path)
                .file_name().unwrap_or_default().to_string_lossy().to_string();
                
             if let Some(real_icon_path) = find_file_recursive(&mod_root, &target_name) {
                 if let Ok(image) = image::open(&real_icon_path) {
                     let _ = image.save(&cache_path); // Save to cache
                     
                     let size = [image.width() as usize, image.height() as usize];
                     let image_buffer = image.to_rgba8();
                     let pixels = image_buffer.as_flat_samples();
                     return Some(egui::ColorImage::from_rgba_unmultiplied(
                         size,
                         pixels.as_slice(),
                     ));
                 }
             }
        }
        
        // 3. Search Base Game (Zip)
        if let Some(archive) = archive {
           if let Some(image) = load_icon_from_archive(archive, item_id) {
               // Convert egui::ColorImage back to DynamicImage to save
               // This is a bit inefficient (decode -> egui -> encode), but easiest with current helpers.
               // Ideally load_icon_from_archive would return DynamicImage. 
               // For now, let's reconstruct it to save.
               
               if let Some(img_buffer) = image::RgbaImage::from_raw(
                   image.size[0] as u32, 
                   image.size[1] as u32, 
                   image.pixels.iter().flat_map(|c| c.to_array()).collect()
               ) {
                   let dynamic_image = image::DynamicImage::ImageRgba8(img_buffer);
                   let _ = dynamic_image.save(&cache_path);
               }
               
               return Some(image);
           }
        }
        
        None
    }

    fn find_file_recursive(dir: &std::path::Path, file_name: &str) -> Option<PathBuf> {
        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(found) = find_file_recursive(&path, file_name) {
                        return Some(found);
                    }
                } else {
                    if let Some(name) = path.file_name() {
                         if name.to_string_lossy().eq_ignore_ascii_case(file_name) {
                             return Some(path);
                         }
                    }
                }
            }
        }
        None
    }

    fn load_icon_from_archive(archive: &mut zip::ZipArchive<std::fs::File>, item_id: &str) -> Option<egui::ColorImage> {
        // ... (existing logic) ...
        // We should add saving to cache here if we want complete caching.
        // For brevity in this diff, I'll essentially paste the old logic but add saving capability later 
        // OR better: make this function return the Bytes/DynamicImage so the caller can save.
        
        // 1. Find JSON Index
        let json_name = format!("{}.json", item_id);
        let mut json_index = None;
        
        for i in 0..archive.len() {
            if let Ok(file) = archive.by_index(i) {
                 if let Some(name) = file.enclosed_name().and_then(|s| s.to_str()) {
                     if name.ends_with(&json_name) || name.ends_with(&json_name.to_lowercase()) {
                         json_index = Some(i);
                         break;
                     }
                 }
            }
        }
        
        let mut icon_path = None;
        if let Some(index) = json_index {
            if let Ok(mut file) = archive.by_index(index) {
                let mut s = String::new();
                if file.read_to_string(&mut s).is_ok() {
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&s) {
                        if let Some(icon) = json.get("Icon").and_then(|v| v.as_str()) {
                            icon_path = Some(icon.to_string());
                        }
                    }
                }
            }
        }
        
        if let Some(path) = icon_path {
             let target_path = path.replace("\\", "/");
             let mut icon_index = None;
             
             // Optimization: exact match first?
             
             for i in 0..archive.len() {
                 if let Ok(file) = archive.by_index(i) {
                     if let Some(name) = file.enclosed_name().and_then(|s| s.to_str()) {
                         if (name.to_lowercase().ends_with(&target_path.to_lowercase()) || 
                            name.to_lowercase().contains(&target_path.to_lowercase())) && name.ends_with(".png") {
                             icon_index = Some(i);
                             break;
                         }
                     }
                 }
             }
             
             if let Some(index) = icon_index {
                 if let Ok(mut file) = archive.by_index(index) {
                     let mut buffer = Vec::new();
                     if file.read_to_end(&mut buffer).is_ok() {
                          // HERE: Save to cache if we had the path. 
                          // But we don't have safe access to `item_id` and `cache_dir` cleanly without passing more args.
                          // Let's just return the image and `load_icon` wrapper can deal with saving if refactored.
                          // For now, let's keep it simple: return image.
                          
                          if let Ok(image) = image::load_from_memory(&buffer) {
                                // Save to Cache hack? No, let's do it properly next step if needed.
                                
                                let size = [image.width() as usize, image.height() as usize];
                                let image_buffer = image.to_rgba8();
                                let pixels = image_buffer.as_flat_samples();
                                return Some(egui::ColorImage::from_rgba_unmultiplied(
                                  size,
                                  pixels.as_slice(),
                                ));
                          }
                     }
                 }
             }
        }
        None
    }
