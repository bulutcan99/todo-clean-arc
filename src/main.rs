#![allow(unused)]

use settings::Settings;

mod settings;
mod model;
mod security;
mod web;

#[tokio::main]
async fn main() {
	let settings = Settings::new();
	match settings {
		Ok(settings) => {
			let db_conn = model::db::init_db(settings.database)
				.await.expect("Failed to connect to database.");
		}
		Err(e) => {
			eprintln!("Error: {}", e);
		}
	}
	println!("Hello, world!");
}
