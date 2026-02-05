use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ModsConfig {
    #[serde(rename = "Mods")]
    pub mods: HashMap<String, ModSettings>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModSettings {
    #[serde(rename = "Enabled")]
    pub enabled: bool,
}
