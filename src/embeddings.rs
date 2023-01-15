//! Get a vector representation of a given input that can be easily consumed by machine learning models and algorithms.
//!
//! Related guide: [Embeddings](https://beta.openai.com/docs/guides/embeddings)

use serde::{ Deserialize, Serialize };
use reqwest::Client;
use super::{ BASE_URL, authorization, models::ModelID, Usage };

#[derive(Serialize)]
struct CreateEmbeddingsRequestBody<'a> {
    model: ModelID,
    input: Vec<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<&'a str>,
}

#[derive(Deserialize)]
pub struct Embeddings {
    pub data: Vec<Embedding>,
    pub model: ModelID,
    pub usage: Usage,
}

impl Embeddings {
    /// Creates an embedding vector representing the input text.
    ///
    /// # Arguments
    ///
    /// * `model` - ID of the model to use.
    ///   You can use the [List models](https://beta.openai.com/docs/api-reference/models/list)
    ///   API to see all of your available models, or see our [Model overview](https://beta.openai.com/docs/models/overview)
    ///   for descriptions of them.
    /// * `input` - Input text to get embeddings for, encoded as a string or array of tokens.
    ///   To get embeddings for multiple inputs in a single request, pass an array of strings or array of token arrays.
    ///   Each input must not exceed 8192 tokens in length.
    /// * `user` - A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    ///   [Learn more](https://beta.openai.com/docs/guides/safety-best-practices/end-user-ids).
    pub async fn new(model: ModelID, input: Vec<&str>, user: Option<&str>) -> Result<Self, reqwest::Error> {
        let client = Client::builder().build()?;

        let response: Embeddings = authorization(client.post(format!("{BASE_URL}/embeddings")))
            .json(&CreateEmbeddingsRequestBody { model, input, user })
            .send().await?.json().await?;

        Ok(response)
    }
}

#[derive(Deserialize)]
pub struct Embedding {
    #[serde(rename = "embedding")]
    pub vec: Vec<f32>,
}

impl Embedding {
    pub async fn new(model: ModelID, input: &str, user: Option<&str>) -> Result<Self, reqwest::Error> {
        let embeddings = Embeddings::new(model, vec![input], user);

        Ok(
            embeddings
                .await.expect("should create embeddings")
                .data.swap_remove(0)
        )
    }
}

// TODO: Find a more deterministic way of testing this
#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;

    #[tokio::test]
    async fn embeddings() {
        dotenv().ok();

        let embeddings = Embeddings::new(
            ModelID::TextEmbeddingAda002,
            vec!["The food was delicious and the waiter..."],
            None,
        ).await.expect("should create embeddings");

        assert_eq!(
            embeddings.data.first()
                .expect("should have one embedding").vec.first()
                .expect("should have at least one number"),
            &0.0023064255,
        )
    }

    #[tokio::test]
    async fn embedding() {
        dotenv().ok();

        let embedding = Embedding::new(
            ModelID::TextEmbeddingAda002,
            "The food was delicious and the waiter...",
            None,
        ).await.expect("should create embedding");

        assert_eq!(
            embedding.vec.first()
                .expect("should have at least one number"),
            &0.0023064255,
        )
    }
}