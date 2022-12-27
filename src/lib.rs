use reqwest::{
    Client,
    header::{ AUTHORIZATION, HeaderMap, HeaderValue },
};
use serde::Deserialize;
use models::{ list_models, ModelObject, retrieve_model };
use completions::{ create_completion, CreateCompletionRequestBody, TextCompletionObject };
use embeddings::{ create_embeddings, CreateEmbeddingsRequestBody, EmbeddingObject };

pub mod models;
pub mod completions;
pub mod embeddings;

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
    pub model: Option<String>,
    pub usage: Option<Usage>,
}

#[derive(Deserialize, Debug)]
pub struct Usage {
    pub prompt_tokens: u16,
    pub completion_tokens: Option<u16>,
    pub total_tokens: u32,
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

    // EMBEDDINGS

    /// Creates an embedding vector representing the input text.
    ///
    /// # Examples
    ///
    /// ```
    /// use openai::{ OpenAI, embeddings::CreateEmbeddingsRequestBody };
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
    ///     let response = openai.create_embeddings(CreateEmbeddingsRequestBody {
    ///         model: "text-embedding-ada-002",
    ///         input: "The food was delicious and the waiter...",
    ///         user: None,
    ///     }).await.expect("should be list of embedding(s)");
    ///
    ///     assert_eq!(
    ///         response.data.first()
    ///             .expect("there should be at least one embedding").embedding.first()
    ///             .expect("there should be at least one float"),
    ///         &0.0023064255,
    ///     );
    /// }
    /// ```
    pub async fn create_embeddings(&self, body: CreateEmbeddingsRequestBody<'_>) -> Result<ListObject<EmbeddingObject>, reqwest::Error> {
        create_embeddings(self, body).await
    }
}
