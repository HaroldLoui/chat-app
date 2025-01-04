use std::sync::Mutex;
use std::time::Duration;

use rusqlite::Connection;
use tauri::{AppHandle, Emitter, Runtime, State};
use tauri_plugin_http::reqwest::Client;

use crate::response::{ContextMessage, OpenAiClient, Params, ResponseContent};
use crate::mapper;
use crate::mapper::ApiConfig;

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
pub fn update_enable_stream(
    conn: State<'_, Mutex<Connection>>,
    stream: bool,
) -> Result<(), String> {
    let conn = conn.lock().unwrap();
    mapper::update_enable_stream(&conn, stream).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn query_associated_context(conn: State<'_, Mutex<Connection>>) -> Result<bool, String> {
    let conn = conn.lock().unwrap();
    mapper::query_associated_context(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_associated_context(
    conn: State<'_, Mutex<Connection>>,
    context: bool,
) -> Result<(), String> {
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
    context: Option<ContextMessage>,
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
    let http_client = OpenAiClient::new(&client);
    let params = Params::from(url, key, stream);
    match http_client.send(params, content, context).await {
        ResponseContent::Text(content) => {
            let _ = app.emit(MESSAGE_EVENT_NAME, content);
        }
        ResponseContent::Array(contents) => {
            for content in contents {
                let _ = app.emit(STREAM_MESSAGE_EVENT_NAME, content);
                std::thread::sleep(Duration::from_millis(100));
            }
        }
    }
    Ok(())
}

