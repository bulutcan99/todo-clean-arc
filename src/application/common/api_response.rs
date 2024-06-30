use axum::{
	http::StatusCode,
	Json
	,
	response::IntoResponse,
};
use axum::response::Response;
use serde::Serialize;

use crate::application::common::util::date_time::DateService;

#[derive(Debug, Serialize)]
pub struct ApiResponse<T>
where
	T: Serialize,
{
	code: u16,
	internal_code: Option<u32>,
	error: bool,
	message: Option<String>,
	result: Option<T>,
	timestamp: u64,
}

impl<T> ApiResponse<T>
where
	T: Serialize,
{
	pub fn success(message: Option<String>, result: Option<T>) -> Self {
		Self {
			code: 200,
			internal_code: None,
			error: false,
			message,
			result,
			timestamp: DateService::current_timestamp(),
		}
	}

	pub fn error(code: u16, message: Option<String>, result: Option<T>) -> Self {
		Self {
			code,
			internal_code: None,
			error: true,
			message,
			result,
			timestamp: DateService::current_timestamp(),
		}
	}
}

impl<T> IntoResponse for ApiResponse<T>
where
	T: Serialize,
{
	fn into_response(self) -> Response {
		let status = StatusCode::from_u16(self.code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
		(status, Json(self)).into_response()
	}
}
// //Example:
// async fn hello_handler() -> impl IntoResponse {
// 	ApiResponse::success(Some("Hello, world!".to_string()), Some("This is the result".to_string()))
// }
//
// async fn error_handler() -> impl IntoResponse {
// 	ApiResponse::error(
// 		404,
// 		Some(1001),
// 		Some("Resource not found".to_string()),
// 		None::<String>,
// 	)
// }