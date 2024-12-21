use tauri::{
    generate_handler, plugin::{Builder, TauriPlugin}, Runtime
};

mod commands;
mod mapper;
mod models;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("chat-openai")
        .invoke_handler(generate_handler![commands::send_message])
        .build()
}
