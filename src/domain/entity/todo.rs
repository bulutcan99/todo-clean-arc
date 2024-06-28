use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::common::date_time::DateService;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Todo {
	pub id: Option<Uuid>,
	pub title: String,
	pub description: Option<String>,
	pub is_done: bool,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

impl Todo {
	pub fn new(title: String, description: Option<String>, is_done: bool) -> Self {
		Todo {
			id: None,
			title,
			description,
			is_done,
			created_at: DateService::get_curent_timestamp_utc(),
			updated_at: DateService::get_curent_timestamp_utc(),
		}
	}
}