// src/cli/mod.rs
pub mod model;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "unispec")]
#[command(
    about = "unispec",
    long_about = "UniSpec - Spec-Driven Development

A TUI-based tool for managing specs and topics across different areas.

Run 'unispec' without arguments to launch the interactive TUI.

QUICK START:
  unispec init              Launch the TUI
  unispec init              Initialize a new project (creates spec/ and .agent/ folders)                  Launch the TUI
  unispec topic add <name>  Add a new topic to the current area
  unispec area list        List all areas

TOPICS:
  unispec topic add <topic> [-a <area>]      Create a new topic
  unispec topic list [-a <area>]             List topics in an area
  unispec topic push <topic> <area>          Push topic to another area
  unispec topic pull <topic> <area>          Pull topic from another area
  unispec topic remove <topic>                Remove a topic
  unispec topic show <topic>                 Show topic details
  unispec topic progress [-a <area>]         Show progress across topics

AREAS:
  unispec area add <name>                    Add a new area
  unispec area remove <name>                 Remove an area
  unispec area list                          List all areas
  unispec area rename <old> <new>            Rename an area
  unispec area default <name>                Set default area
  unispec area health                        Show area health stats

INDEX:
  unispec index add --topic <name> --path <path>    Link topic to path
  unispec index remove --topic <name> --path <path> Remove link
  unispec index list [--topic <name>] [--path <path>]  List links
  unispec index find <query> --by topic|path        Find links

AGENT MODES:
  unispec mode list               List available agent modes
  unispec mode activate <name>    Activate an agent mode
  unispec mode info <name>        Show mode details
  unispec mode add <path>         Add a mode from a directory
  unispec mode remove <name>      Remove a mode
  unispec mode current            Show current mode

AGENT CONNECTORS:
  unispec connector new <name> <description> <command>  Create a connector
  unispec connector list          List all connectors
  unispec connector run <name>    Run a connector
  unispec connector delete <name> Delete a connector
  unispec connector mcp           Generate MCP config for connectors

OTHER:
  unispec set <area>               Set default area
  unispec init [-r <path>]         Initialize project
  unispec mcp                      Launch MCP server for agent integration
  unispec pkg list                 List available packages
  unispec pkg install <pkg>       Install a package
  unispec pkg remove <pkg>        Remove a package

