use log::{error, warn};
use std::fs::File;
use std::process::exit;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: u16,
    pub whitelist: Vec<String>,
}

impl Config {
    pub fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 1657,
            whitelist: vec![],
        }
    }

    pub fn generate_default_config(path: &str) -> Result<(), std::io::Error> {
        let config = Config::default();
        let file = File::create(path)?;
        serde_yaml::to_writer(file, &config).unwrap();
        Ok(())
    }

    pub fn load_from_yaml(path: &str) -> Self {
        let file = File::open(path).unwrap_or_else(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                warn!(
                    "config file not found, generating default config file config.yaml, please edit config.yaml and restart"
                );
                Self::generate_default_config("config.yaml").unwrap_or_else(|e| {
                    error!("generating default config failed error: {:?}", e);
                });
            } else {
                error!("opening config file failed error: {:?}", e);
            }
            exit(1);
        });
        let config: Config = serde_yaml::from_reader(file).unwrap_or_else(|e| {
            error!("loading config failed error: {:?}", e);
            exit(1);
        });
        config
    }
}
