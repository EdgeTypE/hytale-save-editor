use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WorldConfig {
    #[serde(rename = "DisplayName")]
    pub display_name: String,
    #[serde(rename = "Seed")]
    pub seed: i64,
    #[serde(rename = "GameMode")]
    pub game_mode: String,
    #[serde(rename = "IsPvpEnabled")]
    pub is_pvp_enabled: bool,
    #[serde(rename = "ClientEffects")]
    pub client_effects: ClientEffects,
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClientEffects {
    #[serde(rename = "SunHeightPercent")]
    pub sun_height_percent: f64,
    #[serde(rename = "SunAngleDegrees")]
    pub sun_angle_degrees: f64,
    #[serde(rename = "SunIntensity")]
    pub sun_intensity: f64,
    #[serde(flatten)]
    pub other: HashMap<String, serde_json::Value>,
}
