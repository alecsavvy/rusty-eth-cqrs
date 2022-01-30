use axum::Error as AxumError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Axum error: {0}")]
    AxumError(String),
    #[error("idk man: {0}")]
    IdkMan(String), // just to get started
}

impl From<AxumError> for AppError {
    fn from(e: AxumError) -> Self {
        AppError::AxumError(e.to_string())
    }
}
