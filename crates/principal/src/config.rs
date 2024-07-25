use std::{fs, path::PathBuf};

use anyhow::Context;
use serde::Deserialize;

pub const CONFIG_FOLDER: &str = ".tasc-rs-principal";
pub const CONFIG_FILE: &str = "config.toml";

pub fn get_default_config_file() -> Option<PathBuf> {
    dirs::home_dir().map(|path| path.join(CONFIG_FOLDER).join(CONFIG_FILE))
}

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_host")]
    pub host: String,

    #[serde(default = "default_port")]
    pub port: u16,

    #[serde(default)]
    pub require_ssl: bool,

    pub database: Database,
}

fn default_host() -> String {
    String::from("0.0.0.0")
}
fn default_port() -> u16 {
    3000
}

#[derive(Debug, Deserialize)]
pub struct Database {
    pub url: String,
}

impl Config {
    pub fn route(self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

pub fn get_config(config_path: &PathBuf) -> Result<Config, anyhow::Error> {
    let config_contents = fs::read_to_string(config_path).context("could not read config file")?;
    let config =
        toml::from_str::<Config>(&config_contents).context("could not read provided config")?;
    Ok(config)
}
