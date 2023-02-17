pub const BASE_URL: &str = "https://api.openai.com/v1";

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