INDEX:
  unispec index add --topic X --path Y [--exports a,b]  Add link with exports
  unispec index exports --topic X   List exports for topic
  unispec index query <q> --by name   Query exports by name/type/description
  unispec index depends --topic X    Show what depends on topic
  unispec index lookup --id X         Lookup export by ID
  unispec index backlinks --topic X  Show backlinks
 "
)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Cli {
    #[arg(short, long)]
    pub quiet: bool,
    #[arg(short, long)]
    pub verbose: bool,
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new UniSpec project
    Init {
        /// Root directory to initialize (default: current directory)
        #[arg(short, long)]
        root: Option<PathBuf>,
        /// Add Amazon Q Developer integration (.amazonq/prompts)
        #[arg(long)]
        amazon_q: bool,
        /// Add Antigravity integration (.agent/workflows)
        #[arg(long)]
        antigravity: bool,
        /// Add Augment CLI integration (.augment/commands)
        #[arg(long)]
        auggie: bool,
        /// Add Claude Code integration (.claude/commands/unispec)
        #[arg(long)]
        claude_code: bool,
        /// Add Cline integration (.clinerules/workflows)
        #[arg(long)]
        cline: bool,
        /// Add Codex integration (~/.codex/prompts)
        #[arg(long)]
        codex: bool,
        /// Add CodeBuddy integration (.codebuddy/commands/unispec)
        #[arg(long)]
        codebuddy: bool,
        /// Add Continue integration (.continue/prompts)
        #[arg(long)]
        continue_: bool,
        /// Add CoStrict integration (.cospec/unispec/commands)
        #[arg(long)]
        costrict: bool,
        /// Add Crush integration (.crush/commands/unispec)
        #[arg(long)]
        crush: bool,
        /// Add Cursor integration (.cursor/commands)
        #[arg(long)]
        cursor: bool,
        /// Add Factory Droid integration (.factory/commands)
        #[arg(long)]
        factory: bool,
        /// Add Gemini CLI integration (.gemini/commands/unispec)
        #[arg(long)]
        gemini_cli: bool,
        /// Add GitHub integration (.github/prompts)
        #[arg(long)]
        github: bool,
        /// Add iFlow integration (.iflow/commands)
        #[arg(long)]
        iflow: bool,
        /// Add Kilo Code integration (.kilocode/workflows)
        #[arg(long)]
        kilo_code: bool,
        /// Add Kiro integration (.kiro/prompts)
        #[arg(long)]
        kiro: bool,
        /// Add OpenCode integration (.opencode/command)
        #[arg(long)]
        opencode: bool,
        /// Add Pi integration (.pi/prompts)
        #[arg(long)]
        pi: bool,
        /// Add Qoder integration (.qoder/commands/unispec)
        #[arg(long)]
        qoder: bool,
        /// Add Qwen Code integration (.qwen/commands)
        #[arg(long)]
        qwen_code: bool,
        /// Add RooCode integration (.roo/commands)
        #[arg(long)]
        roo_code: bool,
        /// Add Windsurf integration (.windsurf/workflows)
        #[arg(long)]
        windsurf: bool,
        /// Add TRAE integration (.trae/rule)
        #[arg(long)]
        trae: bool,
    },
    /// Set the default area
    Set { area: String },
    /// Manage areas (add, remove, list, rename, default, health)
    #[command(subcommand)]
    Area(AreaCommands),
    /// Manage topics (add, list, push, pull, remove, show, progress)
    #[command(subcommand)]
    Topic(TopicCommands),
    /// Index commands (full, watch)
    #[command(subcommand)]
    Index(IndexCommands),
    /// Launch MCP server for agent integration
    Mcp {
        /// Project directory to run MCP server for
        path: Option<PathBuf>,
    },
    /// Spec commands (show, add)
    #[command(subcommand)]
    Spec(SpecCommands),
    /// Readiness queue commands (add)
    #[command(subcommand)]
    Queue(QueueCommands),
    /// Change management commands (add, list, archive)
    #[command(subcommand)]
    Change(ChangeCommands),
    /// Get a structured next-action payload for a topic
    Next {
        /// Topic name (required)
        #[arg(short, long)]
        topic: String,
        /// Area the topic lives in. Defaults to the `area` field in
        /// `.agent/config.toml`, or "Staging" if no config exists.
        #[arg(short, long)]
        area: Option<String>,
        /// Emit the full payload as JSON instead of a human summary
        #[arg(long)]
        json: bool,
    },
    /// Agent mode commands
    #[command(subcommand)]
    Mode(ModeCommands),
    /// Agent connector commands
    #[command(subcommand)]
    Connector(ConnectorCommands),
    /// Manage packages (list, install, remove)
    #[command(subcommand)]
    Pkg(PkgCommands),
    /// Ingest a codebase and create specs from it
    #[command(subcommand)]
    Ingest(IngestCommands),
    /// Parse a single file with tree-sitter (auto-detects language)
    #[command(subcommand)]
    Parse(ParseCommands),
    /// Control platypus mascot display
    #[command(subcommand)]
    Patty(PattyCommands),
    /// Auto-orchestration commands (build, verify, test, agent)
    #[command(subcommand)]
    Auto(AutoCommands),
}

