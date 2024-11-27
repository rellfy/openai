use dotenvy::dotenv;
use openai::{completions::Completion, Credentials};
use std::io::stdin;

#[tokio::main]
async fn main() {
    // Make sure you have a file named `.env` with the `OPENAI_KEY` environment variable defined!
    dotenv().unwrap();
    let credentials = Credentials::from_env();

    loop {
        println!("Prompt:");

        let mut prompt = String::new();

        stdin().read_line(&mut prompt).unwrap();

        let completion = Completion::builder("gpt-3.5-turbo-instruct")
            .prompt(&prompt)
            .max_tokens(1024)
            .credentials(credentials.clone())
            .create()
            .await
            .unwrap();

        let response = &completion.choices.first().unwrap().text;

        println!("\nResponse:{response}\n");
    }
}
