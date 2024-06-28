use chrono::{DateTime, Local};
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct User {
	pub id: Option<Uuid>,
	pub name: String,
	pub surname: String,
	pub email: String,
	pub password_hash: String,
	pub created_at: DateTime<Local>,
	pub updated_at: DateTime<Local>,
}

impl User {
	pub fn new(name: String, surname: String, email: String, password_hash: String) -> Self {
		let now = Local::now();
		User {
			id: None,
			name,
			surname,
			email,
			password_hash,
			created_at: now,
			updated_at: now,
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
		self.updated_at = Local::now();
	}
}

