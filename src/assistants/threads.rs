use serde::Deserialize;
use std::collections::HashMap;

use crate::assistants::ToolResources;

#[derive(Debug, Deserialize, Clone)]
pub struct Thread {
    pub id: String,
    pub object: String,
    pub created_at: u32,
    /// A set of resources that are used by the assistant's tools. The resources are specific to the type of tool. For example, the code_interpreter tool requires a list of file IDs, while the file_search tool requires a list of vector store IDs.
    pub tool_resources: Option<ToolResources>,
    /// Set of 16 key-value pairs that can be attached to an object. This can be useful for storing additional information about the object in a structured format. Keys can be a maximum of 64 characters long and values can be a maximum of 512 characters long.
    pub metadata: Option<HashMap<String, String>>,
}
