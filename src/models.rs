use serde::Deserialize;
use reqwest::header::{ AUTHORIZATION, HeaderMap, HeaderValue };
use super::{ ListObject, OpenAI };

#[derive(Deserialize)]
pub struct ModelObject {
    pub id: String,
    pub object: String,
    pub owned_by: String,
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
