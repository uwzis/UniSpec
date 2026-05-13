// src/commands/init.rs
use crate::fs::ensure_dir;
use anyhow::Result;
use include_dir::{include_dir, Dir};
use std::fs;
use std::path::Path;

/// `.agent/modes/default/` is embedded into the binary at compile time so
/// `unispec init` works on a fresh `cargo install` with no system data files.
static EMBEDDED_DEFAULT_MODE: Dir<'_> =
    include_dir!("$CARGO_MANIFEST_DIR/.agent/modes/default");

fn copy_dir_recursive(src: &Path, dst: &Path) -> Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }

    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        let new_path = dst.join(entry.file_name());

        if path.is_dir() {
            copy_dir_recursive(&path, &new_path)?;
        } else {
            fs::copy(&path, &new_path)?;
        }
    }
    Ok(())
}

/// Populate `<dst>` from the embedded default mode if `<dst>` is missing or empty.
fn extract_embedded_default_mode(dst: &Path) -> Result<()> {
    ensure_dir(dst)?;
    EMBEDDED_DEFAULT_MODE.extract(dst)?;
    Ok(())
}

/// Source the default mode at `dst` from the first available source:
/// 1. `~/.config/unispec/.agent/modes/default/`
/// 2. `/usr/share/unispec/.agent/modes/default/`
/// 3. The embedded copy compiled into the binary.
///
/// No-op if `dst` already contains files.
fn install_default_mode(dst: &Path) -> Result<&'static str> {
    let already_populated = dst.exists()
        && fs::read_dir(dst)
            .map(|mut it| it.next().is_some())
            .unwrap_or(false);
    if already_populated {
        return Ok("existing");
    }

    let global = crate::fs::global_modes_dir().join("default");
    let system = crate::fs::system_modes_dir().join("default");

    if global.exists() {
        copy_dir_recursive(&global, dst)?;
        return Ok("global");
    }
    if system.exists() {
        copy_dir_recursive(&system, dst)?;
        return Ok("system");
    }

    extract_embedded_default_mode(dst)?;
    Ok("embedded")
}

pub fn run_init(root: Option<&std::path::Path>) -> Result<()> {
    let root = root.unwrap_or_else(|| std::path::Path::new("."));
    let spec_root = root.join("spec");

    // Default mode pipeline.
    let default_areas = ["Staging", "Working", "Testing", "Fixing", "Build"];

    // Ensure each area directory exists. area.md is filled below from the
    // installed default mode (preferred) or a generic fallback.
    for area in &default_areas {
        ensure_dir(&spec_root.join(area))?;
    }

    // Create .agent/ + config.toml.
    let agent_root = root.join(".agent");
    ensure_dir(&agent_root)?;
    ensure_dir(&agent_root.join("modes"))?;

    let config_file = agent_root.join("config.toml");
    if !config_file.exists() {
        fs::write(&config_file, CONFIG_TEMPLATE)?;
    }

    // Install the default mode into .agent/modes/default/.
    let default_mode = agent_root.join("modes").join("default");
    let source = install_default_mode(&default_mode)?;

    // Populate spec/<Area>/area.md from the mode's per-area templates, falling
    // back to a generic stub if the template is missing.
    for area in &default_areas {
        let area_md = spec_root.join(area).join("area.md");
        if area_md.exists() {
            continue;
        }
        let template = default_mode
            .join("areas")
            .join(area.to_lowercase())
            .join("area.md");
        if template.exists() {
            fs::copy(&template, &area_md)?;
        } else {
            let content = DEFAULT_AREA_STUB.replace("{area}", area);
            fs::write(&area_md, content)?;
        }
    }

    // Copy every workflow shipped with the mode into .agent/workflows/ for
    // active use. We glob the directory so future workflow additions land here
    // automatically.
    let workflows_dst = agent_root.join("workflows");
    ensure_dir(&workflows_dst)?;
    let workflows_src = default_mode.join("workflows");
    if workflows_src.exists() {
        for entry in fs::read_dir(&workflows_src)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_file() {
                continue;
            }
            let dst = workflows_dst.join(entry.file_name());
            if !dst.exists() {
                fs::copy(&path, &dst)?;
            }
        }
    }

    // Copy skill.md to .agent/ for active use.
    let skill_src = default_mode.join("skill.md");
    let skill_dst = agent_root.join("skill.md");
    if skill_src.exists() && !skill_dst.exists() {
        fs::copy(&skill_src, &skill_dst)?;
    }

    // Modes README.
    let modes_readme = agent_root.join("modes").join("README.md");
    if !modes_readme.exists() {
        fs::write(&modes_readme, MODES_README)?;
    }

    // Constitution — non-negotiable project principles every agent must
    // respect. Only written when missing so re-running init doesn't clobber
    // amendments. Date placeholders are filled at write time.
    let constitution_path = agent_root.join("constitution.md");
    if !constitution_path.exists() {
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        let body = CONSTITUTION_TEMPLATE.replace("<DATE>", &today);
        fs::write(&constitution_path, body)?;
    }

    println!(
        "
    ██╗   ██╗███╗   ██╗██╗    ███████╗██████╗ ███████╗ ██████╗
    ██║   ██║████╗  ██║██║    ██╔════╝██╔══██╗██╔════╝██╔════╝
    ██║   ██║██╔██╗ ██║██║    ███████╗██████╔╝█████╗  ██║
    ██║   ██║██║╚██╗██║██║    ╚════██║██╔═══╝ ██╔══╝  ██║
    ╚██████╔╝██║ ╚████║██║    ███████║██║     ███████╗╚██████╗
     ╚═════╝ ╚═╝  ╚═══╝╚═╝    ╚══════╝╚═╝     ╚══════╝ ╚═════╝
         "
    );
    println!("UniSpec initialized with default mode (source: {})", source);
    println!("Areas: Staging → Working → Testing → Fixing → Build");
    println!();
    println!("Agent commands available:");
    println!("  unispec --help        Available commands; see docs/ for more info");
    println!();
    println!("See .agent/modes/README.md for creating custom modes.");
    Ok(())
}

