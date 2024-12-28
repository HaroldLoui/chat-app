use std::sync::Mutex;

use rusqlite::Connection;
use tauri::State;

use crate::core::mapper;
use crate::core::models::*;

#[tauri::command(rename_all = "snake_case")]
pub fn list_message(
    conn: State<'_, Mutex<Connection>>,
    chat_id: String,
    page_num: usize,
) -> Result<Vec<Message>, String> {
    let conn = conn.lock().unwrap();
    let cursor = (page_num - 1) * 10;
    mapper::query_message_list(
        &conn,
        i64::from_str_radix(&chat_id, 10).unwrap_or_default(),
        cursor,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub fn insert_message(
    conn: State<'_, Mutex<Connection>>,
    chat_id: String,
    content: String,
    sender: Sender,
) -> Result<Message, String> {
    let conn = conn.lock().unwrap();
    let message = Message {
        chat_id: i64::from_str_radix(&chat_id, 10).unwrap_or_default(),
        content,
        ..Message::from(sender)
    };
    let id = message.id.clone();
    mapper::insert_message(&conn, message).map_err(|e| e.to_string())?;
    mapper::query_message_by_id(&conn, id).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn insert_chat_box(conn: State<'_, Mutex<Connection>>) -> Result<(), String> {
    let chat_box = ChatBox::default();
    let conn = conn.lock().unwrap();
    mapper::insert_chat_box(&conn, chat_box).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_chat_box(conn: State<'_, Mutex<Connection>>, title: String) -> Result<Vec<ChatBox>, String> {
    let conn = conn.lock().unwrap();
    mapper::query_chat_box_list(&conn, Some(title)).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_chat_box_title(conn: State<'_, Mutex<Connection>>, id: String, title: String) -> Result<(), String> {
    let conn = conn.lock().unwrap();
    mapper::update_chat_box_title(
        &conn,
        i64::from_str_radix(&id, 10).unwrap_or_default(),
        title,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_chat_box(conn: State<'_, Mutex<Connection>>, id: String) -> Result<(), String> {
    let id = i64::from_str_radix(&id, 10).unwrap_or_default();
    let conn = conn.lock().unwrap();
    mapper::delete_chat_box(&conn, id).map_err(|e| e.to_string())
}
