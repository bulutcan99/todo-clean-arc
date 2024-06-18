#[cfg(test)]
mod tests {
	use crate::model::db::init_db;
	use crate::settings::Settings;

	#[tokio::test]
	async fn test_model_db_new_db_pool() -> Result<(), Box<dyn std::error::Error>> {
		let settings = Settings::new();
		let settings = match settings {
			Ok(settings) => settings,
			Err(e) => panic!("Error: {}", e),
		};
		let db = init_db(settings.database).await?;
		Ok(())
	}
}