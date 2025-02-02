use std::{error::Error, fs, net::IpAddr, path::Path};

use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

use crate::providers::config::TaskRunnerConfig;

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct Config {
    pub server: ServerConfig,
    pub task_runner: TaskRunnerConfig,
}

#[derive(Serialize, Deserialize, Clone, Copy, Debug)]
pub struct ServerConfig {
    pub listen_address: IpAddr,
    pub port: u16,
    #[serde(default)]
    pub workers: Option<usize>,
}

impl Config {
    fn read_config(config_source: &str) -> anyhow::Result<Self> {
        let mut tera = Tera::default();

        tera.add_raw_template("test-config", config_source)?;
        let ctx = Context::new();
        let final_config = tera.render("test-config", &ctx)?;

        let config = toml::from_str(&final_config)?;

        Ok(config)
    }

    pub fn read_from_file(file_path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let config_file_contents = fs::read_to_string(file_path)?;
        let config = Self::read_config(&config_file_contents)?;
        Ok(config)
    }
}
