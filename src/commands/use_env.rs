use std::fs;

use anyhow::{Context, Result};

use crate::models::config::VltConfig;
use crate::utils::output::{self, Icon};
use crate::utils::project;

pub fn run(env_name: &str) -> Result<()> {
    let root = std::env::current_dir().context("failed to read current directory")?;
    project::ensure_initialized(&root)?;
    let source_path = project::ensure_env_exists(&root, env_name)?;
    let target_path = root.join(".env");
    let config_path = root.join(".vlt/config.toml");

    let contents = fs::read_to_string(&source_path)
        .with_context(|| format!("failed to read {}", source_path.display()))?;

    fs::write(&target_path, contents)
        .with_context(|| format!("failed to write {}", target_path.display()))?;

    let mut config = VltConfig::load_or_default(&config_path)?;
    config.active_env = Some(env_name.to_owned());
    config.save(&config_path)?;

    output::print_line(Icon::Success, format!("Activated environment: {env_name}"));

    Ok(())
}
