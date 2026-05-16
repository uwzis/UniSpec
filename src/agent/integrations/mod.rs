// src/agent/integrations/mod.rs
//
// Per-AI-tool format adapters. Each adapter emits the project's workflow
// content into the on-disk shape the target tool expects (paths, filenames,
// frontmatter). The generic adapter is always written so any agent that
// honours `AGENTS.md` works out of the box.
//
// Add a new adapter by implementing `IntegrationAdapter` and registering it
// in `all_adapters()`.

use anyhow::Result;
use std::path::Path;

pub mod claude;
pub mod cline;
pub mod cursor;
pub mod generic;
pub mod windsurf;

pub trait IntegrationAdapter {
    fn name(&self) -> &str;
    fn cli_flag(&self) -> &str;
    fn output_dir(&self) -> &str;
    fn output_filename(&self, workflow_name: &str) -> String;
    fn format_skill(&self, workflow_name: &str, content: &str) -> String;
    fn format_command(&self, workflow_name: &str, content: &str) -> String;
}

/// Workflow files shipped on disk under `.agent/workflows/` after init.
pub fn workflow_names() -> Vec<&'static str> {
    vec!["build", "test", "verify", "ingest", "unispec:spec"]
}

/// Read a workflow's body from the live `.agent/workflows/<name>.md` file.
/// Returns an empty string if the file is missing — callers still write the
/// adapter output so the caller knows where it would have landed.
fn read_workflow(project_root: &Path, name: &str) -> String {
    let path = project_root.join(".agent").join("workflows").join(format!("{}.md", name));
    std::fs::read_to_string(&path).unwrap_or_default()
}

pub fn all_adapters() -> Vec<Box<dyn IntegrationAdapter>> {
    vec![
        Box::new(claude::ClaudeCodeAdapter),
        Box::new(cursor::CursorAdapter),
        Box::new(windsurf::WindsurfAdapter),
        Box::new(cline::ClineAdapter),
        Box::new(generic::GenericAdapter),
    ]
}

pub fn find_adapter(cli_flag: &str) -> Option<Box<dyn IntegrationAdapter>> {
    all_adapters()
        .into_iter()
        .find(|a| a.cli_flag() == cli_flag)
}

/// Write every workflow file for a single adapter under the project root.
/// The adapter chooses whether to format as "skill" or "command" — the
/// default here uses `format_command`. For adapters that only support one,
/// the methods can return identical content.
pub fn write_adapter_for_project(
    adapter: &dyn IntegrationAdapter,
    project_root: &Path,
) -> Result<Vec<std::path::PathBuf>> {
    let out_dir = project_root.join(adapter.output_dir());
    std::fs::create_dir_all(&out_dir).ok();

    let mut written = vec![];
    for w in workflow_names() {
        let body = read_workflow(project_root, w);
        let formatted = adapter.format_command(w, &body);
        let filename = adapter.output_filename(w);
        let dest = out_dir.join(&filename);
        // Don't clobber an existing file — agents may have hand-edited.
        if !dest.exists() {
            std::fs::write(&dest, formatted)?;
            written.push(dest);
        }
    }
    Ok(written)
}

/// Generic adapter writes ONE file (`AGENTS.md`) regardless of workflow
/// count. Caller dispatches it specially.
pub fn write_generic_adapter(project_root: &Path) -> Result<std::path::PathBuf> {
    let adapter = generic::GenericAdapter;
    let path = project_root.join("AGENTS.md");
    if path.exists() {
        return Ok(path);
    }
    let mut body = String::from("# AGENTS.md\n\nUniversal entry point for AI agents working on this project. Each section below corresponds to a workflow under `.agent/workflows/`.\n\n");
    for w in workflow_names() {
        let raw = read_workflow(project_root, w);
        body.push_str(&adapter.format_command(w, &raw));
        body.push_str("\n\n---\n\n");
    }
    std::fs::write(&path, body)?;
    Ok(path)
}
