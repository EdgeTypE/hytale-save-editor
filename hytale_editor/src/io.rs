use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use crate::models::*;

pub fn load_save_data(root_path: PathBuf) -> Result<SaveData, String> {
    if !root_path.exists() {
        return Err("Directory does not exist".to_string());
    }

    let permissions = load_json(&root_path.join("permissions.json"));
    let mods = load_json(&root_path.join("config.json"));
    let whitelist = load_json(&root_path.join("whitelist.json"));
    let bans = load_json(&root_path.join("bans.json"));

    let memories_path = root_path.join("universe").join("memories.json");
    let memories = load_json(&memories_path);

    // Load Worlds
    let mut worlds = HashMap::new();
    let worlds_dir = root_path.join("universe").join("worlds");
    if let Ok(entries) = fs::read_dir(&worlds_dir) {
        for entry in entries.flatten() {
            if let Ok(ft) = entry.file_type() {
                if ft.is_dir() {
                    let world_name = entry.file_name().to_string_lossy().to_string();
                    let config_path = entry.path().join("config.json");
                    if let Some(config) = load_json::<WorldConfig>(&config_path) {
                        worlds.insert(world_name, config);
                    }
                }
            }
        }
    }

    // Load Players
    let mut players = HashMap::new();
    let players_dir = root_path.join("universe").join("players");
    if let Ok(entries) = fs::read_dir(&players_dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                // The filename is the key (uuid usually)
                let filename = path.file_stem().map(|s| s.to_string_lossy().to_string()).unwrap_or_default();
                if filename != "uuid" { // Explicitly ignore scanning uuid.json literal if it's a directory or weird artifact, but usually uuid.json is the file.
                    // Actually, the prompt showed `x/universe/players/uuid.json` as a literal file maybe?
                    // No, `60ba750f...json`. `uuid` in the prompt likely meant the directory or placeholder.
                    // But wait, there is `x/universe/players/uuid.json` in prompt.
                    // "x/universe/players/uuid.json: 60ba750f...json" -> This implies the prompt meant "inside players dir, files named by uuid".

                    if let Some(player_file) = load_json::<PlayerFile>(&path) {
                        players.insert(filename, player_file);
                    }
                }
            }
        }
    }

    Ok(SaveData {
        root_path,
        permissions,
        mods,
        whitelist,
        bans,
        memories,
        worlds,
        players,
    })
}

pub fn save_json<T: serde::Serialize>(path: &Path, data: &T) -> Result<(), String> {
    let content = serde_json::to_string_pretty(data)
        .map_err(|e| format!("Failed to serialize data: {}", e))?;
    fs::write(path, content)
        .map_err(|e| format!("Failed to write file {:?}: {}", path, e))
}

fn load_json<T: serde::de::DeserializeOwned>(path: &Path) -> Option<T> {
    if !path.exists() {
        return None;
    }
    let content = fs::read_to_string(path).ok()?;
    match serde_json::from_str(&content) {
        Ok(data) => Some(data),
        Err(e) => {
            eprintln!("Failed to parse JSON {:?}: {}", path, e);
            None
        }
    }
}
