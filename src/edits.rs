//! Given a prompt and an instruction, the model will return an edited version of the prompt.

use super::{openai_post, ApiResponseOrError, OpenAiError, Usage};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct Edit {
    pub created: u32,
    #[serde(skip_deserializing)]
    pub choices: Vec<String>,
    pub usage: Usage,
    #[serde(rename = "choices")]
    choices_bad: Vec<EditChoice>,
}

#[derive(Deserialize, Clone)]
struct EditChoice {
    text: String,
}

#[derive(Serialize, Builder, Debug, Clone)]
#[builder(pattern = "owned")]
#[builder(name = "EditBuilder")]
#[builder(setter(strip_option, into))]
pub struct EditRequest {
    /// ID of the model to use.
    /// You can use the `text-davinci-edit-001` or `code-davinci-edit-001` model with this endpoint.
    pub model: String,
    /// The input text to use as a starting point for the edit.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub input: Option<String>,
    /// The instruction that tells the model how to edit the prompt.
    pub instruction: String,
    /// How many edits to generate for the input and instruction.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(setter(into = false), default)]
    pub n: Option<u16>,
    /// What [sampling temperature](https://towardsdatascience.com/how-to-sample-from-language-models-682bceb97277) to use.
    /// Higher values means the model will take more risks.
    /// Try 0.9 for more creative applications, and 0 (argmax sampling) for ones with a well-defined answer.
    ///
    /// We generally recommend altering this or `top_p` but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub temperature: Option<f32>,
    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p probability mass.
    /// So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    ///
    /// We generally recommend altering this or `temperature` but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub top_p: Option<f32>,
}

impl Edit {
    async fn create(request: &EditRequest) -> ApiResponseOrError<Self> {
        let response: Result<Self, OpenAiError> = openai_post("edits", request).await?;

        match response {
            Ok(mut edit) => {
                for choice in &edit.choices_bad {
                    edit.choices.push(choice.text.clone());
                }

                Ok(edit)
            }
            Err(_) => response,
        }
    }

    pub fn builder(model: &str, instruction: impl Into<String>) -> EditBuilder {
        EditBuilder::create_empty()
            .model(model)
            .instruction(instruction)
    }
}

impl EditBuilder {
    pub async fn create(self) -> ApiResponseOrError<Edit> {
        Edit::create(&self.build().unwrap()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::set_key;
    use dotenvy::dotenv;
    use std::env;

    #[tokio::test]
    #[ignore]
    async fn edit() {
        dotenv().ok();
        set_key(env::var("OPENAI_KEY").unwrap());

        let edit = Edit::builder("text-davinci-edit-001", "Fix the spelling mistakes")
            .input("What day of the wek is it?")
            .temperature(0.0)
            .create()
            .await
            .unwrap();

        assert_eq!(
            edit.choices.first().unwrap(),
            "What day of the week is it?\n"
        );
    }
}
