use axum::{
    extract::{Path, Query},
    http::StatusCode,
    Json,
};
use serde::Deserialize;

use crate::db;
use crate::models::{CreateUser, UpdateUser, User};

/// Query parameters for the `/users/sorted` endpoint.
/// Both are optional; defaults are applied in the handler.
#[derive(Deserialize)]
pub struct SortParams {
    pub key: Option<String>,     // e.g., "id", "name", "age"
    pub reverse: Option<String>, // "true" or "false"
}

/// GET /users
/// Returns a JSON array of all users.
pub async fn list_users() -> Result<Json<Vec<User>>, (StatusCode, String)> {
    match db::get_all_users() {
        Ok(users) => Ok(Json(users)),
        // We map internal errors to a 500 response with the error message.
        // In production, you might want to hide internal error details.
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// GET /users/sorted?key=...&reverse=...
/// Sorts users by the given key (id, name, age) and optional reverse order.
pub async fn sorted_users(
    Query(params): Query<SortParams>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let mut users = db::get_all_users()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let key = params.key.unwrap_or_else(|| "id".to_string());
    let reverse = params.reverse.unwrap_or_else(|| "false".to_string()) == "true";

    // Sort in ascending order first; we'll reverse later if needed.
    match key.as_str() {
        "id" => users.sort_by_key(|u| u.id),
        "name" => users.sort_by(|a, b| a.name.cmp(&b.name)),
        "age" => users.sort_by_key(|u| u.age),
        _ => return Err((StatusCode::BAD_REQUEST, "Invalid sort key".to_string())),
    }

    // `sort_by_key` and `sort_by` are stable sorts; reversing after gives a correct descending order.
    if reverse {
        users.reverse();
    }

    Ok(Json(users))
}

/// POST /users
/// Creates a new user from the JSON body.
pub async fn create_user(
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    // Basic validation – the database would also reject an empty string if `NOT NULL`,
    // but we want a clear 400 error instead of a 500.
    if payload.name.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Name cannot be empty".to_string()));
    }

    match db::add_user(payload) {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// PUT /users/:id
/// Partially updates a user. The JSON body may contain `name`, `age`, or both.
pub async fn update_user(
    Path(id): Path<i64>, // Extracts the `id` from the URL path
    Json(payload): Json<UpdateUser>,
) -> Result<Json<&'static str>, (StatusCode, String)> {
    // Ensure at least one field is provided; otherwise it's a bad request.
    if payload.name.is_none() && payload.age.is_none() {
        return Err((StatusCode::BAD_REQUEST, "At least one field required".to_string()));
    }

    match db::update_user(id, payload) {
        Ok(true) => Ok(Json("User updated")),
        Ok(false) => Err((StatusCode::NOT_FOUND, "User not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

/// DELETE /users/:id
/// Deletes a user by id.
pub async fn delete_user(
    Path(id): Path<i64>,
) -> Result<Json<&'static str>, (StatusCode, String)> {
    match db::delete_user(id) {
        Ok(true) => Ok(Json("User deleted")),
        Ok(false) => Err((StatusCode::NOT_FOUND, "User not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
