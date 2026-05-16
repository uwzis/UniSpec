// src/commands/workspace.rs
//
// Lightweight multi-repo coordination for UniSpec. A workspace is a folder
// holding `.unispec-workspace/workspace.yaml` listing named pointers to
// other UniSpec project roots.
//
// The YAML shape is fixed and small enough to hand-roll a parser, avoiding
// a new dependency:
//
//   name: my-app
//   version: 1
//   links:
//     api: /abs/path/to/api
//     web: /abs/path/to/web

use anyhow::Result;
use serde::Serialize;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

pub fn workspace_dir(root: &Path) -> PathBuf {
    root.join(".unispec-workspace")
}

pub fn workspace_file(root: &Path) -> PathBuf {
    workspace_dir(root).join("workspace.yaml")
}

#[derive(Serialize, Default, Clone)]
pub struct Workspace {
    pub name: String,
    pub version: u32,
    pub links: BTreeMap<String, String>,
}

fn parse_workspace(content: &str) -> Workspace {
    let mut ws = Workspace::default();
    ws.version = 1;
    let mut in_links = false;
    for raw in content.lines() {
        let line = raw.trim_end();
        if line.is_empty() {
            continue;
        }
        if !line.starts_with(' ') && !line.starts_with('-') {
            // Top-level key.
            in_links = false;
            if let Some(rest) = line.strip_prefix("name:") {
                ws.name = rest.trim().to_string();
            } else if let Some(rest) = line.strip_prefix("version:") {
                ws.version = rest.trim().parse().unwrap_or(1);
            } else if line.starts_with("links:") {
                in_links = true;
                // Handle the `links: {}` inline empty form.
                let rest = line["links:".len()..].trim();
                if rest.starts_with('{') {
                    in_links = false;
                }
            }
        } else if in_links {
            // Indented key.
            let trimmed = line.trim_start();
            if let Some((k, v)) = trimmed.split_once(':') {
                let key = k.trim().to_string();
                let val = v.trim().trim_matches('"').to_string();
                if !key.is_empty() {
                    ws.links.insert(key, val);
                }
            }
        }
    }
    ws
}

fn serialize_workspace(ws: &Workspace) -> String {
    let mut out = String::new();
    out.push_str(&format!("name: {}\n", ws.name));
    out.push_str(&format!("version: {}\n", ws.version));
    if ws.links.is_empty() {
        out.push_str("links: {}\n");
    } else {
        out.push_str("links:\n");
        for (k, v) in &ws.links {
            out.push_str(&format!("  {}: {}\n", k, v));
        }
    }
    out
}

pub fn load(root: &Path) -> Result<Workspace> {
    let path = workspace_file(root);
    if !path.exists() {
        return Err(anyhow::anyhow!(
            "No workspace at {}. Run `unispec workspace init <name>` first.",
            path.display()
        ));
    }
    let content = std::fs::read_to_string(&path)?;
    Ok(parse_workspace(&content))
}

pub fn save(root: &Path, ws: &Workspace) -> Result<()> {
    let dir = workspace_dir(root);
    std::fs::create_dir_all(&dir)?;
    std::fs::write(workspace_file(root), serialize_workspace(ws))?;
    Ok(())
}

pub fn run_init(root: &Path, name: &str) -> Result<PathBuf> {
    if workspace_file(root).exists() {
        return Err(anyhow::anyhow!(
            "Workspace already exists at {}",
            workspace_file(root).display()
        ));
    }
    let ws = Workspace {
        name: name.to_string(),
        version: 1,
        links: BTreeMap::new(),
    };
    save(root, &ws)?;
    Ok(workspace_file(root))
}

pub fn run_link(root: &Path, name: &str, path: &str) -> Result<Workspace> {
    let mut ws = load(root)?;
    let abs = PathBuf::from(path);
    let abs = if abs.is_absolute() {
        abs
    } else {
        root.join(path)
    };
    ws.links.insert(name.to_string(), abs.display().to_string());
    save(root, &ws)?;
    Ok(ws)
}

#[derive(Serialize)]
pub struct LinkStatus {
    pub name: String,
    pub path: String,
    pub path_exists: bool,
    pub has_unispec: bool,
}

pub fn run_list(root: &Path) -> Result<(String, Vec<LinkStatus>)> {
    let ws = load(root)?;
    let mut out = vec![];
    for (name, path) in &ws.links {
        let p = PathBuf::from(path);
        out.push(LinkStatus {
            name: name.clone(),
            path: path.clone(),
            path_exists: p.exists(),
            has_unispec: p.join("spec").exists() && p.join(".agent").exists(),
        });
    }
    Ok((ws.name, out))
}

#[derive(Serialize)]
pub struct RepoTopics {
    pub name: String,
    pub path: String,
    pub topics: Vec<RepoTopic>,
    pub error: Option<String>,
}

#[derive(Serialize)]
pub struct RepoTopic {
    pub area: String,
    pub topic: String,
}

pub fn run_status(root: &Path) -> Result<(String, Vec<RepoTopics>)> {
    let ws = load(root)?;
    let mut out = vec![];
    for (name, path) in &ws.links {
        let p = PathBuf::from(path);
        if !p.exists() {
            out.push(RepoTopics {
                name: name.clone(),
                path: path.clone(),
                topics: vec![],
                error: Some(format!("Path does not exist: {}", path)),
            });
            continue;
        }
        let spec_dir = p.join("spec");
        if !spec_dir.exists() {
            out.push(RepoTopics {
                name: name.clone(),
                path: path.clone(),
                topics: vec![],
                error: Some("No spec/ directory — not a UniSpec project".to_string()),
            });
            continue;
        }
        let mut topics = vec![];
        if let Ok(area_entries) = std::fs::read_dir(&spec_dir) {
            for area_entry in area_entries.flatten() {
                let area_path = area_entry.path();
                if !area_path.is_dir() {
                    continue;
                }
                let area_name = area_entry.file_name().to_string_lossy().to_string();
                if let Ok(topic_entries) = std::fs::read_dir(&area_path) {
                    for topic_entry in topic_entries.flatten() {
                        if topic_entry.path().is_dir()
                            && topic_entry.path().join("topic.md").exists()
                        {
                            topics.push(RepoTopic {
                                area: area_name.clone(),
                                topic: topic_entry
                                    .file_name()
                                    .to_string_lossy()
                                    .to_string(),
                            });
                        }
                    }
                }
            }
        }
        topics.sort_by(|a, b| (&a.area, &a.topic).cmp(&(&b.area, &b.topic)));
        out.push(RepoTopics {
            name: name.clone(),
            path: path.clone(),
            topics,
            error: None,
        });
    }
    Ok((ws.name, out))
}
