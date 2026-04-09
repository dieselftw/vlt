use std::path::Path;

use anyhow::{Result, bail};

pub fn ensure_initialized(root: &Path) -> Result<()> {
    if !root.join(".vlt").exists() {
        bail!("Project not initialized. Run `vlt init` first.");
    }

    if !root.join(".vlt/config.toml").exists() || !root.join(".vlt/env.rules").exists() {
        bail!("vlt project is incomplete. Run `vlt init` again.");
    }

    Ok(())
}
