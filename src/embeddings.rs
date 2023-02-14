//! Get a vector representation of a given input that can be easily consumed by machine learning models and algorithms.
//!
//! Related guide: [Embeddings](https://beta.openai.com/docs/guides/embeddings)

use serde::{ Deserialize, Serialize };
use reqwest::Client;
use super::{ models::ModelID, Usage };
use openai_utils::{ BASE_URL, authorization };

#[derive(Serialize)]
struct CreateEmbeddingsRequestBody<'a> {
    model: ModelID,
    input: Vec<&'a str>,
    #[serde(skip_serializing_if = "str::is_empty")]
    user: &'a str,
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
    pub async fn new(model: ModelID, input: Vec<&str>, user: &str) -> Result<Self, reqwest::Error> {
        let client = Client::builder().build()?;

        authorization!(client.post(format!("{BASE_URL}/embeddings")))
            .json(&CreateEmbeddingsRequestBody { model, input, user })
            .send().await?.json().await
    }

    pub fn distances(&self) -> Vec<f32> {
        let mut distances = Vec::new();
        let mut last_embedding: Option<&Embedding> = None;

        for embedding in &self.data {
            if let Some(other) = last_embedding {
                distances.push(embedding.distance(other));
            }

            last_embedding = Some(embedding);
        }

        distances
    }
}

#[derive(Deserialize)]
pub struct Embedding {
    #[serde(rename = "embedding")]
    pub vec: Vec<f32>,
}

impl Embedding {
    pub async fn new(model: ModelID, input: &str, user: &str) -> Result<Self, reqwest::Error> {
        let embeddings = Embeddings::new(model, vec![input], user);

        Ok(
            embeddings
                .await.expect("should create embeddings")
                .data.swap_remove(0)
        )
    }

    pub fn distance(&self, other: &Self) -> f32 {
        let dot_product: f32 = self.vec.iter().zip(other.vec.iter()).map(|(x, y)| x * y).sum();
        let product_of_lengths = (self.vec.len() * other.vec.len()) as f32;

        dot_product / product_of_lengths
    }
}

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
            "",
        ).await.unwrap();

        assert!(!embeddings.data.first().unwrap().vec.is_empty());
    }

    #[tokio::test]
    async fn embedding() {
        dotenv().ok();

        let embedding = Embedding::new(
            ModelID::TextEmbeddingAda002,
            "The food was delicious and the waiter...",
            "",
        ).await.unwrap();

        assert!(!embedding.vec.is_empty());
    }

    #[tokio::test]
    async fn distances() {
        dotenv().ok();

        let embeddings = Embeddings::new(
            ModelID::TextEmbeddingAda002,
            vec![
                "The food was delicious and the waiter...",
                "I loved the service! When they came to take my order...",
                "Disgusting. All over the floor, there was...",
                "Hated it there! Bad bad bad! Bad food, bad service, bad...",
            ],
            "",
        ).await.unwrap();

        let distances = embeddings.distances();

        dbg!(&distances);

        assert!(distances[0] < distances[1]);
    }

    #[test]
    fn right_angle() {
        let embeddings = Embeddings {
            data: vec![
                Embedding { vec: vec![1.0, 0.0, 0.0] },
                Embedding { vec: vec![0.0, 1.0, 0.0] },
            ],
            model: ModelID::TextEmbeddingAda002,
            usage: Usage { prompt_tokens: 0, completion_tokens: Some(0), total_tokens: 0 },
        };

        assert_eq!(embeddings.distances()[0], 0.0);
    }

    #[test]
    fn non_right_angle() {
        let embeddings = Embeddings {
            data: vec![
                Embedding { vec: vec![1.0, 1.0, 0.0] },
                Embedding { vec: vec![0.0, 1.0, 0.0] },
            ],
            model: ModelID::TextEmbeddingAda002,
            usage: Usage { prompt_tokens: 0, completion_tokens: Some(0), total_tokens: 0 },
        };

        assert_ne!(embeddings.distances()[0], 0.0);
    }
}