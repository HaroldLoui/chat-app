use std::sync::Mutex;

use rusqlite::Connection;
use tauri::State;

use crate::core::mapper;
use crate::core::models::*;

type CmdResult<T = ()> = Result<T, String>;

#[tauri::command(rename_all = "snake_case")]
pub fn list_message(
    conn: State<'_, Mutex<Connection>>,
    chat_id: String,
) -> CmdResult<Vec<Message>> {
    let conn = conn.lock().unwrap();
    mapper::query_message_list(&conn, i64::from_str_radix(&chat_id, 10).unwrap_or_default())
        .map_err(|e| e.to_string())
}

#[tauri::command(rename_all = "snake_case")]
pub fn insert_message(
    conn: State<'_, Mutex<Connection>>,
    chat_id: String,
    content: String,
    sender: Sender,
) -> CmdResult<Message> {
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
pub fn insert_chat_box(conn: State<'_, Mutex<Connection>>) -> CmdResult {
    let chat_box = ChatBox::default();
    let conn = conn.lock().unwrap();
    mapper::insert_chat_box(&conn, chat_box).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn list_chat_box(conn: State<'_, Mutex<Connection>>, title: String) -> CmdResult<Vec<ChatBox>> {
    let conn = conn.lock().unwrap();
    mapper::query_chat_box_list(&conn, Some(title)).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn update_chat_box_title(
    conn: State<'_, Mutex<Connection>>,
    id: String,
    title: String,
) -> CmdResult {
    let conn = conn.lock().unwrap();
    mapper::update_chat_box_title(
        &conn,
        i64::from_str_radix(&id, 10).unwrap_or_default(),
        title,
    )
    .map_err(|e| e.to_string())
}

#[tauri::command]
pub fn delete_chat_box(conn: State<'_, Mutex<Connection>>, id: String) -> CmdResult {
    let id = i64::from_str_radix(&id, 10).unwrap_or_default();
    let conn = conn.lock().unwrap();
    mapper::delete_chat_box(&conn, id).map_err(|e| e.to_string())
}
