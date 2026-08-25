use anyhow::Result;
use rusqlite::{params, Connection};
use std::fs;
use std::path::{Path, PathBuf};

use crate::models::{CreateUser, UpdateUser, User};

/// a handle to the sqlite database
/// It stores the path to the database file and provides crud operations
pub struct Database {
    path: PathBuf,
}

impl Database {
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        let db = Database { path };
        db.init_db()?;
        Ok(db)
    }

    /// opens a new sqlite connection to the database file
    fn open_connection(&self) -> Result<Connection> {
        if let Some(parent) = self.path.parent() {
            fs::create_dir_all(parent)?;
        }
        let conn = Connection::open(&self.path)?;
        Ok(conn)
    }

    fn init_db(&self) -> Result<()> {
        let conn = self.open_connection()?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS users (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                age INTEGER NOT NULL
            )",
            [],
        )?;
        Ok(())
    }

    pub fn get_all_users(&self) -> Result<Vec<User>> {
        let conn = self.open_connection()?;
        let mut stmt = conn.prepare("SELECT id, name, age FROM users")?;

        let users_iter = stmt.query_map([], |row| {
            Ok(User {
                id: row.get(0)?,
                name: row.get(1)?,
                age: row.get(2)?,
            })
        })?;

        let mut users = Vec::new();
        for user in users_iter {
            users.push(user?);
        }
        Ok(users)
    }

    pub fn add_user(&self, data: CreateUser) -> Result<User> {
        let conn = self.open_connection()?;
        conn.execute(
            "INSERT INTO users (name, age) VALUES (?1, ?2)",
            params![data.name, data.age],
        )?;
        let id = conn.last_insert_rowid();
        Ok(User {
            id,
            name: data.name,
            age: data.age,
        })
    }

    /// updates a users fields Only Some fields are changed
    pub fn update_user(&self, id: i64, data: UpdateUser) -> Result<bool> {
        let conn = self.open_connection()?;
        let mut updated = false;

        if let Some(name) = data.name {
            let affected = conn.execute(
                "UPDATE users SET name = ?1 WHERE id = ?2",
                params![name, id],
            )?;
            updated |= affected > 0;
        }
        if let Some(age) = data.age {
            let affected = conn.execute(
                "UPDATE users SET age = ?1 WHERE id = ?2",
                params![age, id],
            )?;
            updated |= affected > 0;
        }
        Ok(updated)
    }

    pub fn delete_user(&self, id: i64) -> Result<bool> {
        let conn = self.open_connection()?;
        let affected = conn.execute("DELETE FROM users WHERE id = ?1", params![id])?;
        Ok(affected > 0)
    }
}
