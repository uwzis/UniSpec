// src/commands/next.rs
//
// `next` returns a structured "what should the agent do next" payload for a
// topic. It composes existing primitives (spec/task file reads, change
// listing, queue check) into a single decision-grade response so an MCP
// client can drive the workflow without re-parsing markdown.
//
// The data sources are all already on disk — this module does no writes.

use anyhow::Result;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Serialize)]
pub struct TaskItem {
    pub index: usize,
    pub text: String,
    pub completed: bool,
    /// `None` for tasks in the topic's main `<topic>_task.md`; `Some(<name>)`
    /// for tasks living in a change's `<change>_task.md`.
    pub from_change: Option<String>,
}

#[derive(Serialize)]
pub struct ChangeItem {
    pub name: String,
    pub status: String,
    pub has_spec: bool,
    pub has_task: bool,
}

#[derive(Serialize)]
pub struct NextOutput {
    pub topic: String,
    pub area: String,
    pub status: String,
    pub open_tasks: Vec<TaskItem>,
    pub completed_tasks: Vec<TaskItem>,
    pub pending_changes: Vec<ChangeItem>,
    pub archived_changes: Vec<ChangeItem>,
    pub context_files: Vec<String>,
    pub rules: Vec<String>,
    pub next_action: String,
    pub blockers: Vec<String>,
}

fn topic_safe(topic: &str) -> String {
    topic.replace('/', "-").replace(' ', "-")
}

/// Try the area as-given, then lowercased, returning the directory if either
/// exists (matches `change.rs::resolve_topic_dir`).
fn resolve_topic_dir(area: &str, topic: &str) -> Result<PathBuf> {
    let upper = crate::fs::spec_dir().join(area).join(topic);
    if upper.exists() {
        return Ok(upper);
    }
    let lower = crate::fs::spec_dir().join(area.to_lowercase()).join(topic);
    if lower.exists() {
        return Ok(lower);
    }
    Err(anyhow::anyhow!(
        "Topic '{}' does not exist in area '{}'",
        topic,
        area
    ))
}

/// Make a path relative to the project root for inclusion in `context_files`.
fn relativize(path: &std::path::Path) -> String {
    let root = crate::fs::project_root().unwrap_or_else(|_| PathBuf::from("."));
    match path.strip_prefix(&root) {
        Ok(rel) => rel.to_string_lossy().to_string(),
        Err(_) => path.to_string_lossy().to_string(),
    }
}

/// Parse `- [ ]` / `- [x]` / `- [X]` lines into (text, completed) pairs in
/// file order. Indentation-tolerant.
fn parse_task_lines(content: &str) -> Vec<(String, bool)> {
    let mut out = vec![];
    for line in content.lines() {
        let t = line.trim_start();
        if t.starts_with("- [") {
            let completed = t.starts_with("- [x]") || t.starts_with("- [X]");
            let text = t
                .splitn(2, ']')
                .nth(1)
                .map(|s| s.trim().to_string())
                .unwrap_or_default();
            out.push((text, completed));
        }
    }
    out
}

fn rules_for_area(area: &str) -> Vec<String> {
    match area.to_lowercase().as_str() {
        "staging" => vec![
            "Spec is being written. Do not write code yet.".to_string(),
            "Use spec_add to create the spec and task files if they do not exist.".to_string(),
            "Use change_add to add new features to an existing topic — never rerun spec_add on a topic that already has a spec.".to_string(),
            "Add the topic to spec/Staging/queue.md via queue_add before pushing to Working.".to_string(),
        ],
        "working" => vec![
            "Implementation phase. Write code under src/ and flip task checkboxes using tasks_complete.".to_string(),
            "The spec is frozen — do not modify <topic>_spec.md. If requirements need to change, pull the topic back to Staging.".to_string(),
            "Link every new source file to the topic via index_add with link_type \"implementation\".".to_string(),
            "If there are pending changes, implement their <change>_task.md alongside the main task list and change_archive them when complete.".to_string(),
        ],
        "testing" => vec![
            "Testing phase. Run tests and report results — do not edit code in this area.".to_string(),
            "If tests fail, push to Fixing. If tests pass, push to Build.".to_string(),
            "Record test outcomes via notes_add so the next stage can see what ran.".to_string(),
        ],
        "fixing" => vec![
            "Fix failing tests. Do not change the spec.".to_string(),
            "Add the topic to spec/Fixing/queue.md via queue_add before pushing back to Testing.".to_string(),
            "Scope: stay narrow to the failing behaviour — no speculative refactors.".to_string(),
        ],
        "build" => vec![
            "Topic is shipped. Do not modify.".to_string(),
            "To revise a Build topic, use topics_pull to bring it back to the default area first.".to_string(),
        ],
        _ => vec![format!(
            "Custom area '{}'. Follow the project's mode-specific conventions.",
            area
        )],
    }
}

