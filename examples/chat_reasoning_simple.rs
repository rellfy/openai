use std::io::{Write, stdout};

use dotenvy::dotenv;
use openai::{
    Credentials,
    chat::{ChatCompletion, ChatCompletionDelta, ChatCompletionMessage, ChatCompletionMessageRole},
};
use tokio::sync::mpsc::{Receiver, error::TryRecvError};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    dotenv().unwrap();
    let credentials = Credentials::from_env();
    let mut messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content: Some("You're an AI that replies to each message verbosely.".to_string()),
        ..Default::default()
    }];

    stdout().flush().unwrap();

    let  user_message_content = "what tools do you have?".to_string();

    messages.push(ChatCompletionMessage {
        role: ChatCompletionMessageRole::User,
        content: Some(user_message_content),
        ..Default::default()
    });

    let chat_stream = ChatCompletionDelta::builder("qwen3.5-plus", messages.clone())
        .credentials(credentials.clone())
        .create_stream()
        .await
        .unwrap();

    let chat_completion: ChatCompletion = listen_for_tokens(chat_stream).await;
    let returned_message = chat_completion.choices.first().unwrap().message.clone();

    messages.push(returned_message);
}

async fn listen_for_tokens(mut chat_stream: Receiver<ChatCompletionDelta>) -> ChatCompletion {
    let mut merged: Option<ChatCompletionDelta> = None;
    let mut thingking = false;
    loop {
        match chat_stream.try_recv() {
            Ok(delta) => {
                let choice = &delta.choices[0];

                if let Some(role) = &choice.delta.role {
                    print!("{:#?}: ", role);
                }
                if thingking == false && choice.delta.reasoning_content.is_some() {
                    thingking = true;
                    print!("🤔 -> \n");
                }
                if thingking == true && choice.delta.reasoning_content.is_none() {
                    thingking = false;
                    print!("\n😄 -> \n");
                }
                if let Some(content) = &choice.delta.content {
                    print!("{}", content);
                }
                if let Some(reason_content) = &choice.delta.reasoning_content {
                    print!("{}", reason_content);
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
