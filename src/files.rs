use std::path::{Path, PathBuf};

use derive_builder::Builder;
use reqwest::multipart::{Form, Part};
use serde::{Deserialize, Serialize};

use crate::{openai_delete, openai_get, openai_post_multipart, OpenAiError};

use super::ApiResponseOrError;

#[derive(Deserialize, Serialize, Clone)]
pub struct File {
    pub id: String,
    pub object: String,
    pub bytes: usize,
    pub created_at: usize,
    pub filename: String,
    pub purpose: String,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct DeletedFile {
    pub id: String,
    pub object: String,
    pub deleted: bool,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Files {
    data: Vec<File>,
    object: String,
}

#[derive(Serialize, Builder, Debug, Clone)]
#[builder(pattern = "owned")]
#[builder(name = "FileUploadBuilder")]
#[builder(setter(strip_option, into))]
pub struct FileUploadRequest {
    file_name: String,
    purpose: String,
}

impl File {
    async fn create(request: &FileUploadRequest) -> ApiResponseOrError<Self> {
        let purpose = request.purpose.clone();
        let upload_file_path = Path::new(request.file_name.as_str());
        let upload_file_path = upload_file_path.canonicalize().unwrap();
        if !upload_file_path.exists() {
            return Ok(Err(file_not_found_error(&upload_file_path)));
        }
        let simple_name = upload_file_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
            .clone();
        let async_file = match tokio::fs::File::open(upload_file_path).await {
            Ok(f) => f,
            Err(e) => {
                return Ok(Err(io_error(e)));
            }
        };
        let file_part = Part::stream(async_file)
            .file_name(simple_name)
            .mime_str("application/jsonl")?;
        let form = Form::new().part("file", file_part).text("purpose", purpose);
        openai_post_multipart("files", form).await
    }

    pub fn builder() -> FileUploadBuilder {
        FileUploadBuilder::create_empty()
    }

    pub async fn delete(id: &str) -> ApiResponseOrError<DeletedFile> {
        openai_delete(format!("files/{}", id).as_str()).await
    }
}

fn file_not_found_error(file_path: &PathBuf) -> OpenAiError {
    OpenAiError {
        message: format!("File {} not found", file_path.display()),
        error_type: "internal".to_string(),
        param: None,
        code: None,
    }
}

fn io_error(err: std::io::Error) -> OpenAiError {
    OpenAiError {
        message: format!("IO Error {}", err.to_string()),
        error_type: "internal".to_string(),
        param: None,
        code: None,
    }
}

impl FileUploadBuilder {
    pub async fn create(self) -> ApiResponseOrError<File> {
        File::create(&self.build().unwrap()).await
    }
}

impl Files {
    pub async fn list() -> ApiResponseOrError<Files> {
        openai_get("files").await
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::time::Duration;

    use dotenvy::dotenv;

    use crate::set_key;

    use super::*;

    fn test_upload_builder() -> FileUploadBuilder {
        File::builder()
            .file_name("test_data/file_upload_test1.jsonl")
            .purpose("fine-tune")
    }

    fn test_upload_request() -> FileUploadRequest {
        test_upload_builder().build().unwrap()
    }

    #[tokio::test]
    async fn upload_file() {
        dotenv().ok();
        set_key(env::var("OPENAI_KEY").unwrap());
        let file_upload = test_upload_builder().create().await.unwrap().unwrap();
        println!(
            "upload: {}",
            serde_json::to_string_pretty(&file_upload).unwrap()
        );
        assert_eq!(file_upload.id.as_bytes()[..5], *"file-".as_bytes())
    }

    #[tokio::test]
    async fn list_files() {
        dotenv().ok();
        set_key(env::var("OPENAI_KEY").unwrap());
        // ensure at least one file exists
        test_upload_builder().create().await.unwrap().unwrap();
        let openai_files = Files::list().await.unwrap().unwrap();
        let file_count = openai_files.data.len();
        assert!(file_count > 0);
        for openai_file in &openai_files.data {
            assert_eq!(openai_file.id.as_bytes()[..5], *"file-".as_bytes())
        }
        println!(
            "files [{}]: {}",
            file_count,
            serde_json::to_string_pretty(&openai_files).unwrap()
        );
    }

    #[tokio::test]
    async fn delete_files() {
        dotenv().ok();
        set_key(env::var("OPENAI_KEY").unwrap());
        // ensure at least one file exists
        test_upload_builder().create().await.unwrap().unwrap();
        // wait to avoid recent upload still processing error
        tokio::time::sleep(Duration::from_secs(5)).await;
        let openai_files = Files::list().await.unwrap().unwrap();
        assert!(openai_files.data.len() > 0);
        let mut files = openai_files.data;
        files.sort_by(|a, b| a.created_at.cmp(&b.created_at));
        for file in files {
            let deleted_file = File::delete(file.id.as_str()).await.unwrap().unwrap();
            assert!(deleted_file.deleted);
            println!("deleted: {} {}", deleted_file.id, deleted_file.deleted)
        }
    }

    #[test]
    fn file_name_path_test() {
        let request = test_upload_request();
        let file_upload_path = Path::new(request.file_name.as_str());
        let file_name = file_upload_path
            .clone()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap();
        assert_eq!(file_name, "file_upload_test1.jsonl");
        let file_upload_path = file_upload_path.canonicalize().unwrap();
        let file_exists = file_upload_path.exists();
        assert!(file_exists)
    }
}
