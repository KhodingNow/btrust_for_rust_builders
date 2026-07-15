use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("RPC Error: {0}")]
    RpcError(String),

    #[error("Configuration Error: {0}")]
    ConfigError(String),

    #[error("HTTP Error: {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("IO Error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON Error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[allow(dead_code)]
    #[error("Invalid Argument: {0}")]
    InvalidArgument(String),
}

impl From<anyhow::Error> for AppError {
    fn from(err: anyhow::Error) -> Self {
        AppError::ConfigError(err.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;
