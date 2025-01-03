use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiConfig {
    pub id: u8,
    pub url: Option<String>,
    pub key: Option<String>,
    pub is_default: bool,
}

pub fn query_default_config(conn: &Connection) -> Result<ApiConfig> {
    let sql = "SELECT id, url, key, is_default FROM api_config WHERE is_default = 1";
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

pub fn query_config_list(conn: &Connection) -> Result<Vec<ApiConfig>> {
    let sql = "SELECT * FROM api_config ORDER BY is_default DESC";
    let mut stmt = conn.prepare(sql)?;
    let iter = stmt.query_map([], |row| {
        Ok(ApiConfig {
            id: row.get(0)?,
            url: row.get(1)?,
            key: row.get(2)?,
            is_default: row.get(3)?,
        })
    })?;
    let mut list = Vec::new();
    for res in iter {
        if let Ok(config) = res {
            list.push(config);
        }
    }
    Ok(list)
}

pub fn query_enable_stream(conn: &Connection) -> Result<bool> {
    let sql = "SELECT enable_stream FROM global_config WHERE id = 0";
    let mut stmt = conn.prepare(sql)?;
    let stream = stmt.query_row([], |row| {
        Ok(row.get(0)?)
    })?;
    Ok(stream)
}

#[allow(unused)]
pub fn query_associated_context(conn: &Connection) -> Result<bool> {
    let sql = "SELECT associated_context FROM global_config WHERE id = 0";
    let mut stmt = conn.prepare(sql)?;
    let context = stmt.query_row([], |row| {
        Ok(row.get(0)?)
    })?;
    Ok(context)
}

pub fn insert_api_config(conn: &Connection, config: ApiConfig) -> Result<()> {
    if config.is_default {
        conn.execute("UPDATE api_config SET is_default = 0 WHERE is_default != 0", ())?;
    }
    conn.execute(
        "INSERT INTO api_config(url, key, is_default) VALUES(?1, ?2, ?3)",
        (&config.url, &config.key, &config.is_default)
    )?;
    Ok(())
}

pub fn update_default_config(conn: &Connection, id: u32) -> Result<()> {
    conn.execute("UPDATE api_config SET is_default = 0 WHERE is_default != 0", ())?;
    conn.execute("UPDATE api_config SET is_default = 1 WHERE id = ?1", (&id,))?;
    Ok(())
}

pub fn delete_api_config(conn: &Connection, id: u32) -> Result<()> {
    conn.execute("DELETE FROM api_config WHERE id = ?1", (&id,))?;
    Ok(())
}