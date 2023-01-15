# openai
An unofficial OpenAI API library for Rust.

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

## Implementation Progress
- [ ] Models
- [ ] Completions
- [ ] Edits
- [ ] Images
- [x] Embeddings
- [ ] Files
- [ ] Fine-tunes
- [ ] Moderations
