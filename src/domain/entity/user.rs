use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

use crate::application::common::date_time::DateService;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
	pub id: Option<Uuid>,
	pub name: String,
	pub surname: String,
	pub email: String,
	pub password_hash: String,
	pub created_at: DateTime<Utc>,
	pub updated_at: DateTime<Utc>,
}

impl User {
	pub fn new(name: String, surname: String, email: String, password_hash: String) -> Self {
		User {
			id: None,
			name,
			surname,
			email,
			password_hash,
			created_at: DateService::get_curent_timestamp_utc(),
			updated_at: DateService::get_curent_timestamp_utc(),
		}
	}

	pub fn update(&mut self, name: Option<String>, surname: Option<String>, email: Option<String>, password_hash: Option<String>) {
		if let Some(password_hash) = password_hash {
			self.password_hash = password_hash;
		}
		if let Some(name) = name {
			self.name = name;
		}
		if let Some(surname) = surname {
			self.surname = surname;
		}
		if let Some(email) = email {
			self.email = email;
		}
		self.updated_at = DateService::get_curent_timestamp_utc();
	}
}

