use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Todo {
	pub id: u8,
	pub user_id: u8,
	pub title: String,
	pub description: String,
	pub is_done: bool,
}