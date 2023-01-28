use serde::Deserialize;

pub mod completions;
pub mod edits;
pub mod embeddings;
pub mod models;

#[derive(Deserialize)]
pub struct Usage {
    pub prompt_tokens: u16,
    pub completion_tokens: Option<u16>,
    pub total_tokens: u32,
}
