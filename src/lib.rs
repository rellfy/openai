use reqwest::RequestBuilder;
use serde::{de::DeserializeOwned, Deserialize};
use openai_bootstrap::authorization;

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

#[derive(Deserialize)]
struct ErrorResponse {
    error: OpenAiError,
}

#[derive(Deserialize, Debug)]
pub struct OpenAiError {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: String,
    pub param: Option<String>,
    pub code: Option<String>,
}

impl std::fmt::Display for OpenAiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for OpenAiError {}

#[derive(Deserialize)]
#[serde(untagged)]
enum ApiResponse<T> {
    Ok(T),
    Err(ErrorResponse),
}

type ModifiedApiResponse<T> = Result<Result<T, OpenAiError>, reqwest::Error>;

async fn handle_api<T>(request: RequestBuilder) -> ModifiedApiResponse<T>
where
    T: DeserializeOwned,
{
    let api_response: ApiResponse<T> = authorization!(request).send().await?.json().await?;

    match api_response {
        ApiResponse::Ok(t) => Ok(Ok(t)),
        ApiResponse::Err(error) => Ok(Err(error.error)),
    }
}
