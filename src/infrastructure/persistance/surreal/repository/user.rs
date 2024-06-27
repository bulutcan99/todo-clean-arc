use surrealdb::Error;

use crate::domain::user::User;
use crate::infrastructure::storage::surreal::surreal_db::DBEnginer;
use crate::util::config::Settings;

pub trait UserRepositoryTrait {
	fn get_all_users(&self) -> Result<Vec<User>, Error>;
	fn get_user_by_id(&self, id: &str) -> Result<User, Error>;
	fn insert_user(&self, content: &User) -> Result<Vec<User>, Error>;
	fn update_user(&self, id: &str, user: &User) -> Result<User, Error>;
	fn delete_user(&self, id: &str) -> Result<User, Error>;
}

pub struct UserRepository<T: DBEnginer> {
	table: String,
	db: T,
}

impl<T> UserRepository<T>
where
	T: DBEnginer,
{
	pub fn new(config: &Settings, db: T) -> Self {
		let table_name = config.database.user_table.as_deref().unwrap_or("user");
		UserRepository {
			table: table_name.to_string(),
			db,
		}
	}
}

impl<T> UserRepositoryTrait for UserRepository<T>
where
	T: DBEnginer,
{
	async fn get_all_users(&self) -> Result<Vec<User>, Error> {
		let records = self.db.select(&self.table).await?;
		Ok(records)
	}

	async fn get_user_by_id(&self, id: &str) -> Result<User, Error> {
		if let Some(record) = self.db.select((&self.table, id)).await? {
			return Ok(record);
		}

		Err(Error::Db(
			surrealdb::error::Db::Thrown(
				format!("User with id {} not found", id)
			)
		))
	}

	async fn insert_user(&self, content: &User) -> Result<Vec<User>, Error> {
		let record = self.db.insert(&self.table).content(&content).await?;
		Ok((record))
	}

	async fn update_user(&self, id: &str, user: &User) -> Result<User, Error> {
		let record = self.db
			.update((&self.table, id))
			.content(&user)
			.await?.
			unwrap();

		Ok(record)
	}

	async fn delete_user(&self, id: &str) -> Result<User, Error> {
		let result = self.db.delete((&self.table, id)).await?.unwrap();
		Ok(result)
	}
}