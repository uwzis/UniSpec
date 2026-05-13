// src/tui/state.rs
use crate::agent::mode::DisplayType;
use crate::fs::{self, spec::SpecMetadata};
use anyhow::Result;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub struct TopicNode {
    pub topic: String,
    pub path: PathBuf,
    pub area: String,
    pub area_type: DisplayType,
    pub status: String,
    pub tasks_total: usize,
    pub tasks_completed: usize,
    pub tasks_in_progress: usize,
    pub metadata: SpecMetadata,
    pub children: Vec<TopicNode>,
    pub is_checked_out: bool,
    pub checked_out_by: Option<String>,
    pub short: String,
    /// True when this node lives inside a topic's `changes/` subtree
    /// (or `changes/archive/`).
    pub is_change: bool,
    /// True for the `changes/` directory itself (the container).
    pub is_changes_container: bool,
}

impl TopicNode {
    pub fn relative_path(&self) -> String {
        let area_path = fs::spec_dir().join(&self.area);
        if let Ok(rel) = self.path.strip_prefix(area_path) {
            rel.to_string_lossy().to_string()
        } else {
            self.topic.clone()
        }
    }

    pub fn extract_short_from_file(path: &Path) -> String {
        if let Ok(content) = std::fs::read_to_string(path) {
            if let Some(short_start) = content.find("short:") {
                let after_short = &content[short_start..];
                if let Some(newline) = after_short.find('\n') {
                    let short_line = after_short[7..newline].trim().to_string();
                    if !short_line.is_empty() && short_line != "<Add a one-liner description here>"
                    {
                        return short_line;
                    }
                }
            }
        }
        String::new()
    }
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum NavState {
    AreaSelection,
    TopicList(String),
    NestedSpecs(PathBuf),
    FindResults { topic: String, paths: Vec<String> },
}

#[derive(Clone, Debug)]
pub struct AppState {
    pub nav_state: NavState,
    pub topics: Vec<TopicNode>,
}

impl AppState {
    pub fn new() -> Result<Self> {
        Ok(AppState {
            nav_state: NavState::AreaSelection,
            topics: vec![],
        })
    }

    pub fn load_areas(&self) -> Result<Vec<String>> {
        let ordered_areas = crate::agent::mode::get_area_order();

        let spec_dir = fs::spec_dir();
        let mut existing_areas = std::collections::HashSet::new();
        if spec_dir.exists() {
            for entry in std::fs::read_dir(spec_dir)? {
                let entry = entry?;
                if entry.path().is_dir() {
                    let area_filename = crate::agent::mode::get_area_filename();
                    if entry.path().join(&area_filename).exists() {
                        existing_areas.insert(entry.file_name().to_string_lossy().to_string());
                    }
                }
            }
        }

        if !ordered_areas.is_empty() {
            let areas: Vec<String> = ordered_areas
                .into_iter()
                .filter(|a| existing_areas.contains(a))
                .collect();
            if areas.is_empty() {
                let mut areas: Vec<String> = existing_areas.into_iter().collect();
                areas.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
                Ok(areas)
            } else {
                Ok(areas)
            }
        } else {
            let mut areas: Vec<String> = existing_areas.into_iter().collect();
            areas.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
            Ok(areas)
        }
    }

    pub fn count_topics_in_area(&self, area: &str) -> usize {
        let area_spec_dir = fs::spec_dir().join(area);
        if !area_spec_dir.exists() {
            return 0;
        }

        std::fs::read_dir(&area_spec_dir)
            .map(|entries| {
                entries
                    .filter_map(|e| e.ok())
                    .filter(|e| e.path().is_dir())
                    .count()
            })
            .unwrap_or(0)
    }

