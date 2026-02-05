use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct BanEntry {
    #[serde(rename = "type")]
    pub ban_type: String,
    pub target: String,
    pub by: String,
    pub timestamp: i64,
    pub reason: String,
}

pub type Bans = Vec<BanEntry>;
