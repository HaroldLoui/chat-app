use std::sync::Mutex;

use rusqlite::Connection;
use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("chat-sqlite")
        .setup(|app, _| {
            let app_path = app.path();
            let resource_path = app_path.resource_dir()?;
            let db_path = resource_path.join("data.db");
            let conn = Connection::open(db_path)?;
            conn.execute_batch(INIT_DB_SQL)?;
            app.manage(Mutex::new(conn));
            Ok(())
        })
        .build()
}

const INIT_DB_SQL: &str = "
BEGIN;
CREATE TABLE IF NOT EXISTS `chat_box`(
    id          INTEGER PRIMARY KEY,
    title       TEXT              NOT NULL,
    count       INTEGER DEFAULT 0 NOT NULL,
    create_time TEXT              NOT NULL
);
CREATE TABLE IF NOT EXISTS `message`(
    id          INTEGER PRIMARY KEY,
    chat_id     INTEGER NOT NULL,
    sender      TEXT NOT NULL,
    content     TEXT NOT NULL,
    create_time TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS `api_config`(
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    url        TEXT,
    key        TEXT,
    is_default INTEGER DEFAULT 0
);
CREATE TABLE IF NOT EXISTS `global_config`(
    id                 INTEGER PRIMARY KEY,
    enable_stream      INTEGER DEFAULT 0,
    associated_context INTEGER DEFAULT 0
);
CREATE TABLE IF NOT EXISTS `proxy`(
    id             INTEGER PRIMARY KEY,
    host           TEXT NULL,
    port           INTEGER NULL,
    username       TEXT NULL,
    password       TEXT NULL,
    is_enable      INTEGER NOT NULL DEFAULT 0,
    authentication INTEGER NOT NULL DEFAULT 0
);
INSERT OR IGNORE INTO `global_config`(id) VALUES (0);
INSERT OR IGNORE INTO `proxy`(id) VALUES (0);
COMMIT;
";
