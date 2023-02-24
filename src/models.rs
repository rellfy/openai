//! List and describe the various models available in the API.
//! You can refer to the [Models](https://beta.openai.com/docs/models)
//! documentation to understand what models are available and the differences between them.

use super::{openai_get, ApiResponseOrError};
use openai_proc_macros::generate_model_id_enum;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Model {
    pub id: ModelID,
    pub created: u32,
    pub owned_by: String,
    pub permission: Vec<ModelPermission>,
    pub root: String,
    pub parent: Option<String>,
}

impl Model {
    //! Retrieves a model instance,
    //! providing basic information about the model such as the owner and permissioning.
    pub async fn new(id: ModelID) -> ApiResponseOrError<Self> {
        openai_get(&format!("models/{id}")).await
    }
}

#[derive(Deserialize)]
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

generate_model_id_enum!();

impl std::fmt::Display for ModelID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let ModelID::Custom(id) = self {
            write!(f, "{id}")
        } else {
            let serialized = serde_json::to_string(self).unwrap();

            write!(f, "{}", &serialized[1..serialized.len() - 1])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;

    #[test]
    fn model_id_serialization() -> Result<(), serde_json::Error> {
        assert_eq!(
            serde_json::ser::to_string(&ModelID::TextDavinci003)?,
            "\"text-davinci-003\"",
        );

        assert_eq!(
            serde_json::ser::to_string(&ModelID::Custom("custom".to_string()))?,
            "\"custom\"",
        );

        Ok(())
    }

    #[test]
    fn model_id_deserialization() -> Result<(), serde_json::Error> {
        assert_eq!(
            serde_json::de::from_str::<ModelID>("\"text-davinci-003\"")?,
            ModelID::TextDavinci003,
        );

        assert_eq!(
            serde_json::de::from_str::<ModelID>("\"custom\"")?,
            ModelID::Custom("custom".to_string()),
        );

        Ok(())
    }

    #[tokio::test]
    async fn model() {
        dotenv().ok();

        let model = Model::new(ModelID::TextDavinci003).await.unwrap().unwrap();

        assert_eq!(model.id, ModelID::TextDavinci003,);
    }

    #[tokio::test]
    async fn custom_model() {
        dotenv().ok();

        let model = Model::new(ModelID::Custom(
            "davinci:ft-personal-2022-12-12-04-49-51".to_string(),
        ))
        .await
        .unwrap()
        .unwrap();

        assert_eq!(
            model.id,
            ModelID::Custom("davinci:ft-personal-2022-12-12-04-49-51".to_string()),
        );
    }
}