#[derive(Subcommand)]
pub enum AutoCommands {
    /// Build a topic (auto-discovers root topic with queue.md if not provided)
    Build {
        /// Topic to build (optional - will auto-discover root topic with queue.md)
        topic: Option<String>,
        /// Area to build in
        #[arg(short, long)]
        area: Option<String>,
        /// Spec file to use
        #[arg(short, long)]
        spec_file: Option<String>,
    },
    /// Verify a topic
    Verify {
        /// Topic to verify
        topic: String,
        /// Area to verify in
        #[arg(short, long)]
        area: Option<String>,
    },
    /// Run tests for a topic
    Test {
        /// Topic to test
        #[arg(short, long)]
        topic: Option<String>,
        /// Pre-build script
        #[arg(long)]
        pre_script: Option<String>,
        /// Post-build script
        #[arg(long)]
        post_script: Option<String>,
    },
    /// Run an agent session
    Agent {
        /// Topic to run agent for
        topic: String,
        /// Session ID
        #[arg(short, long)]
        session_id: Option<String>,
        /// Parent topic
        #[arg(short, long)]
        parent_topic: Option<String>,
        /// Area
        #[arg(short, long)]
        area: Option<String>,
        /// Workflow
        #[arg(short, long)]
        workflow: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum ModeCommands {
    /// List available modes
    List,
    /// Show detailed info about a mode
    Info {
        /// Name of the mode
        name: String,
    },
    /// Activate a mode
    Activate {
        /// Name of the mode to activate
        name: String,
    },
    /// Add a mode from a directory (copies to local or global modes)
    Add {
        /// Path to the mode directory
        path: String,
        /// Add to global modes instead of local
        #[arg(short, long)]
        global: bool,
    },
    /// Remove a mode
    Remove {
        /// Name of the mode to remove
        name: String,
        /// Remove from global modes instead of local
        #[arg(short, long)]
        global: bool,
    },
    /// Show current mode
    Current,
}

#[derive(Subcommand)]
pub enum ConnectorCommands {
    /// Create a new connector
    New {
        /// Name of the connector
        name: String,
        /// Description of what it does
        description: String,
        /// Command to run
        command: String,
        /// Arguments (space-separated)
        #[arg(short, long)]
        args: Option<String>,
        /// Environment variables (KEY=value format, comma-separated)
        #[arg(short, long)]
        env: Option<String>,
        /// Working directory
        #[arg(short, long)]
        working_dir: Option<String>,
        /// Timeout in seconds
        #[arg(short, long)]
        timeout: Option<u64>,
    },
    /// List all connectors
    List,
    /// Delete a connector
    Delete {
        /// Name of the connector to delete
        name: String,
    },
    /// Run a connector
    Run {
        /// Name of the connector to run
        name: String,
        /// Additional arguments
        #[arg(last = true)]
        args: Vec<String>,
    },
    /// Edit a connector
    Edit {
        /// Name of the connector
        name: String,
        /// New description
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Generate MCP server configuration for connectors
    Mcp,
}

#[derive(Subcommand)]
pub enum AreaCommands {
    /// Add a new area
    Add {
        /// Name of the area to add
        name: String,
    },
    /// Remove an area
    Remove {
        /// Name of the area to remove
        name: String,
    },
    /// List all areas
    List,
    /// Rename an area
    Rename {
        /// Current name of the area
        old: String,
        /// New name for the area
        new: String,
    },
    /// Set the default area
    Default {
        /// Name of the area to set as default
        name: String,
    },
    /// Show area health (topic counts by status)
    Health,
    /// Manage area order in the TUI
    Order {
        /// Subcommand for order management
        #[command(subcommand)]
        action: AreaOrderCommands,
    },
}

#[derive(Subcommand)]
pub enum AreaOrderCommands {
    /// Show current area order
    Show,
    /// Add areas to the order
    Add {
        /// Area names to add
        areas: Vec<String>,
        /// Position to insert at (0-based index, default: end)
        #[arg(short, long)]
        position: Option<usize>,
    },
    /// Remove areas from the order
    Remove {
        /// Area names to remove
        areas: Vec<String>,
    },
    /// Reset order to default (no custom order)
    Reset,
}

#[derive(Subcommand)]
pub enum QueueCommands {
    /// Add a topic to an area's readiness queue (`spec/<area>/queue.md`).
    /// Required before pushing out of areas listed under `[readiness].areas`
    /// in `mode.toml` (default: Staging, Fixing).
    Add {
        /// Topic name to enqueue
        topic: String,
        /// Area whose queue to add to. Defaults to the `area` field in
        /// `.agent/config.toml`, or "Staging" if no config exists.
        #[arg(short, long)]
        area: Option<String>,
        /// 0-based position in the queue. Negative or out-of-range = append
        /// to the end (default).
        #[arg(short, long, default_value_t = -1)]
        position: i32,
    },
}

#[derive(Subcommand)]
pub enum ChangeCommands {
    /// Create a new change folder inside a topic. The original spec is left
    /// untouched; a new `changes/<change>/` directory is written with
    /// proposal.md, an optional design.md, and dedicated spec/task files.
    Add {
        /// Topic the change applies to
        #[arg(short, long)]
        topic: String,
        /// Change identifier (e.g. "add-2fa")
        #[arg(short, long)]
        change: String,
        /// Area the topic lives in. Defaults to the `area` field in
        /// `.agent/config.toml`, or "Staging" if no config exists.
        #[arg(short, long)]
        area: Option<String>,
        /// Proposal body (why this change exists). Required, ≥ 11 chars.
        #[arg(long, allow_hyphen_values = true)]
        proposal: String,
        /// Optional technical design body.
        #[arg(long, allow_hyphen_values = true)]
        design: Option<String>,
        /// Spec body for the new requirements introduced by this change.
        /// Required, ≥ 11 chars.
        #[arg(long, allow_hyphen_values = true)]
        spec_content: String,
        /// Task body for the new tasks introduced by this change.
        /// Required, ≥ 11 chars.
        #[arg(long, allow_hyphen_values = true)]
        task_content: String,
    },
    /// List all changes for a topic.
    List {
        /// Topic to list changes for
        #[arg(short, long)]
        topic: String,
        /// Area the topic lives in (default from config or Staging)
        #[arg(short, long)]
        area: Option<String>,
        /// Include archived changes
        #[arg(long)]
        archived: bool,
    },
    /// Archive a change (mark it complete and move into changes/archive/).
    Archive {
        /// Topic the change lives under
        #[arg(short, long)]
        topic: String,
        /// Change name to archive
        #[arg(short, long)]
        change: String,
        /// Area the topic lives in (default from config or Staging)
        #[arg(short, long)]
        area: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum SpecCommands {
    /// Show the master spec (`spec/master.md`)
    Show {
        /// Spec name (default: master)
        #[arg(default_value = "master")]
        name: Option<String>,
    },
    /// Create a spec + task file for a topic
    Add {
        /// Topic name (use `parent/child` for nested topics)
        #[arg(short, long)]
        topic: String,
        /// Area for the topic. Defaults to the `area` field in
        /// `.agent/config.toml`, or "Staging" if no config exists.
        #[arg(short, long)]
        area: Option<String>,
        /// One-line description (required)
        #[arg(short, long)]
        short: String,
        /// Full spec body (matches the spec template). Required, ≥ 11 chars.
        /// `allow_hyphen_values` lets the value contain lines starting with `- `
        /// (markdown bullets) without clap treating them as new flags.
        #[arg(long, allow_hyphen_values = true)]
        spec_content: String,
        /// Full task body (matches the task template). Required, ≥ 11 chars.
        /// `allow_hyphen_values` lets the value contain markdown bullets.
        #[arg(long, allow_hyphen_values = true)]
        task_content: String,
    },
}

#[derive(Subcommand)]
pub enum TopicCommands {
    /// Add a new topic
    Add {
        /// Name of the topic to create
        topic: String,
        /// Area to create the topic in. Defaults to the `area` field in
        /// `.agent/config.toml`, or "Staging" if no config exists.
        #[arg(short, long)]
        area: Option<String>,
        /// Short one-line description (required)
        #[arg(short, long)]
        short: String,
        /// Topic content/markdown (required, use - to read from stdin)
        #[arg(short, long, default_value = "")]
        content: String,
    },
    /// List topics in an area
    List {
        /// Area to list topics from. Defaults to the `area` field in
        /// `.agent/config.toml`, or "Staging" if no config exists.
        #[arg(short = 'a', long)]
        area: Option<String>,
        /// Show hierarchical view
        #[arg(short = 'H', long)]
        hierarchy: bool,
    },
    /// Push a topic to another area
    Push {
        /// Name of the topic to push
        topic: String,
        /// Target area to push to. Defaults to the `area` field in
        /// `.agent/config.toml`, or "Staging" if no config exists.
        #[arg(short, long)]
        area: Option<String>,
        /// Source area to push from. Defaults to the `area` field in
        /// `.agent/config.toml`, or "Staging" if no config exists.
        #[arg(short, long)]
        from: Option<String>,
    },
    /// Pull a topic from another area
    Pull {
        /// Name of the topic to pull
        topic: String,
        /// Source area to pull from. Defaults to the `area` field in
        /// `.agent/config.toml`, or "Staging" if no config exists.
        #[arg(short, long)]
        area: Option<String>,
    },
    /// Remove a topic
    Remove {
        /// Name of the topic to remove
        topic: String,
        /// Area the topic lives in. Defaults to the `area` field in
        /// `.agent/config.toml`, or "Staging" if no config exists.
        #[arg(short, long)]
        area: Option<String>,
        /// Force removal without confirmation
        #[arg(short, long)]
        force: bool,
    },
    /// Show topic details
    Show {
        /// Name of the topic to show
        topic: String,
        /// Show all files in the topic (not just current area's files)
        #[arg(short, long)]
        all: bool,
        /// Show files from a specific area (useful when topic has files from multiple areas)
        #[arg(short, long)]
        from: Option<String>,
    },
    /// Show progress across all topics
    Progress {
        /// Area to show progress for (default: current area)
        #[arg(short, long)]
        area: Option<String>,
    },
    /// Manage topic order in an area
    Order {
        /// Subcommand for order management
        #[command(subcommand)]
        action: OrderCommands,
    },
}

#[derive(Subcommand)]
pub enum OrderCommands {
    /// Show current order
    Show {
        /// Area to show order for (default: current area)
        #[arg(short, long)]
        area: Option<String>,
    },
    /// Add topics to the order
    Add {
        /// Area to add topics to (default: current area)
        #[arg(short, long)]
        area: Option<String>,
        /// Topic names to add
        topics: Vec<String>,
        /// Position to insert at (0-based index, default: end)
        #[arg(short, long)]
        position: Option<usize>,
    },
    /// Remove topics from the order
    Remove {
        /// Area to remove topics from (default: current area)
        #[arg(short, long)]
        area: Option<String>,
        /// Topic names to remove
        topics: Vec<String>,
    },
    /// Reset order to alphabetical
    Reset {
        /// Area to reset (default: current area)
        #[arg(short, long)]
        area: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum IndexCommands {
    /// Add a link between a topic and a path
    Add {
        /// Topic name
        #[arg(short, long)]
        topic: String,
        /// Path to link (file or directory)
        #[arg(short, long)]
        path: String,
        /// Area name (auto-detected from topic if not specified)
        #[arg(short, long)]
        area: Option<String>,
        /// Type: 'file' or 'directory' (auto-detected if not specified)
        #[arg(short, long)]
        link_type: Option<String>,
        /// Tags (comma-separated)
        #[arg(short, long)]
        tags: Option<String>,
        /// Annotation/note about this link
        #[arg(short, long)]
        annotation: Option<String>,
        /// Exports (comma-separated names)
        #[arg(long)]
        exports: Option<String>,
        /// Export descriptions (comma-separated, matching exports order)
        #[arg(long)]
        descriptions: Option<String>,
        /// Export types (comma-separated: function,class,endpoint,model,service,config)
        #[arg(long)]
        export_types: Option<String>,
        /// Function signatures (semicolon-separated)
        #[arg(long)]
        signatures: Option<String>,
    },
    /// Remove a link between a topic and a path
    Remove {
        /// Topic name
        #[arg(short, long)]
        topic: String,
        /// Path to unlink
        #[arg(short, long)]
        path: String,
    },
    /// List all links (optionally filtered by topic or path)
    List {
        /// Filter by topic name
        #[arg(short, long)]
        topic: Option<String>,
        /// Filter by path
        #[arg(short, long)]
        path: Option<String>,
        /// Filter by tag
        #[arg(long)]
        tag: Option<String>,
    },
    /// Find links by topic, path, or tag
    Find {
        /// Query (topic name, path, or tag)
        query: String,
        /// Search by: 'topic', 'path', or 'tag'
        #[arg(short, long, default_value = "topic")]
        by: String,
    },
    /// Show index stats
    Full,
    /// Start watcher
    Watch,
    /// Clean up orphaned links (topics/paths that no longer exist)
    Cleanup,
    /// List all unique tags in the index
    Tags,
    /// Generate graph.json for visualization
    Graph,
    /// Generate backlinks file for a topic
    Backlinks {
        /// Topic name
        #[arg(short, long)]
        topic: String,
    },
    /// List exports for a topic
    Exports {
        /// Topic name
        #[arg(short, long)]
        topic: Option<String>,
    },
    /// Query exports by name, type, description, or ID
    Query {
        /// Search query
        query: String,
        /// Search by: name, type, description, or id
        #[arg(short, long, default_value = "name")]
        by: String,
    },
    /// Find what depends on a topic (reverse lookups)
    Depends {
        /// Topic name
        #[arg(short, long)]
        topic: String,
    },
    /// Find export by full ID (e.g., user-login:login_user)
    Lookup {
        /// Full export ID
        #[arg(short, long)]
        id: String,
    },
    /// Find callers of a symbol
    Callers {
        /// Symbol name to find callers for
        #[arg(short, long)]
        symbol: String,
    },
}

#[derive(Subcommand)]
pub enum PattyCommands {
    /// Enable platypus mascot
    Enable,
    /// Disable platypus mascot
    Disable,
    /// Show current status
    Status,
}

#[derive(Subcommand)]
pub enum PkgCommands {
    /// List available packages from the repository
    List {
        /// Repository URL (default: official UniSpec repo)
        #[arg(short, long)]
        repo: Option<String>,
    },
    /// Search for packages
    Search {
        /// Search query
        query: String,
        /// Repository URL (default: official UniSpec repo)
        #[arg(short, long)]
        repo: Option<String>,
    },
    /// Install a package (mode, connectors, workflows)
    Install {
        /// Package name or GitHub URL
        package: String,
        /// Install globally (user-wide)
        #[arg(short, long)]
        global: bool,
        /// Repository URL
        #[arg(short, long)]
        repo: Option<String>,
    },
    /// Remove an installed package
    Remove {
        /// Package name
        package: String,
        /// Remove from global installation
        #[arg(short, long)]
        global: bool,
    },
    /// List installed packages
    Installed {
        /// Show globally installed packages
        #[arg(short, long)]
        global: bool,
    },
}

#[derive(Subcommand)]
pub enum IngestCommands {
    /// Ingest a codebase directory and create specs from it
    Run {
        /// Path to the codebase to ingest
        path: String,
        /// Target area to create specs in
        #[arg(short, long, default_value = "Ingested")]
        area: String,
        /// Topic name for the ingested code
        #[arg(short, long)]
        topic: Option<String>,
        /// Languages to parse (default: all supported)
        #[arg(short, long)]
        languages: Option<String>,
        /// Watch for file changes and re-ingest automatically
        #[arg(short, long)]
        watch: bool,
    },
    /// Start live file watching for auto-indexing
    Watch {
        /// Path to watch (default: current directory)
        #[arg(short, long)]
        path: Option<String>,
        /// Topic to link code to
        #[arg(short, long)]
        topic: Option<String>,
    },
    /// Recursively ingest a codebase bottom-up
    Recursive {
        /// Path to the codebase to ingest
        path: String,
        /// Target area to create specs in
        #[arg(short, long, default_value = "Planned")]
        area: String,
    },
    /// Stop watching
    Stop,
}

#[derive(Subcommand)]
pub enum ParseCommands {
    /// Parse a single file and extract code elements
    File {
        /// Path to the file to parse
        path: String,
        /// Language to use (auto-detected if not specified)
        #[arg(short, long)]
        language: Option<String>,
        /// What to extract: functions, structs, enums, imports, all (default: all)
        #[arg(short, long, default_value = "all")]
        item_type: String,
        /// Filter by name pattern
        #[arg(short, long)]
        pattern: Option<String>,
        /// Output as JSON (for agent consumption)
        #[arg(long)]
        json: bool,
    },
}
