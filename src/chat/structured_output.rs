use std::mem::take;

use schemars::{
    schema::{Schema, SchemaObject},
    visit::{visit_schema_object, Visitor},
    JsonSchema,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum JsonSchemaStyle {
    OpenAI,
    Grok,
}

#[derive(Serialize, Debug, Clone, Eq, PartialEq)]
pub struct ChatCompletionResponseFormatJsonSchema {
    /// The name of the response format. Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64.
    pub name: String,
    /// A description of what the response format is for, used by the model to determine how to respond in the format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The schema for the response format, described as a JSON Schema object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<Value>,
    /// Whether to enable strict schema adherence when generating the output.
    /// If set to true, the model will always follow the exact schema defined in the schema field.
    /// Only a subset of JSON Schema is supported when strict is true.
    /// To learn more, read the [Structured Outputs guide](https://platform.openai.com/docs/guides/structured-outputs).
    ///
    /// defaults to false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

impl ChatCompletionResponseFormatJsonSchema {
    pub fn new<T: JsonSchema>(strict: bool, json_style: JsonSchemaStyle) -> Self {
        let (schema, description) = generate_json_schema::<T>(json_style);
        ChatCompletionResponseFormatJsonSchema {
            name: T::schema_name(),
            description,
            schema: Some(schema),
            strict: Some(strict),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug, Eq, PartialEq)]
pub struct ToolCallFunctionDefinition {
    /// The name of the function to be called. Must be a-z, A-Z, 0-9, or contain underscores and dashes, with a maximum length of 64.
    pub name: String,
    /// A description of what the function does, used by the model to choose when and how to call the function.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The parameters the functions accepts, described as a JSON Schema object.
    /// See the [guide](https://platform.openai.com/docs/guides/function-calling) for examples,
    /// and the [JSON Schema reference](https://json-schema.org/understanding-json-schema/reference) for documentation about the format.
    /// Omitting `parameters` defines a function with an empty parameter list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Value>,
    /// Whether to enable strict schema adherence when generating the function call.
    /// If set to true, the model will follow the exact schema defined in the `parameters` field.
    /// Only a subset of JSON Schema is supported when `strict` is `true`.
    /// Learn more about Structured Outputs in the [function calling guide](https://platform.openai.com/docs/api-reference/chat/docs/guides/function-calling).
    ///
    /// defaults to false
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

impl ToolCallFunctionDefinition {
    /// Create a new ToolCallFunctionDefinition with the given strictness and JSON Schema style.
    ///
    /// Note: Grok does not support strict schema adherence.
    pub fn new<T: JsonSchema>(strict: bool, json_style: JsonSchemaStyle) -> Self {
        let (schema, description) = generate_json_schema::<T>(json_style);
        let strict = match json_style {
            JsonSchemaStyle::OpenAI => Some(strict),
            JsonSchemaStyle::Grok => None,
        };
        ToolCallFunctionDefinition {
            description,
            name: T::schema_name(),
            parameters: Some(schema),
            strict,
        }
    }
}

/// Generate a JSON Schema with the given style.
///
/// IMPORTANT: Both OpenAI and Grok do not support the `format` and `minimum` JSON Schema attributes.
/// As a result, numeric type constraints (like `u8`, `i32`, etc) cannot be enforced - all integers
/// will be treated as `i64` and all floating point numbers as `f64`.
pub fn generate_json_schema<T: JsonSchema>(json_style: JsonSchemaStyle) -> (Value, Option<String>) {
    let mut settings = schemars::r#gen::SchemaSettings::default();
    settings.option_nullable = false;
    settings.inline_subschemas = true;
    settings.option_add_null_type = match json_style {
        JsonSchemaStyle::OpenAI => true,
        JsonSchemaStyle::Grok => false,
    };
    let mut generator = schemars::SchemaGenerator::new(settings);
    let mut schema = T::json_schema(&mut generator).into_object();
    let description = schema.metadata().description.clone();
    let mut processor = SchemaPostProcessor { style: json_style };
    processor.visit_schema_object(&mut schema);
    let schema = serde_json::to_value(schema).expect("unreachable");
    (schema, description)
}

pub struct SchemaPostProcessor {
    pub style: JsonSchemaStyle,
}

impl Visitor for SchemaPostProcessor {
    fn visit_schema_object(&mut self, schema: &mut SchemaObject) {
        if let Some(sub) = &mut schema.subschemas {
            sub.any_of = take(&mut sub.one_of);
        }
        schema.format = None;
        if let Some(sub) = &mut schema.object {
            if self.style == JsonSchemaStyle::OpenAI {
                if sub.additional_properties.is_none() {
                    sub.additional_properties = Some(Box::new(Schema::Bool(false)));
                }
                sub.required = sub.properties.keys().map(|s| s.clone()).collect();
            }
        }
        if let Some(num) = &mut schema.number {
            num.multiple_of = None;
            num.exclusive_maximum = None;
            num.exclusive_minimum = None;
            num.maximum = None;
            num.minimum = None;
        }
        if let Some(str) = &mut schema.string {
            str.max_length = None;
            str.min_length = None;
            str.pattern = None;
        }
        visit_schema_object(self, schema);
    }
}
