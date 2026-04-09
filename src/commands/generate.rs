use std::collections::BTreeSet;

use anyhow::{Context, Result};

use crate::models::env_file::EnvFile;
use crate::models::rules::{RuleType, VltRules};
use crate::utils::output::{self, Icon};
use crate::utils::project;

pub fn run() -> Result<()> {
    let root = std::env::current_dir().context("failed to read current directory")?;
    project::ensure_initialized(&root)?;

    let rules = VltRules::load(&root.join(".vlt/env.rules"))?;
    let mut keys = rules.vars.keys().cloned().collect::<BTreeSet<_>>();

    for env_name in project::available_envs(&root)? {
        let env = EnvFile::load(&project::env_file_path(&root, &env_name))?;
        keys.extend(env.values.keys().cloned());
    }

    let mut lines = Vec::new();
    for key in keys {
        let comment = match rules.vars.get(&key) {
            Some(rule) => format_rule_comment(rule),
            None => "type=string, required=no, note=missing from env.rules".to_owned(),
        };
        lines.push(format!("{key}= # {comment}"));
    }

    let mut contents = String::from("");
    if !lines.is_empty() {
        contents.push_str(&lines.join("\n"));
        contents.push('\n');
    }

    std::fs::write(root.join(".env.example"), contents).context("failed to write .env.example")?;
    output::print_line(Icon::Success, "Generated `.env.example`.");
    Ok(())
}

fn format_rule_comment(rule: &crate::models::rules::VarRule) -> String {
    let mut parts = vec![
        format!("type={}", rule_type_label(&rule.rule_type)),
        format!("required={}", if rule.required { "yes" } else { "no" }),
    ];

    if let Some(description) = rule.description.as_ref().filter(|value| !value.trim().is_empty()) {
        parts.push(format!("description={description}"));
    }

    if matches!(rule.rule_type, RuleType::Int | RuleType::Float) {
        if let Some(min) = rule.min {
            parts.push(format!("min={min}"));
        }
        if let Some(max) = rule.max {
            parts.push(format!("max={max}"));
        }
    }

    if matches!(rule.rule_type, RuleType::Enum)
        && let Some(values) = rule.values.as_ref()
        && !values.is_empty()
    {
        parts.push(format!("values={}", values.join("|")));
    }

    parts.join(", ")
}

fn rule_type_label(rule_type: &RuleType) -> &'static str {
    match rule_type {
        RuleType::String => "string",
        RuleType::Int => "int",
        RuleType::Float => "float",
        RuleType::Bool => "bool",
        RuleType::Enum => "enum",
        RuleType::Secret => "secret",
    }
}
