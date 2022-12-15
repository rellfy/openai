// Every request needs:
//  - API key
//  - organization (if there are multiple)
//  - a reqwest client
// We could create a struct that has all of these things,
// and every request could be an implemented function

use reqwest::{ Client, header::{ AUTHORIZATION, HeaderMap, HeaderValue } };
use serde::Deserialize;
use models::ModelObject;

mod models;

#[derive(Deserialize)]
pub struct ListObject<T> {
    pub data: Vec<T>,
    pub object: String,
}

pub struct OpenAI<'a> {
    key: &'a str,
    organization: Option<&'a str>,
    client: Client,
}

impl OpenAI<'_> {
    pub fn new<'a>(key: &'a str, organization: Option<&'a str>) -> OpenAI<'a> {
        OpenAI {
            key,
            organization,
            client: Client::new(),
        }
    }

    pub async fn list_models(&self) -> Result<ListObject<ModelObject>, reqwest::Error> {
        let mut headers = HeaderMap::new();

        headers.append(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", self.key))
                .expect("HeaderValue should be created from key string"),
        );

        if self.organization.is_some() {
            headers.append(
                "OpenAI-Organization",
                HeaderValue::from_str(self.organization.expect("organization should be some"))
                    .expect("HeaderValue should be created from organization string"),
            );
        }

        let request = self.client.get("https://api.openai.com/v1/models")
            .headers(headers);

        request.send().await?.json().await
    }
}
