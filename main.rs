use std::collections::HashMap;
use std::ptr;


// Name Assignment (variables and constants)
// TODO: Assign current bitcoin mining reward

pub const MINING_REWARD: f64 = 3.125; // Current reward after halving (2024) 
// TODO Assignment block height
pub const CURRENT_BLOCK_HEIGHT: u64 = 840_000; // Approx current height
// TODO: Assign the number of satoshis in one Bitcoin
pub const BTC_TO_SATS: u64 = 100_000_000;

#[derive(Debug, Clone, PartialEq)]
pub struct Utxo {
	pub txid: String,
	pub vout: u32,
	pub value: u64,
}

/// Calculate the total Bitcoin reward for a given number of mined blocks
pub fn calculate_total_reward(blocks_mined: u64) -> f64 {
	blocks_mined as f64 * MINING_REWARD
}

/// Return true if the transaction fee is between 0.00001 and 0.01 BTC.
pub fn is_valid_tx_fee(fee: f64) -> bool {
	fee >= 0.00001 && fee <= 0.01
}

/// Return true if the wallet balance is greater than 50.0 BTC.
pub fn is_large_balance(balance: f64) -> bool {
	balance > 50.0
} 

/// Return the priority of a transaction ("high", "medium", "low") based on a fee rate.
pub fn tx_priority(size_bytes: u64, fee_btc: f64) -> &'static str {
	let fee_rate = fee_btc / size_bytes as f64;

	if fee_rate > 0.00005 {
		"high"
	} else if fee_rate > 0.00001 {
		"medium"
	} else {
		"low"
	}
}

/// Return true if the network string equals "mainnet" (case-insensitive).
pub fn is_mainnet(network: &str) -> bool {
	network.to_lowercase() == "mainnet"
}

/// Return true if value is in the inclusive range 100..=200.
pub fn is_in_range(value: i64) -> bool {
	(100..=200).contains(&value)
}

/// Return true if both references point to the exact same object in memory.
pub fn is_same_wallet<T>(wallet1: &T, wallet2: &T) -> bool {
	ptr::eq(wallet1, wallet2)
}

/// Normalize a Bitcooin address by trimming whitespace and lowecase.
pub fn normalize_address(address: &str) -> String {
	address.trim().to_lowercase()
}

/// Find the first transaction with a fee greater than 0.005 BTC.
pub fn find_high_fee(fee_list: &[f64]) -> Option<(usize, f64)>
{
	fee_list
		.iter()
		.enumerate()
		.find(|&(_, &fee)| fee > 0.005)
		.map(|(index, &fee)| (index, fee))
}

/// Return basic wallet details as a tuple of (name, balance).
pub fn get_wallet_details() -> (String, f64) {
	("Default Wallet".to_string(), 1.5)
}


/// Get the status of a transaction from the mempool or "not found".
pub fn get_tx_status(tx_pool: &HashMap<String, String>, txid: &str) -> String {
	tx_pool
		.get(txid)
		.cloned()
		.unwrap_or_else(|| "not found".to_string())
}

/// Destructure wallet_info and format a status string.
pub fn unpack_wallet_info(wallet_info: (String, f64)) -> String {
	let (name, balance) = wallet_info;
	format!("Wallet {} has balance: {} BTC", name, balance)
}

/// Convert BTC to satoshis (1 BTC = 100, 000, 000 sats).
pub fn calculate_sats(btc: f64) -> u64 {
	(btc * BTC_TO_SATS as f64) as u64
} 

/// Generate a mock Bitcoin address of length 32 with the given prefix.
pub fn generate_address(prefix: &str) -> String {
	
	// For now, return a simple mock address without a rand dependency
		
	let suffix = "1234567890abcdef1234567890abcdef";
	let remaining_len = 32 - prefix.len();
	if remaining_len <= suffix.len() {
		format!("{}{}", prefix, &suffix[..remaining_len])
	} else {
		format!("{}{}", prefix, suffix)
	}	

} 

