use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::{Deserialize, Serialize};

pub type ApiResult<T> = Result<T, BackendError>;

#[derive(Debug, PartialEq, Clone)]
pub enum BackendError {
    InvalidCredentials,
    InvalidToken,
    TokenNotFound,
    NoCookieFound,
    SomethingWentWrong,
    SerializationFailed,
    JWTEncodingFailed,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct BackendErrorMessage {
    pub code: u16,
    pub reason: String,
}

impl BackendErrorMessage {
    fn new(code: u16, reason: impl Into<String>) -> Self {
        Self {
            code,
            reason: reason.into(),
        }
    }
}

impl IntoResponse for BackendError {
    fn into_response(self) -> Response {
        match self {
            BackendError::InvalidToken => (
                StatusCode::UNAUTHORIZED,
                Json(BackendErrorMessage::new(401, "Invalid Token")),
            )
                .into_response(),
            BackendError::InvalidCredentials => (
                StatusCode::UNAUTHORIZED,
                Json(BackendErrorMessage::new(401, "Invalid Credentials")),
            )
                .into_response(),
            BackendError::NoCookieFound => (
                StatusCode::UNAUTHORIZED,
                Json(BackendErrorMessage::new(401, "No Cookie Found")),
            )
                .into_response(),
            BackendError::TokenNotFound => (
                StatusCode::UNAUTHORIZED,
                Json(BackendErrorMessage::new(401, "No Token Found")),
            )
                .into_response(),
            BackendError::SomethingWentWrong => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(BackendErrorMessage::new(500, "Something Went Wrong")),
            )
                .into_response(),
            BackendError::JWTEncodingFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(BackendErrorMessage::new(500, "JWT Encoding Failed")),
            )
                .into_response(),
            BackendError::SerializationFailed => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(BackendErrorMessage::new(500, "Serialization Failed")),
            )
                .into_response(),
        }
    }
}
