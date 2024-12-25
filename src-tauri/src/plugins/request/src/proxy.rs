use anyhow::Result;
use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use tauri_plugin_http::reqwest::{Client, Proxy};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppProxy {
    pub id: u8,
    pub host: Option<String>,
    pub port: Option<u16>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub is_enable: bool,
    pub authentication: bool,
}

impl AppProxy {
    fn address(&self) -> Option<String> {
        if self.host.is_none() || self.port.is_none() {
            return None;
        }
        let address = format!("{}:{}", self.host.clone().unwrap(), self.port.unwrap());
        Some(address)
    }

    fn auth(&self) -> bool {
        if !self.authentication {
            return false;
        }
        let username = self.username.clone();
        let password = self.password.clone();
        username.is_some_and(|u| !u.is_empty() && password.is_some_and(|p| !p.is_empty()))
    }

    pub fn create_client(app_proxy: Option<AppProxy>) -> Client {
        let builder = Client::builder();
        if let Some(app_proxy) = app_proxy {
            let address = app_proxy.address().unwrap_or_default();
            if address.is_empty() || !app_proxy.is_enable {
                return builder.no_proxy().build().unwrap();
            }
            if let Ok(mut proxy) = Proxy::all(address) {
                if app_proxy.authentication {
                    let username = app_proxy.username.unwrap_or_default();
                    let password = app_proxy.password.unwrap_or_default();
                    if !username.is_empty() && !password.is_empty() {
                        proxy = proxy.basic_auth(&username, &password);
                    }
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
            host: row.get(1)?,
            port: row.get(2)?,
            username: row.get(3)?,
            password: row.get(4)?,
            is_enable: row.get(5)?,
            authentication: row.get(6)?,
        })
    })?;
    Ok(proxy)
}

pub fn update_proxy(conn: &Connection, entity: AppProxy) -> Result<()> {
    let mut conditions = Vec::new();
    conditions.push("host = ?1");
    conditions.push("port = ?2");
    conditions.push("username = ?3");
    conditions.push("password = ?4");
    conditions.push("is_enable = ?5");
    conditions.push("authentication = ?6");
    let sql = format!("UPDATE proxy SET {} WHERE id = 0", conditions.join(", "));
    let authentication = entity.auth();
    conn.execute(
        &sql,
        (
            entity.host,
            entity.port,
            entity.username,
            entity.password,
            entity.is_enable,
            authentication,
        ),
    )?;
    Ok(())
}

pub fn enable_proxy(conn: &Connection, enable: bool) -> Result<()> {
    conn.execute("UPDATE proxy SET is_enable = ?1 WHERE id = 0", (enable,))?;
    Ok(())
}
