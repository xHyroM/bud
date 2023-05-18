use std::{fs::{self, File}, io};

use serde::Deserialize;
use lazy_static::lazy_static;

use crate::constants;

#[derive(Debug, Deserialize)]
pub struct Config {
  #[serde(default = "DefaultValues::hello")]
  pub hello: String,
}

struct DefaultValues;
impl DefaultValues {
  fn hello() -> String {
    "world".to_string()
  }
}

lazy_static! {
  pub static ref CONFIG: Config = Config::new().unwrap();
}

impl Config {
  pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
    Self::initialize()?;

    let config_path = format!("{}/config.toml", *constants::FOLDER);
    
    let contents = fs::read_to_string(config_path)?;
    let config = toml::from_str(&contents)?;
    Ok(config)
  }

  fn initialize() -> Result<File, io::Error> {
    fs::create_dir_all((*constants::FOLDER).clone()).and_then(|_| {
      let config_path = format!("{}/config.toml", *constants::FOLDER);
      fs::File::create(config_path)
    })
  }
}