pub fn run_next(topic: &str, area: Option<&str>) -> Result<NextOutput> {
    let area = area.unwrap_or("Staging").to_string();

    let topic_dir = resolve_topic_dir(&area, topic)?;
    let topic_md = topic_dir.join("topic.md");

    let safe = topic_safe(topic);
    let spec_filename = format!("{}_spec.md", safe);
    let task_filename = format!("{}_task.md", safe);
    let spec_path = topic_dir.join(&spec_filename);
    let task_path = topic_dir.join(&task_filename);

    // Main task file → open / completed task lists with global indices.
    let mut open_tasks: Vec<TaskItem> = vec![];
    let mut completed_tasks: Vec<TaskItem> = vec![];

    if task_path.exists() {
        let content = std::fs::read_to_string(&task_path)?;
        for (i, (text, completed)) in parse_task_lines(&content).into_iter().enumerate() {
            let item = TaskItem {
                index: i,
                text,
                completed,
                from_change: None,
            };
            if completed {
                completed_tasks.push(item);
            } else {
                open_tasks.push(item);
            }
        }
    }

    // Changes — non-archived and archived, via the existing list logic.
    let all_changes =
        crate::commands::change::run_change_list(topic, Some(&area), true).unwrap_or_default();
    let mut pending_changes: Vec<ChangeItem> = vec![];
    let mut archived_changes: Vec<ChangeItem> = vec![];

    for c in &all_changes {
        let item = ChangeItem {
            name: c.name.clone(),
            status: c.status.clone(),
            has_spec: c.has_spec,
            has_task: c.has_task,
        };
        if c.status == "archived" {
            archived_changes.push(item);
        } else {
            pending_changes.push(item);
        }
    }

    // Per-change task lines, indexed within each change's own task file.
    for c in &pending_changes {
        let change_safe = c.name.replace('/', "-").replace(' ', "-");
        let change_task = topic_dir
            .join("changes")
            .join(&change_safe)
            .join(format!("{}_task.md", change_safe));
        if !change_task.exists() {
            continue;
        }
        let content = match std::fs::read_to_string(&change_task) {
            Ok(s) => s,
            Err(_) => continue,
        };
        for (i, (text, completed)) in parse_task_lines(&content).into_iter().enumerate() {
            let item = TaskItem {
                index: i,
                text,
                completed,
                from_change: Some(c.name.clone()),
            };
            if completed {
                completed_tasks.push(item);
            } else {
                open_tasks.push(item);
            }
        }
    }

    // Context files (relative paths) — only include what exists.
    let mut context_files: Vec<String> = vec![];
    if topic_md.exists() {
        context_files.push(relativize(&topic_md));
    }
    if spec_path.exists() {
        context_files.push(relativize(&spec_path));
    }
    if task_path.exists() {
        context_files.push(relativize(&task_path));
    }
    for c in &pending_changes {
        let change_safe = c.name.replace('/', "-").replace(' ', "-");
        let cdir = topic_dir.join("changes").join(&change_safe);
        let proposal = cdir.join("proposal.md");
        let design = cdir.join("design.md");
        let cspec = cdir.join(format!("{}_spec.md", change_safe));
        let ctask = cdir.join(format!("{}_task.md", change_safe));
        if proposal.exists() {
            context_files.push(relativize(&proposal));
        }
        if design.exists() {
            context_files.push(relativize(&design));
        }
        if cspec.exists() {
            context_files.push(relativize(&cspec));
        }
        if ctask.exists() {
            context_files.push(relativize(&ctask));
        }
    }

    // Rules — area-specific, plus a generic line about pending changes when
    // any exist (it's actionable enough to be worth surfacing here).
    let mut rules = rules_for_area(&area);
    if !pending_changes.is_empty() {
        rules.push(format!(
            "{} pending change(s) attached to this topic — see pending_changes and run change_list for details.",
            pending_changes.len()
        ));
    }

    // Blockers.
    let mut blockers: Vec<String> = vec![];
    if !spec_path.exists() {
        blockers.push(format!(
            "No spec file at {}. Run spec_add to create <topic>_spec.md and <topic>_task.md.",
            relativize(&spec_path)
        ));
    }
    if crate::agent::mode::area_requires_readiness(&area) {
        let queue_file = crate::agent::mode::get_readiness_queue_file();
        let queue_path = crate::fs::spec_dir().join(&area).join(&queue_file);
        let in_queue = if queue_path.exists() {
            std::fs::read_to_string(&queue_path)
                .map(|content| {
                    content.lines().any(|line| {
                        let t = line.trim();
                        t.starts_with("- ") && t.contains(topic)
                    })
                })
                .unwrap_or(false)
        } else {
            false
        };
        if !in_queue {
            blockers.push(format!(
                "Topic '{}' is not listed in {}/{}. Add it via queue_add before pushing out of {}.",
                topic, area, queue_file, area
            ));
        }
    }

    // Status.
    let total_tasks = open_tasks.len() + completed_tasks.len();
    let status = if !blockers.is_empty() {
        "blocked".to_string()
    } else if total_tasks == 0 {
        // No spec covered above; here we have a spec but no task lines yet.
        "not-started".to_string()
    } else if open_tasks.is_empty() {
        "complete".to_string()
    } else if !completed_tasks.is_empty() {
        "in-progress".to_string()
    } else {
        "not-started".to_string()
    };

    // next_action — one sentence, prioritised by what's most urgent.
    let next_action = if !blockers.is_empty() {
        format!("Resolve blocker: {}", blockers[0])
    } else if !spec_path.exists() {
        format!(
            "Create the spec via spec_add {{ topic: \"{}\", area: \"{}\", short, spec_content, task_content }}.",
            topic, area
        )
    } else if area.eq_ignore_ascii_case("build") {
        "Topic is shipped. No further action — to revise, topics_pull it back to the default area first.".to_string()
    } else if status == "complete" {
        match area.to_lowercase().as_str() {
            "staging" => format!(
                "All tasks complete. Run queue_add {{ topic: \"{}\", area: \"Staging\" }} then topics_push to Working.",
                topic
            ),
            "working" => format!(
                "All tasks complete. topics_push {{ topic: \"{}\", area: \"Testing\", source_area: \"Working\" }} to advance.",
                topic
            ),
            "testing" => format!(
                "All tasks complete. If tests pass, topics_push to Build; if any failed, push to Fixing.",
            ),
            "fixing" => format!(
                "All tasks complete. Run queue_add {{ topic: \"{}\", area: \"Fixing\" }} then topics_push back to Testing.",
                topic
            ),
            _ => "All tasks complete. Push to the next area.".to_string(),
        }
    } else if let Some(t) = open_tasks.first() {
        match &t.from_change {
            Some(change) => format!(
                "Work on task {} of change '{}': {}.",
                t.index, change, t.text
            ),
            None => format!("Work on task {}: {}. Use tasks_complete {{ topic: \"{}\", task_index: {} }} when done.", t.index, t.text, topic, t.index),
        }
    } else {
        // No tasks at all but spec exists — agent should populate task_content
        // (this lands in "not-started" status).
        "Spec exists but no tasks are defined yet. Use task_write to add implementation tasks.".to_string()
    };

    Ok(NextOutput {
        topic: topic.to_string(),
        area,
        status,
        open_tasks,
        completed_tasks,
        pending_changes,
        archived_changes,
        context_files,
        rules,
        next_action,
        blockers,
    })
}
