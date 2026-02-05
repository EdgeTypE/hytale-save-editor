use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct Permissions {
    pub users: HashMap<String, UserPermission>,
    pub groups: HashMap<String, Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserPermission {
    pub groups: Vec<String>,
}
