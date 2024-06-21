use std::sync::Arc;

use surrealdb::{Error, Surreal};
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use tokio::sync::Mutex;
use util::config::config::Settings;

pub struct DatabaseClient {
	db: Arc<Mutex<Surreal<Client>>>,
}

impl DatabaseClient {
	pub fn new() -> Self {
		DatabaseClient {
			db: Arc::new(Mutex::new(Surreal::init())),
		}
	}

	pub async fn connect(&self, config: &Settings) -> Result<(), Error> {
		let db = self.db.clone();
		let db_lock = db.lock().await;

		let url = config.database.url.as_deref().unwrap_or("localhost:8000");
		let username = config.database.username.as_deref().unwrap_or("root");
		let password = config.database.password.as_deref().unwrap_or("root");

		db_lock.connect::<Ws>(&*url).await?;
		db_lock.signin(Root {
			username,
			password,
		}).await?;
		db_lock.use_ns("todo").use_db("todo").await?;

		Ok(())
	}
}
