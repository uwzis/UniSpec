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

    // Delta merge: if the change spec contains delta sections (ADDED /
    // MODIFIED / REMOVED / RENAMED Requirements), apply them to the topic's
    // canonical <topic>_spec.md before moving the change to archive. If no
    // delta sections are present, skip the merge (backward compatible with
    // older changes that just stored a parallel spec).
    let change_spec_path = from.join(format!("{}_spec.md", change_safe));
    if change_spec_path.exists() {
        let topic_safe = topic.replace('/', "-").replace(' ', "-");
        let canonical_path = topic_dir.join(format!("{}_spec.md", topic_safe));
        if canonical_path.exists() {
            let change_spec = std::fs::read_to_string(&change_spec_path)?;
            if let Some(delta) = parse_delta_spec(&change_spec) {
                let canonical = std::fs::read_to_string(&canonical_path)?;
                let merged = apply_delta(&canonical, &delta);
                std::fs::write(&canonical_path, merged)?;
            }
        }
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

// ---------------------------------------------------------------------------
// Delta parser / merger
// ---------------------------------------------------------------------------

#[derive(Debug, Default)]
struct DeltaSpec {
    added: Vec<RequirementBlock>,
    modified: Vec<RequirementBlock>,
    /// Names of requirements to delete from the canonical spec.
    removed: Vec<String>,
    /// (old_name, new_name) pairs to rename in place.
    renamed: Vec<(String, String)>,
}

#[derive(Debug, Clone)]
struct RequirementBlock {
    name: String,
    /// Full block text starting with `### Requirement: <name>` line.
    body: String,
}

/// Returns `None` if the change spec contains no delta sections — caller
/// treats that as "skip merge".
fn parse_delta_spec(content: &str) -> Option<DeltaSpec> {
    // Skip frontmatter if present so `## ADDED Requirements` etc. don't get
    // confused with anything inside the YAML block.
    let body = strip_frontmatter(content);

    // Identify the four section names case-insensitively. If none are
    // present, this isn't a delta spec.
    let lines: Vec<&str> = body.lines().collect();
    let section_idxs: Vec<(usize, &str)> = lines
        .iter()
        .enumerate()
        .filter_map(|(i, l)| {
            let t = l.trim();
            // Match `## <NAME> Requirements` (case-insensitive on the name).
            if let Some(rest) = t.strip_prefix("## ") {
                let rest_lower = rest.to_lowercase();
                for tag in ["added", "modified", "removed", "renamed"] {
                    let prefix = format!("{} requirements", tag);
                    if rest_lower == prefix
                        || rest_lower.starts_with(&format!("{} ", prefix))
                    {
                        return Some((i, tag));
                    }
                }
            }
            None
        })
        .collect();

    if section_idxs.is_empty() {
        return None;
    }

    let mut delta = DeltaSpec::default();

    for (k, (start_idx, tag)) in section_idxs.iter().enumerate() {
        let end_idx = if k + 1 < section_idxs.len() {
            section_idxs[k + 1].0
        } else {
            lines.len()
        };
        // Section content is between [start_idx+1, end_idx).
        let section_lines = &lines[start_idx + 1..end_idx];

        match *tag {
            "added" | "modified" => {
                let blocks = parse_requirement_blocks(section_lines);
                if *tag == "added" {
                    delta.added.extend(blocks);
                } else {
                    delta.modified.extend(blocks);
                }
            }
            "removed" => {
                // Each `### Requirement: <name>` line names a deletion.
                for l in section_lines {
                    let t = l.trim();
                    if let Some(name) = t.strip_prefix("### Requirement:") {
                        delta.removed.push(name.trim().to_string());
                    } else if let Some(rest) = t.strip_prefix("- ") {
                        // Allow bullet-style `- ### Requirement: X` as well.
                        if let Some(name) = rest.trim().strip_prefix("### Requirement:") {
                            delta.removed.push(name.trim().to_string());
                        }
                    }
                }
            }
            "renamed" => {
                // Expect alternating `- FROM:` / `- TO:` lines (in any order).
                let mut current_from: Option<String> = None;
                for l in section_lines {
                    let t = l.trim().trim_start_matches('-').trim();
                    if let Some(rest) = t.strip_prefix("FROM:") {
                        if let Some(name) = rest.trim().strip_prefix("### Requirement:") {
                            current_from = Some(name.trim().to_string());
                        }
                    } else if let Some(rest) = t.strip_prefix("TO:") {
                        if let Some(name) = rest.trim().strip_prefix("### Requirement:") {
                            if let Some(from_name) = current_from.take() {
                                delta.renamed.push((from_name, name.trim().to_string()));
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Some(delta)
}

/// Split a chunk of section content into `### Requirement: <name>` blocks.
/// Each block runs from its header line until the next `### Requirement:`
/// header or the next `## ` top-level section.
fn parse_requirement_blocks(section_lines: &[&str]) -> Vec<RequirementBlock> {
    let mut out = vec![];
    let mut current: Option<RequirementBlock> = None;
    for l in section_lines {
        let t = l.trim_start();
        if let Some(name_part) = t.strip_prefix("### Requirement:") {
            if let Some(b) = current.take() {
                out.push(b);
            }
            current = Some(RequirementBlock {
                name: name_part.trim().to_string(),
                body: format!("{}\n", l),
            });
        } else if t.starts_with("## ") {
            // Another top-level section started — flush.
            if let Some(b) = current.take() {
                out.push(b);
            }
            break;
        } else if let Some(ref mut b) = current {
            b.body.push_str(l);
            b.body.push('\n');
        }
    }
    if let Some(b) = current {
        out.push(b);
    }
    // Trim trailing whitespace from each block body so reassembled specs
    // don't accumulate blank lines.
    for b in &mut out {
        while b.body.ends_with("\n\n") {
            b.body.pop();
        }
    }
    out
}

fn strip_frontmatter(content: &str) -> &str {
    if content.trim_start().starts_with("---") {
        if let Some(end) = content.find("\n---") {
            return &content[end + 5..];
        }
    }
    content
}

/// Returns the canonical spec with the delta applied. Operations are run in
/// the order RENAMED → REMOVED → MODIFIED → ADDED so name lookups remain
/// stable: renames land first, then deletions, then in-place edits, then
/// appends.
fn apply_delta(canonical: &str, delta: &DeltaSpec) -> String {
    // Split canonical into (frontmatter_prefix, body). We keep frontmatter
    // verbatim and only mutate the body.
    let (prefix, body) = split_frontmatter(canonical);

    let mut blocks = split_into_blocks(body);

    // RENAMED — change the name in the header line and the block's `name`.
    for (from, to) in &delta.renamed {
        if let Some(idx) = blocks.iter().position(|b| matches!(b, Block::Req(r) if &r.name == from)) {
            if let Block::Req(ref mut r) = blocks[idx] {
                let old_hdr = format!("### Requirement: {}", from);
                let new_hdr = format!("### Requirement: {}", to);
                r.body = r.body.replacen(&old_hdr, &new_hdr, 1);
                r.name = to.clone();
            }
        }
    }

    // REMOVED — drop matching blocks.
    blocks.retain(|b| match b {
        Block::Req(r) => !delta.removed.iter().any(|n| n == &r.name),
        _ => true,
    });

    // MODIFIED — replace matching blocks' body.
    for new_block in &delta.modified {
        if let Some(idx) = blocks
            .iter()
            .position(|b| matches!(b, Block::Req(r) if r.name == new_block.name))
        {
            if let Block::Req(ref mut r) = blocks[idx] {
                r.body = ensure_trailing_newline(&new_block.body);
            }
        }
    }

    // ADDED — append at the end (preceded by a blank line for readability).
    let has_existing_reqs = blocks.iter().any(|b| matches!(b, Block::Req(_)));
    for new_block in &delta.added {
        // Skip duplicates by name to keep merges idempotent.
        if blocks
            .iter()
            .any(|b| matches!(b, Block::Req(r) if r.name == new_block.name))
        {
            continue;
        }
        if has_existing_reqs || !blocks.is_empty() {
            blocks.push(Block::Text("\n".to_string()));
        }
        blocks.push(Block::Req(RequirementBlock {
            name: new_block.name.clone(),
            body: ensure_trailing_newline(&new_block.body),
        }));
    }

    let mut out = String::from(prefix);
    for b in &blocks {
        match b {
            Block::Text(s) => out.push_str(s),
            Block::Req(r) => out.push_str(&r.body),
        }
    }

    out
}

fn ensure_trailing_newline(s: &str) -> String {
    if s.ends_with('\n') {
        s.to_string()
    } else {
        format!("{}\n", s)
    }
}

fn split_frontmatter(content: &str) -> (String, &str) {
    if content.trim_start().starts_with("---") {
        if let Some(end) = content.find("\n---") {
            // Include the trailing `---` and the newline after it in the prefix.
            let prefix_end = end + 5; // past `\n---\n` (4) or `\n---` + EOF
            let prefix_end = prefix_end.min(content.len());
            let prefix = content[..prefix_end].to_string();
            let body = &content[prefix_end..];
            return (prefix, body);
        }
    }
    (String::new(), content)
}

enum Block {
    Text(String),
    Req(RequirementBlock),
}

/// Tokenize a spec body into a stream of `Text` chunks and `Requirement`
/// blocks. A requirement block starts with `### Requirement: <name>` and ends
/// at the next `### Requirement:` header, the next `## ` top-level section,
/// or EOF.
fn split_into_blocks(body: &str) -> Vec<Block> {
    let mut blocks: Vec<Block> = vec![];
    let mut text_buf = String::new();
    let mut req: Option<RequirementBlock> = None;

    for line in body.lines() {
        let trimmed = line.trim_start();
        let is_req_header = trimmed.starts_with("### Requirement:");
        let is_top_section = trimmed.starts_with("## ");

        if is_req_header {
            if !text_buf.is_empty() {
                blocks.push(Block::Text(std::mem::take(&mut text_buf)));
            }
            if let Some(b) = req.take() {
                blocks.push(Block::Req(b));
            }
            let name = trimmed
                .strip_prefix("### Requirement:")
                .unwrap_or("")
                .trim()
                .to_string();
            req = Some(RequirementBlock {
                name,
                body: format!("{}\n", line),
            });
        } else if is_top_section && req.is_some() {
            blocks.push(Block::Req(req.take().unwrap()));
            text_buf.push_str(line);
            text_buf.push('\n');
        } else if let Some(ref mut r) = req {
            r.body.push_str(line);
            r.body.push('\n');
        } else {
            text_buf.push_str(line);
            text_buf.push('\n');
        }
    }
    if !text_buf.is_empty() {
        blocks.push(Block::Text(text_buf));
    }
    if let Some(b) = req {
        blocks.push(Block::Req(b));
    }
    // Preserve trailing newline behaviour: if the original body ended without
    // a newline, the last block will already reflect that.
    if !body.ends_with('\n') {
        if let Some(last) = blocks.last_mut() {
            match last {
                Block::Text(s) => {
                    if s.ends_with('\n') {
                        s.pop();
                    }
                }
                Block::Req(r) => {
                    if r.body.ends_with('\n') {
                        r.body.pop();
                    }
                }
            }
        }
    }
    blocks
}
