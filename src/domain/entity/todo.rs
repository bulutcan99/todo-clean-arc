use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Todo {
	pub id: Option<Uuid>,
	pub title: String,
	pub description: String,
	pub is_done: bool,
} 