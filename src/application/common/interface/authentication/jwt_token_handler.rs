use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{application::common::r#type::AppResult, domain::entities::user::User};
use crate::domain::entity::user::User;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtPayload {
	pub user_id: i32,
	pub exp: u64,
}

#[async_trait]
pub trait IJwtTokenHandler: Send + Sync {
	async fn generate_token(&self, user: &User) -> String;
	fn decode_token(&self, token: &str) -> AppResult<JwtPayload>;
}
