use crate::config::Settings;

pub mod repository;


pub trait DBEnginer {
	fn new() -> Self;
	fn connect(&self, config: &Settings) -> Result<(), Error>;
}

pub struct DB {
	pub client: Lazy<Surreal<Client>>,
}

impl DBEnginer for DB {
	fn new() -> Self {
		DB {
			client: Lazy::new(|| Surreal::init()),
		}
	}

	async fn connect(&self, config: &Settings) -> Result<(), Error> {
		let db = self.client.clone();
		let url = config.database.url.as_deref().unwrap_or("localhost:8000");
		let username = config.database.username.as_deref().unwrap_or("root");
		let password = config.database.password.as_deref().unwrap_or("root");
		let ns = config.database.db_name.as_deref().unwrap_or("todo");
		let db_name = config.database.db_name.as_deref().unwrap_or("todo");

		db.connect::<Ws>(url).await?;
		db.signin(Root {
			username,
			password,
		}).await?;

		db.use_ns(ns).use_db(db_name).await?;
		Ok(())
	}
}