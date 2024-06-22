use anyhow::Error;
use axum::http::{HeaderValue, Method};
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use tower_http::cors::CorsLayer;

use todo::api::router::create_router;
use todo::domain::user::User;
use todo::infrastructure::storage::db::surreal_db::DB;
use todo::infrastructure::storage::db::todo_repository::TodoRepository;
use todo::infrastructure::storage::db::user_repository::UserRepository;
use todo::util::config::Settings;

#[tokio::main]
async fn main() -> Result<(), Error> {
	let settings = Settings::new().expect("Failed to load settings");
	println!("Config built.");
	let surreal_db = DB::new();
	surreal_db.connect(&settings).await?;
	let todo_repo = TodoRepository::new(&settings, surreal_db.client.clone());
	let user_repo = UserRepository::new(&settings, surreal_db.client.clone());
	let user = User {
		id: None,
		name: "Bulut".to_string(),
		surname: "Gocer".to_string(),
		email: "bulut@gmail.com".to_string(),
		password: "12345".to_string(),
		created_at: None,
		updated_at: None,
		todos: vec![],
	};
	user_repo.insert_user(&user).await?;
	let all_users = user_repo.get_all_users().await?;
	println!("🚀 SurrealDB initialize successfully");
	let cors = CorsLayer::new()
		.allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
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
