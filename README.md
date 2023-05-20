# openai

[![crates.io](https://img.shields.io/crates/v/openai.svg)](https://crates.io/crates/openai/)
[![Rust workflow](https://github.com/rellfy/openai/actions/workflows/rust.yml/badge.svg)](https://github.com/rellfy/openai/actions/workflows/rust.yml)

An unofficial Rust library for the OpenAI API.

> **Warning**
>
> There may be breaking changes between versions while in alpha.
> See [Implementation Progress](#implementation-progress).

## Core Principles

- Modularity
- Library, not a wrapper
- Idiomatic Rust
- Environmental variables should be the prioritized method of authentication,
  but not the only way to do things

## Examples

Examples can be found in the `examples` directory.

As the package is still a work in progress and there may be breaking changes,
examples are not available for all the crate's functionality.

Currently, there are examples for the `completions` module and the `chat` module.
For other modules, refer to the `tests` submodules for some reference.

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

`██████████` Moderations

## Contributing

All contributions are welcome. Unit tests are encouraged.

> **Fork Notice**
>
> This package was initially developed by [Valentine Briese](https://github.com/valentinegb/openai).  
> As the original repo was archived, this is a fork and continuation of the project.
