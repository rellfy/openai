//! Get a vector representation of a given input that can be easily consumed by machine learning models and algorithms.
//!
//! Related guide: [Embeddings](https://beta.openai.com/docs/guides/embeddings)

use super::{ OpenAI, openai_headers, ListObject };
use serde::{ Deserialize, Serialize };

#[derive(Deserialize, Debug)]
pub struct EmbeddingObject {
    pub object: String,
    pub index: u32,
    pub embedding: Vec<f32>,
}

#[derive(Serialize, Debug)]
pub struct CreateEmbeddingsRequestBody<'a> {
    /// ID of the model to use.
    /// You can use the [List models](https://beta.openai.com/docs/api-reference/models/list) API to see all of your available models,
    /// or see our [Model overview](https://beta.openai.com/docs/models/overview) for descriptions of them.
    pub model: &'a str,
    /// Input text to get embeddings for, encoded as a string or array of tokens.
    /// To get embeddings for multiple inputs in a single request, pass an array of strings or array of token arrays.
    /// Each input must not exceed 8192 tokens in length.
    pub input: &'a str,
    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    /// [Learn more](https://beta.openai.com/docs/guides/safety-best-practices/end-user-ids).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<&'a str>,
}

pub(super) async fn create_embeddings(openai: &OpenAI<'_>, body: CreateEmbeddingsRequestBody<'_>) -> Result<ListObject<EmbeddingObject>, reqwest::Error> {
    let request = openai.client.post("https://api.openai.com/v1/embeddings")
        .headers(openai_headers(openai))
        .json(&body);

    request.send().await?.json().await
}
