use std::{
    fs::{self},
    sync::Mutex,
};

use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::constants;

mod license_field;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    #[serde(default = "license_field::default_values::license")]
    pub license: license_field::License,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            license: license_field::default_values::license(),
        }
    }
}

lazy_static! {
    pub static ref CONFIG_PATH: Mutex<String> = Mutex::new(String::new());
}

lazy_static! {
    pub static ref CONFIG: Config = {
        let config_path = CONFIG_PATH.lock().unwrap().to_string();
        match Config::new(config_path) {
            Ok(config) => config,
            Err(err) => {
                logger::error!("Failed to load global config: {}", err);
                std::process::exit(1);
            }
        }
    };
}

impl Config {
    pub fn new(config_path: String) -> Result<Self, Box<dyn std::error::Error>> {
        if fs::metadata(&config_path).is_err() {
            logger::debug!("Global config file not found, creating one...");
            fs::create_dir_all((*constants::DATA_FOLDER).clone())?;

            let config = Self::default();
            config.save()?;

            return Ok(config);
        }

        let contents = fs::read_to_string(&config_path)?;
        let config = toml::from_str(&contents)?;
        Ok(config)
    }

    pub fn serialize(&self) -> Result<String, Box<dyn std::error::Error>> {
        let config = toml::to_string(&self)?;
        Ok(config)
    }

    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = CONFIG_PATH.lock().unwrap().to_string();
        let config = self.serialize()?;

        fs::write(config_path, config)?;
        Ok(())
    }
}

pub fn setup(config_path: String) {
    let mut config_path_mutex = CONFIG_PATH.lock().unwrap();
    *config_path_mutex = config_path;
    drop(config_path_mutex);

    lazy_static::initialize(&CONFIG);
}