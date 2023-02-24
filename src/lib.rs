use openai_bootstrap::{authorization, ApiResponse, OpenAiError, BASE_URL};
use reqwest::{Client, Method};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod completions;
pub mod edits;
pub mod embeddings;
pub mod models;

#[derive(Deserialize)]
pub struct Usage {
    pub prompt_tokens: u16,
    pub completion_tokens: Option<u16>,
    pub total_tokens: u32,
}

type ApiResponseOrError<T> = Result<Result<T, OpenAiError>, reqwest::Error>;

/// `body` must be set. If there should be no body, set to `""`.
async fn openai_request<J, T>(method: Method, route: &str, body: &J) -> ApiResponseOrError<T>
where
    J: Serialize + ?Sized,
    T: DeserializeOwned,
{
    let client = Client::new();
    let request = client
        .request(method, BASE_URL.to_owned() + route)
        .json(body);
    let api_response: ApiResponse<T> = authorization!(request).send().await?.json().await?;

    match api_response {
        ApiResponse::Ok(t) => Ok(Ok(t)),
        ApiResponse::Err(error) => Ok(Err(error.error)),
    }
}
