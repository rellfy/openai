use openai_bootstrap::{authorization, BASE_URL};
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
