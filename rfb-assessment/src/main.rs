mod cli;
mod commands;
mod config;
mod errors;
mod rpc;

use clap::Parser;
use cli::{Cli, Commands};
use config::RpcConfig;
use rpc::RpcClient;

fn main() -> errors::AppResult<()> {
    let cli = Cli::parse();

    let config = if let Some(config_path) = cli.config {
        RpcConfig::from_file(std::path::Path::new(&config_path))?
    } else if cli.rpc_pass.is_some() || cli.rpc_url.is_some() || cli.rpc_user.is_some() {
        RpcConfig::from_cli(cli.rpc_url, cli.rpc_user, cli.rpc_pass)?
    } else {
        RpcConfig::from_env()?
    };

    let client = RpcClient::new(config);

    match cli.command {
        Commands::BlockchainInfo => {
            commands::execute_blockchain_info(&client)?;
        }
        Commands::WalletInfo { wallet } => {
            commands::execute_wallet_info(&client, &wallet)?;
        }
        Commands::Balance { wallet } => {
            commands::execute_balance(&client, &wallet)?;
        }
        Commands::NewAddress { wallet, label } => {
            commands::execute_new_address(&client, &wallet, label.as_deref())?;
        }
        Commands::Rpc {
            wallet,
            method,
            params,
        } => {
            let params_json: Vec<serde_json::Value> = params
                .iter()
                .map(|p| {
                    if let Ok(num) = p.parse::<f64>() {
                        serde_json::json!(num)
                    } else {
                        serde_json::json!(p)
                    }
                })
                .collect();

            let result = if let Some(wallet_name) = wallet {
                client.call_wallet(&wallet_name, &method, &params_json)?
            } else {
                client.call(&method, &params_json)?
            };
            println!("{}", serde_json::to_string_pretty(&result)?);
        }
    }

    Ok(())
}
