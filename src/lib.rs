use reqwest::{header::AUTHORIZATION, Client, Method, RequestBuilder};
use reqwest_eventsource::{CannotCloneRequestError, EventSource, RequestBuilderExt};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::sync::Mutex;

pub mod chat;
pub mod completions;
pub mod edits;
pub mod embeddings;
pub mod models;
pub mod moderations;

const BASE_URL: &str = "https://api.openai.com/v1/";

static API_KEY: Mutex<String> = Mutex::new(String::new());

#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Ok(T),
    Err { error: OpenAiError },
}

#[derive(Deserialize, Clone, Copy, Debug)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

type ApiResponseOrError<T> = Result<Result<T, OpenAiError>, reqwest::Error>;

async fn openai_request<F, T>(method: Method, route: &str, builder: F) -> ApiResponseOrError<T>
where
    F: FnOnce(RequestBuilder) -> RequestBuilder,
    T: DeserializeOwned,
{
    let client = Client::new();
    let mut request = client.request(method, BASE_URL.to_owned() + route);

    request = builder(request);

    let api_response: ApiResponse<T> = request
        .header(AUTHORIZATION, format!("Bearer {}", API_KEY.lock().unwrap()))
        .send()
        .await?
        .json()
        .await?;

    match api_response {
        ApiResponse::Ok(t) => Ok(Ok(t)),
        ApiResponse::Err { error } => Ok(Err(error)),
    }
}

async fn openai_request_stream<F>(
    method: Method,
    route: &str,
    builder: F,
) -> Result<EventSource, CannotCloneRequestError>
where
    F: FnOnce(RequestBuilder) -> RequestBuilder,
{
    let client = Client::new();
    let mut request = client.request(method, BASE_URL.to_owned() + route);

    request = builder(request);

    let stream = request
        .header(AUTHORIZATION, format!("Bearer {}", API_KEY.lock().unwrap()))
        .eventsource()?;

    Ok(stream)
}

async fn openai_get<T>(route: &str) -> ApiResponseOrError<T>
where
    T: DeserializeOwned,
{
    openai_request(Method::GET, route, |request| request).await
}

async fn openai_post<J, T>(route: &str, json: &J) -> ApiResponseOrError<T>
where
    J: Serialize + ?Sized,
    T: DeserializeOwned,
{
    openai_request(Method::POST, route, |request| request.json(json)).await
}

/// Sets the key for all OpenAI API functions.
///
/// ## Examples
///
/// Use environment variable `OPENAI_KEY` defined from `.env` file:
///
/// ```rust
/// use openai::set_key;
/// use dotenvy::dotenv;
/// use std::env;
///
/// dotenv().ok();
/// set_key(env::var("OPENAI_KEY").unwrap());
/// ```
pub fn set_key(value: String) {
    *API_KEY.lock().unwrap() = value;
}
