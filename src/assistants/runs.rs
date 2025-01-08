use derive_builder::Builder;
use either::Either;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::{assistants::Tool, chat::ToolCall, client::OpenAiClient, ApiResponseOrError};

use super::{
    messages::{Attachment, IncompleteDetails, Role},
    ResponseFormat, ToolResources,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Run {
    pub id: String,
    pub object: String,
    pub created_at: u32,
    /// The ID of the assistant used for this run.
    pub assistant_id: String,
    /// The ID of the thread associated with this run.
    pub thread_id: String,
    /// The status of the run.
    pub status: Status,
    /// Details on the action required to continue the run. Will be null if no action is required.
    pub required_action: Option<RequiredAction>,

    /// The last error that occurred during this run.
    pub last_error: Option<LastError>,

    /// The time at which the run will expire.
    pub expires_at: Option<u32>,
    /// The time at which the run was started.
    pub started_at: Option<u32>,
    /// The time at which the run was completed.
    pub completed_at: Option<u32>,
    /// The time at which the run was cancelled.
    pub cancelled_at: Option<u32>,
    /// The time at which the run was failed.
    pub failed_at: Option<u32>,
    /// The time at which the run was incomplete.
    pub incomplete_details: Option<IncompleteDetails>,

    /// The model used for this run.
    pub model: String,

    /// The instructions given to the assistant.
    pub instructions: String,

    /// The tools used for this run.
    pub tools: Vec<Tool>,

    /// The usage of the run.
    pub usage: Option<Usage>,

    /// The truncation strategy used for this run.
    pub truncation_strategy: Option<TruncationStrategy>,

    /// Whether to run tool calls in parallel.
    pub parallel_tool_calls: bool,

    /// The tool choice used for this run.
    pub tool_choice: ToolChoice,

    /// Set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information about the object in a structured format. Keys can be a maximum of 64 characters long and values can be a maximum of 512 characters long.
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Status {
    Queued,
    InProgress,
    RequiresAction,
    Cancelling,
    Cancelled,
    Failed,
    Completed,
    Incomplete,
    Expired,
}

impl Status {
    pub fn is_terminal(&self) -> bool {
        !matches!(self, Status::InProgress | Status::Queued)
    }
}

#[derive(Debug, serde_double_tag::Deserialize, serde_double_tag::Serialize, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum RequiredAction {
    SubmitToolOutputs { tool_calls: Vec<ToolCall> },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LastError {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TruncationStrategy {
    #[serde(rename = "type")]
    pub kind: String,
    pub last_messages: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(transparent)]
pub struct ToolChoice {
    #[serde(with = "either::serde_untagged")]
    pub inner: Either<ToolChoiceStrategy, ToolChoiceFunction>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ToolChoiceStrategy {
    None,
    Auto,
    Required,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ToolChoiceFunction {
    FileSearch,
    Function { name: String },
}

#[derive(Serialize, Builder, Debug, Clone, Default)]
#[builder(pattern = "owned")]
#[builder(name = "CreateThreadRunBuilder")]
#[builder(setter(strip_option, into))]
pub struct CreateThreadRunRequest {
    /// ID of the assistant to use.
    pub assistant_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub model: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub instructions: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tools: Option<Vec<Tool>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tool_resources: Option<ToolResources>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub metadata: Option<HashMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub parallel_tool_calls: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub response_format: Option<ResponseFormat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tool_choice: Option<ToolChoice>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub max_completion_tokens: Option<u32>,

    /// the thread to create
    pub thread: CreateThreadRequest,
}

#[derive(Serialize, Builder, Debug, Clone, Default)]
#[builder(pattern = "owned")]
#[builder(name = "CreateThreadBuilder")]
#[builder(setter(strip_option, into))]
pub struct CreateThreadRequest {
    pub messages: Vec<CreateThreadMessageRequest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tool_resources: Option<ToolResources>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Serialize, Builder, Debug, Clone)]
#[builder(pattern = "owned")]
#[builder(name = "CreateThreadMessageBuilder")]
#[builder(setter(strip_option, into))]
pub struct CreateThreadMessageRequest {
    pub role: Role,
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub attachments: Option<Vec<Attachment>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubmitToolOutputsRequest {
    pub tool_outputs: Vec<ToolOutput>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolOutput {
    pub tool_call_id: String,
    pub output: String,
}

#[derive(Serialize, Builder, Debug, Clone, Default)]
#[builder(pattern = "owned")]
#[builder(name = "CreateRunBuilder")]
#[builder(setter(strip_option, into))]
pub struct CreateRunRequest {
    pub assistant_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub additional_messages: Option<Vec<CreateThreadMessageRequest>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub tools: Option<Vec<Tool>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(default)]
    pub max_completion_tokens: Option<u32>,
}

impl OpenAiClient {
    pub async fn create_thread_run(
        &self,
        request: CreateThreadRunRequest,
    ) -> ApiResponseOrError<Run> {
        self.post(format!("threads/runs"), Some(request)).await
    }

    pub async fn create_run(
        &self,
        thread_id: &str,
        request: CreateRunRequest,
    ) -> ApiResponseOrError<Run> {
        self.post(format!("threads/{thread_id}/runs"), Some(request))
            .await
    }

    pub async fn poll_run(&self, mut run: Run) -> ApiResponseOrError<Run> {
        while !run.status.is_terminal() {
            run = self
                .get_run(run.thread_id.as_str(), run.id.as_str())
                .await?;
        }
        Ok(run)
    }

    pub async fn get_run(&self, thread_id: &str, run_id: &str) -> ApiResponseOrError<Run> {
        self.get(format!("threads/{thread_id}/runs/{run_id}")).await
    }

    pub async fn submit_tool_outputs_and_poll(
        &self,
        run: Run,
        request: SubmitToolOutputsRequest,
    ) -> ApiResponseOrError<Run> {
        let run: Run = self
            .post(
                format!(
                    "threads/{}/runs/{}/submit_tool_outputs",
                    run.thread_id, run.id
                ),
                Some(request),
            )
            .await?;

        self.poll_run(run).await
    }
}
