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
I'm still working on making examples in the `examples` directory. If you're looking to work with the `completions` module, you're in luck! Because that's the only module there is an example for right now. For other modules, you can look at the `tests` submodules for some reference.

Examples come slowly because this project, in its current state, changes very quickly, and it's not fun making sure all examples accurately reflect the latest version. But, they are coming, don't worry!

## Implementation Progress
`██████████` Models

`████████░░` Completions

`██████████` Edits

`░░░░░░░░░░` Images

`█████████░` Embeddings

`░░░░░░░░░░` Files

`░░░░░░░░░░` Fine-tunes

`░░░░░░░░░░` Moderations
