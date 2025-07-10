use axum::{response::{Html, IntoResponse, Response}, http::StatusCode};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    SqlxError(sqlx::Error),
    BcryptError(bcrypt::BcryptError),
    MultipartError(axum::extract::multipart::MultipartError),
    CsvError(csv::Error),
    UserAlreadyExists,
    Unauthorized,
    // Add other error types as needed
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::SqlxError(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Database error: {}", e))
            }
            AppError::BcryptError(e) => {
                (StatusCode::INTERNAL_SERVER_ERROR, format!("Password hashing error: {}", e))
            }
            AppError::MultipartError(e) => {
                (StatusCode::BAD_REQUEST, format!("Upload error: {}", e))
            }
            AppError::CsvError(e) => {
                (StatusCode::BAD_REQUEST, format!("CSV parsing error: {}", e))
            }
            AppError::UserAlreadyExists => {
                (StatusCode::BAD_REQUEST, "Username already exists.".to_string())
            }
            AppError::Unauthorized => {
                (StatusCode::UNAUTHORIZED, "Unauthorized.".to_string())
            }
        };

        (status, Html(format!("<h1>Error: {}</h1><p><a href=\"/login\">Try again</a></p>", error_message))).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> AppError {
        AppError::SqlxError(err)
    }
}

impl From<bcrypt::BcryptError> for AppError {
    fn from(err: bcrypt::BcryptError) -> AppError {
        AppError::BcryptError(err)
    }
}

impl From<axum::extract::multipart::MultipartError> for AppError {
    fn from(err: axum::extract::multipart::MultipartError) -> AppError {
        AppError::MultipartError(err)
    }
}

impl From<csv::Error> for AppError {
    fn from(err: csv::Error) -> AppError {
        AppError::CsvError(err)
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
