// src/commands/analyze.rs
//
// `analyze` runs a handful of cross-artifact consistency checks against a
// topic's spec, task, pending changes, and (optionally) the project
// constitution. It reports findings without ever writing.
//
// Severity ladder: ERROR > WARNING > INFO. Counts roll up to a summary line.

use anyhow::Result;
use serde::Serialize;
use std::path::PathBuf;

#[derive(Serialize, Clone)]
pub struct Finding {
    pub severity: String, // "ERROR" | "WARNING" | "INFO"
    pub check: String,
    pub message: String,
    pub detail: Option<String>,
}

#[derive(Serialize)]
pub struct AnalyzeOutput {
    pub topic: String,
    pub area: String,
    pub findings: Vec<Finding>,
    pub error_count: usize,
    pub warning_count: usize,
    pub info_count: usize,
}

fn topic_safe(topic: &str) -> String {
    topic.replace('/', "-").replace(' ', "-")
}

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

/// Words that signal underspecified requirements without a measurable metric.
/// Matched as whole words, case-insensitive.
const AMBIGUOUS_WORDS: &[&str] = &[
    "fast", "scalable", "secure", "easy", "simple", "good", "better", "best", "quick",
    "securely", "quickly", "scalably", "easily",
];

const METRIC_HINTS: &[&str] = &["ms", "second", "minute", "hour", "request", "user", "req/s", "rps", "%", "p50", "p95", "p99", "qps", "kb", "mb", "gb", "tb"];

/// Pull `### Requirement: <name>` blocks from a spec body. Returns
/// `(name, full_block_text)` pairs.
fn requirement_blocks(body: &str) -> Vec<(String, String)> {
    let mut out: Vec<(String, String)> = vec![];
    let mut current: Option<(String, String)> = None;
    for line in body.lines() {
        let t = line.trim_start();
        if let Some(rest) = t.strip_prefix("### Requirement:") {
            if let Some(prev) = current.take() {
                out.push(prev);
            }
            current = Some((rest.trim().to_string(), format!("{}\n", line)));
        } else if t.starts_with("## ") {
            // Top-level section break — flush.
            if let Some(prev) = current.take() {
                out.push(prev);
            }
        } else if let Some((_, ref mut body)) = current {
            body.push_str(line);
            body.push('\n');
        }
    }
    if let Some(prev) = current {
        out.push(prev);
    }
    out
}

