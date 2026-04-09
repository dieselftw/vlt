use anyhow::{Context, Result};

use crate::models::config::VltConfig;
use crate::models::env_file::EnvFile;
use crate::models::rules::VltRules;
use crate::utils::output::{self, Icon};
use crate::utils::project;

pub fn run() -> Result<()> {
    let root = std::env::current_dir().context("failed to read current directory")?;
    project::ensure_initialized(&root)?;
    let config = VltConfig::load_or_default(&root.join(".vlt/config.toml"))?;
    let rules = VltRules::load_or_default(&root.join(".vlt/env.rules"))?;
    let available_envs = project::available_envs(&root)?;

    match &config.active_env {
        Some(active) => output::print_line(Icon::Info, format!("Active environment: {active}")),
        None => output::print_line(Icon::Warning, "Active environment: none"),
    }

    if available_envs.is_empty() {
        output::print_line(Icon::Warning, "No environments found.");
    } else {
        output::print_line(
            Icon::Success,
            format!("Available environments: {}", available_envs.join(", ")),
        );
    }

    if let Some(active) = &config.active_env {
        let active_env_path = root.join(format!(".vlt/env.{active}"));
        if !active_env_path.exists() {
            output::print_line(
                Icon::Error,
                format!("Active environment file is missing for `{active}`."),
            );
        } else {
            let active_env = EnvFile::load(&active_env_path)?;
            let missing_values = rules.missing_values(&active_env.values);

            if missing_values.is_empty() {
                output::print_line(Icon::Success, "All required keys have values.");
            } else {
                output::print_line(
                    Icon::Warning,
                    format!("Missing values: {}", missing_values.join(", ")),
                );
            }
        }
    }

    let dot_env_path = root.join(".env");
    if dot_env_path.exists() {
        let dot_env = EnvFile::load(&dot_env_path)?;
        let drift = rules.unknown_keys(&dot_env.values);
        if drift.is_empty() {
            output::print_line(Icon::Success, "No drift detected in `.env`.");
        } else {
            output::print_line(Icon::Warning, format!("Drift in `.env`: {}", drift.join(", ")));
        }
    } else {
        output::print_line(Icon::Info, "`.env` has not been generated yet.");
    }

    Ok(())
}
