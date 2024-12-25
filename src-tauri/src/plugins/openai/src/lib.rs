use tauri::{
    generate_handler, plugin::{Builder, TauriPlugin}, Runtime
};

mod commands;
mod mapper;
mod models;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("chat-openai")
        .invoke_handler(generate_handler![
            commands::send_message,
            commands::list_api_config,
            commands::insert_api_config,
            commands::set_default_config,
            commands::delete_api_config,
        ])
        .build()
}
