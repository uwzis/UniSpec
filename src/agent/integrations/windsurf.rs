// src/agent/integrations/windsurf.rs
use super::IntegrationAdapter;

pub struct WindsurfAdapter;

impl IntegrationAdapter for WindsurfAdapter {
    fn name(&self) -> &str {
        "Windsurf"
    }
    fn cli_flag(&self) -> &str {
        "windsurf"
    }
    fn output_dir(&self) -> &str {
        ".windsurf/workflows"
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
        // Windsurf workflows are plain markdown with a single `# Title`.
        format!("# UniSpec workflow: {}\n\n{}", stem, content)
    }
}
