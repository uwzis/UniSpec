use crate::tui::platypus::PLATYPUS_FRAMES;
use crate::tui::state::{AppState, NavState, TopicNode};
use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::prelude::CrosstermBackend;
use ratatui::prelude::{Color, Style};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};
use ratatui::Terminal;
use ratatui::{
    layout::{Constraint, Direction, Layout},
    widgets::ListState,
};
use std::io::{self, Stdout};
use std::time::{Duration, Instant};

#[derive(Clone, Copy, PartialEq)]
pub enum PlatypusState {
    Idle,
    Happy,
    Sad,
    Love,
    Searching,
    Working,
    Celebrating,
}

impl PlatypusState {
    fn to_index(&self) -> usize {
        match self {
            PlatypusState::Idle => 0,
            PlatypusState::Happy => 1,
            PlatypusState::Sad => 2,
            PlatypusState::Love => 3,
            PlatypusState::Searching => 4,
            PlatypusState::Working => 5,
            PlatypusState::Celebrating => 6,
        }
    }
}

pub struct App {
    pub state: AppState,
    pub should_exit: bool,
    pub last_refresh: Instant,
    pub frame: usize,
    pub list_state: ListState,
    pub history: Vec<(NavState, Vec<TopicNode>)>,
    pub current_area: Option<String>,
    pub message: Option<String>,
    pub input_mode: bool,
    pub input_buffer: String,
    pub input_prompt: String,
    pub pending_args: Vec<String>,
    pub pending_impact: Option<String>,
    pub pending_change_type: Option<String>,
    pub pending_short: Option<String>,
    pub pending_topic_name: Option<String>,
    pub message_timer: Option<Instant>,
    pub find_paths: Vec<String>,
    pub saved_topics: Vec<TopicNode>,
    pub saved_nav_state: Option<NavState>,
    pub pending_link_path: Option<String>,
    pub pending_link_topic: Option<String>,
    pub platypus_state: PlatypusState,
    pub animation_step: usize,
    pub expression_timer: Option<Instant>,
    pub platypus_enabled: bool,
    pub preview_content: Option<String>,
    pub preview_topic: Option<String>, // Track what's being previewed
    pub selected_short: Option<String>, // Short description of currently selected item
    /// One-shot signal that the next iteration of the main render loop must
    /// invalidate ratatui's internal buffer and repaint from scratch. Set after
    /// the TUI hands the terminal to an external program (e.g. `$EDITOR`) so
    /// ratatui doesn't diff against its stale pre-suspend buffer and skip
    /// cells that the editor visibly clobbered.
    pub needs_full_redraw: bool,
}

impl App {
    pub fn new() -> Result<Self> {
        Ok(App {
            state: AppState::new()?,
            should_exit: false,
            last_refresh: Instant::now(),
            frame: 0,
            list_state: ListState::default(),
            history: vec![],
            current_area: None,
            message: None,
            input_mode: false,
            input_buffer: String::new(),
            input_prompt: String::new(),
            pending_args: vec![],
            pending_impact: None,
            pending_change_type: None,
            pending_short: None,
            pending_topic_name: None,
            message_timer: None,
            find_paths: vec![],
            saved_topics: vec![],
            saved_nav_state: None,
            pending_link_path: None,
            pending_link_topic: None,
            platypus_state: PlatypusState::Idle,
            animation_step: 0,
            expression_timer: None,
            platypus_enabled: true,
            preview_content: None,
            preview_topic: None,
            selected_short: None,
            needs_full_redraw: false,
        })
    }

    fn get_platypus_frame(&self, step: usize) -> String {
        let frame_index = step % 4;
        let state_index = self.platypus_state.to_index();
        PLATYPUS_FRAMES[state_index][frame_index].to_string()
    }

    fn trigger_expression(&mut self, state: PlatypusState, duration_secs: u64) {
        self.platypus_state = state;
        self.animation_step = 0;
        self.expression_timer = Some(Instant::now() + Duration::from_secs(duration_secs));
    }

    fn handle_enter_key(&mut self) {
        // Enter opens the file/directory in the default app
        match &self.state.nav_state {
            NavState::FindResults { .. } => {
                if let Some(path) = self
                    .find_paths
                    .get(self.list_state.selected().unwrap_or(0))
                    .cloned()
                {
                    self.open_file(&path);
                }
            }
            NavState::AreaSelection => {
                let areas = self.state.load_areas().unwrap_or_default();
                let selected = self.list_state.selected().unwrap_or(0);
                if let Some(area_name) = areas.get(selected) {
                    let area_path = crate::fs::spec_dir().join(area_name);
                    self.open_file(&area_path.to_string_lossy());
                }
            }
            _ => {
                if let Some(topic) = self
                    .state
                    .topics
                    .get(self.list_state.selected().unwrap_or(0))
                    .cloned()
                {
                    self.open_file(&topic.path.to_string_lossy());
                }
            }
        }
    }

    fn get_chunk_indices(&self) -> (usize, usize, usize) {
        // Returns (list_idx, short_idx, help_idx) based on current state
        let has_platypus = self.platypus_enabled;
        let has_preview = self.preview_content.is_some();

        if has_platypus && has_preview {
            (3, 4, 5) // status(0), platypus(1), preview(2), list(3), short(4), help(5)
        } else if has_platypus {
            (2, 3, 4) // status(0), platypus(1), list(2), short(3), help(4)
        } else if has_preview {
            (2, 3, 4) // status(0), preview(1), list(2), short(3), help(4)
        } else {
            (1, 2, 3) // status(0), list(1), short(2), help(3)
        }
    }

    fn update_selected_short(&mut self) {
        self.selected_short = None;

        match &self.state.nav_state {
            NavState::AreaSelection => {
                let areas = self.state.load_areas().unwrap_or_default();
                if let Some(selected) = self.list_state.selected() {
                    if let Some(area_name) = areas.get(selected) {
                        // Try to read area.md and get short from frontmatter
                        let area_path = crate::fs::spec_dir().join(area_name).join("area.md");
                        if let Ok(content) = std::fs::read_to_string(&area_path) {
                            self.selected_short =
                                TopicNode::extract_short_from_file(&area_path).into();
                        }
                    }
                }
            }
            NavState::TopicList(_) | NavState::NestedSpecs(_) => {
                if let Some(selected) = self.list_state.selected() {
                    if let Some(topic) = self.state.topics.get(selected) {
                        self.selected_short = topic.short.clone().into();
                    }
                }
            }
            NavState::FindResults { .. } => {
                // For find results, could show the file path
            }
        }
    }

