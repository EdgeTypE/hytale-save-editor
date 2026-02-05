use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorldConfig {
    #[serde(rename = "Version")]
    pub version: Option<u32>,
    #[serde(rename = "UUID")]
    pub uuid: Option<MongoUUID>,
    #[serde(rename = "DisplayName")]
    pub display_name: String, // Keeping this mandatory as we need it for display
    #[serde(rename = "Seed")]
    pub seed: Option<i64>,
    #[serde(rename = "SpawnProvider")]
    pub spawn_provider: Option<SpawnProvider>,
    #[serde(rename = "WorldGen")]
    pub world_gen: Option<WorldGen>,
    #[serde(rename = "WorldMap")]
    pub world_map: Option<WorldMap>,
    #[serde(rename = "ChunkStorage")]
    pub chunk_storage: Option<ChunkStorage>,
    #[serde(rename = "ChunkConfig")]
    pub chunk_config: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "IsTicking")]
    pub is_ticking: Option<bool>,
    #[serde(rename = "IsBlockTicking")]
    pub is_block_ticking: Option<bool>,
    #[serde(rename = "IsPvpEnabled")]
    pub is_pvp_enabled: Option<bool>,
    #[serde(rename = "IsFallDamageEnabled")]
    pub is_fall_damage_enabled: Option<bool>,
    #[serde(rename = "IsGameTimePaused")]
    pub is_game_time_paused: Option<bool>,
    #[serde(rename = "GameTime")]
    pub game_time: Option<String>,
    #[serde(rename = "ClientEffects")]
    pub client_effects: Option<ClientEffects>,
    #[serde(rename = "RequiredPlugins")]
    pub required_plugins: Option<HashMap<String, serde_json::Value>>,
    #[serde(rename = "GameMode")]
    pub game_mode: Option<String>,
    #[serde(rename = "IsSpawningNPC")]
    pub is_spawning_npc: Option<bool>,
    #[serde(rename = "IsSpawnMarkersEnabled")]
    pub is_spawn_markers_enabled: Option<bool>,
    #[serde(rename = "IsAllNPCFrozen")]
    pub is_all_npc_frozen: Option<bool>,
    #[serde(rename = "GameplayConfig")]
    pub gameplay_config: Option<String>,
    #[serde(rename = "IsCompassUpdating")]
    pub is_compass_updating: Option<bool>,
    #[serde(rename = "IsSavingPlayers")]
    pub is_saving_players: Option<bool>,
    #[serde(rename = "IsSavingChunks")]
    pub is_saving_chunks: Option<bool>,
    #[serde(rename = "SaveNewChunks")]
    pub save_new_chunks: Option<bool>,
    #[serde(rename = "IsUnloadingChunks")]
    pub is_unloading_chunks: Option<bool>,
    #[serde(rename = "IsObjectiveMarkersEnabled")]
    pub is_objective_markers_enabled: Option<bool>,
    #[serde(rename = "DeleteOnUniverseStart")]
    pub delete_on_universe_start: Option<bool>,
    #[serde(rename = "DeleteOnRemove")]
    pub delete_on_remove: Option<bool>,
    #[serde(rename = "ResourceStorage")]
    pub resource_storage: Option<ResourceStorage>,
    #[serde(rename = "Plugin")]
    pub plugin: Option<HashMap<String, serde_json::Value>>,
    
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MongoUUID {
   #[serde(rename = "$binary")]
   pub binary: String,
   #[serde(rename = "$type")]
   pub type_: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpawnProvider {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "SpawnPoint")]
    pub spawn_point: SpawnPoint,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SpawnPoint {
    #[serde(rename = "X")]
    pub x: f64,
    #[serde(rename = "Y")]
    pub y: f64,
    #[serde(rename = "Z")]
    pub z: f64,
    #[serde(rename = "Pitch")]
    pub pitch: f64,
    #[serde(rename = "Yaw")]
    pub yaw: f64,
    #[serde(rename = "Roll")]
    pub roll: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorldGen {
    #[serde(rename = "Type")]
    pub type_: String,
    #[serde(rename = "WorldStructure")]
    pub world_structure: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorldMap {
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChunkStorage {
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ResourceStorage {
    #[serde(rename = "Type")]
    pub type_: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientEffects {
    #[serde(rename = "SunHeightPercent")]
    pub sun_height_percent: f64,
    #[serde(rename = "SunAngleDegrees")]
    pub sun_angle_degrees: f64,
    #[serde(rename = "BloomIntensity")]
    pub bloom_intensity: f64,
    #[serde(rename = "BloomPower")]
    pub bloom_power: f64,
    #[serde(rename = "SunIntensity")]
    pub sun_intensity: f64,
    #[serde(rename = "SunshaftIntensity")]
    pub sunshaft_intensity: f64,
    #[serde(rename = "SunshaftScaleFactor")]
    pub sunshaft_scale_factor: f64,
    
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}
