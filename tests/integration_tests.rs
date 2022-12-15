use openai::OpenAI;
use dotenv::dotenv;
use std::env;

#[tokio::test]
async fn can_list_models() {
    dotenv().expect("should load .env file");

    let key = env::var("OPENAI_KEY").expect("env var OPENAI_KEY should be defined in .env file");
    let openai = OpenAI::new(&key, None);

    openai.list_models().await.expect("should list models");
}
