use dotenvy::dotenv;
use openai::chat::{
    ChatCompletion, ChatCompletionDelta, ChatCompletionFunctionDefinition, ChatCompletionMessage,
    ChatCompletionMessageRole, ChatCompletionToolDefinition, Content, ToolCall, ToolCallFunction,
    ToolChoiceMode,
};
use openai::new_content;
use openai::Credentials;
use std::io::{stdin, stdout, Write};
use tokio::sync::mpsc::error::TryRecvError;
use tokio::sync::mpsc::Receiver;
#[tokio::main]
async fn main() {
    dotenv().ok();
    let credentials = Credentials::from_env();
    let mut message = vec![ChatCompletionMessage{
        role: ChatCompletionMessageRole::System,
        content: Some(new_content!("你是一个善于rust语言编程的优秀的ai助手，而且你可以调用外部的各种工具，你可以利用这些工具帮助用户解决问题")),
        ..Default::default()
    }];
    loop {
        print!("User: ");
        stdout().flush().unwrap();
        let mut user_message = String::new();
        stdin()
            .read_line(&mut user_message)
            .expect("Failed to read line");
        message.push(ChatCompletionMessage {
            content: Some(new_content!(user_message)),
            ..Default::default()
        });

        let chat_stream = ChatCompletion::builder("gemini-2.0-flash", message.clone())
            .credentials(credentials.clone())
            .tools([
                ChatCompletionToolDefinition::Function {
                    function: ChatCompletionFunctionDefinition {
                        name: "find_file".to_string(),
                        description: Some("用来根据pattern模式查询文件， 例如'/*.txt'".to_string()),
                        parameters: Some(serde_json::json!({
                            "type": "object",
                            "properties": {
                                "pattern": {
                                    "type": "string",
                                    "description": "用来匹配文件的模式"
                                }
                            },
                            "required": ["pattern"]
                        })),
                    },
                },
                ChatCompletionToolDefinition::Function {
                    function: ChatCompletionFunctionDefinition {
                        name: "get_current_time".to_string(),
                        description: Some("获取当前时间".to_string()),
                        parameters: Some(serde_json::json!({
                            "type": "object",
                            "properties": {},
                            "required": []
                        })),
                    },
                },
            ])
            .tool_choice(ToolChoiceMode::Auto)
            .create_stream()
            .await
            .unwrap();
        let chat_completion = listen_for_tokens(chat_stream).await;
        message.push(
            chat_completion
                .choices
                .first()
                .unwrap()
                .message
                .clone()
                .into(),
        );
    }
}

async fn listen_for_tokens(mut chat_stream: Receiver<ChatCompletionDelta>) -> ChatCompletion {
    let mut merged: Option<ChatCompletionDelta> = None;
    let mut first = true;
    loop {
        match chat_stream.try_recv() {
            Ok(delta) => {
                let choice = &delta.choices[0];
                if first {
                    if let Some(role) = &choice.delta.role {
                        print!("{:#?}: ", role);
                    }
                    first = false;
                }
                if let Some(content) = &choice.delta.content {
                    print!("{}", content);
                }
                if let Some(tool_calls) = &choice.delta.tool_calls {
                    for tool_call in tool_calls {
                        println!("Tool call: {:#?}", tool_call);
                    }
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
    merged.unwrap().into()
}
