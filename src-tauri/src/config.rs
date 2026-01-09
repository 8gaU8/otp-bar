use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenData {
    pub secret: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub priority: Option<i32>,
    #[serde(default)]
    pub usage_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    #[serde(default)]
    pub tokens: HashMap<String, TokenData>,
}

impl Config {
    fn ensure_config_exists(config_path: &PathBuf) -> Result<(), String> {
        if !config_path.exists() {
            fs::create_dir_all(
                config_path
                    .parent()
                    .ok_or("Failed to get parent directory of config path")?,
            )
            .map_err(|e| format!("Failed to create config directory: {}", e))?;
            fs::write(config_path, "")
                .map_err(|e| format!("Failed to create empty config file: {}", e))?;
        }
        println!("Config file exists at {:?}", config_path);
        Ok(())
    }

    pub fn load(config_path: &PathBuf) -> Result<Self, String> {
        Self::ensure_config_exists(config_path)
            .map_err(|e| format!("Failed to ensure config exists: {}", e))?;

        let content = fs::read_to_string(config_path)
            .map_err(|e| format!("Failed to read config file: {}", e))?;

        toml::from_str(&content).map_err(|e| format!("Failed to parse TOML config: {}", e))
    }

    pub fn save(&self, config_path: &PathBuf) -> Result<(), String> {
        // Ensure parent directory exists
        Self::ensure_config_exists(config_path)
            .map_err(|e| format!("Failed to ensure config exists: {}", e))?;

        let content = toml::to_string_pretty(self)
            .map_err(|e| format!("Failed to serialize config to TOML: {}", e))?;

        fs::write(config_path, content).map_err(|e| format!("Failed to write config file: {}", e))
    }

    pub fn add_token(&mut self, name: String, secret: String) {
        self.tokens.insert(
            name,
            TokenData {
                secret,
                priority: None,
                usage_count: 0,
            },
        );
    }

    pub fn increment_usage(&mut self, name: &str) {
        if let Some(token_data) = self.tokens.get_mut(name) {
            token_data.usage_count += 1;
        }
    }

    pub fn get_token(&self, name: &str) -> Option<&String> {
        self.tokens.get(name).map(|t| &t.secret)
    }

    pub fn list_token_names(&self) -> Vec<String> {
        let mut tokens: Vec<(&String, &TokenData)> = self.tokens.iter().collect();

        // Sort by:
        // 1. Usage count (descending) - most used first
        // 2. Priority (ascending) - lower priority values first
        // 3. Name (alphabetically)
        tokens.sort_by(|(name_a, data_a), (name_b, data_b)| {
            // Create sort keys as tuples for cleaner comparison
            let key_a = (
                data_a.priority.unwrap_or(i32::MAX),    // None priority sorts last
                std::cmp::Reverse(data_a.usage_count), // Reverse for descending order
                name_a,
            );
            let key_b = (
                data_b.priority.unwrap_or(i32::MAX),
                std::cmp::Reverse(data_b.usage_count),
                name_b,
            );
            key_a.cmp(&key_b)
        });

        tokens.into_iter().map(|(name, _)| name.clone()).collect()
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
    fn test_list_token_names_alphabetical() {
        let mut config = Config::default();
        config.add_token("zebra".to_string(), "SECRET1".to_string());
        config.add_token("apple".to_string(), "SECRET2".to_string());
        config.add_token("banana".to_string(), "SECRET3".to_string());

        let names = config.list_token_names();
        assert_eq!(names, vec!["apple", "banana", "zebra"]);
    }

    #[test]
    fn test_list_token_names_with_priority() {
        let mut config = Config::default();

        // Add tokens with priority
        config.tokens.insert(
            "token_b".to_string(),
            TokenData {
                secret: "SECRET2".to_string(),
                priority: Some(3),
                usage_count: 0,
            },
        );
        config.tokens.insert(
            "token_a".to_string(),
            TokenData {
                secret: "SECRET1".to_string(),
                priority: Some(1),
                usage_count: 0,
            },
        );

        // Add tokens without priority
        config.add_token("zebra".to_string(), "SECRET4".to_string());
        config.add_token("apple".to_string(), "SECRET5".to_string());

        let names = config.list_token_names();
        // Prioritized tokens first (sorted by priority), then alphabetically sorted tokens
        assert_eq!(names, vec!["token_a", "token_b", "apple", "zebra"]);
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
        config.tokens.insert(
            "token2".to_string(),
            TokenData {
                secret: "HXDMVJECJJWSRB3H".to_string(),
                priority: Some(1),
                usage_count: 0,
            },
        );

        config.save(&config_path).expect("Failed to save config");

        // Load config
        let loaded_config = Config::load(&config_path).expect("Failed to load config");

        assert_eq!(
            loaded_config.get_token("token1"),
            Some(&"JBSWY3DPEHPK3PXP".to_string())
        );
        assert_eq!(
            loaded_config.get_token("token2"),
            Some(&"HXDMVJECJJWSRB3H".to_string())
        );
        assert_eq!(
            loaded_config.tokens.get("token2").unwrap().priority,
            Some(1)
        );

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

    #[test]
    fn test_increment_usage() {
        let mut config = Config::default();
        config.add_token("test".to_string(), "SECRET123".to_string());

        // Check initial usage count
        assert_eq!(config.tokens.get("test").unwrap().usage_count, 0);

        // Increment usage
        config.increment_usage("test");
        assert_eq!(config.tokens.get("test").unwrap().usage_count, 1);

        config.increment_usage("test");
        assert_eq!(config.tokens.get("test").unwrap().usage_count, 2);

        // Incrementing non-existent token should not panic
        config.increment_usage("nonexistent");
    }

    #[test]
    fn test_list_token_names_with_usage_count() {
        let mut config = Config::default();

        // Add tokens with different usage counts
        config.tokens.insert(
            "token_a".to_string(),
            TokenData {
                secret: "SECRET1".to_string(),
                priority: None,
                usage_count: 5,
            },
        );
        config.tokens.insert(
            "token_b".to_string(),
            TokenData {
                secret: "SECRET2".to_string(),
                priority: None,
                usage_count: 10,
            },
        );
        config.tokens.insert(
            "token_c".to_string(),
            TokenData {
                secret: "SECRET3".to_string(),
                priority: None,
                usage_count: 2,
            },
        );

        let names = config.list_token_names();
        // Should be sorted by usage count (descending): token_b (10), token_a (5), token_c (2)
        assert_eq!(names, vec!["token_b", "token_a", "token_c"]);
    }

    #[test]
    fn test_list_token_names_with_usage_and_priority() {
        let mut config = Config::default();

        // Token with high usage but no priority
        config.tokens.insert(
            "frequently_used".to_string(),
            TokenData {
                secret: "SECRET1".to_string(),
                priority: None,
                usage_count: 100,
            },
        );
        
        // Token with low usage and priority
        config.tokens.insert(
            "priority_token".to_string(),
            TokenData {
                secret: "SECRET2".to_string(),
                priority: Some(1),
                usage_count: 5,
            },
        );
        
        // Token with medium usage and no priority
        config.tokens.insert(
            "sometimes_used".to_string(),
            TokenData {
                secret: "SECRET3".to_string(),
                priority: None,
                usage_count: 50,
            },
        );

        let names = config.list_token_names();
        // Should be sorted by usage count first (descending)
        // frequently_used (100) -> sometimes_used (50) -> priority_token (5)
        assert_eq!(names, vec!["frequently_used", "sometimes_used", "priority_token"]);
    }
}
