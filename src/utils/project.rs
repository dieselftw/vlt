use std::path::{Path, PathBuf};

use anyhow::{Result, bail};

use crate::models::env_file::EnvFile;
use crate::models::rules::VltRules;

pub fn ensure_initialized(root: &Path) -> Result<()> {
    if !root.join(".vlt").exists() {
        bail!("Project not initialized. Run `vlt init` first.");
    }

    if !root.join(".vlt/config.toml").exists() || !root.join(".vlt/env.rules").exists() {
        bail!("vlt project is incomplete. Run `vlt init` again.");
    }

    Ok(())
}

pub fn validate_env_name(env_name: &str) -> Result<()> {
    if env_name.is_empty()
        || !env_name.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
    {
        bail!("Environment names may only contain letters, numbers, hyphens, and underscores.");
    }

    Ok(())
}

pub fn env_file_path(root: &Path, env_name: &str) -> PathBuf {
    root.join(format!(".vlt/env.{env_name}"))
}

pub fn ensure_env_exists(root: &Path, env_name: &str) -> Result<PathBuf> {
    validate_env_name(env_name)?;
    let path = env_file_path(root, env_name);
    if !path.exists() {
        bail!("Environment `{env_name}` was not found. Run `vlt create {env_name}` first.");
    }

    Ok(path)
}

pub fn available_envs(root: &Path) -> Result<Vec<String>> {
    let mut envs = Vec::new();
    let vlt_dir = root.join(".vlt");

    if !vlt_dir.exists() {
        return Ok(envs);
    }

    for entry in std::fs::read_dir(vlt_dir)? {
        let entry = entry?;
        if let Some(name) = entry.file_name().to_str()
            && let Some(env_name) = name.strip_prefix("env.")
            && env_name != "rules"
        {
            envs.push(env_name.to_owned());
        }
    }

    envs.sort();
    Ok(envs)
}

pub fn scaffold_env_file(root: &Path) -> Result<EnvFile> {
    let base_template_path = root.join(".env.base");
    if base_template_path.exists() {
        return EnvFile::load(&base_template_path);
    }

    let rules = VltRules::load(&root.join(".vlt/env.rules"))?;
    Ok(EnvFile::parse(&rules.scaffold_env_file()))
}
