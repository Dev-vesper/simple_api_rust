use axum::{extract::State, Json};

use crate::db::Database;
use crate::error::{ApiError, ApiResult};
use crate::models::{CreateUser, UpdateUser, User};
use crate::validation::{SortKey, SortedUsersQuery, UserId, ValidatedJson, ValidatedQuery};

pub async fn list_users(State(db): State<Database>) -> ApiResult<Json<Vec<User>>> {
    let users = db.get_all_users().map_err(ApiError::internal)?;
    Ok(Json(users))
}

pub async fn sorted_users(
    State(db): State<Database>,
    ValidatedQuery(query): ValidatedQuery<SortedUsersQuery>,
) -> ApiResult<Json<Vec<User>>> {
    let mut users = db.get_all_users().map_err(ApiError::internal)?;

    match query.key.unwrap_or(SortKey::Id) {
        SortKey::Id => users.sort_by_key(|user| user.id),
        SortKey::Name => users.sort_by(|a, b| a.name.cmp(&b.name)),
        SortKey::Age => users.sort_by_key(|user| user.age),
    }

    if query.reverse.unwrap_or(false) {
        users.reverse();
    }

    Ok(Json(users))
}

pub async fn create_user(
    State(db): State<Database>,
    ValidatedJson(payload): ValidatedJson<CreateUser>,
) -> ApiResult<Json<User>> {
    let user = db.add_user(payload.normalized()).map_err(ApiError::internal)?;
    Ok(Json(user))
}

pub async fn update_user(
    State(db): State<Database>,
    id: UserId,
    ValidatedJson(payload): ValidatedJson<UpdateUser>,
) -> ApiResult<Json<&'static str>> {
    match db.update_user(id.get(), payload.normalized()) {
        Ok(true) => Ok(Json("User updated")),
        Ok(false) => Err(ApiError::not_found("User not found")),
        Err(error) => Err(ApiError::internal(error)),
    }
}

pub async fn delete_user(
    State(db): State<Database>,
    id: UserId,
) -> ApiResult<Json<&'static str>> {
    match db.delete_user(id.get()) {
        Ok(true) => Ok(Json("User deleted")),
        Ok(false) => Err(ApiError::not_found("User not found")),
        Err(error) => Err(ApiError::internal(error)),
    }
}
