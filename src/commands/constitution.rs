// src/commands/constitution.rs
//
// Constitution helpers. The constitution lives at `.agent/constitution.md`
// and defines non-negotiable principles every agent action must respect.
// Both helpers are read-only.

use anyhow::Result;
use std::path::PathBuf;

pub fn constitution_path() -> PathBuf {
    crate::fs::agent_dir().join("constitution.md")
}

pub fn read_constitution() -> Result<String> {
    let path = constitution_path();
    if !path.exists() {
        return Err(anyhow::anyhow!(
            "Constitution file not found at {}. Run `unispec init` to create it.",
            path.display()
        ));
    }
    Ok(std::fs::read_to_string(&path)?)
}

pub struct CheckOutput {
    pub constitution: String,
    pub action: String,
    pub note: String,
}

/// Pairs the constitution text with the proposed action so the calling
/// agent can self-evaluate. Intentionally simple: real semantic enforcement
/// happens in the model, not in a Rust regex.
pub fn check_against_constitution(action: &str) -> Result<CheckOutput> {
    let constitution = read_constitution()?;
    Ok(CheckOutput {
        constitution,
        action: action.to_string(),
        note: "Read each principle and confirm the proposed action does not violate any. If a principle is violated, do not proceed.".to_string(),
    })
}
