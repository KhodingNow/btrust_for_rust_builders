use crate::errors::AppResult;
use crate::rpc::RpcClient;

pub fn execute_blockchain_info(client: &RpcClient) -> AppResult<()> {
    let info = client.get_blockchain_info()?;

    println!("____________");
    println!("|BLOCKCHAIN INFORMATION |");
    println!("|_______________________|");
    println!("|Chain: 	{:<24}|", info.chain);
    println!("|Blocks:	{:<24}|", info.blocks);
    println!("|Headers: 	{:<24}|", info.headers);
    println!("|Difficulty: 	{:<24}|", info.difficulty);
    println!("|Verification progress: {:<22}|", info.verificationprogress);
    println!("---------------------------------------");

    Ok(())
}
