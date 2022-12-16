// Every request needs:
//  - API key
//  - organization (if there are multiple)
//  - a reqwest client
// We could create a struct that has all of these things,
// and every request could be an implemented function

use reqwest::Client;
use serde::Deserialize;
use models::{ list_models, ModelObject };

pub mod models;

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

    pub async fn list_models(&self) -> Result<ListObject<ModelObject>, reqwest::Error> {
        list_models(self).await
    }
}
