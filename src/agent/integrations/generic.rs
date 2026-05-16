// src/agent/integrations/generic.rs
//
// Universal fallback. Writes a single AGENTS.md at the project root with
// one section per workflow — readable by any AI tool that recognises
// `AGENTS.md` as a project entry point.
use super::IntegrationAdapter;

pub struct GenericAdapter;

impl IntegrationAdapter for GenericAdapter {
    fn name(&self) -> &str {
        "Generic (AGENTS.md)"
    }
    fn cli_flag(&self) -> &str {
        "generic"
    }
    fn output_dir(&self) -> &str {
        "."
    }
    fn output_filename(&self, _workflow_name: &str) -> String {
        "AGENTS.md".to_string()
    }
    fn format_skill(&self, workflow_name: &str, content: &str) -> String {
        self.format_command(workflow_name, content)
    }
    fn format_command(&self, workflow_name: &str, content: &str) -> String {
        let stem = workflow_name
            .strip_prefix("unispec:")
            .or_else(|| workflow_name.strip_prefix("unispec_"))
            .unwrap_or(workflow_name);
        format!("## Workflow: {}\n\n{}", stem, content)
    }
}
