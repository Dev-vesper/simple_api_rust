use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub age: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub age: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub age: Option<i32>,
}
