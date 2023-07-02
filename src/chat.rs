//! Given a chat conversation, the model will return a chat completion response.

use super::{openai_post, ApiResponseOrError, Usage};
use crate::openai_request_stream;
use derive_builder::Builder;
use futures_util::StreamExt;
use reqwest::Method;
use reqwest_eventsource::{CannotCloneRequestError, Event, EventSource};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use tokio::sync::mpsc::{channel, Receiver, Sender};

/// A full chat completion.
pub type ChatCompletion = ChatCompletionGeneric<ChatCompletionChoice>;

/// A delta chat completion, which is streamed token by token.
pub type ChatCompletionDelta = ChatCompletionGeneric<ChatCompletionChoiceDelta>;

#[derive(Deserialize, Clone, Debug)]
pub struct ChatCompletionGeneric<C> {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<C>,
    pub usage: Option<Usage>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ChatCompletionChoice {
    pub index: u64,
    pub finish_reason: String,
    pub message: ChatCompletionMessage,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ChatCompletionChoiceDelta {
    pub index: u64,
    pub finish_reason: Option<String>,
    pub delta: ChatCompletionMessageDelta,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChatCompletionMessage {
    /// The role of the author of this message.
    pub role: ChatCompletionMessageRole,
    /// The contents of the message
    ///
    /// This is always required for all messages, except for when ChatGPT calls
    /// a function.
    pub content: Option<String>,
    /// The name of the user in a multi-user chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The function that ChatGPT called. This should be "None" usually, and is returned by ChatGPT and not provided by the developer
    ///
    /// [API Reference](https://platform.openai.com/docs/api-reference/chat/create#chat/create-function_call)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<ChatCompletionFunctionCall>,
}

/// Same as ChatCompletionMessage, but received during a response stream.
#[derive(Deserialize, Clone, Debug)]
pub struct ChatCompletionMessageDelta {
    /// The role of the author of this message.
    pub role: Option<ChatCompletionMessageRole>,
    /// The contents of the message
    pub content: Option<String>,
    /// The name of the user in a multi-user chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The function that ChatGPT called
    ///
    /// [API Reference](https://platform.openai.com/docs/api-reference/chat/create#chat/create-function_call)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<ChatCompletionFunctionCallDelta>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ChatCompletionFunctionDefinition {
    /// The name of the function
    pub name: String,
    /// The description of the function
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The parameters of the function formatted in JSON Schema
    /// [API Reference](https://platform.openai.com/docs/api-reference/chat/create#chat/create-parameters)
    /// [See more information about JSON Schema.](https://json-schema.org/understanding-json-schema/)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Value>,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ChatCompletionFunctionCall {
    /// The name of the function ChatGPT called
    pub name: String,
    /// The arguments that ChatGPT called (formatted in JSON)
    /// [API Reference](https://platform.openai.com/docs/api-reference/chat/create#chat/create-function_call)
    pub arguments: String
}

/// Same as ChatCompletionFunctionCall, but received during a response stream.
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq, Eq)]
pub struct ChatCompletionFunctionCallDelta {
    /// The name of the function ChatGPT called
    pub name: Option<String>,
    /// The arguments that ChatGPT called (formatted in JSON)
    /// [API Reference](https://platform.openai.com/docs/api-reference/chat/create#chat/create-function_call)
    pub arguments: Option<String>
}

#[derive(Deserialize, Serialize, Debug, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum ChatCompletionMessageRole {
    System,
    User,
    Assistant,
    Function,
}