    fn get_area_type(&self, area: &str) -> crate::agent::mode::DisplayType {
        let area_lower = area.to_lowercase();

        // First, check area name directly - most reliable
        if area_lower.contains("roadmap") {
            return crate::agent::mode::DisplayType::Roadmap;
        }
        if area_lower.contains("build") && area_lower != "working" {
            return crate::agent::mode::DisplayType::Build;
        }
        if area_lower.contains("working") {
            return crate::agent::mode::DisplayType::Working;
        }
        if area_lower.contains("specing") {
            return crate::agent::mode::DisplayType::Specing;
        }

        // Then check mode config
        if let Ok(mode_name) = crate::agent::current_mode() {
            if let Ok(config) = crate::agent::mode::get_mode_info(&mode_name) {
                if let Some(ref roadmap_config) = config.area_types.roadmap {
                    if area_lower == "roadmap" || area_lower.contains("roadmap") {
                        return crate::agent::mode::DisplayType::Roadmap;
                    }
                }
                if let Some(ref working_config) = config.area_types.working {
                    if area_lower == "working" || area_lower.contains("working") {
                        return crate::agent::mode::DisplayType::Working;
                    }
                }
                if let Some(ref build_config) = config.area_types.build {
                    if area_lower == "build" || area_lower.contains("build") {
                        return crate::agent::mode::DisplayType::Build;
                    }
                }
                if let Some(ref specing_config) = config.area_types.specing {
                    if area_lower == "specing" || area_lower.contains("specing") {
                        return crate::agent::mode::DisplayType::Specing;
                    }
                }
            }
        }
        crate::agent::mode::DisplayType::Standard
    }

