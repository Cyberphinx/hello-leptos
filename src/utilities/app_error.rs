use http::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub struct AppError {
    pub code: StatusCode,
    pub message: String,
}

impl AppError {
    pub fn new(code: StatusCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

// impl IntoResponse for AppError {
//     fn into_response(self) -> axum::response::Response {
//         (
//             self.code,
//             Json(ErrorResponse {
//                 error: self.message.clone(),
//             }),
//         )
//             .into_response()
//     }
// }

// map from reqwest error to AppError
impl From<reqwest::Error> for AppError {
    fn from(value: reqwest::Error) -> Self {
        eprintln!("Reqwest error: {}", value);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Reqwest error: {}", value),
        )
    }
}

// map from json web token error to AppError
impl From<jsonwebtoken::errors::Error> for AppError {
    fn from(value: jsonwebtoken::errors::Error) -> Self {
        eprintln!("JsonWebToken error: {}", value);
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("JsonWebToken error: {}", value),
        )
    }
}

#[derive(Serialize, Deserialize)]
struct ErrorResponse {
    error: String,
}
