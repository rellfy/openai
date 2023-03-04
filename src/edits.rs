// Copyright (C) 2023  Valentine Briese
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Given a prompt and an instruction, the model will return an edited version of the prompt.

use super::{models::ModelID, openai_post, ApiResponseOrError, OpenAiError, Usage};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct Edit {
    pub created: u32,
    #[serde(skip_deserializing)]
    pub choices: Vec<String>,
    pub usage: Usage,
    #[serde(rename = "choices")]
    choices_bad: Vec<Choice>,
}

impl Edit {
    async fn new(body: &CreateEditRequestBody<'_>) -> ApiResponseOrError<Self> {
        let response: Result<Self, OpenAiError> = openai_post("edits", body).await?;

        match response {
            Ok(mut edit) => {
                for choice in &edit.choices_bad {
                    edit.choices.push(choice.text.clone());
                }

                Ok(Ok(edit))
            }
            Err(_) => Ok(response),
        }
    }

    pub fn builder<'a>() -> EditBuilder<'a> {
        EditBuilder::default()
    }
}

#[derive(Deserialize)]
struct Choice {
    text: String,
}

#[derive(Serialize, Default, Builder)]
#[builder(pattern = "owned")]
#[builder(name = "EditBuilder")]
#[builder(setter(strip_option))]
pub struct CreateEditRequestBody<'a> {
    /// ID of the model to use.
    /// You can use the `text-davinci-edit-001` or `code-davinci-edit-001` model with this endpoint.
    pub model: ModelID,
    /// The input text to use as a starting point for the edit.
    #[serde(skip_serializing_if = "str::is_empty")]
    #[builder(default)]
    pub input: &'a str,
    /// The instruction that tells the model how to edit the prompt.
    pub instruction: &'a str,
    /// How many edits to generate for the input and instruction.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
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

impl EditBuilder<'_> {
    pub async fn create(self) -> ApiResponseOrError<Edit> {
        Edit::new(&self.build().unwrap()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;

    #[tokio::test]
    async fn edit() {
        dotenv().ok();

        let edit = Edit::builder()
            .model(ModelID::TextDavinciEdit001)
            .input("What day of the wek is it?")
            .instruction("Fix the spelling mistakes")
            .temperature(0.0)
            .create()
            .await
            .unwrap()
            .unwrap();

        assert_eq!(
            edit.choices.first().unwrap(),
            "What day of the week is it?\n"
        );
    }
}