/// Validate a Bitcoin block height. Returns (is_valid, message).
pub fn validate_block_height(height: i64) -> (bool, String) {
	if height < 0 {
	
		(false, "Block height exceeds realistic maximum".to_string())
	} else {
		(true, "Valid block height".to_string())
	}
}

/// Compute the block reward (in sats) for each block height based on the halving schedule.
pub fn halving_schedule(blocks: &[u64]) -> HashMap<u64, u64> {
	let mut result = HashMap::new();
	let base_reward_sats = 50 * 1000_000_000; // 50 BTC in sats
	let halving_interval = 210_000;

	for &block in blocks {
		let halvings = block / halving_interval;
		let reward = base_reward_sats >> halvings; // Right  shift divides bt 2^halvings
		result.insert(block, reward);
	}


	result
}

/// Find the UTXO with the smallest value that meets or exceeds target.
pub fn find_utxo_with_min_value(utxos: &[Utxo], target: u64) -> Option<Utxo> {
	utxos
		.iter()
		.filter(|utxo| utxo.value >= target)
		.min_by_key(|utxo| utxo.value)
		.cloned()
}

/// Create a UTXO map from txid, vout, and arbitrary extra string fields.
pub fn create_utxo(
	txid: &str,
	vout: u32,
	extra: HashMap<String, String>,

) -> HashMap<String, String> {
	let mut base = HashMap::new();
	base.insert("txid".to_string(), txid.to_string());
	base.insert("vout".to_string(), vout.to_string());

	// Merge extra into base (extra values will override if keys conflict)
	base.extend(extra);
	base
}

/// Implement extract_tx_version function below
pub fn extract_tx_version(raw_tx_hex: &str) -> Result<u32, String> {
	if raw_tx_hex.len() < 8 {
		return Err("Transaction hex string too short".to_string());
	}
	
	// Bitcoin transaction version is the first 4 bytes ( 8 hex chars) in little-endian
	let version_hex = &raw_tx_hex[0..8];

	// Parse hex string to u32 (little endian)
	u32::from_str_radix(version_hex, 16)
		.map(|version| version.to_le())
		.map_err(|_| "Invalid hex string".to_string()) 

}

// Add a main function for testing 

fn main() {
	println!("Bitcoin Course - Week 1 Exercises");
	println!("=================================");

// Test some functions

	println!("Mining reward for 1 block: {} BTC", calculate_total_reward(1));
	println!("Is valid fee 0.00005? {}", is_valid_tx_fee(0.00005));
	println!("Is large balance 100 BTC? {}", is_large_balance(100.0));
	println!("Is mainnet? {}", is_mainnet("MAINNET"));
	println!("Is 150 in range? {}", is_in_range(150));
	println!("Normalized address: {}", normalize_address( " 1A1zP1eP5QGefi2DMPTfTL5SLmv7DivfNa  "));

	let wallet1 = 100;
	let wallet2 = &wallet1;
	let wallet3 = 100;
	println!("Same wallet? {}", is_same_wallet(&wallet1, wallet2));
	println!("Different wallet? {}", is_same_wallet(&wallet1, &wallet3));

	let details = get_wallet_details();
	println!("Wallet details: {:?}", details);
	println!("{:?}, unpack_wallet_info", (details));

	
	println!("100 BTC in sats: {}", calculate_sats(100.0));
	println!("Generate address: {}", generate_address("1A1z"));

	let block_height = 840_000;
	let validation = validate_block_height(block_height);
	println!("Block height {} validation: {:?}", block_height, validation);

	// Test fee finder
	
	let fees = vec![0.001, 0.006, 0.003, 0.007];
	match find_high_fee(&fees) {
		Some((index, fee)) => println!("Found high fee at index {}: {} BTC", index, fee),
		None => println!("No high fee found"),
	}
}




