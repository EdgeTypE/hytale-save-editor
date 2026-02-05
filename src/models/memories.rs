use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Memories {
    #[serde(rename = "Memories")]
    pub memories: Vec<MemoryEntry>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemoryEntry {
    #[serde(rename = "Id")]
    pub id: String,
    #[serde(rename = "NPCRole")]
    pub npc_role: String,
    #[serde(rename = "TranslationKey")]
    pub translation_key: String,
    #[serde(rename = "IsMemoriesNameOverridden")]
    pub is_memories_name_overridden: bool,
    #[serde(rename = "CapturedTimestamp")]
    pub captured_timestamp: i64,
    #[serde(rename = "FoundLocationNameKey")]
    pub found_location_name_key: String,
}
