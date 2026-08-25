use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;

use crate::db;
use crate::models::{CreateUser, UpdateUser, User};

#[derive(Deserialize)]
pub struct SortParams {
    pub key: Option<String>,
    pub reverse: Option<String>,
}

pub async fn list_users() -> Result<Json<Vec<User>>, (StatusCode, String)> {
    match db::get_all_users() {
        Ok(users) => Ok(Json(users)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn sorted_users(
    Query(params): Query<SortParams>,
) -> Result<Json<Vec<User>>, (StatusCode, String)> {
    let mut users = db::get_all_users().map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let key = params.key.unwrap_or_else(|| "id".to_string());
    let reverse = params.reverse.unwrap_or_else(|| "false".to_string()) == "true";

    match key.as_str() {
        "id" => users.sort_by_key(|u| u.id),
        "name" => users.sort_by(|a, b| a.name.cmp(&b.name)),
        "age" => users.sort_by_key(|u| u.age),
        _ => return Err((StatusCode::BAD_REQUEST, "Invalid sort key".to_string())),
    }

    if reverse {
        users.reverse();
    }

    Ok(Json(users))
}

pub async fn create_user(
    Json(payload): Json<CreateUser>,
) -> Result<Json<User>, (StatusCode, String)> {
    if payload.name.is_empty() {
        return Err((StatusCode::BAD_REQUEST, "Name cannot be empty".to_string()));
    }
    match db::add_user(payload) {
        Ok(user) => Ok(Json(user)),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn update_user(
    Path(id): Path<i64>,
    Json(payload): Json<UpdateUser>,
) -> Result<Json<&'static str>, (StatusCode, String)> {
    if payload.name.is_none() && payload.age.is_none() {
        return Err((StatusCode::BAD_REQUEST, "At least one field required".to_string()));
    }
    match db::update_user(id, payload) {
        Ok(true) => Ok(Json("User updated")),
        Ok(false) => Err((StatusCode::NOT_FOUND, "User not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}

pub async fn delete_user(
    Path(id): Path<i64>,
) -> Result<Json<&'static str>, (StatusCode, String)> {
    match db::delete_user(id) {
        Ok(true) => Ok(Json("User deleted")),
        Ok(false) => Err((StatusCode::NOT_FOUND, "User not found".to_string())),
        Err(e) => Err((StatusCode::INTERNAL_SERVER_ERROR, e.to_string())),
    }
}
