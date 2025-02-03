use crate::errors::{NodeError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
struct AliasConfig {
    aliases: HashMap<String, String>,
    #[serde(default)]
    default: Option<String>,
}

impl Default for AliasConfig {
    fn default() -> Self {
        Self {
            aliases: HashMap::new(),
            default: None,
        }
    }
}

fn get_alias_file() -> PathBuf {
    let config_dir = crate::config::get_config_dir();
    config_dir.join("aliases.json")
}

fn load_aliases() -> Result<AliasConfig> {
    let alias_file = get_alias_file();
    if !alias_file.exists() {
        return Ok(AliasConfig::default());
    }

    let content = fs::read_to_string(&alias_file)?;
    let config = serde_json::from_str(&content)
        .map_err(|e| NodeError::ConfigError(format!("Failed to parse alias file: {}", e)))?;
    Ok(config)
}

fn save_aliases(config: &AliasConfig) -> Result<()> {
    let alias_file = get_alias_file();

    if let Some(parent) = alias_file.parent() {
        fs::create_dir_all(parent)?;
    }

    let content = serde_json::to_string_pretty(&config)
        .map_err(|e| NodeError::ConfigError(format!("Failed to serialize aliases: {}", e)))?;
    fs::write(&alias_file, content)?;
    Ok(())
}

pub fn set_alias(name: &str, version: &str) -> Result<()> {
    let mut config = load_aliases()?;
    config.aliases.insert(name.to_string(), version.to_string());
    save_aliases(&config)?;
    Ok(())
}

pub fn remove_alias(name: &str) -> Result<()> {
    let mut config = load_aliases()?;
    if config.aliases.remove(name).is_none() {
        return Err(NodeError::AliasError(format!("Alias '{}' not found", name)));
    }
    save_aliases(&config)?;
    Ok(())
}

pub fn get_alias(name: &str) -> Result<Option<String>> {
    let config = load_aliases()?;
    Ok(config.aliases.get(name).cloned())
}

pub fn list_aliases() -> Result<HashMap<String, String>> {
    let config = load_aliases()?;
    Ok(config.aliases)
}