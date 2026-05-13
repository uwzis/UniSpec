pub mod server;

use serde_json::json;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Tool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

pub fn get_tools() -> Vec<Tool> {
    vec![
        // === Area Management ===
        Tool {
            name: "areas_list".to_string(),
            description: "List all areas".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {}
            }),
        },
        // === Topic Management ===
        Tool {
            name: "topics_list".to_string(),
            description: "List all topics in an area".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "area": { "type": "string", "description": "Area name (default: Staging)" }
                }
            }),
        },
        Tool {
            name: "topics_add".to_string(),
            description: "⚠️ CREATES topic.md FILE WITH FRONTMATTER. You MUST provide: topic, area, content, AND short (one-liner description). Usage: topics_add { topic: \"name\", area: \"Staging\", short: \"What this is in one line\", content: \"WRITE THE ACTUAL CONTENT HERE\" }".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name (e.g., 'myproject')" },
                    "area": { "type": "string", "description": "Area name (e.g., 'Staging')" },
                    "short": { "type": "string", "description": "⚠️ REQUIRED: One-line description (e.g., 'Web app for managing tasks')" },
                    "content": { "type": "string", "description": "⚠️ REQUIRED: The full topic content" }
                },
                "required": ["topic", "area", "short", "content"]
            }),
        },
        Tool {
            name: "topics_delete".to_string(),
            description: "Delete a topic from an area".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "area": { "type": "string", "description": "Area name (default: Staging)" },
                    "force": { "type": "boolean", "description": "Skip confirmation" }
                },
                "required": ["topic"]
            }),
        },
        Tool {
            name: "topics_show".to_string(),
            description: "Show details of a topic".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "area": { "type": "string", "description": "Area name (default: Staging)" }
                },
                "required": ["topic"]
            }),
        },
        Tool {
            name: "read_asset".to_string(),
            description: "Read a topic.md, spec, or task file. Use asset_type: 'topic', 'spec', or 'task'".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "asset_type": { "type": "string", "description": "Asset type: 'topic', 'spec', or 'task'" },
                    "area": { "type": "string", "description": "Area name (default: Staging)" }
                },
                "required": ["topic", "asset_type"]
            }),
        },
        Tool {
            name: "topics_push".to_string(),
            description: "Push/move a topic to another area".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "area": { "type": "string", "description": "Target area" },
                    "source_area": { "type": "string", "description": "Source area (auto-detected if not provided)" }
                },
                "required": ["topic", "area"]
            }),
        },
        Tool {
            name: "topics_pull".to_string(),
            description: "Pull a topic from another area into Working".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "source_area": { "type": "string", "description": "Source area to pull from" }
                },
                "required": ["topic", "source_area"]
            }),
        },
        Tool {
            name: "topics_progress".to_string(),
            description: "Show progress across topics in an area".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "area": { "type": "string", "description": "Area name (default: Staging)" }
                }
            }),
        },
        // === Read Specs ===
        Tool {
            name: "unispec_read_spec".to_string(),
            description: "Read spec.md and task.md for a topic".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "area": { "type": "string", "description": "Area name" }
                },
                "required": ["topic"]
            }),
        },
        // === Tasks ===
        Tool {
            name: "tasks_list".to_string(),
            description: "List all tasks for a topic with their status".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "area": { "type": "string", "description": "Area name (default: Staging)" }
                },
                "required": ["topic"]
            }),
        },
        Tool {
            name: "tasks_complete".to_string(),
            description: "Mark a task as complete".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "task_index": { "type": "integer", "description": "Task index (0-based)" },
                    "note": { "type": "string", "description": "Optional note" },
                    "area": { "type": "string", "description": "Area name (default: Staging)" }
                },
                "required": ["topic", "task_index"]
            }),
        },
        Tool {
            name: "tasks_incomplete".to_string(),
            description: "Mark a task as incomplete".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "task_index": { "type": "integer", "description": "Task index (0-based)" },
                    "note": { "type": "string", "description": "Optional note" },
                    "area": { "type": "string", "description": "Area name (default: Staging)" }
                },
                "required": ["topic", "task_index"]
            }),
        },
        // === Notes ===
        Tool {
            name: "notes_read".to_string(),
            description: "Read notes section from task.md".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "area": { "type": "string", "description": "Area name (default: Staging)" }
                },
                "required": ["topic"]
            }),
        },
        Tool {
            name: "notes_add".to_string(),
            description: "Add a note to the notes section of task.md".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "note": { "type": "string", "description": "Note content to add" },
                    "area": { "type": "string", "description": "Area name (default: Staging)" }
                },
                "required": ["topic", "note"]
            }),
        },
        // === Spec Add (creates spec and task files) ===
        Tool {
            name: "spec_add".to_string(),
            description: "⚠️ CREATES SPEC.MD AND TASK.MD WITH FRONTMATTER. You MUST provide: topic, area, short, spec_content, AND task_content. Usage: spec_add { topic: \"name\", area: \"Staging\", short: \"What this feature does in one line\", spec_content: \"WRITE THE SPEC CONTENT\", task_content: \"WRITE THE TASKS CONTENT\" }".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name (e.g., 'myproject/auth')" },
                    "area": { "type": "string", "description": "Area name (e.g., 'Staging')" },
                    "short": { "type": "string", "description": "⚠️ REQUIRED: One-line description of this spec (e.g., 'User authentication with JWT')" },
                    "spec_content": { "type": "string", "description": "⚠️ REQUIRED: The full spec content" },
                    "task_content": { "type": "string", "description": "⚠️ REQUIRED: The full tasks content" }
                },
                "required": ["topic", "area", "short", "spec_content", "task_content"]
            }),
        },
        // === Spec Writing ===
        Tool {
            name: "spec_write".to_string(),
            description: "Write spec.md content for a topic".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "area": { "type": "string", "description": "Area name (default: Staging)" },
                    "content": { "type": "string", "description": "Full spec.md content" }
                },
                "required": ["topic", "content"]
            }),
        },
        // === Task Writing (requires spec to already exist!) ===
        Tool {
            name: "task_write".to_string(),
            description: "Write task.md content for a topic. REQUIREMENT: Spec file must already exist for this topic!".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name (must have existing spec file)" },
                    "area": { "type": "string", "description": "Area name (default: Staging)" },
                    "content": { "type": "string", "description": "Full task.md content" }
                },
                "required": ["topic", "content"]
            }),
        },
        Tool {
            name: "task_status".to_string(),
            description: "Update task status (pending, working, complete)".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "area": { "type": "string", "description": "Area name (default: Staging)" },
                    "status": { "type": "string", "description": "Status: pending, working, or complete" }
                },
                "required": ["topic", "status"]
            }),
        },
        // === Task Queue (Master Task) ===
        Tool {
            name: "queue_list".to_string(),
            description: "List the task queue (ordered list of topics to work on)".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "area": { "type": "string", "description": "Area name (default: Staging)" }
                }
            }),
        },
        Tool {
            name: "queue_add".to_string(),
            description: "Add a topic to the task queue".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name to add" },
                    "position": { "type": "integer", "description": "Position in queue (0=first, -1=last)" },
                    "area": { "type": "string", "description": "Area name (default: Staging)" }
                },
                "required": ["topic"]
            }),
        },
        Tool {
            name: "queue_remove".to_string(),
            description: "Remove a topic from the task queue".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name to remove" },
                    "area": { "type": "string", "description": "Area name (default: Staging)" }
                },
                "required": ["topic"]
            }),
        },
        Tool {
            name: "queue_check".to_string(),
            description: "Check if a topic is ready to be pushed (in the readiness queue)".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name to check" },
                    "area": { "type": "string", "description": "Area name (default: Staging)" }
                },
                "required": ["topic"]
            }),
        },
        Tool {
            name: "queue_reorder".to_string(),
            description: "Reorder topics in the task queue".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic to move" },
                    "new_position": { "type": "integer", "description": "New position (0-based)" },
                    "area": { "type": "string", "description": "Area name (default: Staging)" }
                },
                "required": ["topic", "new_position"]
            }),
        },
        // === Constitution ===
        Tool {
            name: "constitution_read".to_string(),
            description: "Return the contents of `.agent/constitution.md` — the project's non-negotiable principles. Call this before any action that could violate governance.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {}
            }),
        },
        Tool {
            name: "constitution_check".to_string(),
            description: "Pair the constitution text with a proposed action so the agent can self-evaluate. Required: action (one-sentence description of what you're about to do).".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "action": { "type": "string", "description": "What the agent is about to do" }
                },
                "required": ["action"]
            }),
        },
        // === Next (structured agent feed) ===
        Tool {
            name: "next".to_string(),
            description: "Get a structured next-action payload for a topic. Call this BEFORE every action on a topic — it composes spec/task state, pending changes, queue gating, and area conventions into one decision-grade response with status, open_tasks, pending_changes, context_files, rules, next_action, and blockers. Required: topic. Optional: area (default Staging).".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "area": { "type": "string", "description": "Area name (default: Staging)" }
                },
                "required": ["topic"]
            }),
        },
        // === Change Management ===
        Tool {
            name: "change_add".to_string(),
            description: "Create a change folder inside a topic (does NOT modify the original spec). Writes proposal.md, optional design.md, <change>_spec.md, and <change>_task.md under spec/<area>/<topic>/changes/<change>/. Required: topic, change, proposal, spec_content, task_content. Optional: area (default Staging), design.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Existing topic name" },
                    "area": { "type": "string", "description": "Area name (default: Staging)" },
                    "change": { "type": "string", "description": "Change identifier, e.g. 'add-2fa'" },
                    "proposal": { "type": "string", "description": "Why this change is being added (≥ 11 chars)" },
                    "design": { "type": "string", "description": "Optional technical approach" },
                    "spec_content": { "type": "string", "description": "New requirements introduced by this change (≥ 11 chars)" },
                    "task_content": { "type": "string", "description": "New tasks introduced by this change (≥ 11 chars)" }
                },
                "required": ["topic", "change", "proposal", "spec_content", "task_content"]
            }),
        },
        Tool {
            name: "change_list".to_string(),
            description: "List all changes for a topic. Set include_archived=true to also list changes under changes/archive/.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "area": { "type": "string", "description": "Area name (default: Staging)" },
                    "include_archived": { "type": "boolean", "description": "Include archived changes (default: false)" }
                },
                "required": ["topic"]
            }),
        },
        Tool {
            name: "change_archive".to_string(),
            description: "Mark a change as complete by moving it into changes/archive/.".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "area": { "type": "string", "description": "Area name (default: Staging)" },
                    "change": { "type": "string", "description": "Change name to archive" }
                },
                "required": ["topic", "change"]
            }),
        },
        // === Index Actions (Bind) ===
        Tool {
            name: "index_add".to_string(),
            description: "Add a link between a topic and a file path".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" },
                    "path": { "type": "string", "description": "File path to link" },
                    "area": { "type": "string", "description": "Area (default: Staging)" },
                    "link_type": { "type": "string", "description": "Link type (implementation, test, config, docs)" },
                    "tags": { "type": "string", "description": "Comma-separated tags" },
                    "annotation": { "type": "string", "description": "Brief annotation" }
                },
                "required": ["topic", "path"]
            }),
        },
        Tool {
            name: "unispec_bind_spec".to_string(),
            description: "Bind a code file to a spec".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "spec_path": { "type": "string", "description": "Spec file path" },
                    "file_path": { "type": "string", "description": "Code file path to bind" },
                    "topic": { "type": "string", "description": "Topic name" },
                    "area": { "type": "string", "description": "Area name (default: Staging)" }
                },
                "required": ["spec_path", "file_path", "topic"]
            }),
        },
        // === Index Queries ===
        Tool {
            name: "index_find".to_string(),
            description: "Find links by topic, path, or tag".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "query": { "type": "string", "description": "Search query" },
                    "by": { "type": "string", "enum": ["topic", "path", "tag"], "description": "Search by (default: topic)" }
                },
                "required": ["query"]
            }),
        },
        Tool {
            name: "index_lookup".to_string(),
            description: "Find export by full ID (e.g., user-login:login_user)".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "id": { "type": "string", "description": "Export ID (topic:name)" }
                },
                "required": ["id"]
            }),
        },
        // === Index Listing ===
        Tool {
            name: "index_list".to_string(),
            description: "List all index links with optional filters".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Filter by topic" },
                    "path": { "type": "string", "description": "Filter by path" },
                    "tag": { "type": "string", "description": "Filter by tag" }
                }
            }),
        },
        Tool {
            name: "index_graph".to_string(),
            description: "Export index as graph JSON for visualization".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {}
            }),
        },
        Tool {
            name: "index_backlinks".to_string(),
            description: "Generate backlinks for a topic".to_string(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "topic": { "type": "string", "description": "Topic name" }
                },
                "required": ["topic"]
            }),
        },
    ]
}
