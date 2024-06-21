use surrealdb::{Error, Surreal};
use surrealdb::engine::remote::ws::Client;

use crate::domain::todo::Todo;
use crate::util::config::Settings;

pub struct TodoRepository {
	table: String,
	client: Surreal<Client>,

}

impl TodoRepository {
	pub fn new(config: &Settings, client: Surreal<Client>) -> Self {
		let table_name = config.database.todo_table.as_deref().unwrap_or("todo");
		TodoRepository {
			table: table_name.to_string(),
			client,
		}
	}

	async fn get_all_todos(&self) -> Result<Vec<Todo>, Error> {
		let records = self.client.select(&self.table).await?;
		Ok(records)
	}

	async fn get_todo_by_id(&self, id: &str) -> Result<Todo, Error> {
		if let Some(record) = self.client.select((&self.table, id)).await? {
			return Ok(record);
		}

		Err(Error::Db(
			surrealdb::error::Db::Thrown(
				format!("Todo with id {} not found", id)
			)
		))
	}

	async fn insert_todo(&self, content: &Todo) -> Result<Vec<Todo>, Error> {
		let todo = self.client.create(&self.table).content(&content).await?;
		Ok(todo)
	}

	async fn update_todo(&self, id: &str, content: &Todo) -> Result<Todo, Error> {
		let record = self.client
			.update((&self.table, id))
			.content(content)
			.await?
			.unwrap();
		Ok(record)
	}

	async fn delete_todo(&self, id: &str) -> Result<Todo, Error> {
		let result = self.client.delete((&self.table, id)).await?.unwrap();
		Ok(result)
	}
}