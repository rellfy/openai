//! Given a prompt and an instruction, the model will return an edited version of the prompt.

use serde::{ Deserialize, Serialize };
use super::{ Usage, models::ModelID };
use reqwest::Client;
use openai_utils::{ BASE_URL, authorization };

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
    pub async fn new(body: &CreateEditRequestBody<'_>) -> Result<Self, reqwest::Error> {
        let client = Client::builder().build()?;

        let mut edit: Self = authorization!(client.post(format!("{BASE_URL}/edits")))
            .json(body)
            .send().await?.json().await?;

        for choice in &edit.choices_bad {
            edit.choices.push(choice.text.clone());
        }

        Ok(edit)
    }
}

#[derive(Deserialize)]
struct Choice {
    text: String,
}

#[derive(Serialize, Default)]
pub struct CreateEditRequestBody<'a> {
    /// ID of the model to use.
    /// You can use the `text-davinci-edit-001` or `code-davinci-edit-001` model with this endpoint.
    pub model: ModelID,
    /// The input text to use as a starting point for the edit.
    #[serde(skip_serializing_if = "str::is_empty")]
    pub input: &'a str,
    /// The instruction that tells the model how to edit the prompt.
    pub instruction: &'a str,
    /// How many edits to generate for the input and instruction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u16>,
    /// What [sampling temperature](https://towardsdatascience.com/how-to-sample-from-language-models-682bceb97277) to use.
    /// Higher values means the model will take more risks.
    /// Try 0.9 for more creative applications, and 0 (argmax sampling) for ones with a well-defined answer.
    ///
    /// We generally recommend altering this or `top_p` but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p probability mass.
    /// So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    ///
    /// We generally recommend altering this or `temperature` but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use dotenvy::dotenv;

    #[tokio::test]
    async fn edit() {
        dotenv().ok();

        let edit = Edit::new(&CreateEditRequestBody {
            model: ModelID::TextDavinciEdit001,
            input: "What day of the wek is it?",
            instruction: "Fix the spelling mistakes",
            temperature: Some(0.0),
            ..Default::default()
        }).await.unwrap();

        assert_eq!(edit.choices.first().unwrap(), "What day of the week is it?\n");
    }
}
