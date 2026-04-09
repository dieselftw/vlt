use std::fs;

use anyhow::{Context, Result, bail};

use crate::utils::output::{self, Icon};
use crate::utils::project;

pub fn run(env_name: &str) -> Result<()> {
    let root = std::env::current_dir().context("failed to read current directory")?;
    project::ensure_initialized(&root)?;
    project::validate_env_name(env_name)?;
    let env_path = root.join(format!(".vlt/env.{env_name}"));

    if env_path.exists() {
        bail!("Environment `{env_name}` already exists.");
    }

    let contents = project::scaffold_env_file(&root)?.render_blank();

    if let Some(parent) = env_path.parent() {
        fs::create_dir_all(parent).context("failed to create .vlt directory")?;
    }

    fs::write(&env_path, contents)
        .with_context(|| format!("failed to write {}", env_path.display()))?;

    output::print_line(Icon::Success, format!("Created environment: {env_name}"));

    Ok(())
}
