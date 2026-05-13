// src/commands/change.rs
//
// Change management for topics — inspired by OpenSpec.
//
// Instead of overwriting an existing spec when a new feature is added to a
// topic, a "change" is created in a child folder:
//
//   spec/<area>/<topic>/
//     topic.md
//     <topic>_spec.md         ← original, never modified
//     <topic>_task.md
//     changes/
//       <change-name>/
//         proposal.md         ← why this change exists
//         design.md           ← technical approach (optional)
//         <change-name>_spec.md
//         <change-name>_task.md
//       archive/
//         <archived-change>/  ← completed changes live here
//
// The original spec is the source of truth and is never touched by this
// module; only the `changes/` subtree is read or written.

use anyhow::Result;
use std::path::{Path, PathBuf};

pub struct ChangeAddOutput {
    pub topic: String,
    pub area: String,
    pub change: String,
    pub change_dir: PathBuf,
    pub proposal_path: PathBuf,
    pub design_path: Option<PathBuf>,
    pub spec_path: PathBuf,
    pub task_path: PathBuf,
}

pub struct ChangeInfo {
    pub name: String,
    pub status: String,
    pub has_proposal: bool,
    pub has_design: bool,
    pub has_spec: bool,
    pub has_task: bool,
}

/// Resolve `<spec_dir>/<area>/<topic>` trying upper- then lower-cased area.
/// Returns the existing directory or an error if neither exists.
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

fn change_name_safe(name: &str) -> String {
    name.replace('/', "-").replace(' ', "-")
}

pub fn run_change_add(
    topic: &str,
    area: Option<&str>,
    change: &str,
    proposal: &str,
    design: Option<&str>,
    spec_content: &str,
    task_content: &str,
) -> Result<ChangeAddOutput> {
    let area = area.unwrap_or("Staging");

    let change = change.trim();
    if change.is_empty() {
        return Err(anyhow::anyhow!("'change' parameter is required and must be non-empty"));
    }
    let change_safe = change_name_safe(change);

    let proposal = proposal.trim();
    if proposal.len() <= 10 {
        return Err(anyhow::anyhow!(
            "'proposal' must be at least 11 characters of actual text"
        ));
    }

    let spec_content = spec_content.trim();
    if spec_content.len() <= 10 {
        return Err(anyhow::anyhow!(
            "'spec_content' must be at least 11 characters of actual text"
        ));
    }

    let task_content = task_content.trim();
    if task_content.len() <= 10 {
        return Err(anyhow::anyhow!(
            "'task_content' must be at least 11 characters of actual text"
        ));
    }

    let topic_dir = resolve_topic_dir(area, topic)?;
    let changes_root = topic_dir.join("changes");
    let change_dir = changes_root.join(&change_safe);

    if change_dir.exists() {
        return Err(anyhow::anyhow!(
            "Change '{}' already exists for topic '{}' in area '{}'",
            change_safe,
            topic,
            area
        ));
    }

    std::fs::create_dir_all(&change_dir)?;

    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let date_only = now.split(' ').next().unwrap_or(&now).to_string();
    let author = crate::commands::topic::get_agent_id();

    let proposal_frontmatter = format!(
        "---\nchange: {}\ntopic: {}\nstatus: proposed\ncreated: {}\nauthor: {}\n---\n\n",
        change_safe, topic, now, author
    );
    let proposal_full = format!("{}{}", proposal_frontmatter, proposal);
    let proposal_path = change_dir.join("proposal.md");
    std::fs::write(&proposal_path, proposal_full)?;

    let design_path = if let Some(design) = design {
        let design = design.trim();
        if !design.is_empty() {
            let design_frontmatter = format!(
                "---\nchange: {}\ntopic: {}\ncreated: {}\nauthor: {}\n---\n\n",
                change_safe, topic, now, author
            );
            let design_full = format!("{}{}", design_frontmatter, design);
            let p = change_dir.join("design.md");
            std::fs::write(&p, design_full)?;
            Some(p)
        } else {
            None
        }
    } else {
        None
    };

    let spec_filename = format!("{}_spec.md", change_safe);
    let task_filename = format!("{}_task.md", change_safe);

    let spec_frontmatter = format!(
        "---\ntitle: {}\nchange: {}\ntopic: {}\ncreated: {}\nauthor: {}\nstatus: draft\n---\n\n",
        change_safe, change_safe, topic, now, author
    );
    let task_frontmatter = format!(
        "---\nchange: {}\ntopic: {}\nstatus: pending\ndate: {}\n---\n\n",
        change_safe, topic, date_only
    );

    let spec_full = format!("{}{}", spec_frontmatter, spec_content);
    let task_full = format!("{}{}", task_frontmatter, task_content);

    let spec_path = change_dir.join(&spec_filename);
    let task_path = change_dir.join(&task_filename);
    std::fs::write(&spec_path, spec_full)?;
    std::fs::write(&task_path, task_full)?;

    Ok(ChangeAddOutput {
        topic: topic.to_string(),
        area: area.to_string(),
        change: change_safe,
        change_dir,
        proposal_path,
        design_path,
        spec_path,
        task_path,
    })
}