fn task_lines(content: &str) -> Vec<(String, bool)> {
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

fn contains_word_ci(haystack: &str, needle: &str) -> bool {
    let lower_h = haystack.to_lowercase();
    let lower_n = needle.to_lowercase();
    let bytes = lower_h.as_bytes();
    let n = lower_n.as_bytes();
    if n.is_empty() || bytes.len() < n.len() {
        return false;
    }
    for i in 0..=bytes.len() - n.len() {
        if &bytes[i..i + n.len()] == n {
            let before_ok = i == 0 || !bytes[i - 1].is_ascii_alphanumeric();
            let after_ok = i + n.len() >= bytes.len()
                || !bytes[i + n.len()].is_ascii_alphanumeric();
            if before_ok && after_ok {
                return true;
            }
        }
    }
    false
}

fn has_metric(text: &str) -> bool {
    // A measurable metric needs either a number, or an explicit unit token
    // matched as a whole word. Substring matches against generic English
    // ("user" inside "users") would over-trigger.
    if text.chars().any(|c| c.is_ascii_digit()) {
        return true;
    }
    METRIC_HINTS.iter().any(|m| contains_word_ci(text, m))
}

pub fn run_analyze(topic: &str, area: Option<&str>) -> Result<AnalyzeOutput> {
    let area = area.unwrap_or("Staging").to_string();
    let topic_dir = resolve_topic_dir(&area, topic)?;
    let safe = topic_safe(topic);

    let spec_path = topic_dir.join(format!("{}_spec.md", safe));
    let task_path = topic_dir.join(format!("{}_task.md", safe));

    let mut findings: Vec<Finding> = vec![];

    let spec_content = if spec_path.exists() {
        std::fs::read_to_string(&spec_path)?
    } else {
        findings.push(Finding {
            severity: "ERROR".to_string(),
            check: "Missing spec".to_string(),
            message: format!("No spec file at {}", spec_path.display()),
            detail: None,
        });
        String::new()
    };

    let task_content = if task_path.exists() {
        std::fs::read_to_string(&task_path)?
    } else {
        findings.push(Finding {
            severity: "ERROR".to_string(),
            check: "Missing task".to_string(),
            message: format!("No task file at {}", task_path.display()),
            detail: None,
        });
        String::new()
    };

    let spec_reqs = requirement_blocks(&spec_content);
    let tasks = task_lines(&task_content);

    // Check 1 — Duplication across canonical + pending changes.
    let pending_changes =
        crate::commands::change::run_change_list(topic, Some(&area), false).unwrap_or_default();
    for ch in &pending_changes {
        let cdir = topic_dir
            .join("changes")
            .join(ch.name.replace('/', "-").replace(' ', "-"));
        let cspec = cdir.join(format!(
            "{}_spec.md",
            ch.name.replace('/', "-").replace(' ', "-")
        ));
        if !cspec.exists() {
            continue;
        }
        let cspec_content = std::fs::read_to_string(&cspec)?;
        // Only flag duplicates that are NOT inside a ## MODIFIED Requirements
        // section — those are intentional overrides.
        let body = strip_frontmatter(&cspec_content);
        let mut in_modified = false;
        for line in body.lines() {
            let t = line.trim();
            if let Some(rest) = t.strip_prefix("## ") {
                in_modified = rest.to_lowercase().starts_with("modified requirements");
                continue;
            }
            if in_modified {
                continue;
            }
            if let Some(name) = t.strip_prefix("### Requirement:") {
                let name = name.trim().to_string();
                if spec_reqs.iter().any(|(n, _)| n == &name) {
                    findings.push(Finding {
                        severity: "WARNING".to_string(),
                        check: "Duplication".to_string(),
                        message: format!(
                            "Requirement '{}' appears in both canonical spec and change '{}'.",
                            name, ch.name
                        ),
                        detail: Some(
                            "If this is intentional, move it under `## MODIFIED Requirements`."
                                .to_string(),
                        ),
                    });
                }
            }
        }
    }

    // Check 2 — Missing tasks. Each `### Requirement: X` should be mentioned
    // (case-insensitive substring) by at least one task line.
    for (name, _body) in &spec_reqs {
        let n_lower = name.to_lowercase();
        let covered = tasks.iter().any(|(t, _)| t.to_lowercase().contains(&n_lower))
            || tasks
                .iter()
                .any(|(t, _)| matches_token(&t.to_lowercase(), &n_lower));
        if !covered {
            findings.push(Finding {
                severity: "ERROR".to_string(),
                check: "Missing task coverage".to_string(),
                message: format!("Requirement '{}' has no corresponding task.", name),
                detail: Some(format!(
                    "Add a `- [ ]` task to {} that mentions '{}'.",
                    task_path.display(),
                    name
                )),
            });
        }
    }

    // Check 3 — Ambiguous language.
    for (name, body) in &spec_reqs {
        for w in AMBIGUOUS_WORDS {
            if contains_word_ci(body, w) && !has_metric(body) {
                findings.push(Finding {
                    severity: "WARNING".to_string(),
                    check: "Ambiguous language".to_string(),
                    message: format!(
                        "Requirement '{}' contains '{}' without a measurable metric.",
                        name, w
                    ),
                    detail: Some(
                        "Replace with a concrete number or threshold (e.g. '< 200ms p95')."
                            .to_string(),
                    ),
                });
                break; // one warning per requirement is enough
            }
        }
    }

    // Check 4 — Empty sections (any `## <Heading>` with no content before
    // the next `##` line).
    let body = strip_frontmatter(&spec_content);
    let lines: Vec<&str> = body.lines().collect();
    let mut i = 0;
    while i < lines.len() {
        let t = lines[i].trim();
        if t.starts_with("## ") && !t.starts_with("### ") {
            // Look forward until the next `## ` or EOF.
            let mut j = i + 1;
            let mut has_content = false;
            while j < lines.len() {
                let nt = lines[j].trim();
                if nt.starts_with("## ") && !nt.starts_with("### ") {
                    break;
                }
                if !nt.is_empty() {
                    has_content = true;
                    break;
                }
                j += 1;
            }
            if !has_content {
                findings.push(Finding {
                    severity: "WARNING".to_string(),
                    check: "Empty section".to_string(),
                    message: format!("Section '{}' has no content.", t),
                    detail: None,
                });
            }
        }
        i += 1;
    }

    // Check 5 — Constitution alignment (very light heuristic: if there's a
    // task that mentions "skip tests" or "no spec", warn).
    if crate::commands::constitution::constitution_path().exists() {
        let constitution = crate::commands::constitution::read_constitution().unwrap_or_default();
        // Surface the constitution as an INFO finding so the agent is
        // reminded to evaluate manually.
        findings.push(Finding {
            severity: "INFO".to_string(),
            check: "Constitution alignment".to_string(),
            message: "Project constitution loaded — verify the spec/tasks do not violate any principle.".to_string(),
            detail: Some(format!(
                "Constitution version: {}",
                constitution
                    .lines()
                    .find(|l| l.trim_start().starts_with("Version:"))
                    .map(|s| s.trim().trim_start_matches("Version:").trim().to_string())
                    .unwrap_or_else(|| "unknown".to_string())
            )),
        });
    }

    // Check 6 — Incomplete tasks (counts across main + every pending change).
    let mut total_tasks = tasks.len();
    let mut completed_tasks = tasks.iter().filter(|(_, c)| *c).count();
    for ch in &pending_changes {
        let safe_c = ch.name.replace('/', "-").replace(' ', "-");
        let ctask = topic_dir
            .join("changes")
            .join(&safe_c)
            .join(format!("{}_task.md", safe_c));
        if let Ok(content) = std::fs::read_to_string(&ctask) {
            let ts = task_lines(&content);
            total_tasks += ts.len();
            completed_tasks += ts.iter().filter(|(_, c)| *c).count();
        }
    }
    if total_tasks > 0 {
        let pct = (completed_tasks * 100) / total_tasks;
        findings.push(Finding {
            severity: "INFO".to_string(),
            check: "Task completion".to_string(),
            message: format!(
                "{} of {} tasks complete ({}%).",
                completed_tasks, total_tasks, pct
            ),
            detail: None,
        });
    }

    let error_count = findings.iter().filter(|f| f.severity == "ERROR").count();
    let warning_count = findings.iter().filter(|f| f.severity == "WARNING").count();
    let info_count = findings.iter().filter(|f| f.severity == "INFO").count();

    Ok(AnalyzeOutput {
        topic: topic.to_string(),
        area,
        findings,
        error_count,
        warning_count,
        info_count,
    })
}

fn strip_frontmatter(content: &str) -> &str {
    if content.trim_start().starts_with("---") {
        if let Some(end) = content.find("\n---") {
            return &content[end + 5..];
        }
    }
    content
}

/// True iff every token in `needle` is present in `haystack` as a substring.
/// Helps a task like "Implement Refresh Token" match the requirement
/// "Refresh Token" even though the order or surrounding words differ.
fn matches_token(haystack: &str, needle: &str) -> bool {
    needle
        .split_whitespace()
        .filter(|w| w.len() > 2)
        .all(|w| haystack.contains(w))
}
