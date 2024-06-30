use anyhow::Error;

use crate::infrastructure::config::Settings;

pub trait DBEnginer {
	fn new() -> Self;
	fn connect(&self, config: &Settings) -> Result<(), Error>;
	fn disconnect(&self) -> Result<(), Error>;
}