//! List and describe the various models available in the API.
//! You can refer to the [Models](https://beta.openai.com/docs/models)
//! documentation to understand what models are available and the differences between them.

use super::{openai_get, ApiResponseOrError, Credentials};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Model {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub owned_by: String,
}

#[derive(Deserialize, Clone)]
pub struct ModelPermission {
    pub id: String,
    pub created: u32,
    pub allow_create_engine: bool,
    pub allow_sampling: bool,
    pub allow_logprobs: bool,
    pub allow_search_indices: bool,
    pub allow_view: bool,
    pub allow_fine_tuning: bool,
    pub organization: String,
    pub group: Option<String>,
    pub is_blocking: bool,
}

impl Model {
    /// Retrieves a model instance,
    /// providing basic information about the model such as the owner and permissioning.
    #[deprecated(since = "1.0.0-alpha.16", note = "use `fetch` instead")]
    pub async fn from(id: &str) -> ApiResponseOrError<Self> {
        openai_get(&format!("models/{id}"), None).await
    }

    /// Retrieves a model instance,
    /// providing basic information about the model such as the owner and permissioning.
    pub async fn fetch(id: &str, credentials: Credentials) -> ApiResponseOrError<Self> {
        openai_get(&format!("models/{id}"), Some(credentials)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::DEFAULT_LEGACY_MODEL;
    use dotenvy::dotenv;

    #[tokio::test]
    async fn model() {
        dotenv().ok();
        let credentials = Credentials::from_env();
        let model = Model::fetch(DEFAULT_LEGACY_MODEL, credentials)
            .await
            .unwrap();
        assert_eq!(model.id, DEFAULT_LEGACY_MODEL);
    }
}
