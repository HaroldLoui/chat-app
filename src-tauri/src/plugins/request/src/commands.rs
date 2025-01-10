use std::{sync::Mutex, time::Duration};

use rusqlite::Connection;
use super::proxy::{self as mapper, AppProxy};
use tauri::{http::StatusCode, State};
use tauri_plugin_http::reqwest::Client;

type CmdResult<T = ()> = Result<T, String>;

#[tauri::command]
pub fn query_proxy(conn: State<'_, Mutex<Connection>>) -> CmdResult<AppProxy> {
    let conn = conn.lock().unwrap();
    mapper::query_proxy(&conn).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_proxy(
    conn: State<'_, Mutex<Connection>>,
    client: State<'_, Mutex<Client>>,
    entity: AppProxy,
) -> CmdResult {
    let conn = conn.lock().unwrap();
    // 更新成功则修改Client
    match mapper::update_proxy(&conn, entity) {
        Ok(_) => {
            let app_proxy = mapper::query_proxy(&conn).ok();
            let mut client = client.lock().unwrap();
            *client = AppProxy::create_client(app_proxy);
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
pub fn enable_proxy(
    conn: State<'_, Mutex<Connection>>,
    client: State<'_, Mutex<Client>>,
    enable: bool,
) -> CmdResult {
    let conn = conn.lock().unwrap();
    match mapper::enable_proxy(&conn, enable) {
        Ok(_) => {
            let app_proxy = mapper::query_proxy(&conn).ok();
            let mut client = client.lock().unwrap();
            *client = AppProxy::create_client(app_proxy);
            Ok(())
        }
        Err(e) => Err(e.to_string()),
    }
}

const TEST_PROXY_URL: &str = "https://www.google.com";
#[tauri::command]
pub async fn check_proxy(entity: AppProxy) -> Result<bool, bool> {
    let client = AppProxy::create_client(Some(entity));
    let response_res = client
        .get(TEST_PROXY_URL)
        .timeout(Duration::from_secs(1))
        .send()
        .await;
    match response_res {
        Ok(response) => match response.status() {
            StatusCode::OK => Ok(true),
            _ => Err(false),
        },
        Err(_) => Err(false),
    }
}
