use crate::{client::OpenAiClient, ApiResponseOrError};
use reqwest::{
    multipart::{Form, Part},
    Body,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct File {
    pub id: String,
    pub object: String,
    pub created_at: u32,
    pub bytes: u32,
    pub filename: String,
    pub purpose: FilePurpose,
}

#[derive(Debug, Serialize, Deserialize, Clone, strum_macros::Display)]
#[strum(serialize_all = "snake_case")]
#[serde(rename_all = "snake_case")]
pub enum FilePurpose {
    Assistants,
    AssistantsOutput,
    Batch,
    BatchOutput,
    FineTune,
    FineTuneResults,
    Vision,
}

impl OpenAiClient {
    pub async fn upload_file<B: Into<Body>>(
        &self,
        filename: &str,
        mime_type: &str,
        bytes: B,
        purpose: FilePurpose,
    ) -> ApiResponseOrError<File> {
        let file_part = Part::stream(bytes)
            .file_name(filename.to_string())
            .mime_str(mime_type)?;

        let form = Form::new()
            .part("file", file_part)
            .text("purpose", purpose.to_string());

        self.post_multipart("files", form).await
    }
}
