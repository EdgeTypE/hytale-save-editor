use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct ClientMetadata {
    #[serde(rename = "CreatedWithPatchline")]
    pub created_with_patchline: String,
}
