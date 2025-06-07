use anyhow::Result;
use serde::Deserialize;

/// Notes configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// The templates directory.
    #[serde(default = "default_template_path")]
    pub templates: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            templates: default_template_path(),
        }
    }
}

fn default_template_path() -> String {
    "templates/".into()
}

impl Config {
    pub fn load() -> Result<Self> {
        let content = std::fs::read_to_string("./scribe-notes.toml")?;
        let config = toml::from_str(&content)?;
        Ok(config)
    }
}
