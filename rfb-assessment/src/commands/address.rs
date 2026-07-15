use crate::errors::AppResult;
use crate::rpc::RpcClient;

pub fn execute_new_address(client: &RpcClient, wallet: &str, label: Option<&str>) -> AppResult<()> {
    let address = client.get_new_address(wallet, label)?;

    println!("New address: ");
    println!(" {}", address);
    if let Some(label) = label {
        println!(" Label: {}", label);
    }
    println!(" Wallet: {}", wallet);

    Ok(())
}
