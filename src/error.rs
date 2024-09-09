use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use poise::serenity_prelude::{self, ReactionConversionError};
use prisma_client_rust;
use serde::Serialize;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    Giveaway(String),

    #[error("Unauthorized")]
    Unauthorized,
    
    #[error("Access denied")]
    AccessDenied,
    
    #[error("Command not found")]
    CommandNotFound,
    
    #[error("Guild not found")]
    GuildNotFound,

    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("Serde json error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Parse error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),

    #[error(transparent)]
    Database(#[from] prisma_client_rust::QueryError),

    #[error(transparent)]
    IO(#[from] std::io::Error),

    #[error("{0}")]
    Jwt(#[from] jsonwebtoken::errors::Error),

    #[error("ReactionConversionError: {0}")]
    ReactionConversionError(#[from] ReactionConversionError),
    #[error("Serenity Error {0}")]
    Serenity(#[from] serenity_prelude::Error),
}

impl Into<Error> for &str {
    fn into(self) -> Error {
        Error::Giveaway(self.to_string())
    }
}

// Implement `IntoResponse` for `AppError`
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Error::Jwt(err) => (StatusCode::UNAUTHORIZED, err.to_string()),
            Error::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            Error::AccessDenied => (
                StatusCode::FORBIDDEN,
                "You don't have permission to access this guild".to_string(),
            ),
            Error::GuildNotFound => (
                StatusCode::BAD_REQUEST,
                "Guild not found".to_string(),
            ),
            err => (StatusCode::BAD_REQUEST, err.to_string()),
        };

        let error_response = ErrorResponse {
            status: status.as_u16(),
            message: error_message,
        };

        (status, axum::Json(error_response)).into_response()
    }
}

// Error response structure
#[derive(Serialize)]
struct ErrorResponse {
    status: u16,
    message: String,
}
