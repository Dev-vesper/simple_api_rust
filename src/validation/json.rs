use axum::{
    extract::{FromRequest, Request},
    Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::error::{ApiError, ApiResult};

pub struct ValidatedJson<T>(pub T);

impl<S, T> FromRequest<S> for ValidatedJson<T>
where
    S: Send + Sync,
    T: DeserializeOwned + Validate,
{
    type Rejection = ApiError;

    async fn from_request(req: Request, state: &S) -> ApiResult<Self> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|rejection| ApiError::request(rejection.status(), rejection.body_text()))?;

        value.validate().map_err(ApiError::from)?;

        Ok(ValidatedJson(value))
    }
}
