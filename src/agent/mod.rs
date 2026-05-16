// src/agent/mod.rs
pub mod auto;
pub mod code_parser;
pub mod connector;
pub mod integrations;
pub mod mode;

use crate::fs::agent_dir;
use anyhow::Result;
use std::fs;

/// Agent configuration stored in .agent/config.toml
/// This is the hierarchical config that takes precedence over mode.toml
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AgentConfig {
    /// Current active mode
    #[serde(default = "default_mode")]
    pub current_mode: String,

    /// Protected areas that cannot be deleted
    #[serde(default)]
    pub protected_areas: Vec<String>,

    /// Default area for the current mode
    #[serde(default)]
    pub default_area: Option<String>,

    /// User-defined connectors
    #[serde(default)]
    pub connectors: Vec<ConnectorConfig>,

    /// Custom settings that persist across mode switches
    #[serde(default)]
    pub settings: AgentSettings,
}

fn default_mode() -> String {
    "default".to_string()
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            current_mode: "default".to_string(),
            protected_areas: vec![],
            default_area: None,
            connectors: vec![],
            settings: AgentSettings::default(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AgentSettings {
    /// Custom key-value settings
    #[serde(default)]
    pub custom: std::collections::HashMap<String, String>,
}

impl Default for AgentSettings {
    fn default() -> Self {
        Self {
            custom: std::collections::HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConnectorConfig {
    pub name: String,
    pub description: String,
    pub command: String,
    #[serde(default)]
    pub args: Vec<String>,
    #[serde(default)]
    pub env: std::collections::HashMap<String, String>,
    #[serde(default)]
    pub working_dir: Option<String>,
    #[serde(default)]
    pub timeout: Option<u64>,
}

pub fn load_agent_config() -> Result<AgentConfig> {
    let path = agent_dir().join("config.toml");
    if !path.exists() {
        return Ok(AgentConfig::default());
    }
    let content = fs::read_to_string(&path)?;

    // Try parsing as AgentConfig first
    if let Ok(config) = toml::from_str::<AgentConfig>(&content) {
        return Ok(config);
    }

    // Fallback: try parsing as old LegacyConfig for backwards compatibility
    if let Ok(legacy) = toml::from_str::<LegacyConfig>(&content) {
        return Ok(legacy.into());
    }

    // If both fail, return default
    Ok(AgentConfig::default())
}

/// Legacy config format for backwards compatibility
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct LegacyConfig {
    #[serde(default)]
    area: Option<String>,
    #[serde(default)]
    protected_areas: Vec<String>,
    #[serde(default)]
    paddy_enabled: Option<bool>,
    #[serde(default)]
    current_mode: Option<String>,
}

impl From<LegacyConfig> for AgentConfig {
    fn from(legacy: LegacyConfig) -> Self {
        Self {
            current_mode: legacy.current_mode.unwrap_or_else(|| "default".to_string()),
            protected_areas: legacy.protected_areas,
            default_area: legacy.area,
            connectors: vec![],
            settings: AgentSettings::default(),
        }
    }
}

pub fn save_agent_config(config: &AgentConfig) -> Result<()> {
    let path = agent_dir().join("config.toml");
    fs::create_dir_all(path.parent().unwrap())?;
    let content = toml::to_string_pretty(config)?;
    fs::write(&path, content)?;
    Ok(())
}

pub fn current_mode() -> Result<String> {
    let config = load_agent_config()?;
    Ok(config.current_mode)
}

pub fn set_current_mode(mode: &str) -> Result<()> {
    let mut config = load_agent_config()?;
    config.current_mode = mode.to_string();
    save_agent_config(&config)?;
    Ok(())
}

pub fn set_protected_areas(areas: Vec<String>) -> Result<()> {
    let mut config = load_agent_config()?;
    config.protected_areas = areas;
    save_agent_config(&config)?;
    Ok(())
}

pub fn get_protected_areas() -> Result<Vec<String>> {
    let config = load_agent_config()?;
    let mut protected = config.protected_areas;

    // Also include protected areas from current mode
    if let Ok(mode_config) = mode::get_mode_info(&config.current_mode) {
        for area in mode_config.areas.protected {
            if !protected.contains(&area) {
                protected.push(area);
            }
        }
    }

    Ok(protected)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct McpTool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}
