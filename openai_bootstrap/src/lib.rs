use serde::Deserialize;

pub const BASE_URL: &str = "https://api.openai.com/v1/";

#[macro_export]
macro_rules! authorization {
    ($request:expr) => {{
        use dotenvy::dotenv;
        use reqwest::{header::AUTHORIZATION, RequestBuilder};
        use std::env;

        dotenv().ok();

        let token =
            env::var("OPENAI_KEY").expect("environment variable `OPENAI_KEY` should be defined");

        $request.header(AUTHORIZATION, format!("Bearer {token}"))
    }};
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
pub enum ApiResponse<T> {
    Ok(T),
    Err { error: OpenAiError },
}
