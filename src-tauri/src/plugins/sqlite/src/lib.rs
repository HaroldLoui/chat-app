use std::sync::Mutex;

use anyhow::Result;
use error::SqlError;
use rusqlite::Connection;
use tauri::{
    plugin::{Builder, TauriPlugin}, Manager, Runtime
};

mod error;

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("chat-sqlite")
        .setup(|app, _| {
            let app_path = app.path();
            let resource_path = app_path.resource_dir()?;
            let db_path = resource_path.join("data.db");
            let conn = Connection::open(db_path.clone())?;
            init_db(&conn)?;
            app.manage(Mutex::new(conn));
            Ok(())
        })
        .build()
}

const CHAT_BOX_TABLE_NAME: &'static str = "chat_box";
const CRATE_CHAT_BOX_TABLE_SQL: &'static str = "
CREATE TABLE chat_box (
    id    INTEGER PRIMARY KEY,
    title  TEXT NOT NULL,
    count  INTEGER NOT NULL DEFAULT 0,
    create_time TEXT NOT NULL
)
";

const MESSAGE_TABLE_NAME: &'static str = "message";
const CRATE_MESSAGE_TABLE_SQL: &'static str = "
CREATE TABLE message (
    id    INTEGER PRIMARY KEY,
    chat_id INTEGER NOT NULL,
    sender  TEXT NOT NULL,
    content  TEXT NOT NULL,
    create_time TEXT NOT NULL
)
";

const CONFIG_TABLE_NAME: &'static str = "config";
const CRATE_CONFIG_TABLE_SQL: &'static str = "
CREATE TABLE config (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    url TEXT NULL,
    key TEXT NULL,
    is_default INTEGER DEFAULT 0
)
";

const PROXY_TABLE_NAME: &'static str = "proxy";
const CRATE_PROXY_TABLE_SQL: &'static str = "
CREATE TABLE proxy (
    id INTEGER PRIMARY KEY,
    host TEXT NULL,
    port INTEGER NULL,
    username TEXT NULL,
    password TEXT NULL,
    is_enable INTEGER NOT NULL DEFAULT 0,
    authentication INTEGER NOT NULL DEFAULT 0
);
";

pub fn init_db(conn: &Connection) -> Result<()> {
    init_table(conn, CRATE_CHAT_BOX_TABLE_SQL, CHAT_BOX_TABLE_NAME)?;
    init_table(conn, CRATE_MESSAGE_TABLE_SQL, MESSAGE_TABLE_NAME)?;
    init_table(conn, CRATE_CONFIG_TABLE_SQL, CONFIG_TABLE_NAME)?;
    init_table(conn, CRATE_PROXY_TABLE_SQL, PROXY_TABLE_NAME)?;
    Ok(())
}

fn init_table(conn: &Connection, crate_sql: &str, table_name: &str) -> Result<()> {
    let sql = format!("SELECT id FROM {}", table_name);
    if let Err(e) = conn.prepare(&sql) {
        match SqlError::new(e) {
            SqlError::NoSuchTable => {
                conn.execute(crate_sql, ())?;
                if PROXY_TABLE_NAME == table_name {
                    conn.execute("INSERT INTO proxy(id) VALUES (?1);", (0,))?;
                }
            }
            _ => unreachable!(),
        }
    }
    Ok(())
}

