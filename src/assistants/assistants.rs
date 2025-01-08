use std::collections::HashMap;

use schemars::schema::RootSchema;
use serde::{Deserialize, Serialize};

use crate::{client::{Empty, OpenAiClient}, ApiResponseOrError};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Assistant {
    pub id: String,
    pub object: String,
    pub created_at: u32,
    /// The name of the assistant. The maximum length is 256 characters.
    pub name: Option<String>,
    /// ID of the model to use. You can use the List models API to see all of your available models, or see our Model overview for descriptions of them.
    pub model: String,
    /// The system instructions that the assistant uses. The maximum length is 256,000 characters.
    pub instructions: Option<String>,
    pub tools: Vec<Tool>,
    /// A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the code_interpreter tool requires a list of file IDs, while the file_search tool requires a list of vector store IDs.
    pub tool_resources: Option<ToolResources>,
    /// Set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information about the object in a structured format. Keys can be a maximum of 64 characters long and values can be a maximum of 512 characters long.
    pub metadata: Option<HashMap<String, String>>,
    /// The default model to use for this assistant.
    pub response_format: Option<ResponseFormat>,
}

#[derive(Debug, Clone, serde_double_tag::Deserialize, serde_double_tag::Serialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum Tool {
    CodeInterpreter,
    Function(Function),
    FileSearch(FileSearch),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Function {
    pub name: String,
    pub description: String,
    pub parameters: RootSchema,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionParameters {
    pub title: String,
    pub description: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub required: Vec<String>,
    pub properties: HashMap<String, FunctionProperty>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FunctionProperty {
    pub description: String,
    #[serde(rename = "type")]
    pub type_: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileSearch {
    pub max_num_results: usize,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolResources {
    pub code_interpreter: Option<CodeInterpreterResources>,
    pub file_search: Option<FileSearchResources>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CodeInterpreterResources {
    /// A list of file IDs made available to the `code_interpreter`` tool. There can be a maximum of 20 files associated with the tool.
    pub file_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileSearchResources {
    /// The ID of the vector store attached to this assistant. There can be a maximum of 1 vector store attached to the assistant.
    pub vector_store_ids: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ResponseFormat {
    Auto,
}

#[derive(Serialize, Default, Debug, Clone)]
pub struct CreateAssistantRequest {
    /// ID of the model to use. You can use the List models API to see all of your available models, or see our Model overview for descriptions of them.
    pub model: String,

    /// The name of the assistant. The maximum length is 256 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The description of the assistant. The maximum length is 256 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The system instructions that the assistant uses. The maximum length is 256,000 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// A set of tools that the assistant can use.
    pub tools: Vec<Tool>,
    /// A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the code_interpreter tool requires a list of file IDs, while the file_search tool requires a list of vector store IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_resources: Option<ToolResources>,
    /// Set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information about the object in a structured format. Keys can be a maximum of 64 characters long and values can be a maximum of 512 characters long.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    /// The default model to use for this assistant.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_format: Option<ResponseFormat>,
}

impl OpenAiClient {
    pub async fn create_assistant(
        &self,
        request: CreateAssistantRequest,
    ) -> ApiResponseOrError<Assistant> {
        self.post("assistants", Some(request)).await
    }

    pub async fn get_assistant(&self, assistant_id: &str) -> ApiResponseOrError<Assistant> {
        self.get(format!("assistants/{}", assistant_id)).await
    }

    pub async fn delete_assistant(&self, assistant_id: &str) -> ApiResponseOrError<Empty> {
        self.delete(format!("assistants/{}", assistant_id)).await
    }

    pub async fn update_assistant(
        &self,
        assistant_id: &str,
        request: CreateAssistantRequest,
    ) -> ApiResponseOrError<Assistant> {
        self.post(format!("assistants/{}", assistant_id), Some(request))
            .await
    }
}
