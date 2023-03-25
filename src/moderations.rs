//! Given a input text, outputs if the model classifies it as violating OpenAI's content policy.

use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use super::{ApiResponseOrError, openai_post};

#[derive(Deserialize, Clone, Debug)]
pub struct Moderation {
    pub id: String,
    #[serde(rename = "results")]
    pub results: Vec<ModerationResult>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ModerationResult {
    categories: Categories,
    category_scores: CategoryScores,
    flagged: bool,
}

#[derive(Deserialize, Clone, Debug)]
struct Categories {
    hate: bool,
    #[serde(rename = "hate/threatening")]
    hate_or_threatening: bool,
    #[serde(rename = "self-harm")]
    self_harm: bool,
    sexual: bool,
    #[serde(rename = "sexual/minors")]
    sexual_or_minors: bool,
    violence: bool,
    #[serde(rename = "violence/graphic")]
    violence_or_graphic: bool,
}

#[derive(Deserialize, Clone, Debug)]
struct CategoryScores {
    hate: f64,
    #[serde(rename = "hate/threatening")]
    hate_or_threatening: f64,
    #[serde(rename = "self-harm")]
    self_harm: f64,
    sexual: f64,
    #[serde(rename = "sexual/minors")]
    sexual_or_minors: f64,
    violence: f64,
    #[serde(rename = "violence/graphic")]
    violence_or_graphic: f64,
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
    use std::env;

    use dotenvy::dotenv;

    use crate::set_key;

    use super::*;

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

        assert_eq!(moderation.results.first().unwrap().categories.violence, true);
        assert_eq!(moderation.results.first().unwrap().flagged, true);
    }
}