    pub fn run(&mut self) -> Result<()> {
        let mut terminal = self.setup_terminal()?;
        self.list_state.select(Some(0));

        while !self.should_exit {
            if let Some(timer) = self.message_timer {
                if timer.elapsed() >= Duration::from_secs(3) {
                    self.message = None;
                    self.message_timer = None;
                }
            }
            if self.last_refresh.elapsed() >= Duration::from_millis(500) {
                self.frame += 1;
                if let NavState::TopicList(_) = self.state.nav_state {
                    self.state.update_status();
                    if self.expression_timer.is_none() {
                        if let Some(selected_idx) = self.list_state.selected() {
                            if let Some(topic) = self.state.topics.get(selected_idx) {
                                if topic.tasks_in_progress > 0 {
                                    self.platypus_state = PlatypusState::Working;
                                } else if topic.tasks_total > 0
                                    && topic.tasks_completed == topic.tasks_total
                                {
                                    self.platypus_state = PlatypusState::Celebrating;
                                } else {
                                    self.platypus_state = PlatypusState::Idle;
                                }
                            }
                        }
                    }
                }
                self.last_refresh = Instant::now();
            }

            // Check expression timer
            if let Some(timer) = self.expression_timer {
                if Instant::now() >= timer {
                    self.platypus_state = PlatypusState::Idle;
                    self.expression_timer = None;
                    self.animation_step = 0;
                } else {
                    // Animate every 200ms
                    if self.frame % 2 == 0 {
                        self.animation_step += 1;
                    }
                }
            }

            // If we just returned from an external editor / content dump,
            // invalidate ratatui's internal buffer so the next draw paints
            // every cell rather than diffing against the pre-suspend state.
            if self.needs_full_redraw {
                let _ = terminal.clear();
                self.needs_full_redraw = false;
            }

            terminal.draw(|f: &mut ratatui::Frame| {
                // Determine layout based on state - compute index for preview chunk
                let preview_chunk_idx = if self.preview_content.is_some() {
                    if self.platypus_enabled { 3 } else { 2 }
                } else {
                    usize::MAX // No preview
                };
                
                // Add short description box (between list and help)
                let chunks = if self.platypus_enabled {
                    if self.preview_content.is_some() {
                        // Platypus + Preview + Short: status(0), platypus(1), preview(2), list(3), short(4), help(5)
                        Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([
                                Constraint::Length(3),
                                Constraint::Length(7),
                                Constraint::Length(10),
                                Constraint::Min(1),
                                Constraint::Length(3),
                                Constraint::Length(3),
                            ])
                            .split(f.size())
                    } else {
                        // Platypus + Short: status(0), platypus(1), list(2), short(3), help(4)
                        Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([
                                Constraint::Length(3),
                                Constraint::Length(7),
                                Constraint::Min(1),
                                Constraint::Length(3),
                                Constraint::Length(3),
                            ])
                            .split(f.size())
                    }
                } else {
                    if self.preview_content.is_some() {
                        // Preview + Short: status(0), preview(1), list(2), short(3), help(4)
                        Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([
                                Constraint::Length(3),
                                Constraint::Length(10),
                                Constraint::Min(1),
                                Constraint::Length(3),
                                Constraint::Length(3),
                            ])
                            .split(f.size())
                    } else {
                        // Short only: status(0), list(1), short(2), help(3)
                        Layout::default()
                            .direction(Direction::Vertical)
                            .constraints([
                                Constraint::Length(3),
                                Constraint::Min(1),
                                Constraint::Length(3),
                                Constraint::Length(3),
                            ])
                            .split(f.size())
                    }
                };

                let area_name = self.current_area.as_ref().map(|a| a.clone()).unwrap_or_else(|| "None".to_string());
                let version = crate::version::VERSION;
                let mode_name = crate::agent::current_mode().unwrap_or_else(|_| "unknown".to_string());
                let platypus_status = if self.platypus_enabled { "" } else { " | Paddy: OFF" };
                let preview_status = if self.preview_content.is_some() { " | PREVIEW (press Enter to close)" } else { "" };

                let status_text = match &self.state.nav_state {
                    NavState::FindResults { paths, topic } => {
                        format!("UniSpec v{} | Mode: {} | Topic: {} | Links: {}{}{}", version, mode_name, topic, paths.len(), platypus_status, preview_status)
                    }
                    _ => {
                        format!("UniSpec v{} | Mode: {} | Area: {} | Topics: {}{}{}", version, mode_name, area_name, self.state.topics.len(), platypus_status, preview_status)
                    }
                };
                let status_widget = Paragraph::new(status_text).block(Block::default().borders(Borders::ALL));
                f.render_widget(status_widget, chunks[0]);

                if self.platypus_enabled {
                    let platypus = self.get_platypus_frame(self.animation_step);
                    let platypus_widget = Paragraph::new(platypus.trim());
                    f.render_widget(platypus_widget, chunks[1]);
                }

                match &self.state.nav_state {
                    NavState::AreaSelection => {
                        let areas = self.state.load_areas().unwrap_or_default();
                        let items: Vec<ListItem> = areas.iter().enumerate().map(|(i, a)| {
                            let style = if Some(i) == self.list_state.selected() { Style::default().fg(Color::Yellow) } else { Style::default() };
                            let topic_count = self.state.count_topics_in_area(a);
                            let count_str = if topic_count == 1 {
                                "1 topic".to_string()
                            } else {
                                format!("{} topics", topic_count)
                            };

                            // Check if area has work in progress (has topics with tasks)
                            let has_work = self.state.has_work_in_area(a);
                            let work_indicator = if has_work {
                                let spinner = match self.frame % 4 {
                                    0 => "⠋",
                                    1 => "⠙",
                                    2 => "⠹",
                                    _ => "⠸",
                                };
                                format!(" {}", spinner)
                            } else {
                                String::new()
                            };

                            ListItem::new(format!(">> {} ({}){}", a.clone(), count_str, work_indicator)).style(style)
                        }).collect();
                        let list = List::new(items).block(Block::default().title("Select Area").borders(Borders::ALL));
                        // New layout: status(0), platypus(1), preview(2), list(3), short(4), help(5) or similar
                        let (list_idx, short_idx, help_idx) = self.get_chunk_indices();
                        f.render_stateful_widget(list, chunks[list_idx], &mut self.list_state);
                        
                        // Render short description box
                        if let Some(ref short) = self.selected_short {
                            let short_widget = Paragraph::new(short.clone())
                                .block(Block::default().title("Short Description").borders(Borders::ALL));
                            f.render_widget(short_widget, chunks[short_idx]);
                        }
                    }
                    NavState::FindResults { topic, paths } => {
                        let items: Vec<ListItem> = if paths.is_empty() {
                            vec![ListItem::new("(no files linked - press n to add)".to_string()).style(Style::default().fg(Color::DarkGray))]
                        } else {
                            paths.iter().enumerate().map(|(i, p)| {
                                let style = if Some(i) == self.list_state.selected() { Style::default().fg(Color::Yellow) } else { Style::default() };
                                ListItem::new(p.clone()).style(style)
                            }).collect()
                        };
                        let title = format!("Files linked to: {}", topic);
                        let list = List::new(items).block(Block::default().title(title.as_str()).borders(Borders::ALL))
                            .highlight_symbol(">> ");
                        let (list_idx, short_idx, help_idx) = self.get_chunk_indices();
                        f.render_stateful_widget(list, chunks[list_idx], &mut self.list_state);
                        
                        // Render short description box
                        if let Some(ref short) = self.selected_short {
                            let short_widget = Paragraph::new(short.clone())
                                .block(Block::default().title("Short Description").borders(Borders::ALL));
                            f.render_widget(short_widget, chunks[short_idx]);
                        }
                    }
                    _ => {
                        let area_type = self.state.topics.first().map(|t| t.area_type.clone()).unwrap_or(crate::agent::mode::DisplayType::Standard);

                        let items: Vec<ListItem> = self.state.topics.iter().enumerate().map(|(i, t)| {
                            let style = if Some(i) == self.list_state.selected() { Style::default().fg(Color::Yellow) } else { Style::default() };

                            let change_prefix = if t.is_changes_container {
                                "📁🔀 "
                            } else if t.is_change {
                                "🔀 "
                            } else {
                                ""
                            };
                            let display_line = match t.area_type {
                                crate::agent::mode::DisplayType::Roadmap => {
                                    let impact = t.metadata.impact.as_deref().unwrap_or("");
                                    let change_type = t.metadata.change_type.as_deref().unwrap_or("");

                                    let impact_labels = crate::agent::mode::get_impact_labels("roadmap");
                                    let type_labels = crate::agent::mode::get_change_type_labels("roadmap");

                                    let impact_badge = impact_labels.get(&impact.to_lowercase())
                                        .cloned()
                                        .unwrap_or_else(|| format!("[{}]", impact.to_uppercase()));
                                    let type_badge = type_labels.get(&change_type.to_lowercase())
                                        .cloned()
                                        .unwrap_or_else(|| change_type.to_string());

                                    let short_str = if !t.short.is_empty() { format!(" │ {}", t.short) } else { String::new() };
                                    
                                    if impact.is_empty() && change_type.is_empty() {
                                        format!("{}{}", t.topic, short_str)
                                    } else if impact.is_empty() {
                                        format!("{} │ {}{}", t.topic, type_badge, short_str)
                                    } else if change_type.is_empty() {
                                        format!("{} │ {}{}", t.topic, impact_badge, short_str)
                                    } else {
                                        format!("{} │ {} │ {}{}", t.topic, type_badge, impact_badge, short_str)
                                    }
                                }
                                crate::agent::mode::DisplayType::Build => {
                                    let completed = t.metadata.completed.as_ref()
                                        .or(t.metadata.modified.as_ref())
                                        .map(|d| d.as_str())
                                        .unwrap_or("---");
                                    let current_agent = crate::commands::topic::get_agent_id();
                                    let checkout_info = if t.is_checked_out {
                                        if let Some(ref by) = t.checked_out_by {
                                            if by == &current_agent {
                                                " │ 🔒 checked out (you)".to_string()
                                            } else {
                                                format!(" │ 🔒 checked out by {}", by)
                                            }
                                        } else {
                                            String::new()
                                        }
                                    } else {
                                        String::new()
                                    };
                                    let short_str = if !t.short.is_empty() { format!(" │ {}", t.short) } else { String::new() };
                                    format!("✅ {} │ completed: {}{}{}", t.topic, completed, checkout_info, short_str)
                                }
                                crate::agent::mode::DisplayType::Specing => {
                                    let ready_status = if t.metadata.status.as_deref() == Some("ready") || t.tasks_completed > 0 {
                                        " (ready)".to_string()
                                    } else {
                                        "".to_string()
                                    };
                                    let short_str = if !t.short.is_empty() { format!(" │ {}", t.short) } else { String::new() };
                                    format!("📝 {}{}{}", t.topic, ready_status, short_str)
                                }
                                crate::agent::mode::DisplayType::Working | crate::agent::mode::DisplayType::Standard => {
                                    let progress = if t.tasks_total > 0 { (t.tasks_completed * 100 / t.tasks_total) as u16 } else { 0 };
                                    let bar_width = 20;
                                    let filled = (progress as usize * bar_width) / 100;
                                    let bar = format!("[{}{}]", "=".repeat(filled), " ".repeat(bar_width - filled));

                                    let status_icon = if t.status == "complete" { "✅" } else if t.tasks_in_progress > 0 { "🚧" } else { "⏳" };
                                    let animation = if t.tasks_in_progress > 0 {
                                        if self.frame % 4 < 1 { "⠋" } else { "⠙" }
                                    } else { "-" };

                                    let arrow = if t.status == "spec" { "" } else { " >" };
                                    let progress_str = if t.status == "spec" && t.tasks_total > 0 {
                                        format!(" ({}/{})", t.tasks_completed, t.tasks_total)
                                    } else {
                                        "".to_string()
                                    };
                                    let topic_display = format!("{:<25}", if t.topic.len() > 25 { format!("{}...", &t.topic[..22]) } else { t.topic.clone() });

                                    let current_agent = crate::commands::topic::get_agent_id();
                                    let checkout_info = if t.is_checked_out {
                                        if let Some(ref by) = t.checked_out_by {
                                            if by == &current_agent {
                                                " 🔒(you)".to_string()
                                            } else {
                                                format!(" 🔒(by {})", by)
                                            }
                                        } else {
                                            String::new()
                                        }
                                    } else {
                                        String::new()
                                    };

                                    format!("{} {} {} {} ({}/{}){}{}{}", topic_display, status_icon, bar, animation, t.tasks_completed, t.tasks_total, arrow, checkout_info, progress_str)
                                }
                            };

                            let display_line = if change_prefix.is_empty() {
                                display_line
                            } else {
                                format!("{}{}", change_prefix, display_line)
                            };

                            ListItem::new(display_line).style(style)
                        }).collect();
                        let list = List::new(items).block(Block::default().title("Specs").borders(Borders::ALL))
                            .highlight_symbol(">> ");
                        let (list_idx, short_idx, help_idx) = self.get_chunk_indices();
                        f.render_stateful_widget(list, chunks[list_idx], &mut self.list_state);
                        
                        // Render short description box
                        if let Some(ref short) = self.selected_short {
                            let short_widget = Paragraph::new(short.clone())
                                .block(Block::default().title("Short Description").borders(Borders::ALL));
                            f.render_widget(short_widget, chunks[short_idx]);
                        }
                    }
                }

                // Render preview if active
                if let Some(ref content) = self.preview_content {
                    if preview_chunk_idx < chunks.len() {
                        let preview_widget = Paragraph::new(content.clone())
                            .block(Block::default().title("Preview").borders(Borders::ALL));
                        f.render_widget(preview_widget, chunks[preview_chunk_idx]);
                    }
                }

                let help_text = if self.input_mode {
                    format!("{}: {}", self.input_prompt, self.input_buffer)
                } else {
                    let is_build = self.current_area
                        .as_ref()
                        .map(|a| a.to_lowercase() == "build")
                        .unwrap_or(false);
                    let move_cmd = if is_build { "Pull" } else { "Push" };
                    // In a TopicList view `q` queues the highlighted topic;
                    // in every other view it quits. Help text follows.
                    let q_action = match &self.state.nav_state {
                        NavState::TopicList(_) => "Queue",
                        _ => "Quit",
                    };
                    self.message.clone().unwrap_or_else(|| format!(" 🡙 Move | 🡘  Navigate | ↵ Open | n: New | r: Remove | p: {} | f: Find | q: {}", move_cmd, q_action))
                };
                let help_widget = Paragraph::new(help_text).block(Block::default().borders(Borders::ALL));
                let (_, _, help_idx) = self.get_chunk_indices();
                f.render_widget(help_widget, chunks[help_idx]);
            })?;

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind == KeyEventKind::Press {
                        self.handle_key_input(key.code);
                    }
                }
            }
        }

        Ok(())
    }

    fn setup_terminal(&self) -> Result<Terminal<CrosstermBackend<Stdout>>> {
        crossterm::terminal::enable_raw_mode()?;
        let stdout = io::stdout();
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;
        crossterm::execute!(
            terminal.backend_mut(),
            crossterm::terminal::EnterAlternateScreen
        )?;
        terminal.hide_cursor()?;
        Ok(terminal)
    }

    fn handle_key_input(&mut self, key: KeyCode) {
        if self.input_mode {
            match key {
                KeyCode::Enter => {
                    let input = self.input_buffer.clone();
                    self.pending_args.push(input.clone());
                    self.input_buffer.clear();

                    if self.input_prompt == "Create (t)opic or (s)pec?" {
                        if input.to_lowercase() == "t" {
                            self.input_prompt = "Name for new topic?".to_string();
                        } else if input.to_lowercase() == "s" {
                            self.input_prompt = "Name for new spec?".to_string();
                        } else {
                            self.input_mode = false;
                            self.pending_args.clear();
                        }
                    } else if self.input_prompt == "Name for new topic?" {
                        let topic_name = self.pending_args[0].clone();
                        if topic_name.is_empty() {
                            self.message = Some("❌ Topic name cannot be empty.".to_string());
                            self.input_mode = false;
                            self.pending_args.clear();
                        } else if topic_name.contains('/') {
                            self.message = Some(
                                "❌ Topic name cannot contain '/'. Use '-' instead.".to_string(),
                            );
                            self.input_mode = false;
                            self.pending_args.clear();
                        } else {
                            self.pending_topic_name = Some(topic_name);
                            self.input_prompt = "Short one-line description?".to_string();
                        }
                        self.last_refresh = Instant::now();
                    } else if self.input_prompt == "Short one-line description?" {
                        let short = self.pending_args.last().cloned().unwrap_or_default();
                        if short.len() < 5 {
                            self.message = Some(
                                "❌ Short description must be at least 5 characters.".to_string(),
                            );
                            self.message_timer = Some(Instant::now());
                            self.input_mode = false;
                            self.pending_args.clear();
                            self.pending_topic_name = None;
                            self.pending_short = None;
                        } else {
                            self.pending_short = Some(short);
                            self.input_prompt = "Impact (critical/high/medium/low)?".to_string();
                        }
                    } else if self.input_prompt == "Impact (critical/high/medium/low)?" {
                        let impact = self.pending_args.last().cloned().unwrap_or_default();
                        if ["critical", "high", "medium", "low"]
                            .contains(&impact.to_lowercase().as_str())
                        {
                            self.pending_impact = Some(impact.to_lowercase());
                            self.input_mode = true;
                            self.input_prompt =
                                "Type (feature/bugfix/refactor/docs/security)?".to_string();
                        } else {
                            self.message = Some(
                                "Invalid impact. Use: critical, high, medium, or low".to_string(),
                            );
                            self.message_timer = Some(Instant::now());
                            self.input_mode = false;
                            self.pending_args.clear();
                            self.pending_impact = None;
                        }
                    } else if self.input_prompt == "Type (feature/bugfix/refactor/docs/security)?" {
                        let change_type = self.pending_args.last().cloned().unwrap_or_default();
                        if [
                            "feature",
                            "bugfix",
                            "refactor",
                            "documentation",
                            "docs",
                            "security",
                        ]
                        .contains(&change_type.to_lowercase().as_str())
                        {
                            self.pending_change_type = Some(change_type.to_lowercase());

                            if let Some(area) = &self.current_area.clone() {
                                let topic_name = self.pending_args[0].clone();
                                let full_path = match &self.state.nav_state {
                                    NavState::TopicList(_) => topic_name.clone(),
                                    NavState::NestedSpecs(ref path) => {
                                        let spec_dir = crate::fs::spec_dir();
                                        if let Ok(rel) = path.strip_prefix(spec_dir.join(area)) {
                                            let parent = rel
                                                .to_string_lossy()
                                                .trim_start_matches('/')
                                                .to_string();
                                            format!("{}/{}", parent, topic_name)
                                        } else {
                                            topic_name.clone()
                                        }
                                    }
                                    _ => topic_name.clone(),
                                };

                                let short = self.pending_short.clone();
                                let impact = self.pending_impact.clone();

                                match crate::commands::topic::run_new_with_metadata(
                                    &full_path,
                                    area,
                                    short.as_deref(),
                                    None, // No content from TUI - uses template
                                ) {
                                    Ok(msg) => {
                                        self.message = Some(msg);
                                        self.trigger_expression(PlatypusState::Happy, 2);
                                    }
                                    Err(e) => self.message = Some(format!("Error: {}", e)),
                                }
                                self.state.update_status();
                                self.last_refresh = Instant::now();
                            }
                        } else {
                            self.message = Some(
                                "Invalid type. Use: feature, bugfix, refactor, docs, or security"
                                    .to_string(),
                            );
                            self.message_timer = Some(Instant::now());
                        }
                        self.input_mode = false;
                        self.pending_args.clear();
                        self.pending_impact = None;
                        self.pending_short = None;
                    } else if self.input_prompt == "Name for new area?" {
                        match crate::commands::area::run_add(&self.pending_args[0]) {
                            Ok(_) => {
                                self.message = Some("Success: Created area".to_string());
                                self.trigger_expression(PlatypusState::Happy, 2);
                            }
                            Err(e) => self.message = Some(format!("Error: {}", e)),
                        }
                        self.message_timer = Some(Instant::now());
                        self.input_mode = false;
                        self.pending_args.clear();
                    } else if self.input_prompt == "Target Area?" {
                        if let Some(topic) = self
                            .state
                            .topics
                            .get(self.list_state.selected().unwrap_or(0))
                        {
                            let source_area = self.current_area.as_deref();
                            match crate::commands::topic::run_push(
                                &topic.relative_path(),
                                &self.pending_args[0],
                                source_area,
                            ) {
                                Ok(msg) => {
                                    self.message = Some(msg);
                                    self.trigger_expression(PlatypusState::Love, 2);
                                }
                                Err(e) => self.message = Some(format!("Error: {}", e)),
                            }
                        }
                        self.message_timer = Some(Instant::now());
                        self.input_mode = false;
                        self.pending_args.clear();
                    } else if self.input_prompt.starts_with("Confirm remove") {
                        if self
                            .pending_args
                            .get(0)
                            .map(|s| s.to_lowercase() == "y")
                            .unwrap_or(false)
                        {
                            if self.input_prompt.contains("area") {
                                let area = self
                                    .current_area
                                    .as_ref()
                                    .map(|a| a.clone())
                                    .unwrap_or_else(|| {
                                        let areas = self.state.load_areas().unwrap_or_default();
                                        areas
                                            .get(self.list_state.selected().unwrap_or(0))
                                            .cloned()
                                            .unwrap_or_default()
                                    });
                                if !area.is_empty() {
                                    match crate::commands::area::run_remove(&area) {
                                        Ok(msg) => {
                                            self.message = Some(msg);
                                            self.trigger_expression(PlatypusState::Sad, 2);
                                            self.state.nav_state = NavState::AreaSelection;
                                            self.current_area = None;
                                            self.state.topics.clear();
                                            self.list_state.select(Some(0));
                                        }
                                        Err(e) => self.message = Some(format!("Error: {}", e)),
                                    }
                                }
                            } else {
                                if let Some(topic) = self
                                    .state
                                    .topics
                                    .get(self.list_state.selected().unwrap_or(0))
                                {
                                    match crate::commands::topic::run_delete(
                                        &topic
                                            .path
                                            .strip_prefix(crate::fs::spec_dir().join(
                                                self.current_area.as_deref().unwrap_or("Working"),
                                            ))
                                            .unwrap_or(&topic.path)
                                            .to_string_lossy(),
                                        self.current_area.as_deref().unwrap_or("Working"),
                                        true,
                                    ) {
                                        Ok(msg) => {
                                            self.message = Some(msg);
                                            self.trigger_expression(PlatypusState::Sad, 2);
                                            self.state.update_status();
                                        }
                                        Err(e) => self.message = Some(format!("Error: {}", e)),
                                    }
                                }
                            }
                        }
                        self.message_timer = Some(Instant::now());
                        self.input_mode = false;
                        self.pending_args.clear();
                    } else if self.input_prompt.starts_with("Remove link '")
                        && self.input_prompt.ends_with("'? (y/N)")
                    {
                        let confirm = self
                            .pending_args
                            .first()
                            .map(|s| s.to_lowercase() == "y")
                            .unwrap_or(false);
                        if confirm {
                            let path = self.pending_link_path.clone().unwrap_or_default();
                            let topic = self.pending_link_topic.clone().unwrap_or_default();
                            if !path.is_empty() && !topic.is_empty() {
                                match crate::fs::index::remove_link(&topic, &path) {
                                    Ok(_) => {
                                        self.find_paths.retain(|p| p != &path);
                                        if self.find_paths.is_empty() {
                                            self.state.nav_state = self
                                                .saved_nav_state
                                                .take()
                                                .unwrap_or(NavState::AreaSelection);
                                            self.state.topics = self.saved_topics.clone();
                                            self.message = Some(
                                                "All links removed, returned to topics".to_string(),
                                            );
                                        } else {
                                            self.state.nav_state = NavState::FindResults {
                                                topic: topic.clone(),
                                                paths: self.find_paths.clone(),
                                            };
                                            self.message = Some(format!("Removed: {}", path));
                                        }
                                        self.list_state.select(Some(0));
                                        self.trigger_expression(PlatypusState::Sad, 2);
                                    }
                                    Err(e) => self.message = Some(format!("Error: {}", e)),
                                }
                                self.message_timer = Some(Instant::now());
                            }
                        }
                        self.input_mode = false;
                        self.pending_args.clear();
                        self.pending_link_path = None;
                        self.pending_link_topic = None;
                    } else if self.input_prompt.starts_with("Path to add to") {
                        let topic_name =
                            if let NavState::FindResults { topic, .. } = &self.state.nav_state {
                                topic.clone()
                            } else if let Some(topic) = self
                                .state
                                .topics
                                .get(self.list_state.selected().unwrap_or(0))
                            {
                                topic.relative_path()
                            } else {
                                String::new()
                            };

                        if !topic_name.is_empty() {
                            let path = &self.pending_args[0];
                            let area = self.current_area.as_deref().unwrap_or("Working");
                            let link_type = crate::commands::index::detect_type(path);
                            match crate::fs::index::add_link(&topic_name, area, path, &link_type) {
                                Ok(_) => {
                                    self.message = Some(format!("Linked: {}", path));
                                    self.trigger_expression(PlatypusState::Happy, 2);
                                    if let Ok(links) = crate::fs::index::find_by_topic(&topic_name)
                                    {
                                        let paths: Vec<String> =
                                            links.iter().map(|l| l.path.clone()).collect();
                                        self.find_paths = paths.clone();
                                        self.state.nav_state = NavState::FindResults {
                                            topic: topic_name,
                                            paths,
                                        };
                                        self.list_state.select(Some(0));
                                    }
                                }
                                Err(e) => self.message = Some(format!("Error: {}", e)),
                            }
                            self.message_timer = Some(Instant::now());
                        }
                        self.input_mode = false;
                        self.pending_args.clear();
                    }
                }
                KeyCode::Char(c) => self.input_buffer.push(c),
                KeyCode::Backspace => {
                    self.input_buffer.pop();
                }
                KeyCode::Esc => {
                    self.input_mode = false;
                    self.input_buffer.clear();
                }
                _ => {}
            }
            return;
        }

        // Handle Enter key - open directories/files
        if key == KeyCode::Enter {
            match &self.state.nav_state {
                NavState::FindResults { .. } => {
                    if let Some(path) = self
                        .find_paths
                        .get(self.list_state.selected().unwrap_or(0))
                        .cloned()
                    {
                        self.open_file(&path);
                    }
                }
                NavState::AreaSelection => {
                    let areas = self.state.load_areas().unwrap_or_default();
                    let selected = self.list_state.selected().unwrap_or(0);
                    if let Some(area_name) = areas.get(selected) {
                        let area_path = crate::fs::spec_dir().join(area_name);
                        self.open_file(&area_path.to_string_lossy());
                    }
                }
                _ => {
                    if let Some(topic) = self
                        .state
                        .topics
                        .get(self.list_state.selected().unwrap_or(0))
                        .cloned()
                    {
                        self.open_file(&topic.path.to_string_lossy());
                    }
                }
            }
            return;
        }

        match key {
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                // In a TopicList view `q` adds the highlighted topic to the
                // area's readiness queue. In any other nav state `q` keeps
                // its long-standing "quit the TUI" meaning.
                if let NavState::TopicList(_) = &self.state.nav_state {
                    self.queue_selected_topic();
                } else {
                    self.should_exit = true;
                }
            }
            KeyCode::Down => {
                let len = match &self.state.nav_state {
                    NavState::AreaSelection => self.state.load_areas().unwrap_or_default().len(),
                    NavState::FindResults { .. } => self.find_paths.len(),
                    _ => self.state.topics.len(),
                };
                let i = self.list_state.selected().unwrap_or(0);
                if i < len.saturating_sub(1) {
                    self.list_state.select(Some(i + 1));
                    self.update_selected_short();
                }
            }
            KeyCode::Up => {
                let i = self.list_state.selected().unwrap_or(0);
                if i > 0 {
                    self.list_state.select(Some(i - 1));
                    self.update_selected_short();
                }
            }
            KeyCode::Enter => {
                self.handle_enter_key();
            }
            KeyCode::Left => {
                if let NavState::FindResults { .. } = &self.state.nav_state {
                    self.state.nav_state = self
                        .saved_nav_state
                        .take()
                        .unwrap_or(NavState::AreaSelection);
                    self.state.topics = self.saved_topics.clone();
                    self.find_paths.clear();
                    self.list_state.select(Some(0));
                    self.update_selected_short();
                } else if let Some((prev_state, prev_topics)) = self.history.pop() {
                    self.state.nav_state = prev_state;
                    self.state.topics = prev_topics;
                    self.list_state.select(Some(0));
                    self.update_selected_short();
                } else if let NavState::TopicList(_) | NavState::NestedSpecs(_) =
                    self.state.nav_state
                {
                    self.state.nav_state = NavState::AreaSelection;
                    self.state.topics = vec![];
                    self.current_area = None;
                    self.list_state.select(Some(0));
                    self.update_selected_short();
                }
            }
            KeyCode::Right => match &self.state.nav_state {
                NavState::AreaSelection => {
                    let areas = self.state.load_areas().unwrap_or_default();
                    let selected = self.list_state.selected().unwrap_or(0);
                    if let Some(area_name) = areas.get(selected) {
                        let _ = self.state.load_topics_for_area(area_name);
                        self.current_area = Some(area_name.clone());
                        self.list_state.select(Some(0));
                        self.update_selected_short();
                    }
                }
                NavState::TopicList(_) | NavState::NestedSpecs(_) => {
                    if let Some(topic) = self
                        .state
                        .topics
                        .get(self.list_state.selected().unwrap_or(0))
                        .cloned()
                    {
                        if topic.status != "spec" {
                            self.history
                                .push((self.state.nav_state.clone(), self.state.topics.clone()));
                            self.state.nav_state = NavState::NestedSpecs(topic.path.clone());
                            self.state.topics = topic.children;
                            self.list_state.select(Some(0));
                            self.update_selected_short();
                        }
                    }
                }
                _ => {}
            },
            KeyCode::Char('n') => {
                self.input_mode = true;
                match &self.state.nav_state {
                    NavState::AreaSelection => {
                        self.input_prompt = "Name for new area?".to_string();
                    }
                    NavState::FindResults { topic, .. } => {
                        self.input_prompt = format!("Path to add to {}?", topic);
                    }
                    _ => {
                        self.input_prompt = "Create (t)opic or (s)pec?".to_string();
                    }
                }
            }
            KeyCode::Char('r') => match &self.state.nav_state {
                NavState::AreaSelection => {
                    self.input_mode = true;
                    self.input_prompt = "Confirm remove area? (y/N)".to_string();
                }
                NavState::FindResults { topic, paths } => {
                    if let Some(path) = paths.get(self.list_state.selected().unwrap_or(0)).cloned()
                    {
                        self.input_mode = true;
                        self.input_prompt = format!("Remove link '{}'? (y/N)", path);
                        self.pending_link_path = Some(path);
                        self.pending_link_topic = Some(topic.clone());
                    }
                }
                _ => {
                    self.input_mode = true;
                    self.input_prompt = "Confirm remove item? (y/N)".to_string();
                }
            },
            KeyCode::Char('p') => match &self.state.nav_state {
                NavState::AreaSelection => {
                    self.message = Some("Action not allowed: Select a topic first".to_string());
                    self.message_timer = Some(Instant::now());
                }
                NavState::FindResults { .. } => {
                    self.message = Some("Action not allowed in links view".to_string());
                    self.message_timer = Some(Instant::now());
                }
                _ => {
                    let is_build = self
                        .current_area
                        .as_ref()
                        .map(|a| a.to_lowercase() == "build")
                        .unwrap_or(false);

                    if let Some(topic) = self
                        .state
                        .topics
                        .get(self.list_state.selected().unwrap_or(0))
                    {
                        if is_build {
                            // In Build area: pull to Working (checkout)
                            let topic_name = topic.relative_path();
                            match crate::commands::topic::run_pull(&topic_name, "Build") {
                                Ok(msg) => {
                                    self.message = Some(msg);
                                    self.trigger_expression(PlatypusState::Working, 2);
                                    self.state.update_status();
                                    self.last_refresh = Instant::now();
                                }
                                Err(e) => {
                                    self.message = Some(format!("Error: {}", e));
                                    self.message_timer = Some(Instant::now());
                                }
                            }
                        } else {
                            // In other areas: push to target area
                            if topic.is_checked_out {
                                let by = topic.checked_out_by.as_deref().unwrap_or("unknown");
                                self.message =
                                    Some(format!("Cannot push: Topic is checked out by {}", by));
                                self.message_timer = Some(Instant::now());
                                return;
                            }
                            self.input_mode = true;
                            self.input_prompt = "Target Area?".to_string();
                        }
                    }
                }
            },
            KeyCode::Char('f') => {
                if let NavState::AreaSelection = &self.state.nav_state {
                    self.message = Some("Action not allowed: Select a topic first".to_string());
                    self.message_timer = Some(Instant::now());
                } else if let Some(topic) = self
                    .state
                    .topics
                    .get(self.list_state.selected().unwrap_or(0))
                {
                    let topic_name = topic.relative_path();
                    self.trigger_expression(PlatypusState::Searching, 3);
                    match crate::fs::index::find_by_topic(&topic_name) {
                        Ok(links) => {
                            let paths: Vec<String> = links.iter().map(|l| l.path.clone()).collect();
                            self.saved_topics = self.state.topics.clone();
                            self.saved_nav_state = Some(self.state.nav_state.clone());
                            self.find_paths = paths.clone();
                            self.state.nav_state = NavState::FindResults {
                                topic: topic_name,
                                paths,
                            };
                            self.list_state.select(Some(0));
                            if links.is_empty() {
                                self.message =
                                    Some(format!("No files linked. Press n to add a link."));
                            }
                        }
                        Err(e) => {
                            self.message = Some(format!("Error: {}", e));
                            self.message_timer = Some(Instant::now());
                        }
                    }
                }
            }
            KeyCode::Char('\\') => {
                self.platypus_enabled = !self.platypus_enabled;
                if let Err(e) = crate::fs::config::set_paddy_enabled(self.platypus_enabled) {
                    self.message = Some(format!("Failed to save: {}", e));
                } else if self.platypus_enabled {
                    self.message = Some("Platypus enabled!".to_string());
                    self.trigger_expression(crate::tui::app::PlatypusState::Happy, 2);
                } else {
                    self.message = Some("Platypus disabled.".to_string());
                    self.trigger_expression(crate::tui::app::PlatypusState::Sad, 2);
                }
                self.message_timer = Some(Instant::now());
            }
            _ => {}
        }
    }

    /// Add the currently highlighted topic to the current area's readiness
    /// queue. Wired to `q`/`Q` in `NavState::TopicList`. Surfaces the result
    /// through the existing `self.message` channel so the help row at the
    /// bottom briefly shows the outcome.
    fn queue_selected_topic(&mut self) {
        let selected = self.list_state.selected().unwrap_or(0);
        let topic = match self.state.topics.get(selected) {
            Some(t) => t.topic.clone(),
            None => {
                self.message = Some("No topic highlighted to queue.".to_string());
                self.message_timer = Some(Instant::now());
                return;
            }
        };
        let area = match self.current_area.as_ref() {
            Some(a) => a.clone(),
            None => {
                self.message = Some("Not inside an area; nothing to queue.".to_string());
                self.message_timer = Some(Instant::now());
                return;
            }
        };
        match crate::commands::queue::run_queue_add(&topic, &area, -1) {
            Ok(out) => {
                self.message = Some(format!(
                    "✅ Added '{}' to {}/{}",
                    out.topic, out.area, out.queue_file
                ));
                self.trigger_expression(PlatypusState::Happy, 2);
            }
            Err(e) => {
                self.message = Some(format!("❌ Queue add failed: {}", e));
                self.trigger_expression(PlatypusState::Sad, 2);
            }
        }
        self.message_timer = Some(Instant::now());
    }

    fn open_file(&mut self, path: &str) {
        let abs_path = std::path::Path::new(path);
        let final_path = if abs_path.is_absolute() {
            path.to_string()
        } else {
            std::env::current_dir()
                .map(|cwd| cwd.join(path))
                .map(|p| p.to_string_lossy().to_string())
                .unwrap_or_else(|_| path.to_string())
        };

        // Pick an editor: $EDITOR → nano → vi → fall through to print contents.
        let editor_cmd: Option<String> = std::env::var("EDITOR")
            .ok()
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .or_else(|| binary_on_path("nano").then(|| "nano".to_string()))
            .or_else(|| binary_on_path("vi").then(|| "vi".to_string()));

        // Suspend the TUI so the editor (or our content dump) has the terminal.
        let _ = crossterm::execute!(io::stdout(), crossterm::terminal::LeaveAlternateScreen);
        let _ = crossterm::terminal::disable_raw_mode();

        let outcome: Result<(), String> = if let Some(cmd) = editor_cmd {
            // $EDITOR may contain flags (e.g. `vim -O`); split on whitespace.
            let mut parts = cmd.split_whitespace();
            let bin = parts.next().unwrap_or("");
            let extra_args: Vec<&str> = parts.collect();
            match std::process::Command::new(bin)
                .args(&extra_args)
                .arg(&final_path)
                .status()
            {
                Ok(_) => Ok(()),
                Err(e) => Err(format!("Failed to launch editor '{}': {}", bin, e)),
            }
        } else {
            // No editor on PATH — dump contents and wait for the user to press Enter.
            match std::fs::read_to_string(&final_path) {
                Ok(content) => {
                    println!("--- {} ---", final_path);
                    print!("{}", content);
                    if !content.ends_with('\n') {
                        println!();
                    }
                    println!("--- end of file (press Enter to return) ---");
                    let mut line = String::new();
                    let _ = std::io::stdin().read_line(&mut line);
                    Ok(())
                }
                Err(e) => Err(format!("Could not read {}: {}", final_path, e)),
            }
        };

        // Restore the TUI. Order matches the original setup: raw mode, then
        // alt screen. Entering the alt screen clears it, but ratatui's
        // internal buffer still reflects what it drew *before* we suspended,
        // so a plain `terminal.draw` on the next loop iteration would diff
        // against that stale buffer and skip repainting cells the editor
        // visibly clobbered. Two-part fix: (1) issue an explicit terminal
        // clear + cursor reset via crossterm so the visible screen is blank
        // immediately, and (2) flag the run loop to call `terminal.clear()`
        // before the next `draw`, which invalidates ratatui's buffer and
        // forces a full repaint.
        let _ = crossterm::terminal::enable_raw_mode();
        let _ = crossterm::execute!(io::stdout(), crossterm::terminal::EnterAlternateScreen);
        let _ = crossterm::execute!(
            io::stdout(),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All),
            crossterm::cursor::MoveTo(0, 0)
        );
        self.needs_full_redraw = true;

        if let Err(msg) = outcome {
            self.message = Some(msg);
            self.message_timer = Some(Instant::now());
        }
    }
}

/// Return true if `name` resolves to a file somewhere on the user's `PATH`.
/// Portable substitute for `which` that needs no new dependency.
fn binary_on_path(name: &str) -> bool {
    let Some(path_var) = std::env::var_os("PATH") else {
        return false;
    };
    for dir in std::env::split_paths(&path_var) {
        if dir.join(name).is_file() {
            return true;
        }
    }
    false
}

impl Drop for App {
    fn drop(&mut self) {
        let _ = crossterm::execute!(io::stdout(), crossterm::terminal::LeaveAlternateScreen);
        let _ = crossterm::terminal::disable_raw_mode();
    }
}
