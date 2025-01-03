use std::sync::Mutex;
use std::time::Duration;

use anyhow::{bail, Result as AR};
use rusqlite::Connection;
use serde_json::Value;
use tauri::{AppHandle, Emitter, Runtime, State};
use tauri_plugin_http::reqwest::{Client, Error, Response};

use crate::mapper;
use crate::mapper::ApiConfig;
use crate::models::*;

#[tauri::command]
pub fn list_api_config(conn: State<'_, Mutex<Connection>>) -> Result<Vec<ApiConfig>, String> {
    let conn = conn.lock().unwrap();
    mapper::query_config_list(&conn).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub fn insert_api_config(
    conn: State<'_, Mutex<Connection>>,
    eneity: ApiConfig,
) -> Result<(), String> {
    let conn = conn.lock().unwrap();
    mapper::insert_api_config(&conn, eneity).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn set_default_config(conn: State<'_, Mutex<Connection>>, id: u32) -> Result<(), String> {
    let conn = conn.lock().unwrap();
    mapper::update_default_config(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_api_config(conn: State<'_, Mutex<Connection>>, id: u32) -> Result<(), String> {
    let conn = conn.lock().unwrap();
    mapper::delete_api_config(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn query_enable_stream(conn: State<'_, Mutex<Connection>>) -> Result<bool, String> {
    let conn = conn.lock().unwrap();
    mapper::query_enable_stream(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_enable_stream(conn: State<'_, Mutex<Connection>>, stream: bool) -> Result<(), String> {
    let conn = conn.lock().unwrap();
    mapper::update_enable_stream(&conn, stream).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn query_associated_context(conn: State<'_, Mutex<Connection>>) -> Result<bool, String> {
    let conn = conn.lock().unwrap();
    mapper::query_associated_context(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_associated_context(conn: State<'_, Mutex<Connection>>, context: bool) -> Result<(), String> {
    let conn = conn.lock().unwrap();
    mapper::update_associated_context(&conn, context).map_err(|e| e.to_string())
}

const MESSAGE_EVENT_NAME: &str = "chat:message://received";
const STREAM_MESSAGE_EVENT_NAME: &str = "chat:message://stream_received";
#[tauri::command(rename_all = "snake_case")]
pub async fn send_message<R: Runtime>(
    app: AppHandle<R>,
    client: State<'_, Mutex<Client>>,
    conn: State<'_, Mutex<Connection>>,
    content: String,
) -> Result<(), String> {
    let (config, stream) = {
        let conn = conn.lock().unwrap();
        let config = mapper::query_default_config(&conn).unwrap_or_default();
        let stream = mapper::query_enable_stream(&conn).unwrap_or_default();
        (config, stream)
    };
    let url = config.url.unwrap_or_default();
    if url.is_empty() {
        let _ = app.emit(MESSAGE_EVENT_NAME, "请配置接口地址。");
        return Ok(());
    }
    let key = config.key.unwrap_or_default();
    if key.is_empty() {
        let _ = app.emit(MESSAGE_EVENT_NAME, "请配置密钥。");
        return Ok(());
    }
    let client = client.lock().unwrap().clone();
    let message = RequestMessage::new_text(MessageRole::User, content);
    let body = CompletionRequestBuilder::default()
        .model("gpt-4o-mini".to_string())
        .messages(vec![message])
        .stream(stream)
        .build()
        .unwrap();
    let res = client
        .post(url)
        .header("Authorization", format!("Bearer {}", key))
        .json(&body)
        .send()
        .await;
    if stream {
        match parse_stream_response(res).await {
            Ok(contents) => {
                for content in contents {
                    let _ = app.emit(STREAM_MESSAGE_EVENT_NAME, content);
                    std::thread::sleep(Duration::from_millis(100));
                }
            },
            Err(e) => {
                let _ = app.emit(STREAM_MESSAGE_EVENT_NAME, e.to_string());
                std::thread::sleep(Duration::from_millis(100));
                let _ = app.emit(STREAM_MESSAGE_EVENT_NAME, "DONE");
            }
        }
    } else {
        match parse_response(res).await {
            Ok(response) => {
                let _ = app.emit(MESSAGE_EVENT_NAME, response);
            }
            Err(e) => {
                let _ = app.emit(MESSAGE_EVENT_NAME, e.to_string());
            }
        }
    }
    Ok(())
}

async fn parse_response(response: Result<Response, Error>) -> AR<String> {
    let text = response?.text().await?;
    match parse_text(&text) {
        Some(content) => Ok(content),
        None => bail!("解析响应内容失败: {}", text)
    }
}

fn parse_text(text: &str) -> Option<String> {
    let json: Value = serde_json::from_str(text).ok()?;
    let choices = json["choices"].as_array()?;
    let content = choices[0]["message"]["content"].as_str()?;
    Some(String::from(content))
}

async fn parse_stream_response(response: Result<Response, Error>) -> AR<Vec<String>> {
    let text = response?.text().await?;
    let mut contents = Vec::new();
    for spt in text.split("data:") {
        let text = spt.trim();
        if text.is_empty() {
            continue;
        }
        if text == "[DONE]" {
            break;
        }
        let content = parse_stream_text(text).unwrap_or("DONE".to_string());
        contents.push(content);
    }
    Ok(contents)
}

fn parse_stream_text(text: &str) -> Option<String> {
    let json: Value = serde_json::from_str(text).ok()?;
    let choices = json["choices"].as_array()?;
    let content = choices[0]["delta"]["content"].as_str()?;
    Some(String::from(content))
}
