use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

#[derive(Debug, Default, Clone)]
pub struct EnvFile {
    pub values: BTreeMap<String, String>,
}

impl EnvFile {
    pub fn load(path: &Path) -> Result<Self> {
        let contents = fs::read_to_string(path)
            .with_context(|| format!("failed to read {}", path.display()))?;
        Ok(Self::parse(&contents))
    }

    pub fn load_or_default(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }

        Self::load(path)
    }

    pub fn parse(contents: &str) -> Self {
        let mut values = BTreeMap::new();

        for line in contents.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            if let Some((key, value)) = trimmed.split_once('=') {
                values.insert(key.trim().to_owned(), value.trim().to_owned());
            }
        }

        Self { values }
    }

    pub fn save_with_header(&self, path: &Path, header: Option<&str>) -> Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("failed to create {}", parent.display()))?;
        }

        fs::write(path, self.render(header))
            .with_context(|| format!("failed to write {}", path.display()))
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        self.save_with_header(path, None)
    }

    pub fn insert_missing(&mut self, key: &str) -> bool {
        if self.values.contains_key(key) {
            return false;
        }

        self.values.insert(key.to_owned(), String::new());
        true
    }

    pub fn render_blank(&self) -> String {
        if self.values.is_empty() {
            return String::new();
        }

        let mut lines = self.values.keys().map(|key| format!("{key}=")).collect::<Vec<_>>();
        lines.push(String::new());
        lines.join("\n")
    }

    fn render(&self, header: Option<&str>) -> String {
        let mut sections = Vec::new();

        if let Some(header) = header.filter(|value| !value.trim().is_empty()) {
            sections.push(header.trim_end().to_owned());
        }

        if self.values.is_empty() {
            if sections.is_empty() {
                return String::new();
            }

            sections.push(String::new());
            return sections.join("\n\n");
        }

        let mut lines =
            self.values.iter().map(|(key, value)| format!("{key}={value}")).collect::<Vec<_>>();
        lines.push(String::new());
        sections.push(lines.join("\n"));
        sections.join("\n\n")
    }
}
