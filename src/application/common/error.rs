use std::vec;

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;
use thiserror::Error as ThisError;
use validator::{ValidationErrors, ValidationErrorsKind::Field};

use crate::application::common::api_response::ApiResponse;

#[derive(Debug, ThisError)]
pub enum Error {
	#[error("Oops! Something went wrong.")]
	InternalServerError,
	#[error("{0}")]
	EntityNotFound(String),
	#[error("{0}")]
	EntityExists(String),
	#[error("{0}")]
	EntityValidationFailed(String),
	#[error("Input validation failed.")]
	InputValidation(Vec<FieldError>),
	#[error("{0}")]
	AuthorizationFailed(String),
	#[error("Database operation failed")]
	DatabaseFailed,
}

impl IntoResponse for Error {
	fn into_response(self) -> Response {
		let status_code = self.status_code();
		let message = self.to_string();

		match self {
			Self::InputValidation(fields) => {
				let result = fields;
				ApiResponse::error(status_code.as_u16(), Some(message), Some(result)).into_response()
			}
			_ => {
				ApiResponse::<()>::error(status_code.as_u16(), Some(message), None).into_response()
			}
		}
	}
}

impl Error {
	fn status_code(&self) -> StatusCode {
		match self {
			Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
			Self::EntityNotFound(_) => StatusCode::NOT_FOUND,
			Self::EntityExists(_) => StatusCode::CONFLICT,
			Self::InputValidation(_) => StatusCode::BAD_REQUEST,
			Self::EntityValidationFailed(_) => StatusCode::BAD_REQUEST,
			Self::AuthorizationFailed(_) => StatusCode::UNAUTHORIZED,
			Self::DatabaseFailed => StatusCode::INTERNAL_SERVER_ERROR,
		}
	}
}

impl From<ValidationErrors> for Error {
	fn from(value: ValidationErrors) -> Self {
		let mut fields: Vec<FieldError> = vec![];

		for (field_name, error) in value.errors().iter() {
			if let Field(err) = error {
				let field_error = FieldError {
					name: field_name.to_string(),
					reason: err
						.iter()
						.next()
						.unwrap()
						.message
						.as_ref()
						.unwrap()
						.to_string(),
				};

				fields.push(field_error);
			}
		}

		Self::InputValidation(fields)
	}
}

#[derive(Debug, Serialize)]
struct CommonErrorResponse {
	code: u16,
	message: String,
	timestamp: u64,
}

#[derive(Debug, Serialize)]
struct ValidationErrorResponse {
	code: u16,
	message: String,
	fields: Vec<FieldError>,
	timestamp: u64,
}


#[derive(Debug, Clone, Serialize)]
pub struct FieldError {
	name: String,
	reason: String,
}