#[derive(Serialize, Builder, Debug, Clone)]
#[builder(pattern = "owned")]
#[builder(name = "ChatCompletionBuilder")]
#[builder(setter(strip_option, into))]
pub struct ChatCompletionRequest {
    /// ID of the model to use. Currently, only `gpt-3.5-turbo`, `gpt-3.5-turbo-0301` `gpt-3.5-turbo-0613` and `gpt-4`
    /// are supported.
    model: String,
    /// The messages to generate chat completions for, in the [chat format](https://platform.openai.com/docs/guides/chat/introduction).
    messages: Vec<ChatCompletionMessage>,
    /// What sampling temperature to use, between 0 and 2. Higher values like 0.8 will make the output more random, while lower values like 0.2 will make it more focused and deterministic.
    ///
    /// We generally recommend altering this or `top_p` but not both.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    /// An alternative to sampling with temperature, called nucleus sampling, where the model considers the results of the tokens with top_p probability mass. So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    ///
    /// We generally recommend altering this or `temperature` but not both.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    /// How many chat completion choices to generate for each input message.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    n: Option<u8>,
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    stream: Option<bool>,
    /// Up to 4 sequences where the API will stop generating further tokens.
    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    stop: Vec<String>,
    /// The maximum number of tokens allowed for the generated answer. By default, the number of tokens the model can return will be (4096 - prompt tokens).
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u64>,
    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on whether they appear in the text so far, increasing the model's likelihood to talk about new topics.
    ///
    /// [See more information about frequency and presence penalties.](https://platform.openai.com/docs/api-reference/parameter-details)
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    presence_penalty: Option<f32>,
    /// Number between -2.0 and 2.0. Positive values penalize new tokens based on their existing frequency in the text so far, decreasing the model's likelihood to repeat the same line verbatim.
    ///
    /// [See more information about frequency and presence penalties.](https://platform.openai.com/docs/api-reference/parameter-details)
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency_penalty: Option<f32>,
    /// Modify the likelihood of specified tokens appearing in the completion.
    ///
    /// Accepts a json object that maps tokens (specified by their token ID in the tokenizer) to an associated bias value from -100 to 100. Mathematically, the bias is added to the logits generated by the model prior to sampling. The exact effect will vary per model, but values between -1 and 1 should decrease or increase likelihood of selection; values like -100 or 100 should result in a ban or exclusive selection of the relevant token.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    logit_bias: Option<HashMap<String, f32>>,
    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse. [Learn more](https://platform.openai.com/docs/guides/safety-best-practices/end-user-ids).
    #[builder(default)]
    #[serde(skip_serializing_if = "String::is_empty")]
    user: String,
    /// Describe functions that ChatGPT can call
    /// The latest models of ChatGPT support function calling, which allows you to define functions that can be called from the prompt.
    /// For example, you can define a function called "get_weather" that returns the weather in a given city
    ///
    /// [Function calling API Reference](https://platform.openai.com/docs/api-reference/chat/create#chat/create-functions)
    /// [See more information about function calling in ChatGPT.](https://platform.openai.com/docs/guides/gpt/function-calling)
    #[builder(default)]
    #[serde(skip_serializing_if = "Vec::is_empty")]
    functions: Vec<ChatCompletionFunctionDefinition>,
    /// A string or object of the function to call
    ///
    /// Controls how the model responds to function calls
    ///
    /// - "none" means the model does not call a function, and responds to the end-user.
    /// - "auto" means the model can pick between an end-user or calling a function.
    /// - Specifying a particular function via {"name":\ "my_function"} forces the model to call that function.
    ///
    /// "none" is the default when no functions are present. "auto" is the default if functions are present.
    #[builder(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    function_call: Option<Value>
}

impl<C> ChatCompletionGeneric<C> {
    pub fn builder(
        model: &str,
        messages: impl Into<Vec<ChatCompletionMessage>>,
    ) -> ChatCompletionBuilder {
        ChatCompletionBuilder::create_empty()
            .model(model)
            .messages(messages)
    }
}

impl ChatCompletion {
    pub async fn create(request: &ChatCompletionRequest) -> ApiResponseOrError<Self> {
        openai_post("chat/completions", request).await
    }
}

impl ChatCompletionDelta {
    pub async fn create(
        request: &ChatCompletionRequest,
    ) -> Result<Receiver<Self>, CannotCloneRequestError> {
        let stream =
            openai_request_stream(Method::POST, "chat/completions", |r| r.json(request)).await?;
        let (tx, rx) = channel::<Self>(32);
        tokio::spawn(forward_deserialized_chat_response_stream(stream, tx));
        Ok(rx)
    }

