//! Given a chat conversation, the model will return a chat completion response.
pub mod modules;
pub mod requests;
pub mod types;
pub mod utils;

pub use modules::*;
pub use requests::*;
use serde::{Deserialize, Serialize};
pub use types::*;
pub use utils::*;

#[derive(Deserialize, Serialize, Debug, Clone, Copy, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ChatCompletionMessageRole {
    System,
    User,
    Assistant,
    Function,
    Tool,
    Developer,
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ToolChoiceMode {
    None,
    Auto,
    Required,
}

#[derive(Debug)]
pub enum ChatCompletionDeltaMergeError {
    DifferentCompletionIds,
    DifferentCompletionChoiceIndices,
    FunctionCallArgumentTypeMismatch,
}

impl std::fmt::Display for ChatCompletionDeltaMergeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatCompletionDeltaMergeError::DifferentCompletionIds => {
                f.write_str("Different completion IDs")
            }
            ChatCompletionDeltaMergeError::DifferentCompletionChoiceIndices => {
                f.write_str("Different completion choice indices")
            }
            ChatCompletionDeltaMergeError::FunctionCallArgumentTypeMismatch => {
                f.write_str("Function call argument type mismatch")
            }
        }
    }
}

impl std::error::Error for ChatCompletionDeltaMergeError {}

impl Default for ChatCompletionMessageRole {
    fn default() -> Self {
        Self::User
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Credentials, RequestPagination};
    use dotenvy::dotenv;
    use serde_json::Value;
    use std::time::Duration;
    use tokio::sync::mpsc::Receiver;
    use tokio::time::sleep;

    #[tokio::test]
    async fn chat() {
        dotenv().ok();
        let credentials = Credentials::from_env();

        let chat_completion = ChatCompletion::builder(
            "gpt-3.5-turbo",
            [ChatCompletionMessage {
                role: ChatCompletionMessageRole::User,
                content: Some(Content::new_str("Hello!")),
                name: None,
                function_call: None,
                tool_call_id: None,
                tool_calls: Some(Vec::new()),
            }],
        )
        .temperature(0.0)
        .response_format(ChatCompletionResponseFormat::text())
        .credentials(credentials)
        .create()
        .await
        .unwrap();

        assert_eq!(
            chat_completion
                .choices
                .first()
                .unwrap()
                .message
                .content
                .as_ref()
                .unwrap(),
            &Content::new_str("Hello! How can I assist you today?")
        );
    }

    // Seeds are not deterministic so the only point of the test is to
    // ensure that passing a seed still results in a valid response.
    #[tokio::test]
    async fn chat_seed() {
        dotenv().ok();
        let credentials = Credentials::from_env();

        let chat_completion = ChatCompletion::builder(
            "gpt-3.5-turbo",
            [ChatCompletionMessage {
                role: ChatCompletionMessageRole::User,
                content: Some(Content::new_str(
                    "What type of seed does Mr. England sow in the song? Reply with 1 word.",
                )),
                name: None,
                function_call: None,
                tool_call_id: None,
                tool_calls: Some(Vec::new()),
            }],
        )
        // Determinism currently comes from temperature 0, not seed.
        .temperature(0.0)
        .seed(1337u64)
        .credentials(credentials)
        .create()
        .await
        .unwrap();

        assert_eq!(
            chat_completion
                .choices
                .first()
                .unwrap()
                .message
                .content
                .as_ref()
                .unwrap(),
            &Content::new_str("Love")
        );
    }

    #[tokio::test]
    async fn chat_stream() {
        dotenv().ok();
        let credentials = Credentials::from_env();

        let chat_stream = ChatCompletion::builder(
            "gpt-3.5-turbo",
            [ChatCompletionMessage {
                role: ChatCompletionMessageRole::User,
                content: Some(Content::new_str("Hello!")),
                name: None,
                function_call: None,
                tool_call_id: None,
                tool_calls: Some(Vec::new()),
            }],
        )
        .temperature(0.0)
        .credentials(credentials)
        .create_stream()
        .await
        .unwrap();

        let chat_completion = stream_to_completion(chat_stream).await;

        assert_eq!(
            chat_completion
                .choices
                .first()
                .unwrap()
                .message
                .content
                .as_ref()
                .unwrap(),
            &Content::new_str("Hello! How can I assist you today?")
        );
    }

