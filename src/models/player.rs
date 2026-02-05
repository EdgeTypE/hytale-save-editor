use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerData {
    #[serde(rename = "Components")]
    pub components: PlayerComponents,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerComponents {
    #[serde(rename = "Nameplate", default)]
    pub nameplate: Option<Nameplate>,
    #[serde(rename = "DisplayName", default)]
    pub display_name: Option<DisplayNameComponent>,
    #[serde(rename = "Transform", default)]
    pub transform: Option<Transform>,
    #[serde(rename = "EntityStats", default)]
    pub entity_stats: Option<EntityStats>,
    #[serde(rename = "Player", default)]
    pub player: Option<PlayerComponent>,
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Nameplate {
    #[serde(rename = "Text")]
    pub text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DisplayNameComponent {
    #[serde(rename = "DisplayName")]
    pub display_name: DisplayNameData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DisplayNameData {
    #[serde(rename = "RawText")]
    pub raw_text: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transform {
    #[serde(rename = "Position")]
    pub position: Position,
    #[serde(rename = "Rotation")]
    pub rotation: Rotation,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Position {
    #[serde(rename = "X")]
    pub x: f64,
    #[serde(rename = "Y")]
    pub y: f64,
    #[serde(rename = "Z")]
    pub z: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Rotation {
    #[serde(rename = "Pitch")]
    pub pitch: f64,
    #[serde(rename = "Yaw")]
    pub yaw: f64,
    #[serde(rename = "Roll")]
    pub roll: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntityStats {
    #[serde(rename = "Stats")]
    pub stats: HashMap<String, StatEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StatEntry {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Value")]
    pub value: f64,
    #[serde(rename = "Modifiers", default)]
    pub modifiers: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlayerComponent {
    #[serde(rename = "GameMode")]
    pub game_mode: String,
    #[serde(rename = "Inventory")]
    pub inventory: Inventory,
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Inventory {
    #[serde(rename = "Storage")]
    pub storage: Storage,
    #[serde(rename = "HotBar")]
    pub hotbar: Storage,
    // Add other inventory sections like Armor, Utility, Backpack if needed
    #[serde(rename = "Armor")]
    pub armor: Option<Storage>,
    #[serde(rename = "Utility")]
    pub utility: Option<Storage>,
    #[serde(rename = "Backpack")]
    pub backpack: Option<Storage>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Storage {
    #[serde(rename = "Id")]
    pub id: String, // Added Id since it was likely missing due to flattening or simple oversight. Actually user sample shows Id "Simple".
    #[serde(rename = "Capacity")]
    pub capacity: i32,
    #[serde(rename = "Items", default)]
    pub items: HashMap<String, Item>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "Quantity")]
    pub quantity: i32,
    #[serde(rename = "Durability", default)]
    pub durability: Option<f64>,
}
