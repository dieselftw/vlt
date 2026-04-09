use std::fs;

use anyhow::{Context, Result, bail};

use crate::models::env_file::EnvFile;
use crate::models::rules::VltRules;
use crate::utils::output::{self, Icon};
use crate::utils::project;

pub fn run(env_name: &str) -> Result<()> {
    validate_env_name(env_name)?;

    let root = std::env::current_dir().context("failed to read current directory")?;
    project::ensure_initialized(&root)?;
    let rules_path = root.join(".vlt/env.rules");
    let base_template_path = root.join(".env.base");
    let env_path = root.join(format!(".vlt/env.{env_name}"));

    if env_path.exists() {
        bail!("Environment `{env_name}` already exists.");
    }

    let contents = if base_template_path.exists() {
        let template = EnvFile::load(&base_template_path)?;
        template.render_blank()
    } else {
        let rules = VltRules::load(&rules_path)?;
        rules.scaffold_env_file()
    };

    if let Some(parent) = env_path.parent() {
        fs::create_dir_all(parent).context("failed to create .vlt directory")?;
    }

    fs::write(&env_path, contents)
        .with_context(|| format!("failed to write {}", env_path.display()))?;

    output::print_line(Icon::Success, format!("Created environment: {env_name}"));
    output::print_line(Icon::Info, "Fill in the values before using it.");

    Ok(())
}

fn validate_env_name(env_name: &str) -> Result<()> {
    if env_name.is_empty()
        || !env_name.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '-' || ch == '_')
    {
        bail!("environment name must use only letters, numbers, hyphens, or underscores");
    }

    Ok(())
}
