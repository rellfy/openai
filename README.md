# openai [![crates.io](https://img.shields.io/crates/v/openai.svg)](https://crates.io/crates/openai/) [![Rust workflow](https://github.com/valentinegb/openai/actions/workflows/rust.yml/badge.svg)](https://github.com/valentinegb/openai/actions/workflows/rust.yml)

An unofficial Rust library for the OpenAI API.

> **Note**
>
> The ownership of the `openai` package on crates.io has been transfered to me, Valentine Briese (not the previous owner).
> This is an entirely different project than the one that was on crates.io as `openai` previously!

> **Warning**
> 
> Currently in alpha, I wouldn't recommend using in any production applications.
> See [Implementation Progress](#implementation-progress).

## Core Principles

- Instead of accessing all functions as methods on a single client-like structure,
  functions should be accessed from their own modules.
- Environmental variables should be the prioitized method of authentication,
  but you shouldn't be forced to do things this way.
- This is a LIBRARY, not a WRAPPER!
  The goal here isn't to just give some basic wrapper functions for making HTTP requests,
  it's to "rust-ify" things. We want to create the illusion that the OpenAI API was made in Rust first!
- What is this, C? No, it's Rust! We follow the object-oriented paradigm, not the procedural one.
  What this mainly means is less `create_completion()`, more `Completion::create()`.

## Examples

I'm still working on making examples in the `examples` directory.
Currently, there are examples for the `completions` module and the `chat` module.
For other modules, you can look at the `tests` submodules for some reference.

Examples come slowly because this project, in its current state, changes very quickly,
and it's not fun making sure all examples accurately reflect the latest version. But, they are coming, don't worry!

## Implementation Progress

`██████████` Models

`████████░░` Completions

`████████░░` Chat

`██████████` Edits

`░░░░░░░░░░` Images

`█████████░` Embeddings

`░░░░░░░░░░` Audio

`░░░░░░░░░░` Files

`░░░░░░░░░░` Fine-tunes

`░░░░░░░░░░` Moderations

## Sponsors

I've gotten my first monthly sponsor! 🎉

I'm very proud that I can now commit this **Sponsors** section to this README, and this is thanks to [**Arto Bendiken**](https://github.com/artob). Thanks Arto!

I suppose now I better figure out a good way of organizing this section in case I get any more sponsors-
