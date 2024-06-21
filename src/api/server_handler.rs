use axum::Json;
use axum::response::IntoResponse;

pub async fn health_checker_handler() -> impl IntoResponse {
	const MESSAGE: &str = "HEALTH CHECKED!";

	let json_response = serde_json::json!({
        "status": "success",
        "message": MESSAGE
    });

	Json(json_response)
}