    /// Merges the input delta completion into `self`.
    pub fn merge(
        &mut self,
        other: ChatCompletionDelta,
    ) -> Result<(), ChatCompletionDeltaMergeError> {
        if other.id.ne(&self.id) {
            return Err(ChatCompletionDeltaMergeError::DifferentCompletionIds);
        }
        for other_choice in other.choices.iter() {
            for choice in self.choices.iter_mut() {
                if choice.index != other_choice.index {
                    continue;
                }
                choice.merge(other_choice)?;
            }
        }
        Ok(())
    }
}

impl ChatCompletionChoiceDelta {
    pub fn merge(
        &mut self,
        other: &ChatCompletionChoiceDelta,
    ) -> Result<(), ChatCompletionDeltaMergeError> {
        if self.index != other.index {
            return Err(ChatCompletionDeltaMergeError::DifferentCompletionChoiceIndices);
        }
        if self.delta.role.is_none() {
            if let Some(other_role) = other.delta.role {
                // Set role to other_role.
                self.delta.role = Some(other_role);
            }
        }
        if self.delta.name.is_none() {
            if let Some(other_name) = &other.delta.name {
                // Set name to other_name.
                self.delta.name = Some(other_name.clone());
            }
        }
        // Merge contents.
        match self.delta.content.as_mut() {
            Some(content) => {
                match &other.delta.content {
                    Some(other_content) => {
                        // Push other content into this one.
                        content.push_str(other_content)
                    }
                    None => {}
                }
            }
            None => {
                match &other.delta.content {
                    Some(other_content) => {
                        // Set this content to other content.
                        self.delta.content = Some(other_content.clone());
                    }
                    None => {}
                }
            }
        };

        // merge function calls
        // function call names are concatenated
        // arguments are merged by concatenating them
        match self.delta.function_call.as_mut() {
            Some(function_call) => {
                match &other.delta.function_call {
                    Some(other_function_call) => {
                        // push the arguments string of the other function call into this one
                        match (&mut function_call.arguments, &other_function_call.arguments) {
                            (Some(function_call), Some(other_function_call)) => {
                                function_call.push_str(&other_function_call);
                            }
                            (None, Some(other_function_call)) => {
                                function_call.arguments = Some(other_function_call.clone());
                            }
                            _ => {}
                        }

                    }
                    None => {}
                }
            }
            None => {
                match &other.delta.function_call {
                    Some(other_function_call) => {
                        // Set this content to other content.
                        self.delta.function_call = Some(other_function_call.clone());
                    }
                    None => {}
                }
            }
        };
        Ok(())
    }
}

impl From<ChatCompletionDelta> for ChatCompletion {
    fn from(delta: ChatCompletionDelta) -> Self {
        ChatCompletion {
            id: delta.id,
            object: delta.object,
            created: delta.created,
            model: delta.model,
            usage: delta.usage,
            choices: delta
                .choices
                .iter()
                .map(|choice| ChatCompletionChoice {
                    index: choice.index,
                    finish_reason: clone_default_unwrapped_option_string(&choice.finish_reason),
                    message: ChatCompletionMessage {
                        role: choice
                            .delta
                            .role
                            .unwrap_or_else(|| ChatCompletionMessageRole::System),
                        content: choice.delta.content.clone(),
                        name: choice.delta.name.clone(),
                        function_call: choice.delta.function_call.clone().map(|f| f.into()),

                    },
                })
                .collect(),
        }
    }
}

impl From<ChatCompletionFunctionCallDelta> for ChatCompletionFunctionCall {
    fn from(delta: ChatCompletionFunctionCallDelta) -> Self {
        ChatCompletionFunctionCall {
            name: delta.name.unwrap_or("".to_string()),
            arguments: delta.arguments.unwrap_or_default(),
        }
    }
}

#[derive(Debug)]
pub enum ChatCompletionDeltaMergeError {
    DifferentCompletionIds,
    DifferentCompletionChoiceIndices,
    FunctionCallArgumentTypeMismatch,
}

async fn forward_deserialized_chat_response_stream(
    mut stream: EventSource,
    tx: Sender<ChatCompletionDelta>,
) -> anyhow::Result<()> {
    while let Some(event) = stream.next().await {
        let event = event?;
        match event {
            Event::Message(event) => {
                let completion = serde_json::from_str::<ChatCompletionDelta>(&event.data)?;
                tx.send(completion).await?;
            }
            _ => {}
        }
    }
    Ok(())
}

impl ChatCompletionBuilder {
    pub async fn create(self) -> ApiResponseOrError<ChatCompletion> {
        ChatCompletion::create(&self.build().unwrap()).await
    }

