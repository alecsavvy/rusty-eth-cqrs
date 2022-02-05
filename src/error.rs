use axum::Error as AxumError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Contract Deploy error: {0}")]
    ContractDeployError(String),
    #[error("Web3 error: {0}")]
    Web3Error(String),
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

impl From<ethcontract::web3::error::Error> for AppError {
    fn from(e: ethcontract::web3::error::Error) -> Self {
        AppError::Web3Error(e.to_string())
    }
}

impl From<ethcontract::errors::DeployError> for AppError {
    fn from(e: ethcontract::errors::DeployError) -> Self {
        AppError::ContractDeployError(e.to_string())
    }
}
