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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.tokens.len(), 0);
    }

    #[test]
    fn test_add_and_get_token() {
        let mut config = Config::default();
        config.add_token("test".to_string(), "SECRET123".to_string());
        
        assert_eq!(config.get_token("test"), Some(&"SECRET123".to_string()));
        assert_eq!(config.get_token("nonexistent"), None);
    }

    #[test]
    fn test_list_token_names() {
        let mut config = Config::default();
        config.add_token("zebra".to_string(), "SECRET1".to_string());
        config.add_token("apple".to_string(), "SECRET2".to_string());
        config.add_token("banana".to_string(), "SECRET3".to_string());
        
        let names = config.list_token_names();
        assert_eq!(names, vec!["apple", "banana", "zebra"]);
    }

    #[test]
    fn test_save_and_load() {
        let temp_dir = std::env::temp_dir();
        let config_path = temp_dir.join("test_config.toml");
        
        // Clean up if file exists
        let _ = fs::remove_file(&config_path);
        
        // Create and save config
        let mut config = Config::default();
        config.add_token("token1".to_string(), "JBSWY3DPEHPK3PXP".to_string());
        config.add_token("token2".to_string(), "HXDMVJECJJWSRB3H".to_string());
        
        config.save(&config_path).expect("Failed to save config");
        
        // Load config
        let loaded_config = Config::load(&config_path).expect("Failed to load config");
        
        assert_eq!(loaded_config.get_token("token1"), Some(&"JBSWY3DPEHPK3PXP".to_string()));
        assert_eq!(loaded_config.get_token("token2"), Some(&"HXDMVJECJJWSRB3H".to_string()));
        
        // Clean up
        let _ = fs::remove_file(&config_path);
    }

    #[test]
    fn test_load_nonexistent_file() {
        let temp_dir = std::env::temp_dir();
        let config_path = temp_dir.join("nonexistent_config.toml");
        
        // Make sure file doesn't exist
        let _ = fs::remove_file(&config_path);
        
        let config = Config::load(&config_path).expect("Should return default config");
        assert_eq!(config.tokens.len(), 0);
    }
}

