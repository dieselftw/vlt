use std::collections::BTreeSet;

use anyhow::{Context, Result};

use crate::models::env_file::EnvFile;
use crate::utils::output::{self, Icon};
use crate::utils::project;

pub fn run(env1: &str, env2: &str) -> Result<()> {
    let root = std::env::current_dir().context("failed to read current directory")?;
    project::ensure_initialized(&root)?;

    if env1 == env2 {
        anyhow::bail!("Choose two different environments to diff.");
    }

    let env1_path = project::ensure_env_exists(&root, env1)?;
    let env2_path = project::ensure_env_exists(&root, env2)?;
    let left = EnvFile::load(&env1_path)?;
    let right = EnvFile::load(&env2_path)?;

    output::print_line(Icon::Info, format!("Diff: {env1} vs {env2}"));

    let keys = left.values.keys().chain(right.values.keys()).cloned().collect::<BTreeSet<_>>();

    if keys.is_empty() {
        output::print_line(Icon::Info, "Both environments are empty.");
        return Ok(());
    }

    let mut same = 0usize;
    let mut different = 0usize;
    let mut missing = 0usize;

    for key in keys {
        match (left.values.get(&key), right.values.get(&key)) {
            (Some(left_value), Some(right_value)) if left_value == right_value => {
                same += 1;
                output::print_line(Icon::Success, format!("{key}: present in both"));
            }
            (Some(_), Some(_)) => {
                different += 1;
                output::print_line(Icon::Warning, format!("{key}: values differ"));
            }
            (Some(_), None) => {
                missing += 1;
                output::print_line(Icon::Error, format!("{key}: missing in {env2}"));
            }
            (None, Some(_)) => {
                missing += 1;
                output::print_line(Icon::Error, format!("{key}: missing in {env1}"));
            }
            (None, None) => {}
        }
    }

    output::print_line(
        Icon::Info,
        format!("Summary: {same} same, {different} different, {missing} missing"),
    );
    Ok(())
}
