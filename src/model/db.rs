use std::time::Duration;

use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;

use crate::settings::Database;

pub type Db = Pool<Postgres>;

pub async fn init_db(config_db: Database) -> Result<Db, sqlx::Error> {
	match new_db_pool(&config_db.url, &config_db.max_conn).await {
		Ok(db) => {
			println!("Database connection successful.");
			Ok(db)
		}
		Err(e) => {
			eprintln!("Database connection failed: {}", e);
			Err(e)
		}
	}
}

async fn new_db_pool(
	url: &str,
	max_conn: &u32,
) -> Result<Db, sqlx::Error> {
	PgPoolOptions::new()
		.max_connections(*max_conn)
		.acquire_timeout(Duration::from_secs(30))
		.connect(&url)
		.await
}
