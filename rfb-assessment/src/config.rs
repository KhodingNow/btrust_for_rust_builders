use crate::errors::AppError;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct RpcConfig {
    pub url: String,
    pub username: String,
    pub password: String,
}

#[allow(dead_code)]
impl RpcConfig {
    pub fn from_env() -> Result<Self, AppError> {
        dotenv::dotenv().ok();

        let url = std::env::var("RPC_URL").unwrap_or_else(|_| "http://localhost:18443".to_string());
        let username = std::env::var("RPC_USER").unwrap_or_else(|_| "polar".to_string());
        let password = std::env::var("RPC_PASS").map_err(|_| {
            AppError::ConfigError("RPC_PASS environment variable not set".to_string())
        })?;

        Ok(Self {
            url,
            username,
            password,
        })
    }

    pub fn from_file(path: &Path) -> Result<Self, AppError> {
        let content = fs::read_to_string(path).map_err(AppError::IoError)?;
        let config: serde_json::Value =
            serde_json::from_str(&content).map_err(AppError::JsonError)?;

        Ok(Self {
            url: config["rpc_url"]
                .as_str()
                .unwrap_or("http://localhost:18443")
                .to_string(),
            username: config["rpc_user"].as_str().unwrap_or("polar").to_string(),
            password: config["rpc_pass"]
                .as_str()
                .ok_or_else(|| AppError::ConfigError("Missing rpc_pass in config".to_string()))?
                .to_string(),
        })
    }

    pub fn from_cli(
        url: Option<String>,
        username: Option<String>,
        password: Option<String>,
    ) -> Result<Self, AppError> {
        dotenv::dotenv().ok();
        let env_password = std::env::var("RPC_PASS").ok();
        let env_url = std::env::var("RPC_URL").ok();
        let env_user = std::env::var("RPC_USER").ok();

        let password = password.or(env_password).ok_or_else(|| {
            AppError::ConfigError(
                "Password is required. Set RPC_PASS env var or use --rpc-pass".to_string(),
            )
        })?;

        Ok(Self {
            url: url
                .or(env_url)
                .unwrap_or_else(|| "http://localhost:18443".to_string()),
            username: username.or(env_user).unwrap_or_else(|| "polar".to_string()),
            password,
        })
    }
}
