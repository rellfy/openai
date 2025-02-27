# openai

[![crates.io](https://img.shields.io/crates/v/openai.svg)](https://crates.io/crates/openai/)
[![Rust workflow](https://github.com/rellfy/openai/actions/workflows/test.yml/badge.svg)](https://github.com/rellfy/openai/actions/workflows/test.yml)

An unofficial Rust library for the OpenAI API.

## Examples

Examples can be found in the `examples` directory.

Please note that examples are not available for all the crate's functionality,
PRs are appreciated to expand the coverage.

Currently, there are examples for the `completions` module and the `chat`
module.
For other modules, refer to the `tests` submodules for some reference.

### Chat Example

```rust
// Relies on OPENAI_KEY and optionally OPENAI_BASE_URL.
let credentials = Credentials::from_env();
let messages = vec![
    ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content: Some("You are a helpful assistant.".to_string()),
        name: None,
        function_call: None,
    },
    ChatCompletionMessage {
        role: ChatCompletionMessageRole::User,
        content: Some("Tell me a random crab fact".to_string()),
        name: None,
        function_call: None,
    },
];
let chat_completion = ChatCompletion::builder("gpt-4o", messages.clone())
    .credentials(credentials.clone())
    .create()
    .await
    .unwrap();
let returned_message = chat_completion.choices.first().unwrap().message.clone();
// Assistant: Sure! Here's a random crab fact: ...
println!(
    "{:#?}: {}",
    returned_message.role,
    returned_message.content.unwrap().trim()
);
```

## Implementation Progress

`██████████` Models

`████████░░` Completions (Function calling is supported)

`████████░░` Chat

`██████████` Edits

`░░░░░░░░░░` Images

`█████████░` Embeddings

`░░░░░░░░░░` Audio

`███████░░░` Files

`░░░░░░░░░░` Fine-tunes

`██████████` Moderations

## Contributing

All contributions are welcome. Unit tests are encouraged.

> **Fork Notice**
>
> This package was initially developed by [Valentine Briese](https://github.com/valentinegb/openai).
> As the original repo was archived, this is a fork and continuation of the project.
