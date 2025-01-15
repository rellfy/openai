use std::collections::HashMap;

use crate::{client::OpenAiClient, ApiResponseOrError};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VectorStore {
    pub id: String,
    pub object: String,
    pub created_at: u32,
    pub name: String,
    pub usage_bytes: u32,
    pub file_counts: FileCounts,
    pub status: VectorStoreStatus,
    pub expires_after: Option<ExpiresAfter>,
    pub expires_at: Option<u32>,
    pub last_active_at: Option<u32>,
    pub metadata: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileCounts {
    pub in_progress: u32,
    pub completed: u32,
    pub failed: u32,
    pub cancelled: u32,
    pub total: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, strum_macros::Display)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum VectorStoreStatus {
    Expired,
    InProgress,
    Completed,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExpiresAfter {
    pub anchor: String,
    pub days: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct CreateVectorStoreRequest {
    pub name: String,
    pub file_ids: Option<Vec<String>>,
    pub metadata: Option<HashMap<String, String>>,
    pub expires_after: Option<ExpiresAfter>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VectorStoreFile {
    pub id: String,
    pub object: String,
    pub created_at: u32,
    pub file_id: String,
    pub vector_store_id: String,
    pub usage_bytes: u32,
    pub status: VectorStoreFileStatus,
    pub last_error: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize, Clone, strum_macros::Display)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum VectorStoreFileStatus {
    InProgress,
    Completed,
    Cancelled,
    Failed,
}

impl OpenAiClient {
    pub async fn create_vector_store(
        &self,
        params: CreateVectorStoreRequest,
    ) -> ApiResponseOrError<VectorStore> {
        self.post("vector_stores", params).await
    }

    pub async fn attach_file_to_vector_store(
        &self,
        vector_store_id: &str,
        file_id: &str,
    ) -> ApiResponseOrError<VectorStoreFile> {
        self.post(
            &format!("vector_stores/{}/files", vector_store_id),
            json!({ file_id: file_id }),
        )
        .await
    }
}
