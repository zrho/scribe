use anyhow::Result;
use serde::Deserialize;

/// Notes configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {}

impl Default for Config {
    fn default() -> Self {
        Self {}
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let content = std::fs::read_to_string("./scribe-notes.toml")?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }
}

pub const TEMPLATES_DIR: &'static str = "templates/";
pub const NOTES_INPUT_DIR: &'static str = "notes/";
pub const NOTES_OUTPUT_DIR: &'static str = "dist/notes/";
