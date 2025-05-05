use dotenvy::dotenv;
use openai::{
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole, Content},
    Credentials,
};
use std::io::{stdin, stdout, Write};

#[tokio::main]
async fn main() {
    // Make sure you have a file named `.env` with the `OPENAI_KEY` environment variable defined!
    dotenv().unwrap();
    let credentials = Credentials::from_env();

    let mut messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content: Some(Content::new_str("You are a large language model built into a command line interface as an example of what the `openai` Rust library made by Valentine Briese can do.")),
        ..Default::default()
    }];

    loop {
        print!("User: ");
        stdout().flush().unwrap();

        let mut user_message_content = String::new();

        stdin().read_line(&mut user_message_content).unwrap();
        messages.push(ChatCompletionMessage {
            role: ChatCompletionMessageRole::User,
            content: Some(Content::new_str(&user_message_content)),
            ..Default::default()
        });

        let chat_completion = ChatCompletion::builder("gpt-3.5-turbo", messages.clone())
            .credentials(credentials.clone())
            .create()
            .await
            .unwrap();
        let returned_message = chat_completion.choices.first().unwrap().message.clone();

        println!(
            "{:#?}: {}",
            &returned_message.role,
            &returned_message.content.clone().unwrap()
        );

        messages.push(returned_message);
    }
}
