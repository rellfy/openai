//! Given a prompt and an instruction, the model will return an edited version of the prompt.

use serde::Deserialize;
use super::Usage;
use reqwest::Client;
use openai_utils::{ BASE_URL, authorization };

#[derive(Deserialize)]
pub struct Edit {
    pub created: u32,
    #[serde(flatten)]
    pub choices: Vec<String>,
    pub usage: Usage,
}

impl Edit {
    pub async fn new() -> Result<Self, reqwest::Error> {
//        let client = Client::builder().build()?;
//
//        authorization!(client.post(format!("{BASE_URL}/completions")))
//            .json(body)
//            .send().await?.json().await

        todo!()
    }
}
