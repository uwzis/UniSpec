// src/agent/integrations/claude.rs
use super::IntegrationAdapter;

pub struct ClaudeCodeAdapter;

impl IntegrationAdapter for ClaudeCodeAdapter {
    fn name(&self) -> &str {
        "Claude Code"
    }
    fn cli_flag(&self) -> &str {
        "claude-code"
    }
    fn output_dir(&self) -> &str {
        ".claude/commands/unispec"
    }
    fn output_filename(&self, workflow_name: &str) -> String {
        let stem = workflow_name
            .strip_prefix("unispec:")
            .or_else(|| workflow_name.strip_prefix("unispec_"))
            .unwrap_or(workflow_name);
        format!("{}.md", stem)
    }
    fn format_skill(&self, workflow_name: &str, content: &str) -> String {
        self.format_command(workflow_name, content)
    }
    fn format_command(&self, workflow_name: &str, content: &str) -> String {
        let stem = workflow_name
            .strip_prefix("unispec:")
            .or_else(|| workflow_name.strip_prefix("unispec_"))
            .unwrap_or(workflow_name);
        let description = match stem {
            "spec" => "Create the topic, spec, and task files for a UniSpec feature.",
            "build" => "Implement a UniSpec topic — write code into src/, link files, flip task checkboxes, push to Testing.",
            "verify" => "Verify a UniSpec topic — confirm the implementation satisfies the spec.",
            "test" => "Run tests for a UniSpec topic and record results.",
            "ingest" => "Ingest a codebase into UniSpec.",
            _ => "UniSpec workflow.",
        };
        let frontmatter = format!(
            "---\nname: unispec-{}\ndescription: {}\ncategory: unispec\n---\n\n",
            stem, description
        );
        format!("{}# UniSpec workflow: {}\n\n{}", frontmatter, stem, content)
    }
}
