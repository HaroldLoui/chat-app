use crate::models::*;
use serde::{Deserialize, Serialize};

use anyhow::{bail, Result as AR};
use serde_json::Value;
use tauri_plugin_http::reqwest::{Client, Error, Response};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Params {
    url: String,
    key: String,
    stream: bool,
}

impl Params {
    pub fn from(url: String, key: String, stream: bool) -> Self {
        Self { url, key, stream }
    }
}

#[derive(Debug)]
pub enum ResponseContent {
    Text(String),
    Array(Vec<String>),
}

impl ResponseContent {
    pub fn from_error(stream: bool, error_desc: String) -> Self {
        if stream {
            ResponseContent::Array(vec![error_desc, "DONE".to_string()])
        } else {
            ResponseContent::Text(error_desc)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ContextMessage {
    /// 面具
    mask: String,
    contexts: Vec<RequestMessage>,
}

impl Default for ContextMessage {
    fn default() -> Self {
        Self {
            mask: "你是一个有用的助手".to_string(),
            contexts: Vec::new(),
        }
    }
}

impl ContextMessage {
    pub fn context_messages(&self) -> Vec<RequestMessage> {
        if self.contexts.is_empty() {
            let cm = ContextMessage::default();
            return vec![RequestMessage::new_text(MessageRole::System, cm.mask)];
        }
        let mask_msg = RequestMessage::new_text(MessageRole::System, self.mask.clone());
        let mut contexts = vec![mask_msg];
        contexts.extend_from_slice(self.contexts.as_ref());
        contexts
    }
}

#[derive(Debug)]
pub struct OpenAiClient<'h> {
    client: &'h Client,
}

impl<'h> OpenAiClient<'h> {
    pub fn new(client: &'h Client) -> Self {
        Self { client }
    }

    pub async fn send(&self, p: Params, content: String, context: Option<ContextMessage>) -> ResponseContent {
        let mut messages = Vec::new();
        if context.is_some() {
            messages.extend_from_slice(context.unwrap().context_messages().as_ref());
        }
        let message = RequestMessage::new_text(MessageRole::User, content);
        messages.push(message);
        let body = CompletionRequestBuilder::default()
            .model(RequestModel::default())
            .messages(messages)
            .stream(p.stream)
            .build()
            .unwrap();
        let response_result = self
            .client
            .post(p.url)
            .header("Authorization", format!("Bearer {}", p.key))
            .json(&body)
            .send()
            .await;
        parse_response(response_result, p.stream)
            .await
            .unwrap_or_else(|e| {
                ResponseContent::from_error(p.stream, e.to_string())
            })
    }
}

async fn parse_response(response: Result<Response, Error>, stream: bool) -> AR<ResponseContent> {
    let text = response?.text().await?;
    if stream {
        let mut contents = Vec::new();
        for spt in text.split("data:") {
            let text = spt.trim();
            if !text.is_empty() && text != "[DONE]" {
                let content = parse_text(text, true).unwrap_or("DONE".to_string());
                contents.push(content);
            }
        }
        Ok(ResponseContent::Array(contents))
    } else {
        match parse_text(&text, false) {
            Some(content) => Ok(ResponseContent::Text(content)),
            None => bail!("解析响应内容失败: {}", text),
        }
    }
}

fn parse_text(text: &str, stream: bool) -> Option<String> {
    let json: Value = serde_json::from_str(text).ok()?;
    let choices = json["choices"].as_array()?;
    let key = if stream { "delta" } else { "message" };
    let content = choices[0][key]["content"].as_str()?;
    Some(String::from(content))
}
