mod ai;
mod buddy;
mod error;
mod utils;

pub use self::error::{Error, Result};

#[tokio::main]
async fn main() {
	println!("Initialized");

	// match start().await {
	// 	Ok(_) => println!("\nBye!\n"),
	// 	Err(e) => println!("\nError: {}\n", e),
	// }
}

const DEFAULT_DIR: &str = "buddy";
