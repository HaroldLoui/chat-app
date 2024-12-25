use std::sync::Mutex;

use chrono::DateTime;
use rusqlite::Connection;
use serde_json::Value;
use tauri::{AppHandle, Emitter, Runtime, State};
use tauri_plugin_http::reqwest::Client;

use crate::mapper::ApiConfig;
use crate::models::*;
use crate::mapper;

#[tauri::command]
pub fn list_api_config(conn: State<'_, Mutex<Connection>>) -> Result<Vec<ApiConfig>, String> {
    let conn = conn.lock().unwrap();
    mapper::query_config_list(&conn).map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub fn insert_api_config(conn: State<'_, Mutex<Connection>>, eneity: ApiConfig) -> Result<(), String> {
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

#[tauri::command(rename_all = "snake_case")]
pub async fn send_message<R: Runtime>(
    app: AppHandle<R>,
    client: State<'_, Mutex<Client>>,
    conn: State<'_, Mutex<Connection>>,
    content: String,
) -> Result<(), String> {
    let app_clone = app.clone();
    let client = client.lock().unwrap().clone();
    let conn = conn.lock().unwrap();
    let config = mapper::query_default_config(&conn).unwrap_or_default();
    tokio::spawn(async move {
        let url = config.url.unwrap_or_default();
        if url.is_empty() {
            let _ = app_clone.emit("chat:message://received", "请配置接口地址。");
            return;
        }
        let key = config.key.unwrap_or_default();
        if key.is_empty() {
            let _ = app_clone.emit("chat:message://received", "请配置密钥。");
            return;
        }
        let message = RequestMessage::new_text(MessageRole::User, content);
        let body = CompletionRequestBuilder::default()
            .model("gpt-4o-mini".to_string())
            .messages(vec![message])
            .stream(false)
            .build()
            .unwrap();
        let res = client.post(url)
            .header("Authorization", format!("Bearer {}", key))
            .json(&body)
            .send()
            .await;
        match res {
            Ok(response) => {
                let text_res = response.text().await;
                match text_res {
                    Ok(text) => {
                        match parse_text(&text) {
                            Some(response) => {
                                let _ = app_clone.emit("chat:message://received_time", response.created);
                                let _ = app_clone.emit("chat:message://received", response.content);
                            },
                            None => {
                                let _ = app_clone.emit("chat:message://received", format!("解析响应内容失败: {}", text));
                            },
                        }
                    },
                    Err(e) => {
                        let _ = app_clone.emit("chat:message://received", e.to_string());
                    }
                }
            }
            Err(e) => {
                let _ = app_clone.emit("chat:message://received", e.to_string());
            },
        }
    });
    Ok(())
}

#[derive(Debug)]
struct ApiResponse {
    created: String,
    content: String,
}
fn parse_text(text: &str) -> Option<ApiResponse> {
    let json: Value = serde_json::from_str(text).unwrap();
        let created= json["created"].as_number()?.as_i64()?;
        let date_time = DateTime::from_timestamp(created, 0);
        let choices = json["choices"].as_array()?;
        let content = choices[0]["message"]["content"].as_str()?;
        Some(ApiResponse {
            created: date_time?.naive_local().format("%Y年%m月%d日 %H:%M:%S").to_string(),
            content: String::from(content),
        })
}

#[cfg(test)]
mod tests {
    use chrono::DateTime;
    use serde_json::Value;

    use super::*;

    #[test]
    fn test() {
        let text = include_str!("./response.json");
        let response = parse_text1(text);
        println!("{:?}", response);
    }
    fn parse_text1(text: &str) -> Option<ApiResponse> {
        let json: Value = serde_json::from_str(&text).unwrap();
        let created= json["created"].as_number()?.as_i64()?;
        let date_time = DateTime::from_timestamp(created, 0);
        let choices = json["choices"].as_array()?;
        let content = choices[0]["message"]["content"].as_str()?;
        Some(ApiResponse {
            created: date_time?.naive_local().format("%Y年%m月%d日 %H:%M:%S").to_string(),
            content: String::from(content),
        })
    }
}