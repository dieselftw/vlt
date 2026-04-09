use std::collections::BTreeSet;
use std::path::Path;

use anyhow::Result;
use regex::Regex;
use walkdir::WalkDir;

pub struct ScanResult {
    pub vars: BTreeSet<String>,
}

pub fn scan_project(root: &Path) -> Result<ScanResult> {
    let patterns = compiled_patterns()?;
    let mut vars = BTreeSet::new();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(|entry| !should_skip(entry.path()))
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
    {
        let Ok(contents) = std::fs::read_to_string(entry.path()) else {
            continue;
        };

        for pattern in &patterns {
            for captures in pattern.captures_iter(&contents) {
                if let Some(name) = captures.get(1) {
                    vars.insert(name.as_str().to_owned());
                }
            }
        }
    }

    Ok(ScanResult { vars })
}

fn compiled_patterns() -> Result<Vec<Regex>> {
    Ok(vec![
        Regex::new(r#"process\.env\.([A-Z][A-Z0-9_]*)"#)?,
        Regex::new(r#"os\.environ\[\s*["']([A-Z][A-Z0-9_]*)["']\s*\]"#)?,
        Regex::new(r#"os\.getenv\(\s*["']([A-Z][A-Z0-9_]*)["']"#)?,
        Regex::new(r#"std::env::var\(\s*["']([A-Z][A-Z0-9_]*)["']"#)?,
        Regex::new(r#"os\.Getenv\(\s*["']([A-Z][A-Z0-9_]*)["']"#)?,
        Regex::new(r#"(?:env|getenv|environment)[^"\n']*["']([A-Z][A-Z0-9_]*)["']"#)?,
    ])
}

fn should_skip(path: &Path) -> bool {
    path.components().any(|component| {
        matches!(component.as_os_str().to_str(), Some(".git" | "target" | "node_modules" | ".vlt"))
    })
}
