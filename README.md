# openai

An unofficial Rust library for the OpenAI API.

> **Info**
>
> The ownership of the `openai` package on crates.io has been transfered to me, Valentine Briese (not the previous owner).
> This is an entirely different project than the one that was on crates.io as `openai` previously!

> **Warning**
> 
> Currently in alpha, not yet stable enough to be used in any production applications.
> See [Implementation Progress](#implementation-progress).

## Core Principles

- Instead of accessing all functions as methods on a single client-like structure,
  functions should be accessed from their own modules.
- Environmental variables should be the prioitized method of authentication,
  but you shouldn't be forced to do things this way.
  (Currently, you kind of are, because the API key is needed during compile-time.)
- This is a LIBRARY, not a WRAPPER!
  The goal here isn't to just give some basic wrapper functions for making HTTP requests,
  it's to "rust-ify" things. We want to create the illusion that the OpenAI API was made in Rust first!
- What is this, C? No, it's Rust! We follow the object-oriented paradigm, not the procedural one.
  What this mainly means is less `create_completion()`, more `Completion::new()`.

## Examples

I'm still working on making examples in the `examples` directory.
If you're looking to work with the `completions` module, you're in luck!
Because that's the only module there is an example for right now.
For other modules, you can look at the `tests` submodules for some reference.

Examples come slowly because this project, in its current state, changes very quickly,
and it's not fun making sure all examples accurately reflect the latest version. But, they are coming, don't worry!

## Troubleshooting

### `environment variable OPENAI_KEY should be defined`

An error you will likely run into, and a hopefully pretty self-explanatory one.
For the library to even build, you must have an environment variabled named `OPENAI_KEY` which is set to,
you guessed it, your OpenAI API key. Without your API key, this library can't do anything.
In fact, the library won't build without it because at compile-time it uses your key to fetch all available models
and generate the `ModelID` enumerator.

In a development environment,
this can be best resolved by created a `.env` file in the root of your project with the following contents:

```env
OPENAI_KEY=put-your-api-key-here
```

Then, you need to load the contents of your `.env` file when your program starts.
For this, I recommend a crate such as [dotenvy](https://github.com/allan2/dotenvy).

## Implementation Progress

`██████████` Models

`████████░░` Completions

`██████████` Edits

`░░░░░░░░░░` Images

`█████████░` Embeddings

`░░░░░░░░░░` Files

`░░░░░░░░░░` Fine-tunes

`░░░░░░░░░░` Moderations
