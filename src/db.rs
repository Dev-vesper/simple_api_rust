use anyhow::Result;
use rusqlite::{params, Connection};
use std::fs;
use std::path::Path;

use crate::models::{CreateUser, UpdateUser, User};

const DEFAULT_DB_PATH: &str = "data/app.db";

fn get_db_path() -> String {
    std::env::var("DB_PATH").unwrap_or_else(|_| DEFAULT_DB_PATH.to_string())
}

fn ensure_db_dir() -> Result<()> {
    let db_path = get_db_path();
    let path = Path::new(&db_path);
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }
    Ok(())
}

fn open_connection() -> Result<Connection> {
    ensure_db_dir()?;
    let conn = Connection::open(get_db_path())?;
    Ok(conn)
}

pub fn init_db() -> Result<()> {
    let conn = open_connection()?;
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

pub fn get_all_users() -> Result<Vec<User>> {
    let conn = open_connection()?;
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

pub fn add_user(data: CreateUser) -> Result<User> {
    let conn = open_connection()?;
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

pub fn update_user(id: i64, data: UpdateUser) -> Result<bool> {
    let conn = open_connection()?;
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

pub fn delete_user(id: i64) -> Result<bool> {
    let conn = open_connection()?;
    let affected = conn.execute("DELETE FROM users WHERE id = ?1", params![id])?;
    Ok(affected > 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_and_get_users() {
        std::env::set_var("DB_PATH", "test_data/test.db");
        init_db().unwrap();
        let user = add_user(CreateUser {
            name: "Test".to_string(),
            age: 20,
        })
        .unwrap();
        assert_eq!(user.name, "Test");
        let users = get_all_users().unwrap();
        assert_eq!(users.len(), 1);
    }
}
