use surrealdb::Error;

use crate::domain::todo::Todo;
use crate::infrastructure::storage::surreal::surreal_db::DBEnginer;
use crate::util::config::Settings;

pub trait TodoRepositoryTrait {
	fn get_all_todos(&self) -> Result<Vec<Todo>, Error>;
	fn get_todo_by_id(&self, id: &str) -> Result<Todo, Error>;
	fn insert_todo(&self, content: &Todo) -> Result<Vec<Todo>, Error>;
	fn update_todo(&self, id: &str, content: &Todo) -> Result<Todo, Error>;
	fn delete_todo(&self, id: &str) -> Result<Todo, Error>;
}

pub struct TodoRepository<T: DBEnginer> {
	table: String,
	db: T,

}

impl<T> TodoRepository<T>
where
	T: DBEnginer,
{
	pub fn new(config: &Settings, db: T) -> Self {
		let table_name = config.database.todo_table.as_deref().unwrap_or("todo");
		TodoRepository {
			table: table_name.to_string(),
			db,
		}
	}
}

impl<T> TodoRepositoryTrait for TodoRepository<T>
where
	T: DBEnginer,
{
	fn get_all_todos(&self) -> Result<Vec<Todo>, Error> {
		let records = self.db.select(&self.table)?;
		Ok(records)
	}

	fn get_todo_by_id(&self, id: &str) -> Result<Todo, Error> {
		if let Some(record) = self.db.select((&self.table, id))? {
			return Ok(record);
		}

		Err(Error::Db(
			surrealdb::error::Db::Thrown(
				format!("Todo with id {} not found", id)
			)
		))
	}

	fn insert_todo(&self, content: &Todo) -> Result<Vec<Todo>, Error> {
		let record = self.db.insert(&self.table).content(&content)?;
		Ok((record))
	}

	fn update_todo(&self, id: &str, content: &Todo) -> Result<Todo, Error> {
		let record = self.db
			.update((&self.table, id))
			.content(&content)?
			.unwrap();
		Ok(record)
	}

	fn delete_todo(&self, id: &str) -> Result<Todo, Error> {
		let record = self.db.delete((&self.table, id))?;
		Ok(record)
	}
}