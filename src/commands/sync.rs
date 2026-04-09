use anyhow::{Context, Result, bail};

use crate::models::env_file::EnvFile;
use crate::utils::output::{self, Icon};
use crate::utils::project;

pub fn run(source: &str, target: &str) -> Result<()> {
    let root = std::env::current_dir().context("failed to read current directory")?;
    project::ensure_initialized(&root)?;

    if source == target {
        bail!("Choose two different environments to sync.");
    }

    let source_path = project::ensure_env_exists(&root, source)?;
    let target_path = project::ensure_env_exists(&root, target)?;
    let source_env = EnvFile::load(&source_path)?;
    let mut target_env = EnvFile::load(&target_path)?;

    let mut added = Vec::new();
    for key in source_env.values.keys() {
        if target_env.insert_missing(key) {
            added.push(key.clone());
        }
    }

    if added.is_empty() {
        output::print_line(Icon::Info, format!("Nothing to sync from {source} to {target}."));
        return Ok(());
    }

    target_env.save(&target_path)?;
    output::print_line(Icon::Success, format!("Synced missing keys into: {target}"));
    output::print_line(Icon::Info, format!("Added keys: {}", added.join(", ")));
    output::print_line(Icon::Warning, "Fill in the new values before using this environment.");
    Ok(())
}
