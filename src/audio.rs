//! Given an audio file, the model will return its transcription.

use std::path::Path;

use super::{openai_post_multipart, ApiResponseOrError};
use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use reqwest::multipart::{Form, Part};

#[derive(Deserialize, Clone)]
pub struct Transcription {
    pub text: String,
}

#[derive(Serialize, Builder, Debug, Clone)]
#[builder(pattern = "owned")]
#[builder(name = "TranscriptionBuilder")]
#[builder(setter(strip_option, into))]
pub struct TranscriptionRequest {
    /// ID of the model to use.
    /// You can use the [List models](https://beta.openai.com/docs/api-reference/models/list)
    /// API to see all of your available models,
    /// or see our [Model overview](https://beta.openai.com/docs/models/overview)
    /// for descriptions of them.
    /// At time of writing, only "whisper-1" is allowed.
    pub model: String,
    pub file_name: String,
}

impl Transcription {
    /// Creates a completion for the provided prompt and parameters
    async fn create(request: &TranscriptionRequest) -> ApiResponseOrError<Self> {
        let model = request.model.clone();
        let upload_file_path = Path::new(request.file_name.as_str());
        let upload_file_path = upload_file_path.canonicalize()?;
        let simple_name = upload_file_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .clone();
        let async_file = tokio::fs::File::open(upload_file_path).await?;
        let file_part = Part::stream(async_file)
            .file_name(simple_name)
            .mime_str("audio/wav")?;
        let form = Form::new()
            .part("file", file_part)
            .text("model", model);
        openai_post_multipart("audio/transcriptions", form).await
    }

    pub fn builder(model: &str) -> TranscriptionBuilder {
        TranscriptionBuilder::create_empty().model(model)
    }
}

impl TranscriptionBuilder {
    pub async fn create(self) -> ApiResponseOrError<Transcription> {
        Transcription::create(&self.build().unwrap()).await
    }
}
