use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

// --- Common ---

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct HytaleBinary {
    #[serde(rename = "$binary")]
    pub binary: String,
    #[serde(rename = "$type")]
    pub type_code: String,
}

// --- Permissions ---

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PermissionsConfig {
    pub users: HashMap<String, UserPermissions>,
    pub groups: HashMap<String, Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserPermissions {
    pub groups: Vec<String>,
}

// --- Mods Config ---

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModsConfig {
    #[serde(rename = "Mods")]
    pub mods: HashMap<String, ModEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModEntry {
    #[serde(rename = "Enabled")]
    pub enabled: bool,
}

// --- Whitelist ---

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Whitelist {
    pub enabled: bool,
    pub list: Vec<String>,
}

// --- Bans ---

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BanEntry {
    #[serde(rename = "type")]
    pub ban_type: String,
    pub target: String,
    pub by: String,
    pub timestamp: i64,
    pub reason: String,
}

pub type BanList = Vec<BanEntry>;

// --- Memories ---

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoriesConfig {
    #[serde(rename = "Memories")]
    pub memories: Vec<Memory>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Memory {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "NPCRole")]
    pub npc_role: String,
    #[serde(rename = "TranslationKey")]
    pub translation_key: String,
    #[serde(rename = "IsMemoriesNameOverridden")]
    pub is_overridden: bool,
    #[serde(rename = "CapturedTimestamp")]
    pub captured_timestamp: i64,
    #[serde(rename = "FoundLocationNameKey")]
    pub found_location: String,
}

// --- World Config ---

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct WorldConfig {
    pub version: Option<i32>,
    #[serde(rename = "UUID")]
    pub uuid: Option<HytaleBinary>,
    pub display_name: Option<String>,
    pub seed: Option<i64>,
    pub game_mode: Option<String>,
    pub is_pvp_enabled: Option<bool>,
    pub is_fall_damage_enabled: Option<bool>,
    pub game_time: Option<String>,
    // Capture other fields loosely to avoid breaking on save
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

// --- Player Data ---

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerFile {
    #[serde(rename = "Components")]
    pub components: PlayerComponents,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PlayerComponents {
    pub nameplate: Option<serde_json::Value>,
    pub display_name: Option<serde_json::Value>,
    pub transform: Option<TransformComponent>,
    pub player_memories: Option<PlayerMemoriesComponent>,
    pub velocity: Option<serde_json::Value>,
    pub player: Option<PlayerComponent>,
    pub entity_stats: Option<EntityStatsComponent>,
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct TransformComponent {
    pub position: Position,
    pub rotation: Option<Rotation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Position {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Rotation {
    pub pitch: f64,
    pub yaw: f64,
    pub roll: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PlayerMemoriesComponent {
    pub memories: Vec<Memory>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct PlayerComponent {
    pub version: Option<i32>,
    #[serde(rename = "UUID")]
    pub uuid: Option<HytaleBinary>,
    pub inventory: Option<Inventory>,
    pub game_mode: Option<String>,
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Inventory {
    pub version: Option<i32>,
    pub storage: Option<Storage>,
    pub armor: Option<Storage>,
    pub hot_bar: Option<Storage>,
    pub utility: Option<Storage>,
    pub backpack: Option<Storage>,
    pub tool: Option<Storage>,
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Storage {
    pub id: String,
    pub capacity: i32,
    pub items: HashMap<String, Item>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Item {
    pub id: String,
    pub quantity: i32,
    pub durability: Option<f64>,
    pub max_durability: Option<f64>,
    #[serde(flatten)]
    pub other: HashMap<String, Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct EntityStatsComponent {
    pub stats: HashMap<String, StatEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct StatEntry {
    pub id: String,
    pub value: f64,
    pub modifiers: Option<HashMap<String, Value>>,
}

#[derive(Default)]
pub struct SaveData {
    pub root_path: std::path::PathBuf,
    pub permissions: Option<PermissionsConfig>,
    pub mods: Option<ModsConfig>,
    pub whitelist: Option<Whitelist>,
    pub bans: Option<BanList>,
    pub memories: Option<MemoriesConfig>,
    pub worlds: HashMap<String, WorldConfig>,
    pub players: HashMap<String, PlayerFile>,
}
