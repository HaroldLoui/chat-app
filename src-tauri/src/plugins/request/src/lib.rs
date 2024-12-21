use std::sync::Mutex;

use rusqlite::Connection;
use tauri::{
    plugin::{Builder, TauriPlugin}, Manager, Runtime
};

mod commands;
pub mod proxy;

use proxy::{self as mapper, AppProxy};
use commands::*;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("chat-request")
        .setup(|app, _| {
            let state = app.state::<Mutex<Connection>>();
            let conn = state.lock().unwrap();
            let proxy = mapper::query_proxy(&conn).ok();
            let client = AppProxy::create_client(proxy);
            app.manage(Mutex::new(client));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            query_proxy,
            update_proxy,
            enable_proxy
        ])
        .build()
}
