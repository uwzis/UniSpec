# UniSpec Commands Reference

Comprehensive reference for all UniSpec CLI commands. UniSpec is a specification-driven development tool with a TUI interface for managing topics, areas, and agent workflows.

## Synopsis

```bash
unispec [OPTIONS] [COMMAND]
```

## Global Options

| Option | Description |
|--------|-------------|
| `-q, --quiet` | Suppress output (only errors displayed) |
| `-v, --verbose` | Enable verbose output |
| `--root <path>` | Specify project root directory |
| `--version` | Show version information |
| `--help` | Show help message |

## TUI Mode (Interactive)

Running `unispec` without any arguments launches the interactive Terminal User Interface (TUI).

```bash
unispec
```

### TUI Navigation

The TUI provides keyboard-driven navigation through topics and areas.

| Key | Action |
|-----|--------|
| `↑` / `↓` | Move selection up/down between items |
| `→` | Enter selected area or navigate into topic subdirectory |
| `←` | Go back to previous view / return to area selection |
| `Enter` | Open linked file in default application |
| `n` | Create new item (area, topic, or link depending on view) |
| `r` | Remove selected item (with confirmation prompt) |
| `p` | Push/move topic to another area |
| `f` | Find and display all files linked to selected topic |
| `g` | Go to area selection |
| `/` | Search/filter topics (enters input mode) |
| `\` | Toggle platypus mascot display on/off |
| `q` | Quit the TUI |

### TUI Input Mode

When input mode is active (after pressing `n`, `r`, `p`, or `/`), type your response and press `Enter` to confirm. Press `Esc` to cancel.

### TUI Views

- **Area Selection**: List of all areas (Staging, Working, Build, etc.)
- **Topic List**: Topics within the current area
- **Nested Specs**: Subdirectory view when topic has child topics
- **Find Results**: Files linked to a selected topic

---

## Init

Initialize a new UniSpec project. Creates the `spec/` directory with default areas and the `.agent/` directory with configuration files.

```bash
unispec init [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `-r, --root <path>` | Project root directory (default: current directory) |
| `--amazon_q` | Add Amazon Q Developer integration (`.amazonq/prompts`) |
| `--antigravity` | Add Antigravity integration (`.agent/workflows`) |
| `--auggie` | Add Augment CLI integration (`.augment/commands`) |
| `--claude_code` | Add Claude Code integration (`.claude/commands/unispec`) |
| `--cline` | Add Cline integration (`.clinerules/workflows`) |
| `--codex` | Add Codex integration (`~/.codex/prompts`) |
| `--codebuddy` | Add CodeBuddy integration (`.codebuddy/commands/unispec`) |
| `--continue` | Add Continue integration (`.continue/prompts`) |
| `--costrict` | Add CoStrict integration (`.cospec/unispec/commands`) |
| `--crush` | Add Crush integration (`.crush/commands/unispec`) |
| `--cursor` | Add Cursor integration (`.cursor/commands`) |
| `--factory` | Add Factory Droid integration (`.factory/commands`) |
| `--gemini_cli` | Add Gemini CLI integration (`.gemini/commands/unispec`) |
| `--github` | Add GitHub integration (`.github/prompts`) |
| `--iflow` | Add iFlow integration (`.iflow/commands`) |
| `--kilo_code` | Add Kilo Code integration (`.kilocode/workflows`) |
| `--kiro` | Add Kiro integration (`.kiro/prompts`) |
| `--opencode` | Add OpenCode integration (`.opencode/command`) |
| `--pi` | Add Pi integration (`.pi/prompts`) |
| `--qoder` | Add Qoder integration (`.qoder/commands/unispec`) |
| `--qwen_code` | Add Qwen Code integration (`.qwen/commands`) |
| `--roo_code` | Add RooCode integration (`.roo/commands`) |
| `--windsurf` | Add Windsurf integration (`.windsurf/workflows`) |
| `--trae` | Add TRAE integration (`.trae/rule`) |

### Examples

```bash
# Initialize in current directory
unispec init

# Initialize with Cursor and Windsurf integrations
unispec init --cursor --windsurf

# Initialize with all editor integrations
unispec init --all

# Initialize in specific directory
unispec init --root /path/to/project
```

### What Gets Created

```
.
├── spec/
│   ├── Staging/
│   │   └── area.md
│   ├── Working/
│   │   └── area.md
│   └── Build/
│       └── area.md
└── .agent/
    ├── config.toml
    ├── skill.md
    ├── modes/
    │   └── simple/
    └── workflows/
        ├── osdd:spec.md
        ├── osdd:build.md
        └── osdd:verify.md
```

---

## Set

Set the default area (shorthand for `area default`).

```bash
unispec set <area>
```

### Arguments

| Argument | Description |
|----------|-------------|
| `area` | Name of the area to set as default |

### Examples

```bash
# Set default area to Staging
unispec set Staging
```

---

## Area

Manage areas - logical containers for organizing topics. Common areas are Staging, Working, and Build.

```bash
unispec area <subcommand>
```

### Subcommands

#### Add

Create a new area.

```bash
unispec area add <name>
```

| Argument | Description |
|----------|-------------|
| `name` | Name of the area to create |

**Example:**
```bash
unispec area add "Review"
```

#### Remove

Delete an area and all its topics.

```bash
unispec area remove <name>
```

| Argument | Description |
|----------|-------------|
| `name` | Name of the area to remove |

**Example:**
```bash
unispec area remove Review
```

#### List

Display all areas.

```bash
unispec area list
```

**Example:**
```bash
$ unispec area list
Staging
Working
Build
```

#### Rename

Rename an existing area.

```bash
unispec area rename <old> <new>
```

| Argument | Description |
|----------|-------------|
| `old` | Current area name |
| `new` | New area name |

**Example:**
```bash
unispec area rename Build Ship
```

#### Default

Set the default area for topic operations.

```bash
unispec area default <name>
```

| Argument | Description |
|----------|-------------|
| `name` | Name of the area to set as default |

**Example:**
```bash
unispec area default Working
```

#### Health

Display health statistics for all areas, showing topic counts and status.

```bash
unispec area health
```

**Example:**
```bash
$ unispec area health
Staging: 5 topics (3 in progress, 2 completed)
Working: 12 topics (8 in progress, 4 completed)
Build: 3 topics (all completed)
```

---

## Topic

Manage topics - individual specifications, features, or tasks within areas.

```bash
unispec topic <subcommand>
```

### Subcommands

#### Add

Create a new topic in an area.

```bash
unispec topic add <name> [OPTIONS]
```

| Argument | Description |
|----------|-------------|
| `name` | Name of the topic to create |

| Option | Description |
|--------|-------------|
| `-a, --area <area>` | Area to create topic in (default: Working) |

**Example:**
```bash
# Create topic in Working area
unispec topic add "User Authentication"

# Create topic in Staging area
unispec topic add "API Redesign" -a Staging
```

#### List

List topics in an area.

```bash
unispec topic list [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `-a, --area <area>` | Area to list topics from (default: Working) |
| `-h, --hierarchy` | Show hierarchical view with nested topics |

**Example:**
```bash
# List topics in Working area
unispec topic list

# List topics in Staging area
unispec topic list -a Staging

# Show hierarchical view
unispec topic list -a Working --hierarchy
```

#### Show

Display detailed information about a topic.

```bash
unispec topic show <name>
```

| Argument | Description |
|----------|-------------|
| `name` | Name of the topic to show |

**Example:**
```bash
unispec topic show "User Authentication"
```

#### Push

Move (push) a topic to another area.

```bash
unispec topic push <name> <area> [OPTIONS]
```

| Argument | Description |
|----------|-------------|
| `name` | Name of the topic to move |
| `area` | Target area to move to |

| Option | Description |
|--------|-------------|
| `-s, --source <area>` | Source area (auto-detected if not specified) |

**Example:**
```bash
# Push topic to Build area
unispec topic push "User Authentication" Build

# Push from specific area
unispec topic push "Feature X" Staging -s Working
```

#### Pull

Pull a topic from another area to the current area.

```bash
unispec topic pull <name> <area> [OPTIONS]
```

| Argument | Description |
|----------|-------------|
| `name` | Name of the topic to pull |
| `area` | Source area to pull from |

| Option | Description |
|--------|-------------|
| `-t, --target <area>` | Target area (default: Working) |

**Example:**
```bash
# Pull topic from Staging to Working
unispec topic pull "API Redesign" Staging
```

#### Remove

Delete a topic.

```bash
unispec topic remove <name> [OPTIONS]
```

| Argument | Description |
|----------|-------------|
| `name` | Name of the topic to remove |

| Option | Description |
|--------|-------------|
| `-f, --force` | Remove without confirmation |

**Example:**
```bash
# Remove with confirmation
unispec topic remove "Old Feature"

# Force remove without confirmation
unispec topic remove "Old Feature" --force
```

#### Progress

Show progress across all topics in an area or all areas.

```bash
unispec topic progress [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `-a, --area <area>` | Area to show progress for (default: current area) |

**Example:**
```bash
# Show progress for all areas
unispec topic progress

# Show progress for specific area
unispec topic progress -a Staging
```

---

## Index

Manage links between topics and filesystem paths (files or directories).

```bash
unispec index <subcommand>
```

### Subcommands

#### Add

Create a link between a topic and a file or directory.

```bash
unispec index add --topic <name> --path <path> [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `-t, --topic <name>` | Topic name to link |
| `-p, --path <path>` | File or directory path to link |
| `-a, --area <area>` | Area name (auto-detected if not specified) |
| `-l, --link_type <type>` | Type: 'file' or 'directory' (auto-detected) |
| `--tags` | Tags (comma-separated) |
| `--annotation` | Note about this link |
| `--exports` | Export names (comma-separated) |
| `--descriptions` | Export descriptions (comma-separated) |
| `--export-types` | Export types (comma-separated: function,class,endpoint,model,service,config) |
| `--signatures` | Function signatures (semicolon-separated) |

**Example:**
```bash
# Link a file to a topic
unispec index add --topic "user-login" --path src/auth/login.rs

# Link with tags
unispec index add --topic "user-login" --path src/auth/login.rs --tags "auth,backend"

# Link with exports
unispec index add --topic "user-login" --path src/auth/login.rs \
  --exports "login_user,logout,validate_token" \
  --descriptions "Authenticate user,Clear session,Verify token" \
  --export-types "function,function,function"

# Full example with signatures
unispec index add --topic "user-login" --path src/auth/login.rs \
  --exports "login_user" \
  --descriptions "Authenticate and create session" \
  --export-types "function" \
  --signatures "fn login_user(email: String, pass: String) -> Result<User>"
```

#### Remove

Remove a link between a topic and a path.

```bash
unispec index remove --topic <name> --path <path>
```

| Option | Description |
|--------|-------------|
| `-t, --topic <name>` | Topic name |
| `-p, --path <path>` | Path to unlink |

**Example:**
```bash
unispec index remove --topic "user-login" --path src/auth/login.rs
```

#### List

List all index links, optionally filtered.

```bash
unispec index list [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `-t, --topic <name>` | Filter by topic name |
| `-p, --path <path>` | Filter by path |
| `--tag <tag>` | Filter by tag |

**Example:**
```bash
# List all links
unispec index list

# List links for specific topic
unispec index list --topic "user-login"

# List links for specific path
unispec index list --path src/auth/

# List links with specific tag
unispec index list --tag backend
```

#### Find

Find links by topic name, path, tag, or annotation.

```bash
unispec index find <query> [OPTIONS]
```

| Argument | Description |
|----------|-------------|
| `query` | Search query |

| Option | Description |
|--------|-------------|
| `-b, --by <type>` | Search by: 'topic', 'path', 'tag', or 'annotation' (default: topic) |

**Example:**
```bash
# Find links by topic name
unispec index find "user-login" --by topic

# Find links by path
unispec index find "src/auth" --by path

# Find links by tag
unispec index find "security" --by tag

# Find links by annotation text
unispec index find "password" --by annotation
```

#### Full

Show complete index statistics.

```bash
unispec index full
```

**Example:**
```bash
$ unispec index full
Total links: 42
Topics with links: 15
Files linked: 38
Directories linked: 4
Unique tags: 8
Total exports: 25
```

#### Watch

Start a file system watcher to automatically update links when files change.

```bash
unispec index watch
```

**Example:**
```bash
# Start watching for file changes
unispec index watch
```

#### Cleanup

Remove orphaned links where the topic or path no longer exists.

```bash
unispec index cleanup
```

**Example:**
```bash
# Clean up broken links
unispec index cleanup
```

#### Exports

List exports (functions, classes, etc.) for a topic or all topics.

```bash
unispec index exports [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `-t, --topic <name>` | Topic name to list exports for |

**Example:**
```bash
# List exports for a topic
unispec index exports --topic user-login

# List all exports across all topics
unispec index exports
```

**Output:**
```
Exports for 'user-login':

  login_user (function)
    Description: Authenticate and create session
    ID: user-login:login_user
    Signature: fn login_user(email: String, pass: String) -> Result<User>

  logout (function)
    Description: Clear user session
    ID: user-login:logout

  validate_token (function)
    Description: Verify JWT token
    ID: user-login:validate_token
```

#### Query

Query exports by name, type, description, or ID.

```bash
unispec index query <query> [OPTIONS]
```

| Argument | Description |
|----------|-------------|
| `query` | Search query |

| Option | Description |
|--------|-------------|
| `-b, --by <type>` | Search by: 'name', 'type', 'description', or 'id' (default: name) |

**Example:**
```bash
# Search by export name
unispec index query "login" --by name

# Search by type
unispec index query "function" --by type

# Search by description
unispec index query "authenticate" --by description

# Search by ID
unispec index query "user-login" --by id
```

#### Depends

Find what topics depend on a given topic (what other topics use its exports).

```bash
unispec index depends --topic <name>
```

| Option | Description |
|--------|-------------|
| `-t, --topic <name>` | Topic name to find dependents for |

**Example:**
```bash
unispec index depends --topic user-login

# Output:
# Topics depending on 'user-login':
#   checkout-flow
#     - login_user (user-login:login_user)
#     - validate_token (user-login:validate_token)
```

#### Lookup

Find an export by its full ID.

```bash
unispec index lookup --id <export-id>
```

| Option | Description |
|--------|-------------|
| `-i, --id <id>` | Full export ID (format: topic:name, e.g., user-login:login_user) |

**Example:**
```bash
unispec index lookup --id user-login:login_user

# Output:
# Found: user-login:login_user
#   Name: login_user
#   Type: function
#   Topic: user-login
#   Path: src/auth/login.rs
#   Description: Authenticate and create session
```

#### Backlinks

Generate a backlinks file showing what links to a topic.

```bash
unispec index backlinks --topic <name>
```

| Option | Description |
|--------|-------------|
| `-t, --topic <name>` | Topic name |

**Example:**
```bash
unispec index backlinks --topic user-login
```

#### Tags

List all unique tags in the index.

```bash
unispec index tags
```

**Example:**
```bash
$ unispec index tags
Tags in index:
  auth (5 links)
  backend (12 links)
  security (3 links)
```

#### Graph

Export index as graph JSON for visualization.

```bash
unispec index graph
```

**Example:**
```bash
unispec index graph > graph.json
```

---

## MCP

Launch the MCP (Model Context Protocol) server for AI agent integration.

```bash
unispec mcp [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `path` | Project directory to run MCP server for |

### Description

The MCP server exposes UniSpec functionality as tools that can be called by AI agents like Claude, GPT, and others. Connectors are automatically exposed as MCP tools.

**Example:**
```bash
# Start MCP server
unispec mcp

# Start for specific project
unispec mcp /path/to/project
```

### MCP Tools Available

When the MCP server is running, the following tools are available:

- `unispec_topic_list` - List topics in an area
- `unispec_topic_show` - Show topic details
- `unispec_area_list` - List all areas
- `unispec_area_health` - Show area health
- `unispec_index_find` - Find files linked to a topic
- `unispec_connector_*` - Run custom connectors

---

## Mode

Manage agent modes - different workflow configurations.

```bash
unispec mode <subcommand>
```

### Subcommands

#### List

List all available modes.

```bash
unispec mode list
```

**Example:**
```bash
$ unispec mode list
simple   - Default mode for spec-driven development
custom   - Custom mode
```

#### Info

Show detailed information about a mode.

```bash
unispec mode info <name>
```

| Argument | Description |
|----------|-------------|
| `name` | Name of the mode |

**Example:**
```bash
unispec mode info simple
```

#### Activate

Switch the active mode.

```bash
unispec mode activate <name>
```

| Argument | Description |
|----------|-------------|
| `name` | Name of the mode to activate |

**Example:**
```bash
unispec mode activate simple
```

#### Add

Add a mode from a directory.

```bash
unispec mode add <path> [OPTIONS]
```

| Argument | Description |
|----------|-------------|
| `path` | Path to the mode directory |

| Option | Description |
|--------|-------------|
| `-g, --global` | Add to global modes instead of local |

**Example:**
```bash
# Add local mode
unispec mode add ./modes/my-custom-mode

# Add global mode
unispec mode add /path/to/mode --global
```

#### Remove

Remove a mode.

```bash
unispec mode remove <name> [OPTIONS]
```

| Argument | Description |
|----------|-------------|
| `name` | Name of the mode to remove |

| Option | Description |
|--------|-------------|
| `-g, --global` | Remove from global modes |

**Example:**
```bash
unispec mode remove custom-mode
```

#### Current

Show the currently active mode.

```bash
unispec mode current
```

**Example:**
```bash
$ unispec mode current
simple
```

---

## Connector

Manage connectors - custom commands that become MCP tools. Connectors allow you to define shell commands that can be invoked by AI agents.

```bash
unispec connector <subcommand>
```

### Subcommands

#### New

Create a new connector.

```bash
unispec connector new <name> <description> <command> [OPTIONS]
```

| Argument | Description |
|----------|-------------|
| `name` | Connector name (lowercase with underscores only) |
| `description` | Description of what the connector does |
| `command` | Command to execute |

| Option | Description |
|--------|-------------|
| `-a, --args <args>` | Arguments (space-separated) |
| `-e, --env <env>` | Environment variables (KEY=value, comma-separated) |
| `-w, --working_dir <dir>` | Working directory |
| `-t, --timeout <seconds>` | Timeout in seconds (default: 60) |

**Example:**
```bash
# Create a test runner connector
unispec connector new test "Run test suite" "pytest" -a "tests/ -v"

# Create a linter connector with env vars
unispec connector new lint "Run linter" "ruff" -a "check ." -e "RUFF_CONFIG=pyproject.toml"

# Create connector with custom timeout
unispec connector new build "Build project" "cargo" -a "build --release" -t 300
```

#### List

List all connectors.

```bash
unispec connector list
```

**Example:**
```bash
$ unispec connector list
Available Connectors:

1. test - Run test suite
   Command: pytest tests/ -v

2. lint - Run linter
   Command: ruff check .
   Env: RUFF_CONFIG=pyproject.toml
```

#### Delete

Delete a connector.

```bash
unispec connector delete <name>
```

| Argument | Description |
|----------|-------------|
| `name` | Name of the connector to delete |

**Example:**
```bash
unispec connector delete test
```

#### Run

Execute a connector.

```bash
unispec connector run <name> [args...]
```

| Argument | Description |
|----------|-------------|
| `name` | Name of the connector to run |
| `args` | Additional arguments to pass to the command |

**Example:**
```bash
# Run basic connector
unispec connector run test

# Run with additional arguments
unispec connector run test tests/unit/test_auth.py -v
```

#### Edit

Update a connector's description.

```bash
unispec connector edit <name> [OPTIONS]
```

| Argument | Description |
|----------|-------------|
| `name` | Name of the connector |

| Option | Description |
|--------|-------------|
| `-d, --description <desc>` | New description |

**Example:**
```bash
unispec connector edit test -d "Run test suite with coverage"
```

#### Mcp

Generate MCP server configuration for all connectors.

```bash
unispec connector mcp
```

**Example:**
```bash
# Generate and save to file
unispec connector mcp > claude_desktop_config.json

# View configuration
unispec connector mcp
```

### Connector Configuration Format

Connectors are stored in `.agent/config.toml`. See [Configuration Reference](configuration.md) for the complete format.

```toml
[[connector]]
name = "test"
description = "Run test suite"
command = "pytest"
args = ["tests/", "-v"]
env = { RUST_BACKTRACE = "1" }
working_dir = "/project/root"
timeout = 120
```

See [Configuration Reference](configuration.md#connector-configuration) for all connector options.

---

## Pkg

Manage packages - installable mode templates, skills, and workflows from the community package repository.

```bash
unispec pkg <subcommand>
```

### Subcommands

#### List

List available packages from the repository.

```bash
unispec pkg list [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `-r, --repo <url>` | Repository URL (default: official UniSpec repo) |

**Example:**
```bash
# List packages from default repo
unispec pkg list

# List from custom repo
unispec pkg list -r https://example.com/packages
```

#### Search

Search for packages in the repository.

```bash
unispec pkg search <query> [OPTIONS]
```

| Argument | Description |
|----------|-------------|
| `query` | Search query |

| Option | Description |
|--------|-------------|
| `-r, --repo <url>` | Repository URL |

**Example:**
```bash
unispec pkg search python
```

#### Install

Install a package (mode, connectors, workflows).

```bash
unispec pkg install <package> [OPTIONS]
```

| Argument | Description |
|----------|-------------|
| `package` | Package name or GitHub URL |

| Option | Description |
|--------|-------------|
| `-g, --global` | Install globally (user-wide, available to all projects) |
| `-r, --repo <url>` | Repository URL |

**Example:**
```bash
# Install package (local to project)
unispec pkg install python-mode

# Install from GitHub
unispec pkg install https://github.com/user/unispec-mode

# Install globally (available to all projects)
unispec pkg install python-mode --global
```

#### Remove

Remove an installed package.

```bash
unispec pkg remove <package> [OPTIONS]
```

| Argument | Description |
|----------|-------------|
| `package` | Package name |

| Option | Description |
|--------|-------------|
| `-g, --global` | Remove from global installation |

**Example:**
```bash
unispec pkg remove python-mode
```

#### Installed

List installed packages.

```bash
unispec pkg installed [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `-g, --global` | Show globally installed packages |

**Example:**
```bash
# List local installations
unispec pkg installed

# List global installations
unispec pkg installed --global
```

---

## Patty

Control the platypus mascot ("Paddy") display in the TUI.

```bash
unispec patty <subcommand>
```

### Subcommands

#### Enable

Enable the platypus mascot.

```bash
unispec patty enable
```

#### Disable

Disable the platypus mascot.

```bash
unispec patty disable
```

#### Status

Show current platypus status.

```bash
unispec patty status
```

### Platypus States

The platypus mascot displays different expressions based on context:

| State | Trigger |
|-------|---------|
| Idle | Default state, no activity |
| Happy | Successful operations (create, link) |
| Sad | Removal operations |
| Love | Topic pushed to another area |
| Searching | Finding files linked to topic |
| Working | Topics with tasks in progress |
| Celebrating | All tasks completed |

---

## See Also

- [Configuration Reference](configuration.md) - Config files, environment variables, exit codes
- [Modes Documentation](modes.md) - Custom workflow configurations
- [MCP Documentation](mcp.md) - AI agent integration
- [Getting Started](getting-started.md) - Quick start guide

---

*Questions? Check modes.md for custom workflows or mcp.md for AI integration.*
