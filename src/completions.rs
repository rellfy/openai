//! Given a prompt, the model will return one or more predicted completions,
//! and can also return the probabilities of alternative tokens at each position.

use serde::{ Deserialize, Serialize };
use super::{ models::ModelID, Usage, BASE_URL, authorization };
use std::collections::HashMap;
use reqwest::Client;

#[derive(Deserialize)]
pub struct Completion {
    pub id: String,
    pub created: u32,
    pub model: ModelID,
    pub choices: Vec<CompletionChoice>,
    pub usage: Usage,
}

impl Completion {
    pub async fn new(body: &CreateCompletionRequestBody<'_>) -> Result<Self, reqwest::Error> {
        let client = Client::builder().build()?;

        authorization(client.post(format!("{BASE_URL}/completions")))
            .json(body)
            .send().await?.json().await
    }
}

#[derive(Deserialize)]
pub struct CompletionChoice {
    pub text: String,
    pub index: u16,
    pub logprobs: Option<u16>,
    pub finish_reason: String,
}

#[derive(Serialize, Default)]
pub struct CreateCompletionRequestBody<'a> {
    pub model: ModelID,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<u8>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u16>,
//    #[serde(skip_serializing_if = "Option::is_none")]
//    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_of: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<&'a str, i16>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<&'a str>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;

    #[tokio::test]
    async fn completion() {
        dotenv().ok();
        
        let completion = Completion::new(&CreateCompletionRequestBody {
            model: ModelID::TextDavinci003,
            prompt: Some("Say this is a test"),
            max_tokens: Some(7),
            temperature: Some(0),
            ..Default::default()
        }).await.unwrap();

        assert_eq!(completion.choices.first().unwrap().text, "\n\nThis is indeed a test")
    }
}
