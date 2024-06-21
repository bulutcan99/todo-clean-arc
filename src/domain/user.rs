use chrono::{DateTime, Local};
use serde_derive::{Deserialize, Serialize};
use surrealdb::sql::Thing;

use crate::domain::todo::Todo;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
	pub id: Option<Thing>,
	pub name: String,
	pub surname: String,
	pub email: String,
	pub password: String,
	pub created_at: Option<DateTime<Local>>,
	pub updated_at: Option<DateTime<Local>>,
	pub todos: Vec<Todo>,
}