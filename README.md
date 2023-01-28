# openai
An unofficial Rust library for the OpenAI API.

> ## Currently in alpha
> Not yet stable enough to be used in any production applications.

## Version 0 -> 1 changes

**NEW PLAN: we're scrapping basically everything.**

 - Instead of accessing all functions as methods on a single structure,
 functions will be accessed from their own modules.
 - "What about authorization?"
 Functions will access an established environmental variable.
 This is a bit different from how most libraries do things,
 but when would someone *not* want to use environmental variables?
 I'm sure most people wouldn't mind being forced to do things this way.
 - This is a LIBRARY, not a WRAPPER!
 The goal here isn't to just give some basic wrapper functions for making HTTP requests,
 it's to "rust-ify" things. We want to create the illusion that the OpenAI API was made in Rust first!
 - What is this, C? No, it's Rust! We follow the object-oriented paradigm, not the procedural one.
 What this mainly means is less `create_completion()`, more `Completion::new()`

## Examples
You may refer to the `tests` submodules typically defined in each module for example code.
As of writing this, the most complete module is the `embeddings` module, so here's an example of how to use that:
```rs
use openai::{ embeddings::Embedding, models::ModelID };

#[tokio::main]
async fn main() {
    let embedding = Embedding::new(
        ModelID::TextEmbeddingAda002,
        "The food was delicious and the waiter...",
        None,
    ).await.unwrap();

    println!("{}", embedding.vec.first().unwrap()); // prints "0.0023064255"... probably. This is AI, after all
}
```
The `completions` module is fairly close to completion. (Get it?)

A full example project for this module has been made and can be found in the `examples` subdirectory.
This can be helpful as a template, but there is still more diverse code in the unit tests.

## Implementation Progress
`██████████` Models

`████████░░` Completions

`░░░░░░░░░░` Edits

`░░░░░░░░░░` Images

`█████████░` Embeddings

`░░░░░░░░░░` Files

`░░░░░░░░░░` Fine-tunes

`░░░░░░░░░░` Moderations
