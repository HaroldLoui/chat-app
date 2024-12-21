// #![allow(unused)]

mod commands;
mod core;
mod serializer;

use commands::*;
pub use serializer::*;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|_app| {
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_chat_sqlite::init())
        .plugin(tauri_plugin_chat_openai::init())
        .plugin(tauri_plugin_chat_request::init())
        .invoke_handler(tauri::generate_handler![
            list_message,
            insert_message,
            insert_chat_box,
            list_chat_box,
            update_chat_box_title,
            delete_chat_box,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
