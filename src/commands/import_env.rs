use std::collections::BTreeSet;
use std::path::Path;

use anyhow::{Context, Result, bail};

use crate::commands::scan::BASE_TEMPLATE_HEADER;
use crate::models::env_file::EnvFile;
use crate::models::rules::{VarRule, VltRules};
use crate::utils::output::{self, Icon};
use crate::utils::project;

pub fn run(env_name: &str, input: &Path) -> Result<()> {
    let root = std::env::current_dir().context("failed to read current directory")?;
    project::ensure_initialized(&root)?;
    project::validate_env_name(env_name)?;

    if !input.exists() {
        bail!("Import file was not found: {}", input.display());
    }

    let imported = EnvFile::load(input)?;
    if imported.values.is_empty() {
        output::print_line(Icon::Info, "No variables found in the import file.");
        return Ok(());
    }

    let target_path = project::env_file_path(&root, env_name);
    let mut target = if target_path.exists() {
        EnvFile::load(&target_path)?
    } else {
        project::scaffold_env_file(&root)?
    };

    let mut changed = 0usize;
    let mut imported_keys = BTreeSet::new();
    for (key, value) in &imported.values {
        imported_keys.insert(key.clone());
        if target.values.get(key) != Some(value) {
            target.values.insert(key.clone(), value.clone());
            changed += 1;
        }
    }
    target.save(&target_path)?;

    let base_path = root.join(".env.base");
    let rules_path = root.join(".vlt/env.rules");
    let mut base = EnvFile::load_or_default(&base_path)?;
    let mut rules = VltRules::load_or_default(&rules_path)?;
    let mut base_changed = false;
    let mut rules_changed = false;

    for key in &imported_keys {
        base_changed |= base.insert_missing(key);
        if !rules.vars.contains_key(key) {
            rules.vars.insert(key.clone(), VarRule::discovered());
            rules_changed = true;
        }
    }

    if base_changed {
        base.save_with_header(&base_path, Some(BASE_TEMPLATE_HEADER))?;
    }

    if rules_changed {
        rules.save(&rules_path)?;
    }

    output::print_line(Icon::Success, format!("Imported environment: {env_name}"));
    output::print_line(Icon::Info, format!("Imported {changed} value(s) from {}", input.display()));
    if base_changed || rules_changed {
        output::print_line(Icon::Info, "Updated project templates for newly discovered keys.");
    }
    Ok(())
}
