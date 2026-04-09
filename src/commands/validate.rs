use anyhow::{Context, Result, bail};

use crate::models::env_file::EnvFile;
use crate::models::rules::{RuleType, VarRule, VltRules};
use crate::utils::output::{self, Icon};
use crate::utils::project;

pub fn run() -> Result<()> {
    let root = std::env::current_dir().context("failed to read current directory")?;
    project::ensure_initialized(&root)?;

    let env_path = root.join(".env");
    if !env_path.exists() {
        bail!("`.env` was not found. Run `vlt use <env>` first.");
    }

    let rules = VltRules::load(&root.join(".vlt/env.rules"))?;
    let env = EnvFile::load(&env_path)?;
    let mut issues = Vec::new();

    for (key, rule) in &rules.vars {
        let value = env.values.get(key).map(|value| value.trim()).filter(|value| !value.is_empty());

        if rule.required && value.is_none() {
            issues.push(format!("{key}: required value is missing"));
            continue;
        }

        let Some(value) = value else {
            continue;
        };

        if let Some(issue) = validate_value(key, value, rule) {
            issues.push(issue);
        }
    }

    let unknown = rules.unknown_keys(&env.values);
    if !unknown.is_empty() {
        output::print_line(
            Icon::Warning,
            format!("Unknown keys in `.env`: {}", unknown.join(", ")),
        );
    }

    if issues.is_empty() {
        output::print_line(Icon::Success, "Validation passed.");
        return Ok(());
    }

    for issue in issues {
        output::print_line(Icon::Error, issue);
    }

    bail!("Validation failed.");
}

fn validate_value(key: &str, value: &str, rule: &VarRule) -> Option<String> {
    match rule.rule_type {
        RuleType::String | RuleType::Secret => None,
        RuleType::Bool => {
            if value == "true" || value == "false" {
                None
            } else {
                Some(format!("{key}: expected `true` or `false`"))
            }
        }
        RuleType::Int => match value.parse::<i64>() {
            Ok(parsed) => validate_numeric_bounds(key, parsed as f64, rule),
            Err(_) => Some(format!("{key}: expected an integer")),
        },
        RuleType::Float => match value.parse::<f64>() {
            Ok(parsed) => validate_numeric_bounds(key, parsed, rule),
            Err(_) => Some(format!("{key}: expected a float")),
        },
        RuleType::Enum => {
            let Some(allowed) = rule.values.as_ref() else {
                return Some(format!("{key}: enum rule is missing allowed values"));
            };
            if allowed.iter().any(|candidate| candidate == value) {
                None
            } else {
                Some(format!("{key}: expected one of {}", allowed.join(", ")))
            }
        }
    }
}

fn validate_numeric_bounds(key: &str, value: f64, rule: &VarRule) -> Option<String> {
    if let Some(min) = rule.min
        && value < min
    {
        return Some(format!("{key}: value must be >= {min}"));
    }

    if let Some(max) = rule.max
        && value > max
    {
        return Some(format!("{key}: value must be <= {max}"));
    }

    None
}

#[cfg(test)]
mod tests {
    use super::{validate_numeric_bounds, validate_value};
    use crate::models::rules::{RuleType, VarRule};

    #[test]
    fn rejects_invalid_bool() {
        let rule = VarRule { rule_type: RuleType::Bool, ..VarRule::default() };
        assert!(validate_value("DEBUG", "yes", &rule).is_some());
    }

    #[test]
    fn rejects_out_of_range_int() {
        let rule = VarRule {
            rule_type: RuleType::Int,
            min: Some(10.0),
            max: Some(20.0),
            ..VarRule::default()
        };
        assert_eq!(
            validate_numeric_bounds("PORT", 5.0, &rule),
            Some("PORT: value must be >= 10".to_owned())
        );
    }
}
