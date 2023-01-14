//! Get a vector representation of a given input that can be easily consumed by machine learning models and algorithms.
//!
//! Related guide: [Embeddings](https://beta.openai.com/docs/guides/embeddings)

use serde::{ Deserialize, Serialize };
use reqwest::{ Client, header::AUTHORIZATION };
use super::{ BASE_URL, get_token, models::ModelID };

#[derive(Serialize)]
struct NewEmbeddingRequestBody<'a> {
    model: ModelID,
    input: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    user: Option<&'a str>,
}

#[derive(Deserialize)]
struct NewEmbeddingRequestResponse {
    data: Vec<Embedding>,
}

#[derive(Deserialize, Debug)]
pub struct Embedding {
    #[serde(rename = "embedding")]
    pub vec: Vec<f32>,
}

impl Embedding {
    pub async fn new(model: ModelID, input: &str, user: Option<&str>) -> Result<Self, reqwest::Error> {
        let client = Client::builder().build()?;
        let token = get_token();

        let response: NewEmbeddingRequestResponse = client.post(format!("{BASE_URL}/embeddings"))
            .header(AUTHORIZATION, format!("Bearer {}", token))
            .json(&NewEmbeddingRequestBody { model, input, user })
            .send().await?.json().await?;

        let embedding = response.data.into_iter().next()
            .expect("there should be at least one item in vector");

        Ok(embedding)
    }
}
