use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModAuthor {
    #[serde(alias = "Name")]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModManifest {
    #[serde(alias = "Group")]
    pub group: String,
    #[serde(alias = "Name")]
    pub name: String,
    #[serde(alias = "Version")]
    pub version: String,
    #[serde(alias = "Description")]
    pub description: String,
    #[serde(alias = "Authors")]
    pub authors: Option<Vec<ModAuthor>>,
    #[serde(alias = "Website")]
    pub website: Option<String>,
}

impl ModManifest {
    pub fn id(&self) -> String {
        format!("{}:{}", self.group, self.name)
    }
}