const CONFIG_TEMPLATE: &str = r#"# UniSpec Agent Configuration

# Currently active mode (matches a directory under .agent/modes/).
current_mode = "default"

# Default area used by tools that omit `area`.
area = "Staging"

# Areas protected from destructive operations.
protected_areas = ["Build"]

# Show the platypus mascot in the TUI.
paddy_enabled = false

# Ingest configuration - how `unispec ingest run` parses and stores code analysis.
[ingest]
auto_index = false
index_on_complete = false
capture_functions = true
capture_structs = true
capture_enums = true
capture_imports = true
output_format = "toml"
languages = []

# Connectors - shell commands exposed as MCP tools (unispec_<name>).
# Example:
# [[connector]]
# name = "test"
# description = "Run the test suite"
# command = "pytest"
# args = ["tests/", "-v"]
"#;

const DEFAULT_AREA_STUB: &str = r#"---
area: {area}
short: {area} area
---

# {area}

## Purpose

This is the {area} area. Replace this stub with a description of how topics behave here.

## Guidelines

- Document what kinds of work belong in this area.
- Document what does not.
"#;

const CONSTITUTION_TEMPLATE: &str = r#"# Project Constitution

This file defines the non-negotiable principles for this project. Every agent action must respect these principles. Violations block progression through the pipeline.

## Core Principles

### Principle 1: Spec Before Code
No code may be written for a topic until it has a spec file in Working area.

### Principle 2: Tests Required
No topic may be pushed to Build without passing tests documented in the Testing area.

### Principle 3: No Silent Failures
All errors must be surfaced explicitly. No swallowing exceptions without logging.

### Principle 4: Backward Compatibility
No breaking changes to existing APIs without a MODIFIED Requirements delta in a change file.

### Principle 5: One Topic One Concern
Each topic must have a single clear responsibility. If a spec spans more than one concern, split it into multiple topics.

## Governance

Version: 1.0.0
Ratified: <DATE>
Last Amended: <DATE>
"#;

const MODES_README: &str = r#"# UniSpec Modes

A mode is a complete workflow configuration: areas, templates, workflows, skill prompt, and any per-area overrides. The default mode ships in `.agent/modes/default/` and uses the five-area pipeline:

    Staging → Working → Testing → Fixing → Build

## Creating a mode

1. Create directory: `.agent/modes/<mode_name>/`
2. Add `mode.toml` with metadata (see `default/mode.toml` for a working example)
3. Add `skill.md` with agent persona
4. Add `workflows/*.md` files
5. Add per-area templates under `areas/<area>/area.md` (use lowercase names)
6. Optionally add `templates/{topic,spec,task,area}.md` as global fallbacks

## Activating a mode

Modes are activated through the CLI (not via MCP):

    unispec mode list
    unispec mode activate <mode-name>
    unispec mode current

## Search order

Modes are searched in this order; first match wins:

1. Local: `./.agent/modes/`
2. Global: `~/.config/unispec/.agent/modes/`
3. System: `/usr/share/unispec/.agent/modes/`
4. Embedded: the `default` mode compiled into the binary (used by `unispec init` as a final fallback)
"#;
