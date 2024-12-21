use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ApiConfig {
    pub id: u8,
    pub url: Option<String>,
    pub key: Option<String>,
    pub is_default: bool,
}

pub fn query_default_config(conn: &Connection) -> Result<ApiConfig> {
    let sql = "SELECT id, url, key, is_default FROM config WHERE is_default = 1";
    let mut stmt = conn.prepare(sql)?;
    let config = stmt.query_row([], |row| {
        Ok(ApiConfig {
            id: row.get(0)?,
            url: row.get(1)?,
            key: row.get(2)?,
            is_default: row.get(3)?,
        })
    })?;
    Ok(config)
}