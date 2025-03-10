use reqwest::multipart::Form;
use reqwest::{header::AUTHORIZATION, Client, Method, RequestBuilder, Response};
use reqwest_eventsource::{CannotCloneRequestError, EventSource, RequestBuilderExt};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::env;
use std::env::VarError;
use std::sync::{LazyLock, RwLock};

pub mod chat;
pub mod completions;
pub mod edits;
pub mod embeddings;
pub mod files;
pub mod models;
pub mod moderations;

pub static DEFAULT_BASE_URL: LazyLock<String> =
    LazyLock::new(|| String::from("https://api.openai.com/v1/"));
static DEFAULT_CREDENTIALS: LazyLock<RwLock<Credentials>> =
    LazyLock::new(|| RwLock::new(Credentials::from_env()));

pub trait Tokens {
    fn tokens(&self) -> u64;
}

impl Tokens for String {
    fn tokens(&self) -> u64 {
        self.len() as u64 / 4
    }
}

impl Tokens for str {
    fn tokens(&self) -> u64 {
        self.len() as u64 / 4
    }
}

/// Holds the API key and base URL for an OpenAI-compatible API.
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Credentials {
    api_key: String,
    base_url: String,
}

impl Credentials {
    /// Creates credentials with the given API key and base URL.
    ///
    /// If the base URL is empty, it will use the default.
    pub fn new(api_key: impl Into<String>, base_url: impl Into<String>) -> Self {
        let base_url = base_url.into();
        let base_url = if base_url.is_empty() {
            DEFAULT_BASE_URL.clone()
        } else {
            parse_base_url(base_url)
        };
        Self {
            api_key: api_key.into(),
            base_url,
        }
    }

    /// Fetches the credentials from the ENV variables
    /// OPENAI_KEY and OPENAI_BASE_URL.
    /// # Panics
    /// This function will panic if the key variable is missing from the env.
    /// If only the base URL variable is missing, it will use the default.
    pub fn from_env() -> Credentials {
        let api_key = env::var("OPENAI_KEY").unwrap();
        let base_url_unparsed = env::var("OPENAI_BASE_URL").unwrap_or_else(|e| match e {
            VarError::NotPresent => DEFAULT_BASE_URL.clone(),
            VarError::NotUnicode(v) => panic!("OPENAI_BASE_URL is not unicode: {v:#?}"),
        });
        let base_url = parse_base_url(base_url_unparsed);
        Credentials { api_key, base_url }
    }

    pub fn api_key(&self) -> &str {
        &self.api_key
    }

    pub fn base_url(&self) -> &str {
        &self.base_url
    }
}

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
        f.write_str(&self.message)
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

async fn openai_request_json<F, T>(
    method: Method,
    route: &str,
    builder: F,
    credentials_opt: Option<Credentials>,
) -> ApiResponseOrError<T>
where
    F: FnOnce(RequestBuilder) -> RequestBuilder,
    T: DeserializeOwned,
{
    let api_response = openai_request(method, route, builder, credentials_opt)
        .await?
        .json()
        .await?;
    match api_response {
        ApiResponse::Ok(t) => Ok(t),
        ApiResponse::Err { error } => Err(error),
    }
}

async fn openai_request<F>(
    method: Method,
    route: &str,
    builder: F,
    credentials_opt: Option<Credentials>,
) -> ApiResponseOrError<Response>
where
    F: FnOnce(RequestBuilder) -> RequestBuilder,
{
    let client = Client::new();
    let credentials =
        credentials_opt.unwrap_or_else(|| DEFAULT_CREDENTIALS.read().unwrap().clone());
    let mut request = client.request(method, format!("{}{route}", credentials.base_url));
    request = builder(request);
    let response = request
        .header(AUTHORIZATION, format!("Bearer {}", credentials.api_key))
        .send()
        .await?;
    Ok(response)
}

async fn openai_request_stream<F>(
    method: Method,
    route: &str,
    builder: F,
    credentials_opt: Option<Credentials>,
) -> Result<EventSource, CannotCloneRequestError>
where
    F: FnOnce(RequestBuilder) -> RequestBuilder,
{
    let client = Client::new();
    let credentials =
        credentials_opt.unwrap_or_else(|| DEFAULT_CREDENTIALS.read().unwrap().clone());
    let mut request = client.request(method, format!("{}{route}", credentials.base_url));
    request = builder(request);
    let stream = request
        .header(AUTHORIZATION, format!("Bearer {}", credentials.api_key))
        .eventsource()?;
    Ok(stream)
}

async fn openai_get<T>(route: &str, credentials_opt: Option<Credentials>) -> ApiResponseOrError<T>
where
    T: DeserializeOwned,
{
    openai_request_json(Method::GET, route, |request| request, credentials_opt).await
}

async fn openai_delete<T>(
    route: &str,
    credentials_opt: Option<Credentials>,
) -> ApiResponseOrError<T>
where
    T: DeserializeOwned,
{
    openai_request_json(Method::DELETE, route, |request| request, credentials_opt).await
}

async fn openai_post<J, T>(
    route: &str,
    json: &J,
    credentials_opt: Option<Credentials>,
) -> ApiResponseOrError<T>
where
    J: Serialize + ?Sized,
    T: DeserializeOwned,
{
    openai_request_json(
        Method::POST,
        route,
        |request| request.json(json),
        credentials_opt,
    )
    .await
}

async fn openai_post_multipart<T>(
    route: &str,
    form: Form,
    credentials_opt: Option<Credentials>,
) -> ApiResponseOrError<T>
where
    T: DeserializeOwned,
{
    openai_request_json(
        Method::POST,
        route,
        |request| request.multipart(form),
        credentials_opt,
    )
    .await
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
#[deprecated(
    since = "1.0.0-alpha.16",
    note = "use the `Credentials` struct instead"
)]
pub fn set_key(value: String) {
    let mut credentials = DEFAULT_CREDENTIALS.write().unwrap();
    credentials.api_key = value;
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
#[deprecated(
    since = "1.0.0-alpha.16",
    note = "use the `Credentials` struct instead"
)]
pub fn set_base_url(mut value: String) {
    if value.is_empty() {
        return;
    }
    value = parse_base_url(value);
    let mut credentials = DEFAULT_CREDENTIALS.write().unwrap();
    credentials.base_url = value;
}

fn parse_base_url(mut value: String) -> String {
    if !value.ends_with('/') {
        value += "/";
    }
    value
}

#[cfg(test)]
pub mod tests {
    pub const DEFAULT_LEGACY_MODEL: &str = "gpt-3.5-turbo-instruct";
}
