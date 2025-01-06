#![allow(unused)]

use std::fmt::Display;

use derive_builder::Builder;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

#[derive(Clone, Debug, PartialEq)]
pub enum MessageRole {
    User,
    System,
    Assistant,
    Developer,
}

impl Serialize for MessageRole {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            MessageRole::User => serializer.serialize_str("user"),
            MessageRole::System => serializer.serialize_str("system"),
            MessageRole::Assistant => serializer.serialize_str("assistant"),
            MessageRole::Developer => serializer.serialize_str("developer"),
        }
    }
}

impl<'de> Deserialize<'de> for MessageRole {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "user" => Ok(MessageRole::User),
            "system" => Ok(MessageRole::System),
            "assistant" => Ok(MessageRole::Assistant),
            "developer" => Ok(MessageRole::Developer),
            _ => {
                let msg = format!("Role can only be 'user', 'system' or 'assistant', current is {}", s);
                Err(serde::de::Error::custom(msg))
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Content {
    Text { text: String },
    Image { image_url: String },
    Audio { data: String, format: AudioFormat },
}

impl Content {
    pub fn text(text: &str) -> Result<Self, String> {
        Ok(Content::Text {
            text: String::from(text),
        })
    }

    pub fn image(image_url: &str) -> Result<Self, String> {
        Ok(Content::Image {
            image_url: String::from(image_url),
        })
    }

    pub fn audio(data: &str, format: &str) -> Result<Self, String> {
        AudioFormat::from_str(format).map(|format| Content::Audio {
            data: String::from(data),
            format,
        })
    }

    fn into_value(&self) -> Value {
        match self {
            Content::Text { text } => {
                json!({ "text": text, "type": "text" })
            }
            Content::Image { image_url } => {
                json!({"image_url": {"url": image_url}, "type": "image_url"})
            }
            Content::Audio { data, format } => {
                json!({"input_audio": {"data": data, "format": format}, "type": "input_audio"})
            }
        }
    }

    fn from_value(value: &Value) -> Result<Self, String> {
        let type_ = &value["type"];
        if type_.is_null() {
            return Err("类型为空.".to_string());
        }
        if let Value::String(ct) = type_ {
            match ct.to_lowercase().as_str() {
                "text" => {
                    let text = value["text"]
                        .as_str()
                        .map_or("".to_string(), |t| String::from(t));
                    Ok(Content::Text { text })
                }
                "image_url" => {
                    let image_url = value["image_url"]["url"]
                        .as_str()
                        .map_or("".to_string(), |t| String::from(t));
                    Ok(Content::Image { image_url })
                }
                "input_audio" => {
                    let format = value["input_audio"]["format"].as_str().unwrap_or_default();
                    AudioFormat::from_str(format).map(|format| {
                        let data = value["input_audio"]["data"]
                            .as_str()
                            .map_or("".to_string(), |t| String::from(t));
                        Content::Audio { data, format }
                    })
                }
                _ => Err(format!("类型错误: {}", ct)),
            }
        } else {
            Err("类型错误".to_string())
        }
    }
}

impl Display for Content {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = self.into_value();
        write!(f, "{}", value.to_string())
    }
}

impl Serialize for Content {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        Value::serialize(&self.into_value(), serializer)
    }
}

impl<'de> Deserialize<'de> for Content {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        Content::from_value(&value).map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AudioFormat {
    WAV,
    MP3,
}

impl AudioFormat {
    pub fn from_str(s: &str) -> Result<Self, String> {
        match s.to_lowercase().as_str() {
            "wav" => Ok(AudioFormat::WAV),
            "mp3" => Ok(AudioFormat::MP3),
            _ => Err("不支持的格式".to_string()),
        }
    }
}

impl Serialize for AudioFormat {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            AudioFormat::MP3 => serializer.serialize_str("mp3"),
            AudioFormat::WAV => serializer.serialize_str("wav"),
        }
    }
}

impl<'de> Deserialize<'de> for AudioFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        AudioFormat::from_str(&s).map_err(serde::de::Error::custom)
    }
}

#[derive(Clone, Debug)]
pub enum MessageContent {
    Text(String),
    /// image or audio
    NotText(Vec<Content>),
}

impl MessageContent {
    pub fn text(text: String) -> Self {
        Self::Text(text)
    }

    pub fn image(desc: String, url: String) -> Self {
        let text_content = Content::text(&desc).unwrap();
        let image_content = Content::image(&url).unwrap();
        Self::NotText(vec![text_content, image_content])
    }
}

impl Serialize for MessageContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Text(text) => serializer.serialize_str(&text),
            Self::NotText(contents) => Vec::serialize(contents, serializer),
        }
    }
}

impl<'de> Deserialize<'de> for MessageContent {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        match value {
            Value::String(text) => Ok(MessageContent::Text(text)),
            Value::Array(values) => {
                let mut contents = Vec::new();
                for v in values {
                    match Content::from_value(&v) {
                        Ok(c) => contents.push(c),
                        Err(e) => return Err(serde::de::Error::custom(e)),
                    }
                }
                Ok(MessageContent::NotText(contents))
            },
            _ => Err(serde::de::Error::custom("消息类型错误".to_string())),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ImageUrl {
    pub url: String
}

#[derive(Debug, PartialEq)]
pub struct InputAudio {
    pub data: String,
    pub format: AudioFormat,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestMessage {
    role: MessageRole,
    content: MessageContent,
}

impl RequestMessage {
    pub fn new_text(role: MessageRole, text: String) -> Self {
        RequestMessage { role, content: MessageContent::text(text) }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestModel(String);

impl Default for RequestModel {
    fn default() -> Self {
        Self("gpt-4o-mini".to_string())
    }
}

#[derive(Debug, Default, Builder, Serialize, Deserialize)]
#[builder(setter(into))]
pub struct CompletionRequest {
    model: RequestModel,
    messages: Vec<RequestMessage>,
    stream: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_message() {
        let msg = RequestMessage::new_text(MessageRole::User, "你好".to_string());
        println!("{}", serde_json::to_string(&msg).unwrap());

        let v = json!({"role":"assistant","content":"你好,can i help you"});
        println!("{}", v);
        let msg: RequestMessage = serde_json::from_value(v).expect("msg");
        println!("{:?}", msg);
    }

    #[test]
    fn test_request_model() {
        let model = RequestModel::default();
        println!("{}", serde_json::to_string(&model).unwrap());

        let body = CompletionRequestBuilder::default()
            .model(RequestModel::default())
            .messages(vec![])
            .stream(false)
            .build()
            .unwrap();
        println!("{}", serde_json::to_string(&body).unwrap());
    }
}
