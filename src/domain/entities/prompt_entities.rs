use std::collections::HashMap;

use poem_openapi::{Object, Union};
use serde::{Deserialize, Serialize};

trait IsEmpty {
    fn is_empty(&self) -> bool;
}

impl<T> IsEmpty for &Vec<T> {
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<K, V> IsEmpty for &HashMap<K, V> {
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

fn is_empty(v: impl IsEmpty) -> bool {
    v.is_empty()
}

#[derive(Object, Debug, Serialize, Deserialize, Clone)]
pub struct Function {
    name: String,
    arguments: String,
}

#[derive(Object, Debug, Serialize, Deserialize, Clone)]
pub struct ToolCall {
    id: String,
    r#type: String,
    function: Function,
}

#[derive(Object, Debug, Serialize, Deserialize, Clone)]
pub struct AudioPart {
    data: String,
    format: String,
}

#[derive(Object, Debug, Serialize, Deserialize, Clone)]
pub struct ImagePart {
    uri: String,
    quality: String,
}

#[derive(Object, Debug, Serialize, Deserialize, Clone)]
pub struct TextPart {
    text: String,
}

#[derive(Union, Debug, Serialize, Deserialize, Clone)]
pub enum MessagePart {
    Audio(AudioPart),
    Images(ImagePart),
    Text(TextPart),
}

#[derive(Object, Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    #[oai(validator(pattern = "^(user|assistant|system|tool)$"))]
    role: String,

    #[oai(default, skip_serializing_if_is_none)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    name: Option<String>,

    content: Vec<MessagePart>,

    #[oai(skip_serializing_if_is_none)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    tool_calls: Option<Vec<ToolCall>>,
}

#[derive(Object, Debug, Serialize, Deserialize, Clone)]
pub struct Tool {
    schema_version: String,
    name: String,
    description: String,
    schema: serde_json::Value,
}

#[derive(Object, Debug, Serialize, Deserialize, Clone)]
pub struct Prompt {
    model: String,
    content: Vec<Message>,

    #[oai(default, skip_serializing_if_is_none, validator(minimum(value = -2.0), maximum(value = 2.0)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    frequency_penalty: Option<f64>,

    #[oai(default, skip_serializing_if_is_none)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    logit_bias: Option<HashMap<String, i32>>,

    #[oai(default, skip_serializing_if_is_none)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    logprobs: Option<bool>,

    #[oai(
        default,
        skip_serializing_if_is_none,
        validator(minimum(value = 0.0), maximum(value = 20.0))
    )]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    top_logprobs: Option<u32>,

    #[oai(default, skip_serializing_if_is_none)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    max_output_tokens: Option<u32>,

    #[oai(default, skip_serializing_if_is_none, validator(minimum(value = -2.0), maximum(value = 2.0)))]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    presence_penalty: Option<f64>,

    #[oai(default, skip_serializing_if_is_none)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    stop: Option<Vec<String>>,

    #[oai(
        default,
        skip_serializing_if_is_none,
        validator(minimum(value = 0.0), maximum(value = 2.0))
    )]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    temperature: Option<f64>,

    #[oai(default, skip_serializing_if_is_none)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    top_p: Option<f64>,

    #[oai(default, skip_serializing_if = "is_empty")]
    #[serde(default, skip_serializing_if = "is_empty")]
    tools: Vec<Tool>,

    #[oai(default, skip_serializing_if = "is_empty")]
    #[serde(default, skip_serializing_if = "is_empty")]
    extra: HashMap<String, serde_json::Value>,

    #[oai(default, skip_serializing_if_is_none)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub(crate) _id: Option<String>,
}
