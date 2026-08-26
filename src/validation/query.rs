use axum::{
    extract::{FromRequestParts, Query},
    http::request::Parts,
};
use serde::de::DeserializeOwned;

use crate::error::{ApiError, ApiResult};

pub struct ValidatedQuery<T>(pub T);

impl<S, T> FromRequestParts<S> for ValidatedQuery<T>
where
    S: Send + Sync,
    T: DeserializeOwned,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> ApiResult<Self> {
        let Query(value) = Query::<T>::from_request_parts(parts, _state)
            .await
            .map_err(|rejection| ApiError::request(rejection.status(), rejection.body_text()))?;

        Ok(ValidatedQuery(value))
    }
}