    pub fn has_work_in_area(&self, area: &str) -> bool {
        let area_lower = area.to_lowercase();

        if area_lower == "build" {
            return false;
        }

        let area_spec_dir = fs::spec_dir().join(area);
        if !area_spec_dir.exists() {
            return false;
        }

        if let Ok(entries) = std::fs::read_dir(&area_spec_dir) {
            for entry in entries.flatten() {
                if entry.path().is_dir() {
                    let topic_path = &entry.path();
                    let tasks_path = topic_path.join("tasks.md");

                    if tasks_path.exists() {
                        if let Ok(tasks_content) = std::fs::read_to_string(&tasks_path) {
                            // Check for [-] (in progress) or *[-] (in progress with asterisk)
                            let has_in_progress = tasks_content.lines().any(|line| {
                                let trimmed = line.trim();
                                trimmed.starts_with("- [-]") || trimmed.starts_with("* [-]")
                            });

                            if has_in_progress {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }

    pub fn load_topics_for_area(&mut self, area: &str) -> Result<()> {
        let area_type = self.get_area_type(area);
        let spec_file = self.get_spec_filename_for_area(area);
        let mut topics = vec![];
        let area_spec_dir = fs::spec_dir().join(area);

        // Get order from mode.toml
        let order = crate::agent::mode::get_topic_order(area);

        if area_spec_dir.exists() {
            for entry in std::fs::read_dir(&area_spec_dir)? {
                let entry = entry?;
                if entry.path().is_dir() {
                    // Only include directories that have topic.md files
                    if entry.path().join("topic.md").exists() {
                        topics.push(self.load_topic_recursive(
                            &entry.path(),
                            area.to_string(),
                            area_type.clone(),
                            &spec_file,
                            false,
                        )?);
                    }
                }
            }
        }

        // Sort topics before specs, then alphabetically
        topics.sort_by(|a, b| {
            let a_is_spec = a.status == "spec";
            let b_is_spec = b.status == "spec";

            if a_is_spec != b_is_spec {
                return a_is_spec.cmp(&b_is_spec);
            }

            if !order.is_empty() {
                let a_idx = order
                    .iter()
                    .position(|o| o == &a.topic)
                    .unwrap_or(usize::MAX);
                let b_idx = order
                    .iter()
                    .position(|o| o == &b.topic)
                    .unwrap_or(usize::MAX);
                a_idx.cmp(&b_idx)
            } else {
                a.topic.cmp(&b.topic)
            }
        });

        self.topics = topics;
        self.nav_state = NavState::TopicList(area.to_string());

        // Populate short descriptions
        self.populate_short_descriptions();

        Ok(())
    }

    fn populate_short_descriptions(&mut self) {
        for topic in &mut self.topics {
            topic.short = TopicNode::extract_short_from_file(&topic.path.join("topic.md"));
            for child in &mut topic.children {
                // For specs, child.path is already the spec file path
                // For nested topics, child.path is the directory
                if child.path.is_file() {
                    // It's a spec file - use path directly
                    child.short = TopicNode::extract_short_from_file(&child.path);
                } else {
                    // It's a directory - try topic.md
                    child.short = TopicNode::extract_short_from_file(&child.path.join("topic.md"));
                }
            }
        }
    }

    fn get_area_type(&self, area: &str) -> DisplayType {
        let area_lower = area.to_lowercase();

        // First, check area name directly - most reliable
        if area_lower.contains("roadmap") {
            return DisplayType::Roadmap;
        }
        if area_lower.contains("build") && area_lower != "working" {
            return DisplayType::Build;
        }
        if area_lower.contains("working") {
            return DisplayType::Working;
        }
        if area_lower.contains("specing") {
            return DisplayType::Specing;
        }

        // Then check mode config
        if let Ok(mode_name) = crate::agent::current_mode() {
            if let Ok(config) = crate::agent::mode::get_mode_info(&mode_name) {
                if let Some(ref roadmap_config) = config.area_types.roadmap {
                    if area_lower == "roadmap" || area_lower.contains("roadmap") {
                        return DisplayType::Roadmap;
                    }
                }
                if let Some(ref working_config) = config.area_types.working {
                    if area_lower == "working" || area_lower.contains("working") {
                        return DisplayType::Working;
                    }
                }
                if let Some(ref build_config) = config.area_types.build {
                    if area_lower == "build" || area_lower.contains("build") {
                        return DisplayType::Build;
                    }
                }
                if let Some(ref specing_config) = config.area_types.specing {
                    if area_lower == "specing" || area_lower.contains("specing") {
                        return DisplayType::Specing;
                    }
                }
            }
        }
        DisplayType::Standard
    }

    fn get_spec_filename_for_area(&self, area: &str) -> String {
        let area_lower = area.to_lowercase();

        // Check area name directly first
        if area_lower.contains("roadmap") {
            // Try mode config first, then check what files exist
            if let Ok(mode_name) = crate::agent::current_mode() {
                if let Ok(config) = crate::agent::mode::get_mode_info(&mode_name) {
                    if let Some(ref roadmap_config) = config.area_types.roadmap {
                        let filename = &roadmap_config.spec_file;
                        // Check if file exists, if not try alternatives
                        let spec_dir = fs::spec_dir().join(area);
                        if spec_dir.join(filename).exists() {
                            return filename.clone();
                        }
                    }
                }
            }
            // Fallback: return first existing common name
            return "spec.md".to_string();
        }

        if area_lower.contains("build") {
            if let Ok(mode_name) = crate::agent::current_mode() {
                if let Ok(config) = crate::agent::mode::get_mode_info(&mode_name) {
                    if let Some(ref build_config) = config.area_types.build {
                        return build_config.spec_file.clone();
                    }
                }
            }
            return "specs.md".to_string();
        }

        if area_lower.contains("working") {
            if let Ok(mode_name) = crate::agent::current_mode() {
                if let Ok(config) = crate::agent::mode::get_mode_info(&mode_name) {
                    if let Some(ref working_config) = config.area_types.working {
                        return working_config.spec_file.clone();
                    }
                }
            }
        }

        crate::agent::mode::get_spec_filename_for_area(area)
    }

    fn load_topic_recursive(
        &self,
        path: &Path,
        area: String,
        area_type: DisplayType,
        spec_file: &str,
        in_changes_subtree: bool,
    ) -> Result<TopicNode> {
        let topic = path.file_name().unwrap().to_string_lossy().to_string();
        let is_changes_container = !in_changes_subtree && topic == "changes";
        let child_in_changes = in_changes_subtree || is_changes_container;

        let mut children = vec![];
        if path.exists() {
            for entry in std::fs::read_dir(path)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    children.push(self.load_topic_recursive(
                        &path,
                        area.clone(),
                        area_type.clone(),
                        spec_file,
                        child_in_changes,
                    )?);
                } else if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
                    if filename.ends_with("_spec.md") {
                        let spec_name = filename.trim_end_matches("_spec.md");
                        let task_filename = format!("{}_task.md", spec_name);
                        let task_path = path.parent().unwrap().join(task_filename);
                        let (total, completed, in_progress) = if task_path.exists() {
                            Self::count_tasks_in_dir(&task_path, &area)
                        } else {
                            (0, 0, 0)
                        };

                        // Extract short from spec file
                        let spec_short = TopicNode::extract_short_from_file(&path);

                        children.push(TopicNode {
                            topic: spec_name.to_string(),
                            path: path.clone(),
                            area: area.clone(),
                            area_type: area_type.clone(),
                            status: "spec".to_string(),
                            tasks_total: total,
                            tasks_completed: completed,
                            tasks_in_progress: in_progress,
                            metadata: SpecMetadata::default(),
                            children: vec![],
                            is_checked_out: false,
                            checked_out_by: None,
                            short: spec_short,
                            is_change: in_changes_subtree,
                            is_changes_container: false,
                        });
                    }
                }
            }
        }

        let metadata = self.load_spec_metadata(path, spec_file);

        let (total, completed, in_progress) = match area_type {
            DisplayType::Roadmap | DisplayType::Build => (0, 0, 0),
            DisplayType::Working | DisplayType::Standard | DisplayType::Specing => {
                self.calculate_total_tasks(path, &children, &area)
            }
        };

        let status = if total > 0 && completed == total {
            "complete".to_string()
        } else if in_progress > 0 || completed > 0 {
            "in-progress".to_string()
        } else if metadata.impact.is_some() || metadata.change_type.is_some() {
            "pending".to_string()
        } else {
            "waiting".to_string()
        };

        let is_checked_out = metadata
            .checked_out
            .as_ref()
            .map(|v| !v.is_empty())
            .unwrap_or(false);
        let checked_out_by = metadata.checked_out.clone();

        Ok(TopicNode {
            topic,
            path: path.to_path_buf(),
            area,
            area_type,
            status,
            tasks_total: total,
            tasks_completed: completed,
            tasks_in_progress: in_progress,
            metadata,
            children,
            is_checked_out,
            checked_out_by,
            short: String::new(),
            is_change: in_changes_subtree && !is_changes_container,
            is_changes_container,
        })
    }

    fn load_spec_metadata(&self, path: &Path, _spec_file: &str) -> SpecMetadata {
        // Look for any file ending in _spec.md in the directory
        if let Ok(entries) = std::fs::read_dir(path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
                    if filename.ends_with("_spec.md") {
                        if let Ok(content) = std::fs::read_to_string(&path) {
                            if let Some(metadata) = crate::fs::spec::parse_spec_metadata(&content) {
                                return metadata;
                            }
                        }
                    }
                }
            }
        }

        SpecMetadata::default()
    }

    fn calculate_total_tasks(
        &self,
        path: &Path,
        children: &[TopicNode],
        area: &str,
    ) -> (usize, usize, usize) {
        let (mut total, mut completed, mut in_progress) = Self::count_tasks_in_dir(path, area);
        for child in children {
            let (c_total, c_completed, c_in_progress) =
                self.calculate_total_tasks(&child.path, &child.children, area);
            total += c_total;
            completed += c_completed;
            in_progress += c_in_progress;
        }
        (total, completed, in_progress)
    }

    fn count_tasks_in_dir(path: &Path, _area: &str) -> (usize, usize, usize) {
        let mut total = 0;
        let mut completed = 0;
        let mut in_progress = 0;

        if path.is_file() && path.to_string_lossy().ends_with("_task.md") {
            if let Ok(content) = std::fs::read_to_string(path) {
                for line in content.lines() {
                    let trimmed = line.trim();
                    if trimmed.starts_with("- [") {
                        total += 1;
                        if trimmed.starts_with("- [x]") || trimmed.starts_with("- [X]") {
                            completed += 1;
                        } else if trimmed.starts_with("- [-]") {
                            in_progress += 1;
                        }
                    }
                }
            }
        } else if path.is_dir() {
            if let Ok(entries) = std::fs::read_dir(path) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if let Some(filename) = path.file_name().and_then(|f| f.to_str()) {
                        if filename.ends_with("_task.md") {
                            if let Ok(content) = std::fs::read_to_string(&path) {
                                for line in content.lines() {
                                    let trimmed = line.trim();
                                    if trimmed.starts_with("- [") {
                                        total += 1;
                                        if trimmed.starts_with("- [x]")
                                            || trimmed.starts_with("- [X]")
                                        {
                                            completed += 1;
                                        } else if trimmed.starts_with("- [-]") {
                                            in_progress += 1;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        (total, completed, in_progress)
    }

    pub fn update_status(&mut self) {
        match self.nav_state.clone() {
            NavState::TopicList(area) => {
                let _ = self.load_topics_for_area(&area);
            }
            NavState::NestedSpecs(path) => {
                let spec_dir = fs::spec_dir();
                if let Ok(rel) = path.strip_prefix(&spec_dir) {
                    let parts: Vec<&str> = rel.iter().filter_map(|p| p.to_str()).collect();
                    let area = parts.first().unwrap_or(&"Working").to_string();
                    if path.exists() {
                        if let Ok(new_children) = self.load_nested_topics(&path, area) {
                            self.topics = new_children;
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn load_nested_topics(&self, path: &Path, area: String) -> Result<Vec<TopicNode>> {
        let area_type = self.get_area_type(&area);
        let spec_file = self.get_spec_filename_for_area(&area);
        let mut topics = vec![];
        if path.exists() {
            for entry in std::fs::read_dir(path)? {
                let entry = entry?;
                if entry.path().is_dir() && entry.path().join("topic.md").exists() {
                    topics.push(self.load_topic_recursive(
                        &entry.path(),
                        area.clone(),
                        area_type.clone(),
                        &spec_file,
                        false,
                    )?);
                }
            }
        }
        Ok(topics)
    }
}
