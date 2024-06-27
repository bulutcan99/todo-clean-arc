use axum::Router;
use axum::routing::get;

use crate::presentation::server_handler::health_checker_handler;

pub fn create_router() -> Router {
	Router::new()
		.route("/api/healthchecker", get(health_checker_handler))
}
