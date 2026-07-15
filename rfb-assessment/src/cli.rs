use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "rfb-cli")]
#[command(author = "Thembaletu")]
#[command(version = "0.1.0")]
#[command(about = "Rust for Bitcoin Assessment CLI")]
pub struct Cli {
    #[arg(short = 'u', long, env = "RPC_URL")]
    pub rpc_url: Option<String>,

    #[arg(short = 'U', long, env = "RPC_USER")]
    pub rpc_user: Option<String>,

    #[arg(short = 'P', long, env = "RPC_PASS")]
    pub rpc_pass: Option<String>,

    #[arg(short = 'c', long)]
    pub config: Option<String>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    BlockchainInfo,
    WalletInfo {
        wallet: String,
    },
    Balance {
        wallet: String,
    },
    NewAddress {
        wallet: String,
        label: Option<String>,
    },
    Rpc {
        #[arg(short, long)]
        wallet: Option<String>,
        method: String,
        params: Vec<String>,
    },
}
