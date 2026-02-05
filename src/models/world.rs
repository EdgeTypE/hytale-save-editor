use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorldConfig {
    #[serde(rename = "Version")]
    pub version: u32,
    #[serde(rename = "UUID")]
    pub uuid: MongoUUID,
    #[serde(rename = "DisplayName")]
    pub display_name: String,
    #[serde(rename = "Seed")]
    pub seed: i64,
    #[serde(rename = "SpawnProvider")]
    pub spawn_provider: SpawnProvider,
    #[serde(rename = "WorldGen")]
    pub world_gen: WorldGen,
    #[serde(rename = "WorldMap")]
    pub world_map: WorldMap,
    #[serde(rename = "ChunkStorage")]
    pub chunk_storage: ChunkStorage,
    #[serde(rename = "ChunkConfig")]
    pub chunk_config: HashMap<String, serde_json::Value>,
    #[serde(rename = "IsTicking")]
    pub is_ticking: bool,
    #[serde(rename = "IsBlockTicking")]
    pub is_block_ticking: bool,
    #[serde(rename = "IsPvpEnabled")]
    pub is_pvp_enabled: bool,
    #[serde(rename = "IsFallDamageEnabled")]
    pub is_fall_damage_enabled: bool,
    #[serde(rename = "IsGameTimePaused")]
    pub is_game_time_paused: bool,
    #[serde(rename = "GameTime")]
    pub game_time: String,
    #[serde(rename = "ClientEffects")]
    pub client_effects: ClientEffects,
    #[serde(rename = "RequiredPlugins")]
    pub required_plugins: HashMap<String, serde_json::Value>,
    #[serde(rename = "GameMode")]
    pub game_mode: String,
    #[serde(rename = "IsSpawningNPC")]
    pub is_spawning_npc: bool,
    #[serde(rename = "IsSpawnMarkersEnabled")]
    pub is_spawn_markers_enabled: bool,
    #[serde(rename = "IsAllNPCFrozen")]
    pub is_all_npc_frozen: bool,
    #[serde(rename = "GameplayConfig")]
    pub gameplay_config: String,
    #[serde(rename = "IsCompassUpdating")]
    pub is_compass_updating: bool,
    #[serde(rename = "IsSavingPlayers")]
    pub is_saving_players: bool,
    #[serde(rename = "IsSavingChunks")]
    pub is_saving_chunks: bool,
    #[serde(rename = "SaveNewChunks")]
    pub save_new_chunks: bool,
    #[serde(rename = "IsUnloadingChunks")]
    pub is_unloading_chunks: bool,
    #[serde(rename = "IsObjectiveMarkersEnabled")]
    pub is_objective_markers_enabled: bool,
    #[serde(rename = "DeleteOnUniverseStart")]
    pub delete_on_universe_start: bool,
    #[serde(rename = "DeleteOnRemove")]
    pub delete_on_remove: bool,
    #[serde(rename = "ResourceStorage")]
    pub resource_storage: ResourceStorage,
    #[serde(rename = "Plugin")]
    pub plugin: HashMap<String, serde_json::Value>,
    
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
