use crate::config::RpcConfig;
use crate::errors::{AppError, AppResult};
use base64::prelude::*;
use reqwest::blocking::Client as HttpClient;
use serde_json::{json, Value};
use std::process::Command;

#[derive(Debug, serde::Deserialize)]
pub struct BlockchainInfo {
    pub chain: String,
    pub blocks: u64,
    pub headers: u64,
    pub difficulty: f64,
    pub verificationprogress: f64,
}

#[derive(Debug, serde::Deserialize)]
pub struct WalletInfo {
    pub walletname: String,
    #[allow(dead_code)]
    pub balance: f64,
    pub unconfirmed_balance: f64,
    pub txcount: u64,
}

pub struct RpcClient {
    client: HttpClient,
    config: RpcConfig,
}

impl RpcClient {
    pub fn new(config: RpcConfig) -> Self {
        let client = HttpClient::new();
        Self { client, config }
    }

    fn auth_header(&self) -> String {
        let credentials = format!("{}:{}", self.config.username, self.config.password);
        let encoded = BASE64_STANDARD.encode(credentials);
        format!("Basic {}", encoded)
    }

    // For non-wallet commands - use REST API
    pub fn call(&self, _method: &str, _params: &[Value]) -> AppResult<Value> {
        let url = format!(
            "{}/rest/chaininfo.json",
            self.config.url.trim_end_matches('/')
        );

        let response = self
            .client
            .get(&url)
            .header("Authorization", self.auth_header())
            .send()
            .map_err(|e| AppError::RpcError(format!("Connection failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(AppError::RpcError(format!(
                "HTTP Error: {}",
                response.status()
            )));
        }

        let result: Value = response
            .json()
            .map_err(|e| AppError::RpcError(format!("Failed to parse response: {}", e)))?;
        Ok(result)
    }

    // For wallet commands - use docker exec with bash -c
    pub fn call_wallet(&self, wallet: &str, method: &str, params: &[Value]) -> AppResult<Value> {
        let mut cmd_str = format!(
            "bitcoin-cli -regtest -rpccookiefile=/home/bitcoin/.bitcoin/regtest/.cookie -rpcwallet={} {}",
            wallet, method
        );

        for param in params {
            if let Some(s) = param.as_str() {
                cmd_str.push_str(&format!(" {}", s));
            } else if let Some(n) = param.as_f64() {
                cmd_str.push_str(&format!(" {}", n));
            } else if let Some(b) = param.as_bool() {
                cmd_str.push_str(&format!(" {}", b));
            }
        }

        let output = Command::new("docker")
            .arg("exec")
            .arg("polar-n1-backend1")
            .arg("bash")
            .arg("-c")
            .arg(&cmd_str)
            .output()
            .map_err(|e| AppError::RpcError(format!("Failed to execute docker: {}", e)))?;

        if !output.status.success() {
            let error = String::from_utf8_lossy(&output.stderr);
            return Err(AppError::RpcError(format!("bitcoin-cli error: {}", error)));
        }

        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();

        if let Ok(result) = serde_json::from_str::<Value>(&stdout) {
            return Ok(result);
        }

        if let Ok(number) = stdout.parse::<f64>() {
            return Ok(json!(number));
        }

        if stdout.starts_with('"') && stdout.ends_with('"') {
            return Ok(json!(stdout.trim_matches('"')));
        }

        Ok(json!(stdout))
    }

    pub fn get_blockchain_info(&self) -> AppResult<BlockchainInfo> {
        let result = self.call("getblockchaininfo", &[])?;
        let info: BlockchainInfo = serde_json::from_value(result)?;
        Ok(info)
    }

    pub fn get_wallet_info(&self, wallet: &str) -> AppResult<WalletInfo> {
        // Get the wallet info JSON
        let result = self.call_wallet(wallet, "getwalletinfo", &[])?;

        // Extract fields from the JSON
        let walletname = result["walletname"]
            .as_str()
            .unwrap_or("Unknown")
            .to_string();
        let txcount = result["txcount"].as_u64().unwrap_or(0);

        // Get balance separately
        let balance_result = self.call_wallet(wallet, "getbalance", &[])?;
        let balance = balance_result
            .as_f64()
            .ok_or_else(|| AppError::RpcError("Invalid balance response".to_string()))?;

        // For regtest, unconfirmed balance is typically 0
        let unconfirmed_balance = 0.0;

        Ok(WalletInfo {
            walletname,
            balance,
            unconfirmed_balance,
            txcount,
        })
    }

    pub fn get_balance(&self, wallet: &str) -> AppResult<f64> {
        let result = self.call_wallet(wallet, "getbalance", &[])?;
        let balance = result
            .as_f64()
            .ok_or_else(|| AppError::RpcError("Invalid balance response".to_string()))?;
        Ok(balance)
    }

    pub fn get_new_address(&self, wallet: &str, label: Option<&str>) -> AppResult<String> {
        let params = match label {
            Some(l) => vec![json!(l)],
            None => vec![],
        };
        let result = self.call_wallet(wallet, "getnewaddress", &params)?;
        let address = result
            .as_str()
            .ok_or_else(|| AppError::RpcError("Invalid address response".to_string()))?
            .to_string();
        Ok(address)
    }
}
