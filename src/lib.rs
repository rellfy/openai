use std::env;

pub mod models;
pub mod embeddings;

pub(crate) const BASE_URL: &str = "https://api.openai.com/v1";

pub(crate) fn get_token() -> String {
    env::var("OPENAI_KEY")
        .expect("environment variable `OPENAI_TOKEN` should be defined")
}
