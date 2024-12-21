use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub enum SqlError {
    NoSuchTable,
    SqliteError(rusqlite::Error),
}

impl SqlError {
    pub fn new(e: rusqlite::Error) -> Self {
        let s = e.to_string();
        if s.starts_with("no such table") {
            SqlError::NoSuchTable
        } else {
            SqlError::SqliteError(e)
        }
    }
}

impl Display for SqlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SqlError::NoSuchTable => writeln!(f, "no such table."),
            SqlError::SqliteError(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for SqlError {}
