use std::path::Path;

use anyhow::{Context, Result};

use crate::models::env_file::EnvFile;
use crate::utils::output::{self, Icon};
use crate::utils::project;

pub fn run(env_name: &str, output_path: &Path) -> Result<()> {
    let root = std::env::current_dir().context("failed to read current directory")?;
    project::ensure_initialized(&root)?;
    let source_path = project::ensure_env_exists(&root, env_name)?;
    let env = EnvFile::load(&source_path)?;
    env.save(output_path)?;

    output::print_line(Icon::Success, format!("Exported environment: {env_name}"));
    output::print_line(Icon::Info, format!("Output: {}", output_path.display()));
    Ok(())
}
