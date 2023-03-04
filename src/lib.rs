// Copyright (C) 2023  Valentine Briese
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use openai_bootstrap::{authorization, ApiResponse, OpenAiError, BASE_URL};
use reqwest::{Client, Method, RequestBuilder};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod chat;
pub mod completions;
pub mod edits;
pub mod embeddings;
pub mod models;

#[derive(Deserialize)]
pub struct Usage {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

type ApiResponseOrError<T> = Result<Result<T, OpenAiError>, reqwest::Error>;

async fn openai_request<F, T>(method: Method, route: &str, builder: F) -> ApiResponseOrError<T>
where
    F: FnOnce(RequestBuilder) -> RequestBuilder,
    T: DeserializeOwned,
{
    let client = Client::new();
    let mut request = client.request(method, BASE_URL.to_owned() + route);

    request = builder(request);

    let api_response: ApiResponse<T> = authorization!(request).send().await?.json().await?;

    match api_response {
        ApiResponse::Ok(t) => Ok(Ok(t)),
        ApiResponse::Err { error } => Ok(Err(error)),
    }
}

async fn openai_get<T>(route: &str) -> ApiResponseOrError<T>
where
    T: DeserializeOwned,
{
    openai_request(Method::GET, route, |request| request).await
}

async fn openai_post<J, T>(route: &str, json: &J) -> ApiResponseOrError<T>
where
    J: Serialize + ?Sized,
    T: DeserializeOwned,
{
    openai_request(Method::POST, route, |request| request.json(json)).await
}
