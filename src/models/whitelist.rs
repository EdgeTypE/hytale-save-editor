use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct Whitelist {
    pub enabled: bool,
    pub list: Vec<String>,
}
