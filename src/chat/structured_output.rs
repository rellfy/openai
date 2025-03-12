use std::mem::take;

use schemars::JsonSchema;
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

fn structured_output_post_process(schema: &mut Value, style: JsonSchemaStyle) {
    let obj = match schema {
        Value::Object(obj) => obj,
        _ => return,
    };
    // OpenAI uses `anyOf` instead of `oneOf`
    if let Some(v) = obj.remove("oneOf") {
        obj.insert("anyOf".to_string(), v);
    }
    if let Some(Value::Array(objs)) = obj.get_mut("anyOf") {
        for v in objs.iter_mut() {
            structured_output_post_process(v, style);
        }
    }
    let ty = match obj.get("type") {
        Some(Value::String(s)) => s,
        _ => {
            return;
        }
    };
    match ty.as_str() {
        "array" => {
            if let Some(v) = obj.get_mut("items") {
                structured_output_post_process(v, style);
            }
        }
        "object" => {
            let properties = if let Some(Value::Object(p)) = obj.get_mut("properties") {
                p
            } else {
                return;
            };
            let mut required = Vec::new();
            for (k, v) in properties.iter_mut() {
                // v must be a json schema object
                structured_output_post_process(v, style);
                required.push(Value::String(k.clone()));
            }
            if style == JsonSchemaStyle::OpenAI {
                // OpenAI: All fields or function parameters must be specified as `required`;
                obj.insert("required".to_string(), Value::Array(required));
                // OpenAI: Need to add `additionalProperties`;
                if obj.get("additionalProperties").is_none() {
                    obj.insert("additionalProperties".to_string(), Value::Bool(false));
                }
            }
        }
        "string" => {
            *obj = take(obj)
                .into_iter()
                .filter(|(k, _)| ["type", "enum"].contains(&k.as_str()))
                .collect();
        }
        "number" => {
            // Remove constraints like `multipleOf` for floating point types;
            *obj = take(obj)
                .into_iter()
                .filter(|(k, _)| ["type"].contains(&k.as_str()))
                .collect();
        }
        "integer" => {
            // Remove constraints like `format` and `minimum` for integer types;
            *obj = take(obj)
                .into_iter()
                .filter(|(k, _)| ["type"].contains(&k.as_str()))
                .collect();
        }
        _ => {}
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
    let mut schema = serde_json::to_value(schema).expect("unreachable");
    structured_output_post_process(&mut schema, json_style);
    (schema, description)
}
