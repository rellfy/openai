use super::{
    requests::ChatCompletionRequest, types::*, utils::forward_deserialized_chat_response_stream,
    ChatCompletionDeltaMergeError, ChatCompletionMessageRole,
};
use crate::{
    openai_get, openai_post, openai_request_stream, ApiResponseOrError, Credentials, Usage,
};
use reqwest::Method;
use reqwest_eventsource::CannotCloneRequestError;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc::{channel, Receiver};

pub type ChatCompletion = ChatCompletionGeneric<ChatCompletionChoice>;

/// A delta chat completion, which is streamed token by token.
pub type ChatCompletionDelta = ChatCompletionGeneric<ChatCompletionChoiceDelta>;

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ChatCompletionGeneric<C> {
    #[serde(default)]
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub choices: Vec<C>,
    pub usage: Option<Usage>,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ChatCompletionChoice {
    pub index: u64,
    pub finish_reason: String,
    pub message: ChatCompletionMessage,
}

#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ChatCompletionChoiceDelta {
    pub index: u64,
    pub finish_reason: Option<String>,
    pub delta: ChatCompletionMessageDelta,
}

fn is_none_or_empty_vec<T>(opt: &Option<Vec<T>>) -> bool {
    opt.as_ref().map(|v| v.is_empty()).unwrap_or(true)
}

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Default)]
pub struct ChatCompletionMessage {
    /// The role of the author of this message.
    pub role: ChatCompletionMessageRole,
    /// The contents of the message
    ///
    /// This is always required for all messages, except for when ChatGPT calls
    /// a function.
    pub content: Option<Content>,
    /// The name of the user in a multi-user chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The function that ChatGPT called. This should be "None" usually, and is returned by ChatGPT and not provided by the developer
    ///
    /// [API Reference](https://platform.openai.com/docs/api-reference/chat/create#chat/create-function_call)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<ChatCompletionFunctionCall>,
    /// Tool call that this message is responding to.
    /// Required if the role is `Tool`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    /// Tool calls that the assistant is requesting to invoke.
    /// Can only be populated if the role is `Assistant`,
    /// otherwise it should be empty.
    #[serde(skip_serializing_if = "is_none_or_empty_vec")]
    pub tool_calls: Option<Vec<ToolCall>>,
}

/// Same as ChatCompletionMessage, but received during a response stream.
#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ChatCompletionMessageDelta {
    /// The role of the author of this message.
    pub role: Option<ChatCompletionMessageRole>,
    /// The contents of the message
    pub content: Option<Content>,
    /// The name of the user in a multi-user chat
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The function that ChatGPT called
    ///
    /// [API Reference](https://platform.openai.com/docs/api-reference/chat/create#chat/create-function_call)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_call: Option<ChatCompletionFunctionCallDelta>,
    /// Tool call that this message is responding to.
    /// Required if the role is `Tool`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_call_id: Option<String>,
    /// Tool calls that the assistant is requesting to invoke.
    /// Can only be populated if the role is `Assistant`,
    /// otherwise it should be empty.
    #[serde(skip_serializing_if = "is_none_or_empty_vec")]
    pub tool_calls: Option<Vec<ToolCall>>,
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
        if self.finish_reason.is_none() {
            if let Some(other_finish_reason) = &other.finish_reason {
                // Set finish_reason to other_finish_reason.
                self.finish_reason = Some(other_finish_reason.clone());
            }
        }
        if self.delta.name.is_none() {
            if let Some(other_name) = &other.delta.name {
                // Set name to other_name.
                self.delta.name = Some(other_name.clone());
            }
        }
        if self.delta.tool_call_id.is_none() {
            if let Some(other_tool_call_id) = &other.delta.tool_call_id {
                // Set tool_call_id to other_tool_call_id.
                self.delta.tool_call_id = Some(other_tool_call_id.clone());
            }
        }

        // Merge contents.
        match self.delta.content.as_mut() {
            Some(content) => {
                match &other.delta.content {
                    Some(other_content) => {
                        // Push other content into this one.
                        // TODO 这边要添加完整的合并逻辑
                        if let Content::Str(content) = content {
                            if let Content::Str(other_content) = other_content {
                                content.push_str(other_content);
                            }
                        }
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

        // merge tools
        match self.delta.tool_calls.as_mut() {
            Some(tool_calls) => {
                if let Some(other_tool_calls) = &other.delta.tool_calls {
                    tool_calls.iter_mut().zip(other_tool_calls).for_each(
                        |(tool_call, other_tool_call)| {
                            tool_call.merge(other_tool_call);
                        },
                    );
                }
            }
            None => {
                match &other.delta.tool_calls {
                    Some(other_tool_calls) => {
                        // Set this content to other content.
                        self.delta.tool_calls = Some(other_tool_calls.clone());
                    }
                    None => {}
                }
            }
        };
        Ok(())
    }
}

impl From<ChatCompletionMessageDelta> for ChatCompletionMessage {
    fn from(value: ChatCompletionMessageDelta) -> ChatCompletionMessage {
        ChatCompletionMessage {
            role: value.role.unwrap_or(ChatCompletionMessageRole::Assistant),
            content: value.content,
            name: value.name,
            function_call: value.function_call.map(ChatCompletionFunctionCall::from),
            tool_call_id: value.tool_call_id,
            tool_calls: value.tool_calls,
        }
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
                    message: choice.delta.clone().into(),
                })
                .collect(),
        }
    }
}

impl ChatCompletion {
    pub async fn create(request: ChatCompletionRequest) -> ApiResponseOrError<Self> {
        let credentials_opt = request.credentials.clone();
        openai_post("chat/completions", &request, credentials_opt).await
    }

    /// Get a stored completion.
    pub async fn get(id: &str, credentials: Credentials) -> ApiResponseOrError<Self> {
        let route = format!("chat/completions/{}", id);
        openai_get(route.as_str(), Some(credentials)).await
    }
}

impl ChatCompletionDelta {
    pub async fn create(
        request: ChatCompletionRequest,
    ) -> Result<Receiver<Self>, CannotCloneRequestError> {
        let credentials_opt = request.credentials.clone();
        let stream = openai_request_stream(
            Method::POST,
            "chat/completions",
            move |r| r.json(&request),
            credentials_opt,
        )
        .await?;
        let (tx, rx) = channel::<Self>(32);
        tokio::spawn(forward_deserialized_chat_response_stream(stream, tx));
        Ok(rx)
    }
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

/// A list of messages for a chat completion.
#[derive(Deserialize, Clone, Debug, Eq, PartialEq)]
pub struct ChatCompletionMessages {
    pub data: Vec<ChatCompletionMessage>,
    pub object: String,
    pub first_id: Option<String>,
    pub last_id: Option<String>,
    pub has_more: bool,
}

fn clone_default_unwrapped_option_string(string: &Option<String>) -> String {
    match string {
        Some(value) => value.clone(),
        None => "".to_string(),
    }
}
