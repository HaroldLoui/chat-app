use std::sync::Mutex;

use rusqlite::Connection;
use tauri::State;
use tauri_plugin_http::reqwest::Client;

use super::proxy::{self as mapper, AppProxy};

#[tauri::command]
pub fn query_proxy(conn: State<'_, Mutex<Connection>>) -> Result<AppProxy, String> {
    let conn = conn.lock().unwrap();
    mapper::query_proxy(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_proxy(
    conn: State<'_, Mutex<Connection>>, 
    client: State<'_, Mutex<Client>>, 
    entity: AppProxy
) -> Result<(), String> {
    let conn = conn.lock().unwrap();
    // 更新成功则修改Client
    match mapper::update_proxy(&conn, entity) {
        Ok(_) => {
            let app_proxy = mapper::query_proxy(&conn).ok();
            let mut client = client.lock().unwrap();
            *client = AppProxy::create_client(app_proxy);
            Ok(())
        },
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn enable_proxy(
    conn: State<'_, Mutex<Connection>>, 
    client: State<'_, Mutex<Client>>, 
    enable: bool
) -> Result<(), String> {
    let conn = conn.lock().unwrap();
    match mapper::enable_proxy(&conn, enable) {
        Ok(_) => {
            let app_proxy = mapper::query_proxy(&conn).ok();
            let mut client = client.lock().unwrap();
            *client = AppProxy::create_client(app_proxy);
            Ok(())
        },
        Err(e) => Err(e.to_string()),
    }
}