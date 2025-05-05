use dotenvy::dotenv;
use openai::chat::{ChatCompletion, ChatCompletionDelta, Content};
use openai::{
    chat::{ChatCompletionMessage, ChatCompletionMessageRole},
    Credentials,
};
use std::io::{stdin, stdout, Write};
use tokio::sync::mpsc::{error::TryRecvError, Receiver};

#[tokio::main]
async fn main() {
    // Make sure you have a file named `.env` with the `OPENAI_KEY` environment variable defined!
    dotenv().unwrap();
    let credentials = Credentials::from_env();

    let mut messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content: Some(Content::new_str(
            "You're an AI that replies to each message verbosely.",
        )),
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

        let chat_stream = ChatCompletionDelta::builder("gpt-3.5-turbo", messages.clone())
            .credentials(credentials.clone())
            .create_stream()
            .await
            .unwrap();

        let chat_completion: ChatCompletion = listen_for_tokens(chat_stream).await;
        let returned_message = chat_completion.choices.first().unwrap().message.clone();

        messages.push(returned_message);
    }
}

async fn listen_for_tokens(mut chat_stream: Receiver<ChatCompletionDelta>) -> ChatCompletion {
    let mut merged: Option<ChatCompletionDelta> = None;
    loop {
        match chat_stream.try_recv() {
            Ok(delta) => {
                let choice = &delta.choices[0];
                if let Some(role) = &choice.delta.role {
                    print!("{:#?}: ", role);
                }
                if let Some(content) = &choice.delta.content {
                    print!("{}", content);
                }
                stdout().flush().unwrap();
                // Merge token into full completion.
                match merged.as_mut() {
                    Some(c) => {
                        c.merge(delta).unwrap();
                    }
                    None => merged = Some(delta),
                };
            }
            Err(TryRecvError::Empty) => {
                let duration = std::time::Duration::from_millis(50);
                tokio::time::sleep(duration).await;
            }
            Err(TryRecvError::Disconnected) => {
                break;
            }
        };
    }
    println!();
    merged.unwrap().into()
}
