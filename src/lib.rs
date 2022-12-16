// Every request needs:
//  - API key
//  - organization (if there are multiple)
//  - a reqwest client
// We could create a struct that has all of these things,
// and every request could be an implemented function

use reqwest::{
    Client,
    header::{ AUTHORIZATION, HeaderMap, HeaderValue },
};
use serde::Deserialize;
use models::{ list_models, ModelObject, retrieve_model };
use completions::{ create_completion, CreateCompletionRequestBody, TextCompletionObject };

pub mod models;
pub mod completions;

pub(crate) fn openai_headers(openai: &OpenAI) -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.append(
            AUTHORIZATION,
            HeaderValue::from_str(&format!("Bearer {}", openai.key))
                .expect("HeaderValue should be created from key string"),
        );

    if openai.organization.is_some() {
        headers.append(
                "OpenAI-Organization",
                HeaderValue::from_str(openai.organization.expect("organization should be some"))
                    .expect("HeaderValue should be created from organization string"),
            );
    }

    headers
}

#[derive(Deserialize, Debug)]
pub struct ListObject<T> {
    pub data: Vec<T>,
    pub object: String,
}

pub struct OpenAI<'a> {
    key: &'a str,
    organization: Option<&'a str>,
    client: Client,
}

impl OpenAI<'_> {
    pub fn new<'a>(key: &'a str, organization: Option<&'a str>) -> OpenAI<'a> {
        OpenAI {
            key,
            organization,
            client: Client::new(),
        }
    }

    // MODELS

    /// Lists the currently available models, and provides basic information about each one such as the owner and availability.
    ///
    /// # Examples
    ///
    /// ```
    /// use openai::OpenAI;
    /// use dotenv::dotenv;
    /// use std::env;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     dotenv().expect("should load .env file");
    ///
    ///     let key = env::var("OPENAI_KEY").expect("env var OPENAI_KEY should be defined in .env file");
    ///     let openai = OpenAI::new(&key, None);
    ///
    ///     dbg!(openai.list_models().await.expect("should list models"));
    /// }
    /// ```
    pub async fn list_models(&self) -> Result<ListObject<ModelObject>, reqwest::Error> {
        list_models(self).await
    }

    /// Retrieves a model instance, providing basic information about the model such as the owner and permissioning.
    ///
    /// # Arguments
    ///
    /// * `model` - The ID of the model to use for this request
    ///
    /// # Examples
    ///
    /// ```
    /// use openai::OpenAI;
    /// use dotenv::dotenv;
    /// use std::env;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     dotenv().expect("should load .env file");
    ///
    ///     let key = env::var("OPENAI_KEY").expect("env var OPENAI_KEY should be defined in .env file");
    ///     let openai = OpenAI::new(&key, None);
    ///
    ///     dbg!(openai.retrieve_model("text-davinci-003").await.expect("should retrieve text-davinci-003 model"));
    /// }
    /// ```
    pub async fn retrieve_model(&self, model: &str) -> Result<ModelObject, reqwest::Error> {
        retrieve_model(self, model).await
    }

    // COMPLETIONS

    /// Creates a completion for the provided prompt and parameters
    ///
    /// # Examples
    ///
    /// ```
    /// use openai::{ OpenAI, completions::CreateCompletionRequestBody };
    /// use dotenv::dotenv;
    /// use std::env;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     dotenv().expect("should load .env file");
    ///
    ///     let key = env::var("OPENAI_KEY").expect("env var OPENAI_KEY should be defined in .env file");
    ///     let openai = OpenAI::new(&key, None);
    ///
    ///     let response = openai.create_completion(CreateCompletionRequestBody {
    ///         model: "text-davinci-003",
    ///         prompt: Some("Say this is a test"),
    ///         max_tokens: Some(7),
    ///         temperature: Some(0.0),
    ///         ..CreateCompletionRequestBody::default()
    ///     }).await.expect("should be text completion");
    ///
    ///     assert_eq!(
    ///         response.choices.first().expect("there should be at least one choice").text,
    ///         "\n\nThis is indeed a test",
    ///     );
    /// }
    /// ```
    pub async fn create_completion(&self, body: CreateCompletionRequestBody<'_>) -> Result<TextCompletionObject, reqwest::Error> {
        create_completion(self, body).await
    }
}
