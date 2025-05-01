use config::{Config, File};
use std::error::Error;

#[derive(serde::Deserialize, Debug)]
pub struct Settings {
    pub listen: Vec<ListenConfig>,
}

#[derive(serde::Deserialize, Debug)]
pub struct ListenConfig {
    pub hostname: String,
    pub port: u16,
}

pub fn load() -> Result<Settings, Box<dyn Error>> {
    let mut settings = Config::builder()
        // // Load default settings from a base TOML file
        // .add_source(File::with_name("config/default").required(false))
        // // Load environment-specific settings (e.g., config/development.toml)
        // .add_source(File::with_name(&format!("config/{}", std::env::var("APP_ENV").unwrap_or_else(|_| "development".into()))).required(false))
        // Load a specific instance config
        .add_source(File::with_name("config/ircd").required(true))
        .build()?;

    settings
        .try_deserialize()
        .map_err(|e| Box::new(e) as Box<dyn Error>)
}
