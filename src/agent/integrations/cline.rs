// src/agent/integrations/cline.rs
use super::IntegrationAdapter;

pub struct ClineAdapter;

impl IntegrationAdapter for ClineAdapter {
    fn name(&self) -> &str {
        "Cline"
    }
    fn cli_flag(&self) -> &str {
        "cline"
    }
    fn output_dir(&self) -> &str {
        ".clinerules"
    }
    fn output_filename(&self, workflow_name: &str) -> String {
        let stem = workflow_name
            .strip_prefix("unispec:")
            .or_else(|| workflow_name.strip_prefix("unispec_"))
            .unwrap_or(workflow_name);
        format!("unispec-{}.md", stem)
    }
    fn format_skill(&self, workflow_name: &str, content: &str) -> String {
        self.format_command(workflow_name, content)
    }
    fn format_command(&self, workflow_name: &str, content: &str) -> String {
        let stem = workflow_name
            .strip_prefix("unispec:")
            .or_else(|| workflow_name.strip_prefix("unispec_"))
            .unwrap_or(workflow_name);
        // Cline rule files use plain markdown — no frontmatter.
        format!("# UniSpec rule: {}\n\n{}", stem, content)
    }
}
