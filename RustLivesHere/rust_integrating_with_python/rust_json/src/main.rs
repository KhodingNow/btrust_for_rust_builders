#[derive(Debug, serde::Deserialize)]
struct Data {
	name: String,
	value: i32,
}

fn main() {
/*	let input = "{ \"name\": \"Sharpe Oliver\", \"value\": 134087 }";

	let parsed: Data = serde_json::from_str(input).unwrap();

	println!("{:?}", parsed);

*/

// Rust program mimmicking the functionality of Python

// fn main()...
	let result = 
		sum("{\"name\": \"Rochelle Fegasin\", \"value\": 948129 }");

	println!("{result}");
}

fn sum(input: &str) -> i32 {
	let parsed: Data = serde_json::from_str(input).unwrap();

	parsed.name.len() as i32 + parsed.value

	
	
}
