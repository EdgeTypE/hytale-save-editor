use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Whitelist {
    pub enabled: bool,
    pub list: Vec<String>,
}
