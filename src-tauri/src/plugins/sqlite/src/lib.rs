use std::sync::Mutex;

use anyhow::Result;
use error::SqlError;
use rusqlite::Connection;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
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

const API_CONFIG_TABLE_NAME: &'static str = "api_config";
const CRATE_API_CONFIG_TABLE_SQL: &'static str = "
CREATE TABLE api_config (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    url TEXT NULL,
    key TEXT NULL,
    is_default INTEGER DEFAULT 0
)
";

const GLOBAL_CONFIG_TABLE_NAME: &'static str = "global_config";
const CRATE_GLOBAL_CONFIG_TABLE_SQL: &'static str = "
CREATE TABLE global_config (
    id INTEGER PRIMARY KEY,
    enable_stream INTEGER DEFAULT 0,
    associated_context INTEGER DEFAULT 0
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
    let db_sqls = vec![
        (CRATE_CHAT_BOX_TABLE_SQL, CHAT_BOX_TABLE_NAME),
        (CRATE_MESSAGE_TABLE_SQL, MESSAGE_TABLE_NAME),
        (CRATE_API_CONFIG_TABLE_SQL, API_CONFIG_TABLE_NAME),
        (CRATE_GLOBAL_CONFIG_TABLE_SQL, GLOBAL_CONFIG_TABLE_NAME),
        (CRATE_PROXY_TABLE_SQL, PROXY_TABLE_NAME),
    ];
    for (create_table_sql, table_name) in db_sqls {
        init_table(conn, create_table_sql, table_name)?;
    }
    Ok(())
}

fn init_table(conn: &Connection, crate_sql: &str, table_name: &str) -> Result<()> {
    let sql = format!("SELECT id FROM {}", table_name);
    if let Err(e) = conn.prepare(&sql) {
        match SqlError::new(e) {
            SqlError::NoSuchTable => {
                conn.execute(crate_sql, ())?;
                match table_name {
                    PROXY_TABLE_NAME => {
                        conn.execute("INSERT INTO proxy(id) VALUES (?1);", (0,))?;
                    }
                    GLOBAL_CONFIG_TABLE_NAME => {
                        conn.execute("INSERT INTO global_config(id) VALUES (?1);", (0,))?;
                    }
                    _ => {}
                }
            }
            _ => unreachable!(),
        }
    }
    Ok(())
}
