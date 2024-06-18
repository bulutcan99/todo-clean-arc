use std::time::Duration;

use futures::StreamExt;
use sqlx::{Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use tokio::fs;

use crate::settings::Database;

// CONSTS
const PG_APP_URL: &str = "postgres://app_user:app_pass@localhost:5432/app_db";
const PG_APP_MAX_CONN: u32 = 10;
const SQL_DIR: &str = "src/sql";
const MIGRATION_FILE: &str = "src/migrations/000-create-db.sql";

pub type Db = Pool<Postgres>;


pub async fn init_db(config_db: Database) -> Result<Db, sqlx::Error> {
	let db_url = config_db.url.as_deref().unwrap_or(PG_APP_URL);
	let db_max_conn = config_db.max_conn.unwrap_or(PG_APP_MAX_CONN);

	let db = new_db_pool(db_url, &db_max_conn).await?;
	Ok(db)
}

async fn new_db_pool(url: &str, max_conn: &u32) -> Result<Db, sqlx::Error> {
	PgPoolOptions::new()
		.max_connections(*max_conn)
		.acquire_timeout(Duration::from_secs(30))
		.connect(url)
		.await
}

async fn migration(db: &Db, file: &str) -> Result<(), sqlx::Error> {
	let content = fs::read_to_string(file).await.map_err(|ex| {
		println!("Error reading file: {}", ex);
		ex
	})?;

	let sqls: Vec<&str> = content.split(';').collect();
	for sql in sqls {
		if !sql.trim().is_empty() {
			match sqlx::query(sql).execute(db).await {
				Ok(_) => println!("Migration successful."),
				Err(e) => {
					eprintln!("Migration failed: {}", e);
					return Err(e);
				}
			}
		}
	}

	Ok(())
}

#[cfg(test)]
#[path = "../../src/_tests/model_db.rs"]
mod tests;
