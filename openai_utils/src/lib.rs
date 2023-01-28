pub const BASE_URL: &str = "https://api.openai.com/v1";

#[macro_export]
macro_rules! authorization {
    ($request:expr) => {
        {
            use std::env;
            use reqwest::{ RequestBuilder, header::AUTHORIZATION };
            use dotenvy::dotenv;

            dotenv().ok();

            let token = env::var("OPENAI_KEY")
                .expect("environment variable `OPENAI_KEY` should be defined");

            $request.header(AUTHORIZATION, format!("Bearer {token}"))
        }
    };
}
