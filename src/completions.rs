//! Given a prompt, the model will return one or more predicted completions,
//! and can also return the probabilities of alternative tokens at each position.

use serde::{ Deserialize, Serialize };
use super::{ OpenAI, openai_headers };
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
pub struct TextCompletionObject {
    pub id: String,
    pub object: String,
    pub created: u32,
    pub model: String,
    pub choices: Vec<TextCompletionObjectChoices>,
    pub usage: TextCompletionObjectUsage,
}

#[derive(Deserialize, Debug)]
pub struct TextCompletionObjectChoices {
    pub text: String,
    pub index: u8,
    pub logprobs: Option<u8>,
    pub finish_reason: String,
}

#[derive(Deserialize, Debug)]
pub struct TextCompletionObjectUsage {
    pub prompt_tokens: u16,
    pub completion_tokens: u16,
    pub total_tokens: u32,
}

#[derive(Serialize, Default, Debug)]
pub struct CreateCompletionRequestBody<'a> {
    /// ID of the model to use. You can use the [List models](https://beta.openai.com/docs/api-reference/models/list)
    /// API to see all of your available models, or see our [Model overview](https://beta.openai.com/docs/models/overview)
    /// for descriptions of them.
    pub model: &'a str,
    /// The prompt(s) to generate completions for, encoded as a string, array of strings, array of tokens, or array of token arrays.
    ///
    /// Note that <|endoftext|> is the document separator that the model sees during training,
    /// so if a prompt is not specified the model will generate as if from the beginning of a new document.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<&'a str>,
    /// The suffix that comes after a completion of inserted text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<&'a str>,
    /// The maximum number of [tokens](https://beta.openai.com/tokenizer) to generate in the completion.
    ///
    /// The token count of your prompt plus `max_tokens` cannot exceed the model's context length.
    /// Most models have a context length of 2048 tokens (except for the newest models, which support 4096).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u16>,
    /// What [sampling temperature](https://towardsdatascience.com/how-to-sample-from-language-models-682bceb97277) to use.
    /// Higher values means the model will take more risks. Try 0.9 for more creative applications,
    /// and 0 (argmax sampling) for ones with a well-defined answer.
    ///
    /// We generally recommend altering this or `top_p` but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// An alternative to sampling with temperature, called nucleus sampling,
    /// where the model considers the results of the tokens with top_p probability mass.
    /// So 0.1 means only the tokens comprising the top 10% probability mass are considered.
    ///
    /// We generally recommend altering this or `temperature` but not both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f32>,
    /// How many completions to generate for each prompt.
    ///
    /// **Note:** Because this parameter generates many completions,
    /// it can quickly consume your token quota.
    /// Use carefully and ensure that you have reasonable settings for `max_tokens` and `stop`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u8>,
    /// Whether to stream back partial progress.
    /// If set, tokens will be sent as data-only [server-sent events](https://developer.mozilla.org/en-US/docs/Web/API/Server-sent_events/Using_server-sent_events#Event_stream_format)
    /// as they become available, with the stream terminated by a `data: [DONE]` message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    /// Include the log probabilities on the `logprobs` most likely tokens, as well the chosen tokens.
    /// For example, if `logprobs` is 5, the API will return a list of the 5 most likely tokens.
    /// The API will always return the `logprob` of the sampled token, so there may be up to `logprobs+1` elements in the response.
    ///
    /// The maximum value for `logprobs` is 5.
    /// If you need more than this, please contact us through our [Help center](https://help.openai.com/) and describe your use case.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<u8>,
    /// Echo back the prompt in addition to the completion
    #[serde(skip_serializing_if = "Option::is_none")]
    pub echo: Option<bool>,
    /// Up to 4 sequences where the API will stop generating further tokens.
    /// The returned text will not contain the stop sequence.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<&'a str>,
    /// Number between -2.0 and 2.0.
    /// Positive values penalize new tokens based on whether they appear in the text so far,
    /// increasing the model's likelihood to talk about new topics.
    ///
    /// [See more information about frequency and presence penalties.](https://beta.openai.com/docs/api-reference/parameter-details)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    /// Number between -2.0 and 2.0.
    /// Positive values penalize new tokens based on their existing frequency in the text so far,
    /// decreasing the model's likelihood to repeat the same line verbatim.
    ///
    /// [See more information about frequency and presence penalties.](https://beta.openai.com/docs/api-reference/parameter-details)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    /// Generates `best_of` completions server-side and returns the "best" (the one with the highest log probability per token).
    /// Results cannot be streamed.
    ///
    /// When used with `n`, `best_of` controls the number of candidate completions and `n` specifies how many to return –
    /// `best_of` must be greater than `n`.
    ///
    /// **Note:** Because this parameter generates many completions, it can quickly consume your token quota.
    /// Use carefully and ensure that you have reasonable settings for `max_tokens` and `stop`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_of: Option<u8>,
    /// Modify the likelihood of specified tokens appearing in the completion.
    ///
    /// Accepts a json object that maps tokens (specified by their token ID in the GPT tokenizer)
    /// to an associated bias value from -100 to 100.
    /// You can use this [tokenizer tool](https://beta.openai.com/tokenizer?view=bpe)
    /// (which works for both GPT-2 and GPT-3) to convert text to token IDs.
    /// Mathematically, the bias is added to the logits generated by the model prior to sampling.
    /// The exact effect will vary per model, but values between -1 and 1 should decrease or increase likelihood of selection;
    ///cvalues like -100 or 100 should result in a ban or exclusive selection of the relevant token.
    ///
    /// As an example, you can pass `{"50256": -100}` to prevent the <|endoftext|> token from being generated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<HashMap<&'a str, u8>>,
    /// A unique identifier representing your end-user, which can help OpenAI to monitor and detect abuse.
    /// [Learn more](https://beta.openai.com/docs/guides/safety-best-practices/end-user-ids).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<&'a str>,
}

pub(super) async fn create_completion(openai: &OpenAI<'_>, body: CreateCompletionRequestBody<'_>) -> Result<TextCompletionObject, reqwest::Error> {
    if let Some(stream) = body.stream {
        if stream == true {
            todo!("streaming is not yet implemented");
        }
    }

    let request = openai.client.post("https://api.openai.com/v1/completions")
        .headers(openai_headers(openai))
        .json(&body);

    request.send().await?.json().await
}
