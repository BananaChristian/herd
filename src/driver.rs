use std::fs;

use crate::{builder::Workspace, config::Config};

pub fn load_config() -> Result<Config, std::io::Error> {
    let config_src = "build.toml".to_string();
    let contents = fs::read_to_string(config_src)?;
    let config = toml::from_str(&contents).expect("Failed to parse config");

    Ok(config)
}
