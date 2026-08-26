use axum::{
    extract::{FromRequestParts, Path},
    http::{request::Parts, StatusCode},
};
use serde::Deserialize;

use crate::error::{ApiError, ApiResult};

pub struct UserId(i64);

impl UserId {
    pub fn get(self) -> i64 {
        self.0
    }
}

impl<S> FromRequestParts<S> for UserId
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> ApiResult<Self> {
        let Path(raw) = Path::<i64>::from_request_parts(parts, state)
            .await
            .map_err(|rejection| ApiError::request(rejection.status(), rejection.body_text()))?;

        if raw < 1 {
            return Err(ApiError::request(
                StatusCode::BAD_REQUEST,
                "id must be a positive integer".to_string(),
            ));
        }

        Ok(UserId(raw))
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortKey {
    Id,
    Name,
    Age,
}

#[derive(Debug, Deserialize)]
pub struct SortedUsersQuery {
    pub key: Option<SortKey>,
    pub reverse: Option<bool>,
}
