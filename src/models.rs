use serde::Deserialize;
use super::{ ListObject, OpenAI, openai_headers };

#[derive(Deserialize, Debug)]
pub struct ModelObject {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub owned_by: String,
    pub permission: Vec<ModelPermissionObject>,
    pub root: String,
    pub parent: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct ModelPermissionObject {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub allow_create_engine: bool,
    pub allow_sampling: bool,
    pub allow_logprobs: bool,
    pub allow_search_indices: bool,
    pub allow_view: bool,
    pub allow_fine_tuning: bool,
    pub organization: String,
    pub group: Option<String>,
    pub is_blocking: bool,
}

pub(super) async fn list_models(openai: &OpenAI<'_>) -> Result<ListObject<ModelObject>, reqwest::Error> {
    let request = openai.client.get("https://api.openai.com/v1/models")
            .headers(openai_headers(openai));

    request.send().await?.json().await
}

pub(super) async fn retrieve_model(openai: &OpenAI<'_>, model: &str) -> Result<ModelObject, reqwest::Error> {
    let request = openai.client.get(format!("https://api.openai.com/v1/models/{model}"))
            .headers(openai_headers(openai));

    request.send().await?.json().await
}
