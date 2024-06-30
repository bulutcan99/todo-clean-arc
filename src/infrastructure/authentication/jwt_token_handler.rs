use async_trait::async_trait;
use jsonwebtoken::{decode, DecodingKey, encode, EncodingKey, Header, Validation};

use crate::{
    application::common::{errors::Error, r#type::AppResult},
    application::common::interfaces::authentication::jwt_token_handler::{IJwtTokenHandler, JwtPayload}, config::AuthenticationConfig, domain::entities::user::User, infrastructure::common::date_service::DateService,
};

//TODO: handle this task async
//TODO: handle error here
pub struct JwtTokenHandler {
	secret: String,
}

impl JwtTokenHandler {
	pub fn new() -> Self {
		Self {
			secret: AuthenticationConfig::jwt_secret()
		}
	}

	pub fn get_expire_time(&self) -> u64 {
		let current_time = DateService::get_cuurent_timestamp();

		current_time + 1000 * 60 * 60 * 5
	}
}

#[async_trait]
impl IJwtTokenHandler for JwtTokenHandler {
	async fn generate_token(&self, user: &User) -> String {
		let expire_time = self.get_expire_time();
		let payload = JwtPayload {
			user_id: user.id,
			exp: expire_time,
        };

		let token = encode(
			&Header::default(),
			&payload,
			&EncodingKey::from_secret(self.secret.as_ref()),
		)
			.unwrap();

		token
	}

	fn decode_token(&self, token: &str) -> AppResult<JwtPayload> {
		let decoded_token = decode::<JwtPayload>(
            token,
            &DecodingKey::from_secret(self.secret.as_ref()),
            &Validation::default(),
        );

		match decoded_token {
			Ok(value) => {
				Ok(value.claims)
			}
			Err(_) => Err(Error::AuthorizationFailed(format!("Your access token is not valid. please login again.")))
		}
	}
}
