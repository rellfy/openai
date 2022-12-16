# openai
An unofficial OpenAI API library for Rust.

> ## Currently on v0
> Not yet stable enough to be used in any production applications.

## Implementation Progress
 - [x] Models
 - [ ] Completions
 - [ ] Edits
 - [ ] Images
 - [ ] Embeddings
 - [ ] Files
 - [ ] Fine-tunes
 - [ ] Moderations

## Example Usage
```rs
use openai::OpenAI;
use dotenv::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().expect("should load .env file");

    let key = env::var("OPENAI_KEY").expect("env var OPENAI_KEY should be defined in .env file");
    let openai = OpenAI::new(&key, None);

    dbg!(openai.list_models().await.expect("should list models"));
}
```
