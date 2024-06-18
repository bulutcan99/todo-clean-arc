use sqlx::types::time::Date;

pub struct Todo {
	pub id: u8,
	pub user_id: u8,
	pub title: String,
	pub status: String,
	pub created_at: Date,
	pub updated_at: Date,
}
