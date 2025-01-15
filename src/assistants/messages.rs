use crate::{assistants::Tool, client::OpenAiClient, ApiResponseOrError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    pub id: String,
    pub object: String,
    pub created_at: u32,
    /// The thread ID that this message belongs to.
    pub thread_id: String,
    /// The status of the message, which can be either in_progress, incomplete, or completed.
    pub status: Option<String>,
    /// On an incomplete message, details about why the message is incomplete.
    pub incomplete_details: Option<IncompleteDetails>,
    /// The Unix timestamp (in seconds) for when the message was completed.
    pub completed_at: Option<u32>,
    /// The Unix timestamp (in seconds) for when the message was marked as incomplete.
    pub incomplete_at: Option<u32>,
    /// The entity that produced the message. One of user or assistant
    pub role: Role,
    /// The content of the message.
    pub content: Vec<Content>,
    /// The assistant that produced the message.
    pub assistant_id: Option<String>,
    /// The ID of the run associated with the creation of this message. Value is null when messages are created manually using the create message or create thread endpoints.
    pub run_id: Option<String>,
    /// A list of files attached to the message.
    pub attachments: Option<Vec<Attachment>>,
    /// A set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information about the object in a structured format. Keys can be a maximum of 64 characters long and values can be a maximum of 512 characters long.
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    InProgress,
    Incomplete,
    Completed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct IncompleteDetails {
    pub reason: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    User,
    Assistant,
}

#[derive(Debug, serde_double_tag::Serialize, serde_double_tag::Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum Content {
    Text(Text),
    ImageFile(ImageFile),
    ImageUrl(ImageUrl),
    Refusal(Refusal),
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Text {
    pub value: String,
    pub annotations: Vec<Annotation>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Annotation {
    #[serde(rename = "type")]
    pub kind: String,
    pub text: String,
    pub start_index: u32,
    pub end_index: u32,
    pub file_citation: FileCitation,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileCitation {
    pub file_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageFile {
    pub file_id: String,
    pub detail: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageUrl {
    pub image_url: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Refusal {
    pub refusal: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Attachment {
    pub file_id: String,
    pub tools: Tool,
}

impl OpenAiClient {
    pub async fn list_messages(
        &self,
        thread_id: &str,
        after_id: Option<String>,
    ) -> ApiResponseOrError<Vec<Message>> {
        self.list(format!("threads/{thread_id}/messages"), after_id)
            .await
    }
}
