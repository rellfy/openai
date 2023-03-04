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

use serde::Deserialize;

pub const BASE_URL: &str = "https://api.openai.com/v1/";

#[macro_export]
macro_rules! authorization {
    ($request:expr) => {{
        use dotenvy::dotenv;
        use reqwest::{header::AUTHORIZATION, RequestBuilder};
        use std::env;

        dotenv().ok();

        let token =
            env::var("OPENAI_KEY").expect("environment variable `OPENAI_KEY` should be defined");

        $request.header(AUTHORIZATION, format!("Bearer {token}"))
    }};
}

#[derive(Deserialize, Debug)]
pub struct OpenAiError {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: String,
    pub param: Option<String>,
    pub code: Option<String>,
}

impl std::fmt::Display for OpenAiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for OpenAiError {}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum ApiResponse<T> {
    Ok(T),
    Err { error: OpenAiError },
}
