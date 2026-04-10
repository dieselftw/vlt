use std::fs;
use std::io::IsTerminal;
use std::path::Path;

use anyhow::{Context, Result};
use dialoguer::{Select, theme::ColorfulTheme};

use crate::commands::scan;
use crate::models::config::VltConfig;
use crate::models::rules::VltRules;
use crate::utils::gitignore;
use crate::utils::output::{self, Icon};

pub fn run() -> Result<()> {
    let root = std::env::current_dir().context("failed to read current directory")?;
    let vlt_dir = root.join(".vlt");
    fs::create_dir_all(&vlt_dir).context("failed to create .vlt directory")?;

    let config_path = vlt_dir.join("config.toml");
    let rules_path = vlt_dir.join("env.rules");
    let gitignore_path = root.join(".gitignore");

    let project_type = detect_project_type(&root);

    if !config_path.exists() {
        let config = VltConfig::default();
        config.save(&config_path)?;
    }

    if !rules_path.exists() {
        let rules = VltRules::default();
        rules.save(&rules_path)?;
    }

    if gitignore::ensure_vlt_patterns(&gitignore_path)? {
        output::print_line(Icon::Success, "Updated `.gitignore`.");
    }

    output::print_line(Icon::Success, format!("Initialized vlt for a {project_type} project"));
    if should_scan_now()? {
        scan::sync_discovered_vars_quiet(&root, false)?;
    } else {
        output::print_line(Icon::Info, "Skipped scan. `.env.base` was not created.");
    }

    Ok(())
}

fn should_scan_now() -> Result<bool> {
    if !std::io::stdin().is_terminal() || !std::io::stdout().is_terminal() {
        return Ok(false);
    }

    let options = ["  Scan all variables", "  Skip for now"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose the first setup step")
        .items(&options)
        .default(0)
        .interact()
        .context("failed to read init choice")?;

    Ok(selection == 0)
}

fn detect_project_type(root: &Path) -> &'static str {
    if root.join("package.json").exists() {
        "node"
    } else if root.join("requirements.txt").exists() {
        "python"
    } else if root.join("Cargo.toml").exists() {
        "rust"
    } else if root.join("go.mod").exists() {
        "go"
    } else {
        "generic"
    }
}
