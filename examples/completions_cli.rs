use dotenvy::dotenv;
use openai::{completions::Completion, set_key};
use std::{env, io::stdin};

#[tokio::main]
async fn main() {
    // Make sure you have a file named `.env` with the `OPENAI_KEY` environment variable defined!
    dotenv().unwrap();
    set_key(env::var("OPENAI_KEY").unwrap());

    loop {
        println!("Prompt:");

        let mut prompt = String::new();

        stdin().read_line(&mut prompt).unwrap();

        let completion = Completion::builder("text-davinci-003")
            .prompt(&prompt)
            .max_tokens(1024)
            .create()
            .await
            .unwrap()
            .unwrap();

        let response = &completion.choices.first().unwrap().text;

        println!("\nResponse:{response}\n");
    }
}
