use std::sync::Mutex;

use reqwest::multipart::Form;
use reqwest::{header::AUTHORIZATION, Client, Method, RequestBuilder, Response};
use reqwest_eventsource::{CannotCloneRequestError, EventSource, RequestBuilderExt};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod chat;
pub mod completions;
pub mod edits;
pub mod embeddings;
pub mod files;
pub mod models;
pub mod moderations;

static API_KEY: Mutex<String> = Mutex::new(String::new());
static BASE_URL: Mutex<String> = Mutex::new(String::new());

#[derive(Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct OpenAiError {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: String,
    pub param: Option<String>,
    pub code: Option<String>,
}

impl OpenAiError {
    fn new(message: String, error_type: String) -> OpenAiError {
        OpenAiError {
            message,
            error_type,
            param: None,
            code: None,
        }
    }
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
    Err { error: OpenAiError },
    Ok(T),
}

#[derive(Deserialize, Clone, Copy, Debug, Eq, PartialEq)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

pub type ApiResponseOrError<T> = Result<T, OpenAiError>;

impl From<reqwest::Error> for OpenAiError {
    fn from(value: reqwest::Error) -> Self {
        OpenAiError::new(value.to_string(), "reqwest".to_string())
    }
}

impl From<std::io::Error> for OpenAiError {
    fn from(value: std::io::Error) -> Self {
        OpenAiError::new(value.to_string(), "io".to_string())
    }
}

async fn openai_request_json<F, T>(method: Method, route: &str, builder: F) -> ApiResponseOrError<T>
where
    F: FnOnce(RequestBuilder) -> RequestBuilder,
    T: DeserializeOwned,
{
    let api_response = openai_request(method, route, builder).await?.json().await?;
    match api_response {
        ApiResponse::Ok(t) => Ok(t),
        ApiResponse::Err { error } => Err(error),
    }
}

async fn openai_request<F>(method: Method, route: &str, builder: F) -> ApiResponseOrError<Response>
where
    F: FnOnce(RequestBuilder) -> RequestBuilder,
{
    let client = Client::new();
    let mut request = client.request(method, get_base_url().lock().unwrap().to_owned() + route);

    request = builder(request);

    let response = request
        .header(AUTHORIZATION, format!("Bearer {}", API_KEY.lock().unwrap()))
        .send()
        .await?;
    Ok(response)
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
    let mut request = client.request(method, get_base_url().lock().unwrap().to_owned() + route);

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
    openai_request_json(Method::GET, route, |request| request).await
}

async fn openai_delete<T>(route: &str) -> ApiResponseOrError<T>
where
    T: DeserializeOwned,
{
    openai_request_json(Method::DELETE, route, |request| request).await
}

async fn openai_post<J, T>(route: &str, json: &J) -> ApiResponseOrError<T>
where
    J: Serialize + ?Sized,
    T: DeserializeOwned,
{
    openai_request_json(Method::POST, route, |request| request.json(json)).await
}

async fn openai_post_multipart<T>(route: &str, form: Form) -> ApiResponseOrError<T>
where
    T: DeserializeOwned,
{
    openai_request_json(Method::POST, route, |request| request.multipart(form)).await
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

/// Sets the base url for all OpenAI API functions.
///
/// ## Examples
///
/// Use environment variable `OPENAI_BASE_URL` defined from `.env` file:
///
/// ```rust
/// use openai::set_base_url;
/// use dotenvy::dotenv;
/// use std::env;
///
/// dotenv().ok();
/// set_base_url(env::var("OPENAI_BASE_URL").unwrap_or_default());
/// ```
pub fn set_base_url(value: String) {
    let base_url_mutex = get_base_url();
    if value.is_empty() {
        return;
    }
    let mut base_url = base_url_mutex.lock().unwrap();
    *base_url = value;
    if !base_url.ends_with('/') {
        *base_url += "/";
    }
}

/// Returns the base url for all OpenAI API functions.
/// Defaults to `https://api.openai.com/v1/`.
fn get_base_url() -> &'static Mutex<String> {
    let mut base_url = BASE_URL.lock().unwrap();
    if base_url.is_empty() {
        *base_url = String::from("https://api.openai.com/v1/");
    }
    &BASE_URL
}

#[cfg(test)]
pub mod tests {
    use super::*;

    pub const DEFAULT_LEGACY_MODEL: &str = "gpt-3.5-turbo-instruct";

    #[test]
    fn test_get_base_url_default() {
        assert_eq!(
            get_base_url().lock().unwrap().to_owned(),
            String::from("https://api.openai.com/v1/")
        );

        // empty env var
        set_base_url(String::from(""));
        assert_eq!(
            get_base_url().lock().unwrap().to_owned(),
            String::from("https://api.openai.com/v1/")
        );

        // appends slash
        set_base_url(String::from("https://api.openai.com/v1"));
        assert_eq!(
            get_base_url().lock().unwrap().to_owned(),
            String::from("https://api.openai.com/v1/")
        );
    }

    #[test]
    fn test_get_base_url_set() {
        set_base_url(String::from("https://api.openai.com/v2/"));
        assert_eq!(
            get_base_url().lock().unwrap().to_owned(),
            String::from("https://api.openai.com/v2/")
        );
        // need this here to reset the base url for other tests
        set_base_url(String::from("https://api.openai.com/v1"));
    }
}
