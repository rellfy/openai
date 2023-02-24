use dotenvy::dotenv;
use openai::{
    completions::{Completion, CreateCompletionRequestBody},
    models::ModelID,
};
use std::io::stdin;

#[tokio::main]
async fn main() {
    // Make sure you have a file named `.env` with the `OPENAI_KEY` environment variable defined!
    dotenv().unwrap();

    loop {
        println!("Prompt:");

        let mut prompt = String::new();

        stdin().read_line(&mut prompt).unwrap();

        let completion = Completion::new(&CreateCompletionRequestBody {
            model: ModelID::TextDavinci003,
            prompt: &prompt,
            max_tokens: Some(1024),
            ..Default::default()
        })
        .await
        .unwrap()
        .unwrap();

        let response = &completion.choices.first().unwrap().text;

        println!("\nResponse:{response}\n");
    }
}
