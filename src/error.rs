use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use serde_json::json;
use validator::{ValidationErrors, ValidationErrorsKind};

#[derive(Debug, Serialize)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("{message}")]
    Validation {
        message: String,
        details: Vec<FieldError>,
    },
    #[error("{message}")]
    Request {
        status: StatusCode,
        message: String,
    },
    #[error("{0}")]
    NotFound(&'static str),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

pub type ApiResult<T> = Result<T, ApiError>;

impl ApiError {
    pub fn internal(error: anyhow::Error) -> Self {
        ApiError::Internal(error)
    }

    pub fn not_found(message: &'static str) -> Self {
        ApiError::NotFound(message)
    }

    pub fn request(status: StatusCode, message: String) -> Self {
        ApiError::Request { status, message }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, code, message, details) = match self {
            ApiError::Validation { message, details } => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "VALIDATION_FAILED",
                message,
                details,
            ),
            ApiError::Request { status, message } => (status, "REQUEST_REJECTED", message, vec![]),
            ApiError::NotFound(message) => {
                (StatusCode::NOT_FOUND, "NOT_FOUND", message.to_string(), vec![])
            }
            ApiError::Internal(error) => {
                tracing::error!(error = ?error, "internal server error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "INTERNAL",
                    "internal server error".to_string(),
                    vec![],
                )
            }
        };

        let body = json!({
            "error": {
                "code": code,
                "message": message,
                "details": details,
            }
        });

        (status, Json(body)).into_response()
    }
}

impl From<ValidationErrors> for ApiError {
    fn from(errors: ValidationErrors) -> Self {
        let mut details = Vec::new();
        collect_field_errors(&errors, &mut details);

        ApiError::Validation {
            message: "validation failed".to_string(),
            details,
        }
    }
}

fn collect_field_errors(errors: &ValidationErrors, details: &mut Vec<FieldError>) {
    for (field, kind) in errors.errors() {
        match kind {
            ValidationErrorsKind::Field(field_errors) => {
                for error in field_errors {
                    let message = error
                        .message
                        .as_ref()
                        .map(|message| message.to_string())
                        .unwrap_or_else(|| error.code.to_string());

                    details.push(FieldError {
                        field: field.to_string(),
                        message,
                    });
                }
            }
            ValidationErrorsKind::Struct(nested) => collect_field_errors(nested, details),
            ValidationErrorsKind::List(entries) => {
                for nested in entries.values() {
                    collect_field_errors(nested, details);
                }
            }
        }
    }
}
