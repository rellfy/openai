use std::str::FromStr;

use crate::{ApiResponseOrError, Credentials, OpenAiError, DEFAULT_CREDENTIALS};
use anyhow::Result;
use reqwest::{
    header::{HeaderName, HeaderValue, AUTHORIZATION},
    Client, Method, Response,
};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Clone)]
pub struct OpenAiClient {
    credentials: Credentials,
    client: Client,
}

impl std::fmt::Debug for OpenAiClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "OpenAiClient")
    }
}

#[derive(Debug, Clone, Deserialize)]
struct OpenAiErrorWrapper {
    error: OpenAiError,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Empty {}

impl OpenAiClient {
    pub fn default() -> Result<Self> {
        Self::new(DEFAULT_CREDENTIALS.read().unwrap().clone())
    }

    pub fn new(credentials: Credentials) -> Result<Self> {
        let client = Client::builder()
            .default_headers(
                [
                    (
                        AUTHORIZATION,
                        HeaderValue::from_str(&format!("Bearer {}", credentials.api_key))?,
                    ),
                    (
                        HeaderName::from_str("OpenAI-Beta")?,
                        HeaderValue::from_str("assistants=v2")?,
                    ),
                ]
                .into_iter()
                .collect(),
            )
            .build()?;

        Ok(Self {
            credentials,
            client,
        })
    }

    async fn request_inner<S, R>(
        &self,
        method: Method,
        route: R,
        body: Option<S>,
    ) -> Result<Response, reqwest::Error>
    where
        R: Into<String>,
        S: Serialize,
    {
        let url = format!("{}{}", self.credentials.base_url, route.into());
        log::debug!("OpenAI Request[{}] {}", method.to_string(), url);

        let mut request = self.client.request(method.clone(), url.clone());

        if let Some(body) = body {
            request = request.json(&body);
        }

        let response = request.send().await?;

        log::debug!(
            "OpenAI Response[{}] {} {url}",
            method.to_string(),
            response.status().as_str()
        );
        Ok(response)
    }

    pub async fn request<S, R, T>(
        &self,
        method: Method,
        route: R,
        body: Option<S>,
    ) -> ApiResponseOrError<T>
    where
        R: Into<String>,
        S: Serialize,
        T: DeserializeOwned,
    {
        let response = self.request_inner(method, route, body).await?;
        let api_response = if response.status().is_success() {
            response.json::<T>().await?
        } else {
            let result = response.text().await?;
            if let Ok(api_response) = serde_json::from_str::<OpenAiErrorWrapper>(&result) {
                return Err(api_response.error);
            } else {
                return Err(OpenAiError::new(result, "unknown".to_string()));
            }
        };

        Ok(api_response)
    }
    pub async fn get<R, T>(&self, route: R) -> ApiResponseOrError<T>
    where
        R: Into<String>,
        T: DeserializeOwned,
    {
        self.request::<(), R, T>(Method::GET, route, None).await
    }

    pub async fn post<S, R, T>(&self, route: R, body: S) -> ApiResponseOrError<T>
    where
        R: Into<String>,
        S: Serialize,
        T: DeserializeOwned,
    {
        self.request(Method::POST, route, Some(body)).await
    }

    pub async fn delete<R>(&self, route: R) -> ApiResponseOrError<Empty>
    where
        R: Into<String>,
    {
        self.request::<(), R, Empty>(Method::DELETE, route, None)
            .await
    }

    pub async fn list<R, T>(&self, route: R, after: Option<String>) -> ApiResponseOrError<Vec<T>>
    where
        R: Into<String>,
        T: DeserializeOwned + std::fmt::Debug,
    {
        let mut route = if let Some(after) = after {
            format!("{}?order=asc&after={after}", route.into())
        } else {
            format!("{}?order=asc", route.into())
        };

        let mut has_more = true;
        let mut data = Vec::new();

        while has_more {
            let list: List<T> = self.get(&route).await?;
            data.extend(list.data);
            has_more = list.has_more;
            route = format!(
                "{route}?order=asc&after={}",
                list.last_id.unwrap_or_default()
            );
        }

        Ok(data)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct List<T> {
    pub first_id: Option<String>,
    pub last_id: Option<String>,
    pub data: Vec<T>,
    pub has_more: bool,
}
