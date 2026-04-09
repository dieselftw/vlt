use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RuleType {
    #[default]
    String,
    Int,
    Float,
    Bool,
    Enum,
    Secret,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VarRule {
    #[serde(rename = "type")]
    pub rule_type: RuleType,
    #[serde(default)]
    pub required: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VltRules {
    #[serde(default)]
    pub vars: BTreeMap<String, VarRule>,
}

impl VltRules {
    pub fn load(path: &Path) -> Result<Self> {
        let contents = fs::read_to_string(path)
            .with_context(|| format!("failed to read {}", path.display()))?;

        if contents.trim().is_empty() {
            return Ok(Self::default());
        }

        toml::from_str(&contents).with_context(|| format!("failed to parse {}", path.display()))
    }

    pub fn load_or_default(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }

        Self::load(path)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create {}", parent.display()))?;
        }

        let contents = toml::to_string_pretty(self).context("failed to serialize rules")?;
        fs::write(path, contents).with_context(|| format!("failed to write {}", path.display()))
    }

    pub fn scaffold_env_file(&self) -> String {
        let mut lines = Vec::new();
        for key in self.vars.keys() {
            lines.push(format!("{key}="));
        }
        if lines.is_empty() { String::new() } else { format!("{}\n", lines.join("\n")) }
    }

    pub fn missing_values(&self, values: &BTreeMap<String, String>) -> Vec<String> {
        self.vars
            .keys()
            .filter(|key| values.get(*key).is_none_or(|value| value.trim().is_empty()))
            .cloned()
            .collect()
    }

    pub fn unknown_keys(&self, values: &BTreeMap<String, String>) -> Vec<String> {
        values.keys().filter(|key| !self.vars.contains_key(*key)).cloned().collect()
    }
}
