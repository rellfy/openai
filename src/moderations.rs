//! Given a input text, outputs if the model classifies it as violating OpenAI's content policy.

use super::{openai_post, ApiResponseOrError};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone, Debug)]
pub struct Moderation {
    pub id: String,
    pub model: String,
    pub results: Vec<ModerationResult>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ModerationResult {
    pub flagged: bool,
    pub categories: Categories,
    pub category_scores: CategoryScores,
}

#[derive(Deserialize, Clone, Copy, Debug)]
pub struct Categories {
    pub hate: bool,
    #[serde(rename = "hate/threatening")]
    pub hate_threatening: bool,
    #[serde(rename = "self-harm")]
    pub self_harm: bool,
    pub sexual: bool,
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: bool,
    pub violence: bool,
    #[serde(rename = "violence/graphic")]
    pub violence_graphic: bool,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CategoryScores {
    pub hate: f64,
    #[serde(rename = "hate/threatening")]
    pub hate_threatening: f64,
    #[serde(rename = "self-harm")]
    pub self_harm: f64,
    pub sexual: f64,
    #[serde(rename = "sexual/minors")]
    pub sexual_minors: f64,
    pub violence: f64,
    #[serde(rename = "violence/graphic")]
    pub violence_graphic: f64,
}

#[derive(Serialize, Builder, Debug, Clone)]
#[builder(pattern = "owned")]
#[builder(name = "ModerationBuilder")]
#[builder(setter(strip_option, into))]
pub struct ModerationRequest {
    /// The input text to classify.
    pub input: String,
    /// ID of the model to use.
    /// Two content moderations models are available: `text-moderation-stable` and `text-moderation-latest`.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub model: Option<String>,
}

impl Moderation {
    async fn create(request: &ModerationRequest) -> ApiResponseOrError<Self> {
        openai_post("moderations", request).await
    }

    pub fn builder(input: impl Into<String>) -> ModerationBuilder {
        ModerationBuilder::create_empty().input(input)
    }
}

impl ModerationBuilder {
    pub async fn create(self) -> ApiResponseOrError<Moderation> {
        Moderation::create(&self.build().unwrap()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::set_key;
    use dotenvy::dotenv;
    use std::env;

    #[tokio::test]
    async fn moderations() {
        dotenv().ok();
        set_key(env::var("OPENAI_KEY").unwrap());

        let moderation = Moderation::builder("I want to kill them.")
            .model("text-moderation-latest")
            .create()
            .await
            .unwrap()
            .unwrap();

        assert_eq!(
            moderation.results.first().unwrap().categories.violence,
            true
        );
        assert_eq!(moderation.results.first().unwrap().flagged, true);
    }
}
