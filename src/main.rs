use anyhow::Error;
use tokio::time::{Duration, sleep};

use todo::infrastructure::storage::db::surreal_db::DatabaseClient;
use todo::util::config::Settings;

#[tokio::main]
async fn main() -> Result<(), Error> {
	let settings = Settings::new().expect("Failed to load settings");
	println!("Config built.");
	let surreal_db = DatabaseClient::new();
	surreal_db.connect(&settings).await?;
	application_initializer().await;
	sleep(Duration::from_secs(5)).await;
	println!("🚀 Server started successfully");
	Ok(())
}


async fn application_initializer() {
	println!("Running application");
}
