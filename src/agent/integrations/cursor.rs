// src/agent/integrations/cursor.rs
use super::IntegrationAdapter;

pub struct CursorAdapter;

impl IntegrationAdapter for CursorAdapter {
    fn name(&self) -> &str {
        "Cursor"
    }
    fn cli_flag(&self) -> &str {
        "cursor"
    }
    fn output_dir(&self) -> &str {
        ".cursor/rules"
    }
    fn output_filename(&self, workflow_name: &str) -> String {
        let stem = workflow_name
            .strip_prefix("unispec:")
            .or_else(|| workflow_name.strip_prefix("unispec_"))
            .unwrap_or(workflow_name);
        format!("unispec-{}.mdc", stem)
    }
    fn format_skill(&self, workflow_name: &str, content: &str) -> String {
        self.format_command(workflow_name, content)
    }
    fn format_command(&self, workflow_name: &str, content: &str) -> String {
        let stem = workflow_name
            .strip_prefix("unispec:")
            .or_else(|| workflow_name.strip_prefix("unispec_"))
            .unwrap_or(workflow_name);
        // Cursor `.mdc` rules use a YAML frontmatter with `description`,
        // `globs`, and `alwaysApply` fields.
        let description = format!("UniSpec {} workflow", stem);
        let frontmatter = format!(
            "---\ndescription: {}\nglobs: spec/**/*.md\nalwaysApply: false\n---\n\n",
            description
        );
        format!("{}# UniSpec workflow: {}\n\n{}", frontmatter, stem, content)
    }
}
