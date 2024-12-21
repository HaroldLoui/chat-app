use std::sync::Mutex;

use rusqlite::Connection;
use serde_json::Value;
use tauri::{AppHandle, Emitter, Runtime, State};
use tauri_plugin_http::reqwest::Client;

use crate::models::*;
use crate::mapper;

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
        // println!("{}", serde_json::to_string(&body).unwrap());
        let res = client.post(url)
            // .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", key))
            .json(&body)
            .send()
            .await;
        match res {
            Ok(response) => {
                let text = response.text().await.unwrap();
                println!("解析响应内容: {}", text);
                let json = Value::String(text);
                if let Some(created) = json["created"].as_u64() {
                    let _ = app_clone.emit("chat:message://received_time", created);
                    if let Some(choices) = json["choices"].as_array() {
                        if let Some(content) = choices[0]["message"]["content"].as_str() {
                            let _ = app_clone.emit("chat:message://received", content);
                        } else {
                            let _ = app_clone.emit("chat:message://received", "解析响应内容失败");
                        }
                    } else {
                        let _ = app_clone.emit("chat:message://received", "解析响应内容失败");
                    }
                } else {
                    let _ = app_clone.emit("chat:message://received", "解析响应内容失败");
                }
            }
            Err(e) => {
                let _ = app_clone.emit("chat:message://received", e.to_string());
            },
        }
    });
    Ok(())
}
