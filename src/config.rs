use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(
        deserialize_with = "deserialize_uppercase",
        serialize_with = "serialize_uppercase"
    )]
    pub default_country: String,
}

impl Config {
    pub fn load(config_path: &Path) -> Result<Self> {
        let config = std::fs::read_to_string(config_path)
            .with_context(|| format!("Failed to read config file at {:?}", config_path))?;
        let config: Config = toml::from_str(&config).context("Failed to parse config file")?;
        Ok(config)
    }

    pub fn save(&self, config_path: &Path) -> Result<()> {
        let toml_string = toml::to_string(self)?;
        std::fs::create_dir_all(config_path.parent().unwrap())?;
        std::fs::write(config_path, toml_string)?;
        Ok(())
    }
}

fn deserialize_uppercase<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    Ok(s.to_uppercase())
}

fn serialize_uppercase<S>(value: &str, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&value.to_uppercase())
}
