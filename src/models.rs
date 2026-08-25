use serde::{Deserialize, Serialize};

/// A user as stored in the database and returned by the API.
/// `Serialize` allows conversion to JSON for responses.
/// `Deserialize` is used when we need to accept a full user (though currently not needed).
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub age: i32,
}

/// Payload for creating a new user.
/// `id` is intentionally omitted – the database generates it automatically.
#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub age: i32,
}

/// Payload for updating a user.
/// All fields are optional (`Option`), so clients can send only the fields they want to change.
#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub name: Option<String>,
    pub age: Option<i32>,
}
