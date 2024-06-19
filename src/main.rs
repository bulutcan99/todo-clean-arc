#![allow(unused)]

use config::Settings;

mod config;
mod db;
mod handler;
mod model;
mod service;


#[tokio::main]
async fn main() {
	let settings = Settings::new();
	match settings {
		Ok(settings) => {
			run_application(settings).await;
		}
		Err(e) => {
			eprintln!("Error: {:?}", e);
		}
	}
}

async fn run_application(settings: Settings) {
	println!("Running application");
}
