use std::fmt::Display;
use chrono::{Local, NaiveDateTime};
use rusqlite::{types::{FromSql, FromSqlError}, ToSql};
use serde::{Deserialize, Serialize};
use snowflake::SnowflakeIdBucket;
use static_init::dynamic;

#[dynamic]
pub static mut ID_WORKER: SnowflakeIdBucket = SnowflakeIdBucket::new(1, 1);

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ChatBox {
    #[serde(with = "crate::big_number_serializer")]
    pub id: i64,
    pub title: String,
    pub count: usize,
    #[serde(with = "crate::datetime_format")]
    pub create_time: NaiveDateTime,
}

impl Default for ChatBox {
    fn default() -> Self {
        Self {
            id: ID_WORKER.write().get_id(),
            title: "默认对话".to_string(),
            count: 0,
            create_time: Local::now().naive_local(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Sender {
    AI,
    ME,
}

impl ToSql for Sender {
    fn to_sql(&self) -> rusqlite::Result<rusqlite::types::ToSqlOutput<'_>> {
        match self {
            Self::AI => "AI".to_sql(),
            Self::ME => "ME".to_sql(),
        }
    }
}

impl FromSql for Sender {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value.as_str()? {
            "AI" => Ok(Sender::AI),
            "ME" => Ok(Sender::ME),
            _ => Err(FromSqlError::InvalidType),
        }
    }
}

impl Display for Sender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AI => write!(f, "AI"),
            Self::ME => write!(f, "ME"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    #[serde(with = "crate::big_number_serializer")]
    pub id: i64,
    pub chat_id: i64,
    pub sender: Sender,
    pub content: String,
    #[serde(with = "crate::datetime_format")]
    pub create_time: NaiveDateTime,
}

impl Message {
    pub fn from(sender: Sender) -> Self {
        Self {
            id: ID_WORKER.write().get_id(),
            chat_id: 0,
            sender,
            content: "".to_string(),
            create_time: Local::now().naive_local(),
        }
    }
}