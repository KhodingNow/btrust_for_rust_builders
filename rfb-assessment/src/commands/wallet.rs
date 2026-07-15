use crate::errors::AppResult;
use crate::rpc::RpcClient;

pub fn execute_wallet_info(client: &RpcClient, wallet: &str) -> AppResult<()> {
    let info = client.get_wallet_info(wallet)?;
    let balance = client.get_balance(wallet)?;

    println!("┌─────────────────────────────────────────┐");
    println!("│           WALLET INFORMATION           │");
    println!("├─────────────────────────────────────────┤");
    println!("│ Wallet:              {:<24} │", info.walletname);
    println!("│ Balance:             {:<24} BTC │", balance);
    println!(
        "│ Unconfirmed Balance: {:<24} BTC │",
        info.unconfirmed_balance
    );
    println!("│ Transactions:        {:<24} │", info.txcount);
    println!("└─────────────────────────────────────────┘");

    Ok(())
}

pub fn execute_balance(client: &RpcClient, wallet: &str) -> AppResult<()> {
    let balance = client.get_balance(wallet)?;
    println!("Balance: {} BTC", balance);
    Ok(())
}
