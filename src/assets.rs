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
            let mut assets_path: Option<PathBuf> = None;
            
            // Try to detect path in background or wait for it? 
            // Better to detect it once here or pass it in. 
            // For simplicity, let's re-detect or pass it. 
            // Passing is hard since we spawn in default/new.
            // Let's implement robust detection here.
            
            if let Some(appdata) = dirs::config_dir().and_then(|p| p.parent().map(|p| p.join("Roaming"))) {
                let path = appdata.join("Hytale/install/release/package/game/latest/Assets.zip");
                if path.exists() {
                     assets_path = Some(path);
                }
            }
            if assets_path.is_none() {
                 let path = PathBuf::from(r"C:\Users\Edige\AppData\Roaming\Hytale\install\release\package\game\latest\Assets.zip");
                 if path.exists() {
                     assets_path = Some(path);
                 }
            }

            // Keep archive open? No, `zip` crate takes ownership of file often or borrows.
            // We can keep `Option<ZipArchive<File>>`.
            // But we need to handle "path not found" initially.
            
            let mut archive_opt = None;
            if let Some(path) = &assets_path {
                 if let Ok(file) = std::fs::File::open(path) {
                     if let Ok(archive) = zip::ZipArchive::new(file) {
                         archive_opt = Some(archive);
                     }
                 }
            }

            while let Ok(item_id) = request_rx.recv() {
                if let Some(archive) = &mut archive_opt {
                    let image_data = load_icon_from_archive(archive, &item_id);
                    let _ = result_tx.send((item_id, image_data));
                } else {
                    // Try to init if not ready (re-check path if we allowed setting it later)
                    // For now, just fail
                    let _ = result_tx.send((item_id, None));
                }
            }
        });

        let mut manager = Self {
            hytale_assets_path: None, // We don't really need this in main thread anymore except for UI display
            texture_cache: HashMap::new(),
            pending_requests: HashSet::new(),
            request_tx,
            result_rx,
        };
        
        // Populate path for UI (this is just for display now)
        manager.detect_hytale_assets();
        manager
    }

    pub fn detect_hytale_assets(&mut self) {
        // Just for UI display, worker detects its own
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
fn load_icon_from_archive(archive: &mut zip::ZipArchive<std::fs::File>, item_id: &str) -> Option<egui::ColorImage> {
    // 1. Find JSON Index
    let json_name = format!("{}.json", item_id);
    let mut json_index = None;
    
    // Optimization: This linear scan is still slow per item. 
    // Ideally we'd index once. But for "zamanla gelsin" (come over time), it's acceptable if it doesn't block UI.
    // Indexing the whole zip takes time.
    
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
                      if let Ok(image) = image::load_from_memory(&buffer) {
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
