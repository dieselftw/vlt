use std::io::{self, IsTerminal};
use std::path::Path;

use anyhow::{Context, Result, bail};
use dialoguer::{Select, theme::ColorfulTheme};

use crate::models::env_file::EnvFile;
use crate::models::rules::{RuleType, VarRule, VltRules};
use crate::utils::output::{self, Icon};
use crate::utils::project;
use crate::utils::scanner;

pub const BASE_TEMPLATE_HEADER: &str = "# Do not modify this file directly.\n# This is the template vlt uses when creating environment files.";

pub fn run(apply: bool) -> Result<()> {
    let root = std::env::current_dir().context("failed to read current directory")?;
    project::ensure_initialized(&root)?;
    sync_discovered_vars(&root, apply)
}

pub fn sync_discovered_vars(root: &Path, apply: bool) -> Result<()> {
    sync_discovered_vars_with_output(root, apply, true)
}

pub fn sync_discovered_vars_quiet(root: &Path, apply: bool) -> Result<()> {
    sync_discovered_vars_with_output(root, apply, false)
}

fn sync_discovered_vars_with_output(
    root: &Path,
    apply: bool,
    show_write_paths: bool,
) -> Result<()> {
    let rules_path = root.join(".vlt/env.rules");
    let sample_path = root.join(".env.base");
    let mut rules = VltRules::load_or_default(&rules_path)?;
    let mut sample = EnvFile::load_or_default(&sample_path)?;
    let scan_result = scanner::scan_project(root)?;

    if scan_result.vars.is_empty() {
        output::print_line(Icon::Warning, "No environment variables found.");
        return Ok(());
    }

    output::print_line(
        Icon::Success,
        format!("Found {} environment variable(s).", scan_result.vars.len()),
    );

    let mut missing = Vec::new();
    for var in &scan_result.vars {
        let missing_sample = !sample.values.contains_key(var);
        let missing_rules = !rules.vars.contains_key(var);
        let needs_sync = missing_sample || missing_rules;
        let icon = if needs_sync { Icon::Warning } else { Icon::Success };
        let suffix = match (missing_sample, missing_rules) {
            (false, false) => String::new(),
            (true, true) => " (missing from .env.base and env.rules)".to_owned(),
            (true, false) => " (missing from .env.base)".to_owned(),
            (false, true) => " (missing from env.rules)".to_owned(),
        };
        output::print_line(icon, format!("{var}{suffix}"));
        if needs_sync {
            missing.push(var.clone());
        }
    }

    if missing.is_empty() {
        return Ok(());
    }

    let approved = choose_vars_to_add(&missing, apply)?;
    if approved.is_empty() {
        output::print_line(Icon::Info, "No new variables were added.");
        return Ok(());
    }

    let mut sample_changed = false;
    let mut rules_changed = false;
    for var in approved {
        sample_changed |= sample.insert_missing(&var);
        if !rules.vars.contains_key(&var) {
            rules.vars.insert(var.clone(), discovered_rule());
            rules_changed = true;
        }
    }

    if sample_changed {
        sample.save_with_header(&sample_path, Some(BASE_TEMPLATE_HEADER))?;
        if show_write_paths {
            output::print_line(Icon::Success, "Updated `.env.base`.");
        }
    }

    if rules_changed {
        rules.save(&rules_path)?;
        if show_write_paths {
            output::print_line(Icon::Success, "Updated `.vlt/env.rules`.");
        }
    }

    Ok(())
}

fn choose_vars_to_add(missing: &[String], apply: bool) -> Result<Vec<String>> {
    if apply {
        return Ok(missing.to_vec());
    }

    if !io::stdin().is_terminal() || !io::stdout().is_terminal() {
        bail!(
            "interactive scan requires a terminal; rerun with --apply to add all discovered variables"
        );
    }

    let mut approved = Vec::new();
    let mut add_all = false;
    let theme = ColorfulTheme::default();

    for var in missing {
        if add_all {
            approved.push(var.clone());
            continue;
        }

        let options = ["Yes", "No", "Add all remaining"];
        let selection = Select::with_theme(&theme)
            .with_prompt(format!(
                "{} Add or sync {} to .env.base?",
                output::paint_icon(Icon::Info),
                &var
            ))
            .items(&options)
            .default(0)
            .interact()
            .context("failed to read scan choice")?;

        match selection {
            0 => approved.push(var.clone()),
            1 => {}
            2 => {
                approved.push(var.clone());
                add_all = true;
            }
            _ => unreachable!("select returned an unexpected option index"),
        }
    }

    Ok(approved)
}

fn discovered_rule() -> VarRule {
    VarRule {
        rule_type: RuleType::String,
        required: false,
        default: None,
        description: Some("".to_owned()),
        min: None,
        max: None,
        values: None,
    }
}
