use serde::Deserialize;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use eframe::egui;

#[derive(Debug, Deserialize, Clone)]
pub struct CraftheadProfile {
    pub id: String,
    pub name: String,
    // Add other fields if needed, but name is the priority
}

#[derive(Clone)]
pub struct ProfileCache {
    pub profiles: Arc<Mutex<HashMap<String, Option<CraftheadProfile>>>>,
    pub avatars: Arc<Mutex<HashMap<String, Option<egui::ColorImage>>>>,
}

impl Default for ProfileCache {
    fn default() -> Self {
        Self {
            profiles: Arc::new(Mutex::new(HashMap::new())),
            avatars: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

pub fn fetch_profile(uuid: String, cache: ProfileCache, ctx: egui::Context) {
    let url = format!("https://crafthead.net/hytale/profile/{}", uuid);
    let request = ehttp::Request::get(&url);
    
    let cache_clone = cache.clone();
    let uuid_clone = uuid.clone();
    let ctx_clone = ctx.clone();

    ehttp::fetch(request, move |response| {
        if let Ok(response) = response {
            if response.status == 200 {
                if let Ok(profile) = serde_json::from_slice::<CraftheadProfile>(&response.bytes) {
                    if let Ok(mut profiles) = cache_clone.profiles.lock() {
                        profiles.insert(uuid_clone.clone(), Some(profile));
                    }
                }
            } else {
                 // Mark as failed/not found to stop retrying
                 if let Ok(mut profiles) = cache_clone.profiles.lock() {
                    profiles.insert(uuid_clone.clone(), None);
                }
            }
        }
        ctx_clone.request_repaint();
    });
}

pub fn fetch_avatar(uuid: String, cache: ProfileCache, ctx: egui::Context) {
    let url = format!("https://crafthead.net/hytale/avatar/{}", uuid);
    let request = ehttp::Request::get(&url);

    let cache_clone = cache.clone();
    let uuid_clone = uuid.clone();
    let ctx_clone = ctx.clone();

    ehttp::fetch(request, move |response| {
        if let Ok(response) = response {
             if response.status == 200 {
                let cursor = std::io::Cursor::new(response.bytes);
                if let Ok(image_reader) = image::ImageReader::new(cursor).with_guessed_format() {
                     if let Ok(image) = image_reader.decode() {
                         let size = [image.width() as usize, image.height() as usize];
                         let image_buffer = image.to_rgba8();
                         let pixels = image_buffer.as_flat_samples();
                         let color_image = egui::ColorImage::from_rgba_unmultiplied(
                             size,
                             pixels.as_slice(),
                         );
                         if let Ok(mut avatars) = cache_clone.avatars.lock() {
                             avatars.insert(uuid_clone, Some(color_image));
                         }
                    }
                }
             }
        }
        ctx_clone.request_repaint();
    });
}
