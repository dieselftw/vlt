use std::collections::HashSet;
use std::fs;
use std::path::Path;

use anyhow::{Context, Result};

const REQUIRED_PATTERNS: [&str; 5] = [".env", ".env.*", "!.env.example", "!.env.base", ".vlt"];

pub fn ensure_vlt_patterns(path: &Path) -> Result<bool> {
    let mut lines = if path.exists() {
        fs::read_to_string(path)
            .with_context(|| format!("failed to read {}", path.display()))?
            .lines()
            .map(str::to_owned)
            .collect::<Vec<_>>()
    } else {
        Vec::new()
    };

    let existing = lines.iter().map(|line| line.trim().to_owned()).collect::<HashSet<_>>();
    let mut changed = false;
    for pattern in REQUIRED_PATTERNS {
        if !existing.contains(pattern) {
            lines.push(pattern.to_owned());
            changed = true;
        }
    }

    if changed {
        let mut contents = lines.join("\n");
        if !contents.is_empty() {
            contents.push('\n');
        }
        fs::write(path, contents).with_context(|| format!("failed to write {}", path.display()))?;
    }

    Ok(changed)
}
