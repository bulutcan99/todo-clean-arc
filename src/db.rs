use surrealdb::engine::remote::ws::Ws;
use surrealdb::Surreal;

use crate::config::Settings;

pub struct Database {
	config: Settings,
	client: Surreal<Ws>,
}

impl Database {
	pub async fn new() -> Result<Self, surrealdb::Error> {
		let db = Surreal::new::<Ws>().await?;
		Ok(Self { client })
	}
}