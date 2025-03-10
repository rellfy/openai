use dotenvy::dotenv;
use openai::{
    chat::{ChatCompletion, ChatMessage},
    Credentials,
};

#[tokio::main]
async fn main() {
    // Make sure you have a file named `.env` with the `OPENAI_KEY` environment variable defined!
    dotenv().unwrap();
    // Relies on OPENAI_KEY and optionally OPENAI_BASE_URL.
    let credentials = Credentials::from_env();
    let messages = vec![
        ChatMessage::System {
            content: "You are a helpful assistant.".to_string().into(),
            name: None,
        },
        ChatMessage::User {
            content: "Tell me a random crab fact".to_string().into(),
            name: None,
        },
    ];
    let chat_completion = ChatCompletion::builder("gpt-4o", messages.clone())
        .credentials(credentials.clone())
        .create()
        .await
        .unwrap();
    let returned_message = chat_completion.choices.first().unwrap().message.clone();
    // Assistant: Sure! Here's a random crab fact: Crabs communicate with each other by drumming or waving their pincers.
    println!("Assistant: {}", returned_message.content.unwrap().trim());
}
