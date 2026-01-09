use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub tokens: HashMap<String, String>,
}

impl Config {
    pub fn load(config_path: &PathBuf) -> Result<Self, String> {
        if !config_path.exists() {
            return Ok(Config::default());
        }

        let content = fs::read_to_string(config_path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        toml::from_str(&content)
            .map_err(|e| format!("Failed to parse TOML config: {}", e))
    }

    pub fn save(&self, config_path: &PathBuf) -> Result<(), String> {
        let content = toml::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config to TOML: {}", e))?;

        fs::write(config_path, content)
            .map_err(|e| format!("Failed to write config file: {}", e))
    }

    pub fn add_token(&mut self, name: String, secret: String) {
        self.tokens.insert(name, secret);
    }

    pub fn get_token(&self, name: &str) -> Option<&String> {
        self.tokens.get(name)
    }

    pub fn list_token_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.tokens.keys().cloned().collect();
        names.sort();
        names
    }
}
