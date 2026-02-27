use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PromptMetadata {
    pub id: String,
    pub name: String,
    pub folder: String,
    pub description: String,
    pub filename: String,
    pub use_count: u64,
    pub last_used: Option<String>,
    pub created: String,
    pub updated: String,
    pub icon: Option<String>,
    pub color: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Prompt {
    #[serde(flatten)]
    pub meta: PromptMetadata,
    pub content: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FolderMeta {
    pub name: String,
    pub icon: Option<String>,
    pub color: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PromptIndex {
    pub prompts: Vec<PromptMetadata>,
    pub folders: Vec<String>,
    pub folder_meta: Option<HashMap<String, FolderMeta>>,
    pub seeded: bool,
}

impl Default for PromptIndex {
    fn default() -> Self {
        Self {
            prompts: Vec::new(),
            folders: Vec::new(),
            folder_meta: None,
            seeded: false,
        }
    }
}
