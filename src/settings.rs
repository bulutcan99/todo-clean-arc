use std::env;

use config::{Config, ConfigError, Environment, File};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Database {
	pub url: String,
	pub max_conn: u32,
}

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
	debug: bool,
	pub database: Database,
}

impl Settings {
	pub fn new() -> Result<Self, ConfigError> {
		let run_mode = env::var("RUN_MODE").unwrap_or_else(|_| "development".into());

		let s = Config::builder()
			.add_source(File::with_name("src/config/default"))
			.add_source(
				File::with_name(&format!("src/config/{}", run_mode))
					.required(false),
			)
			.add_source(File::with_name("src/config/local").required(false))
			.add_source(Environment::with_prefix("app"))
			.build()?;

		// Now that we're done, let's access our configuration
		println!("debug: {:?}", s.get_bool("debug"));
		println!("database: {:?}", s.get::<String>("database.url"));

		// You can deserialize (and thus freeze) the entire configuration as
		s.try_deserialize()
	}
}