    #[tokio::test]
    async fn chat_function() {
        dotenv().ok();
        let credentials = Credentials::from_env();

        let chat_stream = ChatCompletion::builder(
            "gpt-4o",
            [
                ChatCompletionMessage {
                    role: ChatCompletionMessageRole::User,
                    content: Some(Content::new_str("What is the weather in Boston?")),
                    name: None,
                    function_call: None,
                    tool_call_id: None,
                    tool_calls: Some(Vec::new()),
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
        .credentials(credentials)
        .create_stream()
        .await
        .unwrap();

        let chat_completion = stream_to_completion(chat_stream).await;

        assert_eq!(
            chat_completion
                .choices
                .first()
                .unwrap()
                .message
                .function_call
                .as_ref()
                .unwrap()
                .name,
            "get_current_weather".to_string(),
        );

        assert_eq!(
            serde_json::from_str::<Value>(
                &chat_completion
                    .choices
                    .first()
                    .unwrap()
                    .message
                    .function_call
                    .as_ref()
                    .unwrap()
                    .arguments
            )
            .unwrap(),
            serde_json::json!({
                "location": "Boston, MA"
            }),
        );
    }

    #[test]
    fn builder_clone_and_eq() {
        let builder_a = ChatCompletion::builder("gpt-4", [])
            .temperature(0.0)
            .seed(65u64);
        let builder_b = builder_a.clone();
        let builder_c = builder_b.clone().temperature(1.0);
        let builder_d = ChatCompletionBuilder::default();
        assert_eq!(builder_a, builder_b);
        assert_ne!(builder_a, builder_c);
        assert_ne!(builder_b, builder_c);
        assert_ne!(builder_a, builder_d);
        assert_ne!(builder_c, builder_d);
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

    #[tokio::test]
    async fn chat_tool_response_completion() {
        dotenv().ok();
        let credentials = Credentials::from_env();

        let chat_completion = ChatCompletion::builder(
            "gpt-4o-mini",
            [
                ChatCompletionMessage {
                    role: ChatCompletionMessageRole::User,
                    content: Some(Content::new_str(
                        "What's 0.9102847*28456? \
                        reply in plain text, \
                        round the number to to 2 decimals \
                        and reply with the result number only, \
                        with no full stop at the end",
                    )),
                    name: None,
                    function_call: None,
                    tool_call_id: None,
                    tool_calls: Some(Vec::new()),
                },
                ChatCompletionMessage {
                    role: ChatCompletionMessageRole::Assistant,
                    content: Some(Content::new_str("Let me calculate that for you.")),
                    name: None,
                    function_call: None,
                    tool_call_id: None,
                    tool_calls: Some(vec![ToolCall {
                        id: "the_tool_call".to_string(),
                        r#type: "function".to_string(),
                        function: ToolCallFunction {
                            name: "mul".to_string(),
                            arguments: "not_required_to_be_valid_here".to_string(),
                        },
                    }]),
                },
                ChatCompletionMessage {
                    role: ChatCompletionMessageRole::Tool,
                    content: Some(Content::new_str("the result is 25903.061423199997")),
                    name: None,
                    function_call: None,
                    tool_call_id: Some("the_tool_call".to_owned()),
                    tool_calls: Some(Vec::new()),
                },
            ],
        )
        // Determinism currently comes from temperature 0, not seed.
        .temperature(0.0)
        .seed(1337u64)
        .credentials(credentials)
        .create()
        .await
        .unwrap();

        assert_eq!(
            chat_completion
                .choices
                .first()
                .unwrap()
                .message
                .content
                .as_ref()
                .unwrap(),
            &Content::new_str("25903.06")
        );
    }

    #[tokio::test]
    async fn get_completion() {
        dotenv().ok();
        let credentials = Credentials::from_env();

        let chat_completion = ChatCompletion::builder(
            "gpt-3.5-turbo",
            [ChatCompletionMessage {
                role: ChatCompletionMessageRole::User,
                content: Some(Content::new_str("Hello!")),
                ..Default::default()
            }],
        )
        .credentials(credentials.clone())
        .store(true)
        .create()
        .await
        .unwrap();

        // Unfortunatelly completions are not available immediately so we need to wait a bit
        sleep(Duration::from_secs(7)).await;

        let retrieved_completion = ChatCompletion::get(&chat_completion.id, credentials.clone())
            .await
            .unwrap();

        assert_eq!(retrieved_completion, chat_completion);
    }

    #[tokio::test]
    async fn get_completion_non_existent() {
        dotenv().ok();
        let credentials = Credentials::from_env();

        match ChatCompletion::get("non_existent_id", credentials.clone()).await {
            Ok(_) => panic!("Expected error"),
            Err(e) => assert_eq!(e.code, Some("not_found".to_string())),
        }
    }

    #[tokio::test]
    async fn get_completion_messages() {
        dotenv().ok();
        let credentials = Credentials::from_env();

        let user_message = ChatCompletionMessage {
            role: ChatCompletionMessageRole::User,
            content: Some(Content::new_str("Tell me a short joke")),
            ..Default::default()
        };

        let chat_completion = ChatCompletion::builder("gpt-3.5-turbo", [user_message.clone()])
            .credentials(credentials.clone())
            .store(true)
            .create()
            .await
            .unwrap();

        // Unfortunatelly completions are not available immediately so we need to wait a bit
        sleep(Duration::from_secs(7)).await;

        let retrieved_messages = ChatCompletionMessages::builder(chat_completion.id)
            .credentials(credentials.clone())
            .fetch()
            .await
            .unwrap();

        assert_eq!(retrieved_messages.data, vec![user_message]);
        assert_eq!(retrieved_messages.has_more, false);
    }

    #[tokio::test]
    async fn get_completion_messages_with_pagination() {
        dotenv().ok();
        let credentials = Credentials::from_env();

        let user_message = ChatCompletionMessage {
            role: ChatCompletionMessageRole::User,
            content: Some(Content::new_str("Tell me a short joke")),
            ..Default::default()
        };

        let chat_completion = ChatCompletion::builder("gpt-3.5-turbo", [user_message.clone()])
            .credentials(credentials.clone())
            .store(true)
            .create()
            .await
            .unwrap();

        dbg!(&chat_completion);

        // Unfortunatelly completions are not available immediately so we need to wait a bit
        sleep(Duration::from_secs(7)).await;

        // Fetch the first page
        let retrieved_messages1 = ChatCompletionMessages::builder(chat_completion.id.clone())
            .credentials(credentials.clone())
            .pagination(RequestPagination {
                limit: Some(1),
                ..Default::default()
            })
            .fetch()
            .await
            .unwrap();

        assert_eq!(retrieved_messages1.data, vec![user_message]);
        assert_eq!(retrieved_messages1.has_more, false);
        assert!(retrieved_messages1.first_id.is_some());
        assert!(retrieved_messages1.last_id.is_some());

        // Fetch the second page, which should be empty
        let retrieved_messages2 = ChatCompletionMessages::builder(chat_completion.id.clone())
            .credentials(credentials.clone())
            .pagination(RequestPagination {
                limit: Some(1),
                after: Some(retrieved_messages1.first_id.unwrap()),
                ..Default::default()
            })
            .fetch()
            .await
            .unwrap();

        assert_eq!(retrieved_messages2.data, vec![]);
        assert_eq!(retrieved_messages2.has_more, false);
        assert!(retrieved_messages2.first_id.is_none());
        assert!(retrieved_messages2.last_id.is_none());
    }
}
