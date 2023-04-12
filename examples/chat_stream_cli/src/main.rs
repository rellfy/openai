use dotenvy::dotenv;
use openai::chat::ChatCompletionDelta;
use openai::{
    chat::{ChatCompletionGeneric, ChatCompletionMessage, ChatCompletionMessageRole},
    set_key, StreamError,
};
use std::{
    env,
    io::{stdin, stdout, Write},
};
use tokio::sync::mpsc::Receiver;

#[tokio::main]
async fn main() -> Result<(), StreamError> {
    // Make sure you have a file named `.env` with the `OPENAI_KEY` environment variable defined!
    dotenv().unwrap();
    set_key(env::var("OPENAI_KEY").unwrap());

    let mut messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content: "You're an AI that replies to each message verbosely.".to_string(),
        name: None,
    }];

    loop {
        print!("User: ");
        stdout().flush().unwrap();

        let mut user_message_content = String::new();

        stdin().read_line(&mut user_message_content).unwrap();
        messages.push(ChatCompletionMessage {
            role: ChatCompletionMessageRole::User,
            content: user_message_content,
            name: None,
        });

        let (mut chat_stream, listener) =
            ChatCompletionDelta::builder("gpt-3.5-turbo", messages.clone())
                .create_stream()
                .await?;

        tokio::join!(listen_for_tokens(chat_stream), listener);

        // messages.push(returned_message);
    }
}

async fn listen_for_tokens(mut chat_stream: Receiver<ChatCompletionDelta>) {
    while let Some(completion) = chat_stream.recv().await {
        let choice = &completion.choices[0];
        if let Some(role) = &choice.delta.role {
            print!("{:#?}: ", role);
        }
        if let Some(content) = &choice.delta.content {
            print!("{}", content);
        }
        if let Some(finish_reason) = &choice.finish_reason {
            // The message being streamed has been fully received.
            print!("\n");
        }
        stdout().flush();
    }
}