fn inspect_change_dir(dir: &Path, archived: bool) -> Option<ChangeInfo> {
    if !dir.is_dir() {
        return None;
    }
    let name = dir.file_name()?.to_string_lossy().to_string();
    let change_safe = change_name_safe(&name);
    let has_proposal = dir.join("proposal.md").exists();
    let has_design = dir.join("design.md").exists();
    let spec_file = format!("{}_spec.md", change_safe);
    let task_file = format!("{}_task.md", change_safe);
    let has_spec = dir.join(&spec_file).exists();
    let has_task = dir.join(&task_file).exists();

    let status = if archived {
        "archived".to_string()
    } else if has_task {
        let task_path = dir.join(&task_file);
        let mut total = 0usize;
        let mut completed = 0usize;
        if let Ok(content) = std::fs::read_to_string(&task_path) {
            for line in content.lines() {
                let t = line.trim_start();
                if t.starts_with("- [") {
                    total += 1;
                    if t.starts_with("- [x]") || t.starts_with("- [X]") {
                        completed += 1;
                    }
                }
            }
        }
        if total == 0 {
            "proposed".to_string()
        } else if completed == total {
            "complete".to_string()
        } else if completed > 0 {
            "in-progress".to_string()
        } else {
            "proposed".to_string()
        }
    } else {
        "proposed".to_string()
    };

    Some(ChangeInfo {
        name,
        status,
        has_proposal,
        has_design,
        has_spec,
        has_task,
    })
}

pub fn run_change_list(
    topic: &str,
    area: Option<&str>,
    include_archived: bool,
) -> Result<Vec<ChangeInfo>> {
    let area = area.unwrap_or("Staging");
    let topic_dir = resolve_topic_dir(area, topic)?;
    let changes_root = topic_dir.join("changes");

    let mut out = vec![];
    if changes_root.exists() {
        for entry in std::fs::read_dir(&changes_root)? {
            let entry = entry?;
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let name = entry.file_name().to_string_lossy().to_string();
            if name == "archive" {
                continue;
            }
            if let Some(info) = inspect_change_dir(&path, false) {
                out.push(info);
            }
        }
    }
    out.sort_by(|a, b| a.name.cmp(&b.name));

    if include_archived {
        let archive_root = changes_root.join("archive");
        if archive_root.exists() {
            let mut archived = vec![];
            for entry in std::fs::read_dir(&archive_root)? {
                let entry = entry?;
                let path = entry.path();
                if !path.is_dir() {
                    continue;
                }
                if let Some(info) = inspect_change_dir(&path, true) {
                    archived.push(info);
                }
            }
            archived.sort_by(|a, b| a.name.cmp(&b.name));
            out.extend(archived);
        }
    }

    Ok(out)
}

pub struct ChangeArchiveOutput {
    pub topic: String,
    pub area: String,
    pub change: String,
    pub from: PathBuf,
    pub to: PathBuf,
}

pub fn run_change_archive(
    topic: &str,
    area: Option<&str>,
    change: &str,
) -> Result<ChangeArchiveOutput> {
    let area = area.unwrap_or("Staging");
    let change_safe = change_name_safe(change.trim());
    if change_safe.is_empty() {
        return Err(anyhow::anyhow!("'change' parameter is required"));
    }

    let topic_dir = resolve_topic_dir(area, topic)?;
    let changes_root = topic_dir.join("changes");
    let from = changes_root.join(&change_safe);
    if !from.exists() {
        return Err(anyhow::anyhow!(
            "Change '{}' does not exist for topic '{}' in area '{}'",
            change_safe,
            topic,
            area
        ));
    }

    let archive_root = changes_root.join("archive");
    std::fs::create_dir_all(&archive_root)?;
    let to = archive_root.join(&change_safe);
    if to.exists() {
        return Err(anyhow::anyhow!(
            "Archived change '{}' already exists; refusing to overwrite",
            change_safe
        ));
    }

    std::fs::rename(&from, &to)?;

    Ok(ChangeArchiveOutput {
        topic: topic.to_string(),
        area: area.to_string(),
        change: change_safe,
        from,
        to,
    })
}
