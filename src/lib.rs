// Every request needs:
//  - API key
//  - organization (if there are multiple)
//  - a reqwest client
// We could create a struct that has all of these things,
// and every request could be an implemented function

use reqwest::{
    Client,
    header::{ AUTHORIZATION, HeaderMap, HeaderValue },
};
use serde::Deserialize;
use models::{ list_models, ModelObject };

pub mod models;

pub(crate) fn openai_headers(openai: &OpenAI) -> HeaderMap {
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

    headers
}

#[derive(Deserialize, Debug)]
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

    /// Lists the currently available models, and provides basic information about each one such as the owner and availability.
    pub async fn list_models(&self) -> Result<ListObject<ModelObject>, reqwest::Error> {
        list_models(self).await
    }
}
