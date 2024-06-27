use anyhow::Error;
use axum::http::{HeaderValue, Method};
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use tower_http::cors::CorsLayer;

use todo::infrastructure::storage::surreal::surreal_db::{DB, DBEnginer};
use todo::infrastructure::storage::surreal::todo_repository::TodoRepository;
use todo::infrastructure::storage::surreal::user_repository::UserRepository;
use todo::presentation::router::create_router;
use todo::util::config::Settings;

#[tokio::main]
async fn main() -> Result<(), Error> {
	let settings = Settings::new().expect("Failed to load settings");
	println!("Config built.");
	let surreal_db = DB::new();
	surreal_db.connect(&settings).await?;
	let todo_repo = TodoRepository::new(&settings, &surreal_db.client);
	let user_repo = UserRepository::new(&settings, &surreal_db.client);
	println!("🚀 SurrealDB initialize successfully");
	let cors = CorsLayer::new()
		.allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
		.allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
		.allow_credentials(true)
		.allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

	let app = create_router().layer(cors);
	println!("🚀 Server started successfully");
	let app = application_initializer().await.expect("Failed to run application");
	Ok(())
}


async fn application_initializer() -> Result<(), Error> {
	println!("Running application");
	Ok(())
}
