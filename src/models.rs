//! List and describe the various models available in the API.
//! You can refer to the [Models](https://beta.openai.com/docs/models)
//! documentation to understand what models are available and the differences between them.

use super::{openai_get, ApiResponseOrError};
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Model {
    pub id: String,
    pub created: u32,
    pub owned_by: String,
    pub permission: Vec<ModelPermission>,
    pub root: String,
    pub parent: Option<String>,
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
    //! Retrieves a model instance,
    //! providing basic information about the model such as the owner and permissioning.
    pub async fn from(id: &str) -> ApiResponseOrError<Self> {
        openai_get(&format!("models/{id}")).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::set_key;
    use dotenvy::dotenv;
    use std::env;

    #[tokio::test]
    async fn model() {
        dotenv().ok();
        set_key(env::var("OPENAI_KEY").unwrap());

        let model = Model::from("text-davinci-003").await.unwrap().unwrap();

        assert_eq!(model.id, "text-davinci-003");
    }
}
