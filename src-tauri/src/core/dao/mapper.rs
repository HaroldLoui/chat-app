use anyhow::Result;
use rusqlite::{named_params, Connection};

use super::models::*;

pub fn query_chat_box_list(conn: &Connection, title: Option<String>) -> Result<Vec<ChatBox>> {
    let title = title.unwrap_or_default();
    let mut sql = String::from("SELECT id, title, count, create_time FROM chat_box");
    if title.len() > 0 {
        sql += &format!(" WHERE title LIKE '%{}%'", title);
    }
    let mut stmt = conn.prepare(&sql)?;
    let chat_iter = stmt.query_map([], |row| {
        Ok(ChatBox {
            id: row.get(0)?,
            title: row.get(1)?,
            count: row.get(2)?,
            create_time: row.get(3)?,
        })
    })?;
    let mut list = Vec::new();
    for chat_res in chat_iter {
        if let Ok(chat) = chat_res {
            list.push(chat);
        }
    }
    Ok(list)
}

pub fn insert_chat_box(conn: &Connection, chat_box: ChatBox) -> Result<()> {
    conn.execute(
        "INSERT INTO chat_box(id, title, create_time) VALUES (?1, ?2, ?3)",
        (&chat_box.id, &chat_box.title, &chat_box.create_time),
    )?;
    Ok(())
}

pub fn update_chat_box_title(conn: &Connection, id: i64, title: String) -> Result<()> {
    conn.execute("UPDATE chat_box SET title = ?1 WHERE id = ?2", (&title, id))?;
    Ok(())
}

pub fn delete_chat_box(conn: &Connection, id: i64) -> Result<()> {
    conn.execute("DELETE FROM message WHERE chat_id = ?1", (id,))?;
    conn.execute("DELETE FROM chat_box WHERE id = ?1", (id,))?;
    Ok(())
}

pub fn insert_message(conn: &Connection, message: Message) -> Result<()> {
    conn.execute(
        "INSERT INTO message(id, chat_id, sender, content, create_time) VALUES (?1, ?2, ?3, ?4, ?5)",
        (
            &message.id,
            &message.chat_id,
            &message.sender,
            &message.content,
            &message.create_time,
        ),
    )?;
    conn.execute("UPDATE chat_box SET count = count + 1 WHERE id = ?1", (&message.chat_id,))?;
    Ok(())
}

pub fn query_message_by_id(conn: &Connection, id: i64) -> Result<Message> {
    let sql = "SELECT id, chat_id, sender, content, create_time FROM message WHERE id = :id";
    let mut stmt = conn.prepare(sql)?;
    let message = stmt.query_row(
        named_params! { ":id": id }, 
        |row| {
            Ok(Message {
                id: row.get(0)?,
                chat_id: row.get(1)?,
                sender: row.get(2)?,
                content: row.get(3)?,
                create_time: row.get(4)?,
            })
        }
    )?;
    Ok(message)
}

pub fn query_message_list(conn: &Connection, chat_id: i64) -> Result<Vec<Message>> {
    let sql = "
        SELECT id, chat_id, sender, content, create_time FROM message 
        WHERE chat_id = :chat_id 
        ORDER BY DATETIME(create_time) DESC, sender
    ";
    let mut stmt = conn.prepare(sql)?;
    let iter = stmt.query_map(
        named_params! { ":chat_id": chat_id },
        |row| {
            Ok(Message {
                id: row.get(0)?,
                chat_id: row.get(1)?,
                sender: row.get(2)?,
                content: row.get(3)?,
                create_time: row.get(4)?,
            })
        },
    )?;
    let mut list = Vec::new();
    for msg in iter {
        if let Ok(msg) = msg {
            list.push(msg);
        }
    }
    list.reverse();
    Ok(list)
}
