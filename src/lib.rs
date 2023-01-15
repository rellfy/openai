use std::env;
use serde::Deserialize;
use reqwest::{ RequestBuilder, header::AUTHORIZATION };

pub mod models;
pub mod embeddings;

pub(crate) const BASE_URL: &str = "https://api.openai.com/v1";

pub(crate) fn authorization(request: RequestBuilder) -> RequestBuilder {
    let token = env::var("OPENAI_KEY")
        .expect("environment variable `OPENAI_KEY` should be defined");

    request.header(AUTHORIZATION, format!("Bearer {token}"))
}

#[derive(Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub total_tokens: u64,
}
