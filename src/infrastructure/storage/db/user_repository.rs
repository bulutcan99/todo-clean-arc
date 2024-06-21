use surrealdb::{Error, Surreal};
use surrealdb::engine::remote::ws::Client;

use crate::domain::user::User;
use crate::util::config::Settings;

pub struct UserRepository {
	table: String,
	client: Surreal<Client>,
}

impl UserRepository {
	pub fn new(config: &Settings, client: Surreal<Client>) -> Self {
		let table_name = config.database.todo_table.as_deref().unwrap_or("user");
		UserRepository {
			table: table_name.to_string(),
			client,
		}
	}

	async fn get_all_users(&self) -> Result<Vec<User>, Error> {
		let records = self.client.select(&self.table).await?;
		Ok(records)
	}

	async fn get_user_by_id(&self, id: &str) -> Result<User, Error> {
		if let Some(record) = self.client.select((&self.table, id)).await? {
			return Ok(record);
		}

		Err(Error::Db(
			surrealdb::error::Db::Thrown(
				format!("User with id {} not found", id)
			)
		))
	}

	async fn insert_user(&self, content: &User) -> Result<Vec<User>, Error> {
		let record = self.client.insert(&self.table).content(&content).await?;
		Ok((record))
	}

	async fn update_user(&self, id: &str, user: &User) -> Result<User, Error> {
		let record = self.client
			.update((&self.table, id))
			.content(&user)
			.await?.
			unwrap();

		Ok(record)
	}

	async fn delete_user(&self, id: &str) -> Result<User, Error> {
		let result = self.client.delete((&self.table, id)).await?.unwrap();
		Ok(result)
	}
}