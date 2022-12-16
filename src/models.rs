use serde::Deserialize;
use reqwest::header::{ AUTHORIZATION, HeaderMap, HeaderValue };
use super::{ ListObject, OpenAI };

#[derive(Deserialize)]
pub struct ModelObject {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub owned_by: String,
    pub permission: Vec<ModelPermissionObject>,
    pub root: String,
    pub parent: Option<String>,
}

#[derive(Deserialize)]
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
    let mut headers = HeaderMap::new();

    headers.append(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", openai.key))
                .expect("HeaderValue should be created from key string"),
        );

    if openai.organization.is_some() {
        headers.append(
                "OpenAI-Organization",
                HeaderValue::from_str(openai.organization.expect("organization should be some"))
                    .expect("HeaderValue should be created from organization string"),
            );
    }

    let request = openai.client.get("https://api.openai.com/v1/models")
            .headers(headers);

    request.send().await?.json().await
}