    pub async fn create_stream(
        mut self,
    ) -> Result<Receiver<ChatCompletionDelta>, CannotCloneRequestError> {
        self.stream = Some(Some(true));
        ChatCompletionDelta::create(&self.build().unwrap()).await
    }
}

fn clone_default_unwrapped_option_string(string: &Option<String>) -> String {
    match string {
        Some(value) => value.clone(),
        None => "".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::set_key;
    use dotenvy::dotenv;
    use std::env;

    #[tokio::test]
    async fn chat() {
        dotenv().ok();
        set_key(env::var("OPENAI_KEY").unwrap());

        let chat_completion = ChatCompletion::builder(
            "gpt-3.5-turbo",
            [ChatCompletionMessage {
                role: ChatCompletionMessageRole::User,
                content: Some("Hello!".to_string()),
                name: None,
                function_call: None,
            }],
        )
        .temperature(0.0)
        .create()
        .await
        .unwrap();

        assert_eq!(
            chat_completion.choices.first().unwrap().message.content.as_ref().unwrap(),
            "Hello! How can I assist you today?"
        );
    }

    #[tokio::test]
    async fn chat_stream() {
        dotenv().ok();
        set_key(env::var("OPENAI_KEY").unwrap());

        let chat_stream = ChatCompletion::builder(
            "gpt-3.5-turbo",
            [ChatCompletionMessage {
                role: ChatCompletionMessageRole::User,
                content: Some("Hello!".to_string()),
                name: None,
                function_call: None,
            }],
        )
        .temperature(0.0)
        .create_stream()
        .await
        .unwrap();

        let chat_completion = stream_to_completion(chat_stream).await;

        assert_eq!(
            chat_completion.choices.first().unwrap().message.content.as_ref().unwrap(),
            "Hello! How can I assist you today?"
        );
    }

    #[tokio::test]
    async fn chat_function() {
        dotenv().ok();
        set_key(env::var("OPENAI_KEY").unwrap());

        let chat_stream = ChatCompletion::builder(
            "gpt-3.5-turbo-0613",
            [
                ChatCompletionMessage {
                    role: ChatCompletionMessageRole::User,
                    content: Some("What is the weather in Boston?".to_string()),
                    name: None,
                    function_call: None,
                }
            ]
        ).functions([ChatCompletionFunctionDefinition {
            description: Some("Get the current weather in a given location.".to_string()),
            name: "get_current_weather".to_string(),
            parameters: Some(serde_json::json!({
                "type": "object",
                "properties": {
                    "location": {
                        "type": "string",
                        "description": "The city and state to get the weather for. (eg: San Francisco, CA)"
                    }
                },
                "required": ["location"]
            })),
        }])
        .temperature(0.2)
        .create_stream()
        .await
        .unwrap();

        let chat_completion = stream_to_completion(chat_stream).await;

        assert_eq!(
            chat_completion.choices.first().unwrap().message.function_call.as_ref().unwrap().name,
            "get_current_weather".to_string(),
        );

        assert_eq!(
            serde_json::from_str::<Value>(&chat_completion.choices.first().unwrap().message.function_call.as_ref().unwrap().arguments).unwrap(),
            serde_json::json!({
                "location": "Boston, MA"
            }),
        );


    }

    async fn stream_to_completion(
        mut chat_stream: Receiver<ChatCompletionDelta>,
    ) -> ChatCompletion {
        let mut merged: Option<ChatCompletionDelta> = None;
        while let Some(delta) = chat_stream.recv().await {
            match merged.as_mut() {
                Some(c) => {
                    c.merge(delta).unwrap();
                }
                None => merged = Some(delta),
            };
        }
        merged.unwrap().into()
    }
}
