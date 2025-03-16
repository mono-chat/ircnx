use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub server_name: String,
    pub server_description: String,
    pub network_name: String,
    pub listen: Vec<ListenAddr>,
    pub max_clients: usize,
    pub motd: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ListenAddr {
    pub hostname: String,
    pub port: u16,
}

impl Config {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let config_str = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&config_str)?;
        Ok(config)
    }

    // pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
    //     let config_str = toml::to_string(self)?;
    //     fs::write(path, config_str)?;
    //     Ok(())
    // }
}