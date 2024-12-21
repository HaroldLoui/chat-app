use anyhow::Result;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use tauri_plugin_http::reqwest::{Client, Proxy};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppProxy {
    pub id: u8,
    pub proxy_address: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub is_enable: bool,
}

impl AppProxy {
    pub fn create_client(app_proxy: Option<AppProxy>) -> Client {
        let builder = Client::builder();
        if let Some(app_proxy) = app_proxy {
            let address = app_proxy.proxy_address.unwrap_or_default();
            if address.is_empty() || !app_proxy.is_enable {
                return builder.no_proxy().build().unwrap();
            }
            if let Ok(mut proxy) = Proxy::all(address) {
                let username = app_proxy.username.unwrap_or_default();
                let password = app_proxy.password.unwrap_or_default();
                if !username.is_empty() && !password.is_empty() {
                    proxy = proxy.basic_auth(&username, &password);
                }
                return builder.proxy(proxy).build().unwrap();
            }
        }
        builder.no_proxy().build().unwrap()
    }
}

pub fn query_proxy(conn: &Connection) -> Result<AppProxy> {
    let sql = "SELECT * FROM proxy WHERE id = 0";
    let mut stmt = conn.prepare(sql)?;
    let proxy = stmt.query_row([], |row| {
        Ok(AppProxy {
            id: row.get(0)?,
            proxy_address: row.get(1)?,
            username: row.get(2)?,
            password: row.get(3)?,
            is_enable: row.get(4)?,
        })
    })?;
    Ok(proxy)
}

pub fn update_proxy(conn: &Connection, entity: AppProxy) -> Result<()> {
    let mut conditions = Vec::new();
    conditions.push(format!("proxy_address = {}", entity.proxy_address.unwrap_or_default()));
    conditions.push(format!("username = {}", entity.username.unwrap_or_default()));
    conditions.push(format!("password = {}", entity.password.unwrap_or_default()));
    let sql = format!("UPDATE proxy SET {} WHERE id = 0", conditions.join(", "));
    println!("{}", sql);
    conn.execute(&sql, [])?;
    Ok(())
}

pub fn enable_proxy(conn: &Connection, enable: bool) -> Result<()> {
    conn.execute("UPDATE proxy SET is_enable = ?1 WHERE id = 0", (enable,))?;
    Ok(())
}