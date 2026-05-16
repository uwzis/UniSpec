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
unispec topic add <name> --short <description> --content <body> [OPTIONS]
```

| Argument | Description |
|----------|-------------|
| `name` | Name of the topic to create |

| Option | Description |
|--------|-------------|
| `-a, --area <area>` | Area to create topic in. Defaults to `.agent/config.toml`'s `area`, then `"Staging"`. |
| `-s, --short <text>` | One-line description (required). |
| `-c, --content <text>` | Topic body content (required; ≥ 20 chars). |

**Example:**
```bash
# Default area from config (falls back to Staging)
unispec topic add user-login \
  --short "Email/password login" \
  --content "Authentication system with JWT and refresh tokens."

# Explicit area override
unispec topic add api-redesign -a Working \
  --short "REST surface revamp" \
  --content "Move every endpoint to /v2 with consistent error envelopes."
```

#### List

List topics in an area.

```bash
unispec topic list [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `-a, --area <area>` | Area to list topics from. Defaults to config, then `"Staging"`. |
| `-H, --hierarchy` | Show hierarchical view with nested topics. |

**Example:**
```bash
# Default area
unispec topic list

# Explicit area
unispec topic list -a Working

# Hierarchical view
unispec topic list -a Working --hierarchy
```

#### Show

Display detailed information about a topic.

```bash
unispec topic show <name> [OPTIONS]
```

| Argument | Description |
|----------|-------------|
| `name` | Name of the topic to show |

| Option | Description |
|--------|-------------|
| `-a, --all` | Show all files in the topic from all areas |
| `-f, --from <area>` | Show files from a specific area (useful when topic has files from multiple areas) |

**Example:**
```bash
# Show current area's files (default area)
unispec topic show "User Authentication"

# Show files from a specific area
unispec topic show "User Authentication" --from Staging

# Show all files from all areas
unispec topic show "User Authentication" --all
```

**Multi-Area Topic Notes:**
When a topic has been pushed between areas, it may contain files from multiple areas:
- `specs.md` - copied from source area
- `tasks_staging.md` - tasks from Staging area
- `tasks_working.md` - tasks from Working area

Use `--from` to view specific area's files, or `--all` to see everything.

#### Push

Move a topic to another area. Source files are removed (this is a move, not a copy).

```bash
unispec topic push <name> [OPTIONS]
```

| Argument | Description |
|----------|-------------|
| `name` | Name of the topic to move |

| Option | Description |
|--------|-------------|
| `-a, --area <target>` | Target area. Defaults to config's `area`, then `"Staging"`. |
| `-f, --from <source>` | Source area. Defaults to config's `area`, then `"Staging"`. |

**Example:**
```bash
# Explicit source and target
unispec topic push user-login --area Working --from Staging

# Target area implied by config (e.g. config.area = "Working")
unispec topic push user-login --from Staging
```

**Readiness gate.** When the source area is `Staging` or `Fixing`, the topic must first appear in `spec/<source>/queue.md`. Run `unispec queue add <name>` (or `--area Fixing`) first. See [queue](#queue) below.

**Auto-area creation.** If the target area directory doesn't exist yet, push creates it on the fly with a minimal `area.md` stub. This is useful when an init created fewer than the five pipeline areas, or when the user has custom areas they haven't materialized yet.

#### Pull

Pull a topic from another area into the current area.

```bash
unispec topic pull <name> [OPTIONS]
```

| Argument | Description |
|----------|-------------|
| `name` | Name of the topic to pull |

| Option | Description |
|--------|-------------|
| `-a, --area <source>` | Source area to pull from. Defaults to config's `area`, then `"Staging"`. |

**Example:**
```bash
# Pull into the current default area
unispec topic pull api-redesign --area Staging
```

#### Remove

Delete a topic from an area.

```bash
unispec topic remove <name> [OPTIONS]
```

| Argument | Description |
|----------|-------------|
| `name` | Name of the topic to remove |

| Option | Description |
|--------|-------------|
| `-a, --area <area>` | Area the topic lives in. Defaults to config's `area`, then `"Staging"`. |
| `-f, --force` | Remove without confirmation. |

**Example:**
```bash
# Confirm-then-delete from default area
unispec topic remove old-feature

# Force-delete from a specific area
unispec topic remove old-feature --area Working --force
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

## Spec

Create and view per-topic spec/task files.

```bash
unispec spec <subcommand>
```

### Subcommands

#### Add

Create `<topic>_spec.md` and `<topic>_task.md` for a topic. The topic must already exist (via `topic add`). Slashes and spaces in the topic name are replaced with `-` when building the filename, so a topic `auth/login` produces `auth-login_spec.md` and `auth-login_task.md` inside `spec/<area>/auth/login/`.

```bash
unispec spec add --topic <name> --short <desc> --spec-content <body> --task-content <tasks> [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `-t, --topic <name>` | Required. Topic name (`parent/child` for nested topics). |
| `-s, --short <text>` | Required. One-line description. |
| `--spec-content <text>` | Required. Spec body (≥ 11 chars). `allow_hyphen_values` is enabled — values starting with `- ` (markdown bullets) are accepted as content, not parsed as flags. |
| `--task-content <text>` | Required. Task body (≥ 11 chars). Same hyphen-value handling. |
| `-a, --area <area>` | Area for the topic. Defaults to config's `area`, then `"Staging"`. |

**Example:**
```bash
unispec spec add \
  --topic user-login \
  --short "Email/password login design" \
  --spec-content "POST /login accepts {email, password}, returns {jwt}." \
  --task-content "- [ ] POST /login route
- [ ] JWT signing helper
- [ ] Integration tests"
```

The server writes the canonical frontmatter (`title`, `short`, `created`, `author`, `status: draft` for spec; `spec`, `short`, `status: pending`, `date` for task). Do not include a `---` frontmatter block in either body — it is stripped before the canonical frontmatter is prepended.

#### Show

Show the master spec file at `spec/master.md`, if present.

```bash
unispec spec show [name]
```

| Argument | Description |
|----------|-------------|
| `name` | Optional name. Defaults to `"master"`. Currently the handler reads `spec/master.md` regardless. |

---

## Change

Manage per-topic change folders (`spec/<area>/<topic>/changes/`). Used to propose new features for a topic that already has a spec, without overwriting the original. See [change-management.md](change-management.md) for the full guide.

```bash
unispec change <subcommand>
```

### Subcommands

#### Add

Create a new change folder inside an existing topic. Writes `proposal.md`, optional `design.md`, `<change>_spec.md`, and `<change>_task.md` under `spec/<area>/<topic>/changes/<change>/`. The topic's original `<topic>_spec.md` and `<topic>_task.md` are not touched.

```bash
unispec change add --topic <name> --change <id> --proposal <text> \
  --spec-content <body> --task-content <tasks> [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `-t, --topic <name>` | Required. Existing topic name. |
| `-c, --change <id>` | Required. Change identifier (kebab-case recommended, e.g. `add-2fa`). |
| `--proposal <text>` | Required. Why this change exists (≥ 11 chars). `allow_hyphen_values` enabled — markdown bullets are allowed. |
| `--spec-content <body>` | Required. New requirements (≥ 11 chars). `allow_hyphen_values` enabled. |
| `--task-content <tasks>` | Required. New tasks (≥ 11 chars). `allow_hyphen_values` enabled. |
| `--design <text>` | Optional. Technical-approach body. When present, writes `design.md`. |
| `-a, --area <area>` | Area the topic lives in. Defaults to config's `area`, then `"Staging"`. |

**Example:**
```bash
unispec change add \
  --topic auth \
  --change add-2fa \
  --proposal "Protect high-value accounts with a second factor." \
  --design "TOTP via authenticator apps; encrypted seed at rest." \
  --spec-content "## 2FA requirements
- TOTP enrolment per user
- 8 recovery codes per user" \
  --task-content "- [ ] Generate TOTP seeds
- [ ] Verify TOTP codes on login
- [ ] Issue and store recovery codes"
```

**Failure modes.**
- Topic doesn't exist → `Topic '<name>' does not exist in area '<area>'`.
- Change folder already present → `Change '<X>' already exists for topic '<topic>' in area '<area>'`.
- Any required content field shorter than 11 trimmed chars → length validation error.

#### List

List every change for a topic with its status and which files it contains.

```bash
unispec change list --topic <name> [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `-t, --topic <name>` | Required. Topic to list changes for. |
| `-a, --area <area>` | Area name. Defaults to config's `area`, then `"Staging"`. |
| `--archived` | Also include changes under `changes/archive/`. |

Statuses come from the change's task file:
- `proposed` — task file has no `- [ ]` / `- [x]` lines yet, or none are completed.
- `in-progress` — some `- [x]` boxes, but not all.
- `complete` — every `- [ ]` line is `- [x]`.
- `archived` — change lives under `changes/archive/` (only surfaced with `--archived`).

**Example output:**
```
Changes for 'auth' in Staging/:
  - add-2fa [in-progress] (proposal, design, spec, task)
  - add-oauth [proposed] (proposal, spec, task)
```

#### Archive

Move a change directory from `changes/<change>/` to `changes/archive/<change>/`, **merging any delta sections into the canonical `<topic>_spec.md` first**. Used when a change has been fully implemented and you want it out of the live change list.

```bash
unispec change archive --topic <name> --change <id> [OPTIONS]
```

**Delta merge on archive.** If the change's `<change>_spec.md` contains any of these top-level sections, the canonical spec is rewritten with the edits applied (RENAMED → REMOVED → MODIFIED → ADDED) before the directory move:

```markdown
## ADDED Requirements
### Requirement: <name>
<content>

## MODIFIED Requirements
### Requirement: <existing name>
<new content that replaces the old block>

## REMOVED Requirements
### Requirement: <name>
**Reason:** <why>

## RENAMED Requirements
- FROM: ### Requirement: Old Name
- TO: ### Requirement: New Name
```

A change spec with no delta sections is archived without touching the canonical spec (backward compatible). See [change-management.md](change-management.md) for the full delta grammar.

| Option | Description |
|--------|-------------|
| `-t, --topic <name>` | Required. Topic name. |
| `-c, --change <id>` | Required. Change name to archive. |
| `-a, --area <area>` | Area name. Defaults to config's `area`, then `"Staging"`. |

**Example:**
```bash
unispec change archive --topic auth --change add-2fa
```

**Failure modes.**
- Source change doesn't exist → `Change '<X>' does not exist for topic '<topic>' in area '<area>'`.
- An archived change with the same name already exists → `Archived change '<X>' already exists; refusing to overwrite`.

The MCP `change_add`, `change_list`, and `change_archive` tools expose the same logic programmatically — see [mcp-tools-reference.md](mcp-tools-reference.md#change-management).

---

## Next

Get a structured next-action payload for a topic. The recommended entry point whenever an agent or human needs to know what to do next on a topic.

```bash
unispec next --topic <name> [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `-t, --topic <name>` | Required. Topic to analyse. |
| `-a, --area <area>` | Area name. Defaults to config's `area`, then `"Staging"`. |
| `--json` | Emit the full payload as JSON instead of the human summary. |

The payload returns: `status` (one of `not-started` / `in-progress` / `complete` / `blocked`), `open_tasks[]` + `completed_tasks[]` (each task carries `index`, `text`, `from_change`), `pending_changes[]` + `archived_changes[]`, `context_files[]` (relative paths to read), `rules[]` (area-specific constraints), `next_action` (one sentence), `blockers[]`.

The MCP tool `next { topic, area? }` returns the same shape. See [mcp-tools-reference.md](mcp-tools-reference.md#next-structured-agent-feed).

---

## Analyze

Cross-artifact consistency checker. Read-only.

```bash
unispec analyze --topic <name> [OPTIONS]
```

| Option | Description |
|--------|-------------|
| `-t, --topic <name>` | Required. Topic to analyse. |
| `-a, --area <area>` | Area name. Defaults to config, then `"Staging"`. |
| `--json` | Emit findings as JSON. |

Runs six checks against the topic's spec, task, and every pending change:

| # | Check | Severity |
|---|-------|----------|
| 1 | Duplication of requirements between canonical spec and a pending change | WARNING |
| 2 | `### Requirement:` rows with no task line referencing them | ERROR |
| 3 | Ambiguous language (`fast / secure / scalable / easy / simple / good / better / best / quick`) without a metric | WARNING |
| 4 | Empty `## <heading>` sections | WARNING |
| 5 | Constitution alignment (surfaces version, agent evaluates manually) | INFO |
| 6 | Overall task completion ratio | INFO |

Exits 0 even when findings are present; severity is reported in the summary line. The MCP tool `analyze { topic, area? }` returns the same structured findings.

---

## Workspace

Multi-repo coordination across linked UniSpec projects.

```bash
unispec workspace <subcommand>
```

### Subcommands

#### Init

Create `.unispec-workspace/workspace.yaml` in the current directory.

```bash
unispec workspace init <name>
```

| Argument | Description |
|----------|-------------|
| `name` | Workspace name (written to the YAML's `name:` field). |

#### Link

Add a named pointer to another UniSpec project.

```bash
unispec workspace link <name> <path>
```

| Argument | Description |
|----------|-------------|
| `name` | Short name to reference the repo by (e.g. `api`, `web`). |
| `path` | Path to the project root. Stored as absolute. |

#### List

List linked repos with status flags (does the path exist? does it have a `spec/` and `.agent/`?).

```bash
unispec workspace list
```

#### Status

Combined view of every topic across every linked repo.

```bash
unispec workspace status [--json]
```

Without `--json`, prints one line per topic prefixed by its area. With `--json`, returns the same structured payload the `workspace_status` MCP tool returns.

---

## Queue

Manage area readiness queues (`spec/<area>/queue.md`). Topics in `Staging` and `Fixing` must be listed in the queue before `topic push` will move them out.

```bash
unispec queue <subcommand>
```

### Subcommands

#### Add

Append a topic to an area's queue. (Or insert at a specific 0-based slot via `--position`.)

```bash
unispec queue add <topic> [OPTIONS]
```

| Argument | Description |
|----------|-------------|
| `topic` | Topic name to enqueue. |

| Option | Description |
|--------|-------------|
| `-a, --area <area>` | Queue's area. Defaults to config's `area`, then `"Staging"`. |
| `-p, --position <n>` | 0-based index. Negative or out-of-range = append to end (default: `-1`). |

**Example:**
```bash
# Add to the Staging queue (config default)
unispec queue add user-login

# Add to the Fixing queue, at the front
unispec queue add user-login --area Fixing --position 0
```

The MCP `queue_add`, `queue_remove`, `queue_check`, `queue_list`, and `queue_reorder` tools expose the same readiness queue programmatically — see [mcp-tools-reference.md](mcp-tools-reference.md).

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
| `-l, --link-type <type>` | Link type. Auto-detected as `file` or `directory` when omitted; common explicit values include `implementation`, `test`, `doc`, `config`. |

**Example:**
```bash
# Link a file to a topic
unispec index add --topic "user-login" --path src/auth/login.rs

# Link a directory to a topic
unispec index add --topic "api-redesign" --path src/api/ --link-type directory

# Specify area explicitly
unispec index add --topic "feature-x" --path src/feature_x.rs -a Staging
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

**Example:**
```bash
# List all links
unispec index list

# List links for specific topic
unispec index list --topic "user-login"

# List links for specific path
unispec index list --path src/auth/
```

#### Find

Find links by topic name or path.

```bash
unispec index find <query> [OPTIONS]
```

| Argument | Description |
|----------|-------------|
| `query` | Topic name or path to search for |

| Option | Description |
|--------|-------------|
| `-b, --by <type>` | Search by: 'topic' or 'path' (default: topic) |

**Example:**
```bash
# Find links by topic name
unispec index find "user-login" --by topic

# Find links by path
unispec index find "src/auth" --by path
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

When the MCP server is running, it exposes 39 built-in tools plus one dynamic `unispec_<name>` tool per configured connector. See [MCP Documentation](mcp.md) for the full schema of each tool.

Tools the server publishes (real names, used verbatim by agents):

- Areas: `areas_list`
- Topics: `topics_list`, `topics_add`, `topics_show`, `topics_delete`, `topics_push`, `topics_pull`, `topics_progress`
- Reading: `read_asset`, `unispec_read_spec`
- Specs & tasks: `spec_add`, `spec_write`, `task_write`, `task_status`, `tasks_list`, `tasks_complete`, `tasks_incomplete`
- Change management: `change_add`, `change_list`, `change_archive`
- Agent feed: `next`
- Analysis: `analyze`
- Constitution: `constitution_read`, `constitution_check`
- Workspace: `workspace_status`
- Notes: `notes_read`, `notes_add`
- Queue: `queue_list`, `queue_add`, `queue_remove`, `queue_check`, `queue_reorder`
- Index: `index_add`, `index_find`, `index_lookup`, `index_list`, `index_graph`, `index_backlinks`, `unispec_bind_spec`
- Dynamic: `unispec_<connector-name>` for each connector defined in `.agent/config.toml`

CLI commands like `unispec area add`, `unispec index remove`, `unispec mode activate`, `unispec connector new`, `unispec pkg install` are **not** exposed via MCP — they're considered setup/config commands. Run them from the shell.

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

## Ingest

Ingest an existing codebase and create specs from it. Uses tree-sitter to parse source code and extract functions, structs, enums, imports, and documentation.

```bash
unispec ingest <subcommand>
```

### Subcommands

#### Run

Ingest a codebase directory and create specs from it.

```bash
unispec ingest run <path> [OPTIONS]
```

| Argument | Description |
|----------|-------------|
| `path` | Path to the codebase to ingest |

| Option | Description |
|--------|-------------|
| `-a, --area <area>` | Target area to create specs in (default: Ingested) |
| `-t, --topic <name>` | Topic name for the ingested code (default: directory name) |
| `-l, --languages <langs>` | Languages to parse (comma-separated: rust,python,js) |
| `-w, --watch` | Watch for file changes and re-ingest automatically |

**Example:**
```bash
# Ingest current project
unispec ingest run .

# Ingest specific directory into custom area
unispec ingest run /path/to/project -a "Backend"

# Ingest specific languages only
unispec ingest run src --languages rust,go
```

**Output:**
Based on `.agent/config.toml` `[ingest]` settings:
- `output_format = "toml"` → saves to `spec/code_analysis.toml`
- `output_format = "md"` → creates `specs.md`, `links.md`, `functions.md` files
- `output_format = "both"` → creates both TOML and MD files
- `auto_index = true` → automatically adds to `spec/index.toml`

#### Watch

Show auto-indexing status (configured in `.agent/config.toml`).

```bash
unispec ingest watch
```

#### Stop

Stop the file watcher.

```bash
unispec ingest stop
```

---

## Parse

Parse a single file using tree-sitter and extract code elements. Auto-detects language from extension or shebang.

```bash
unispec parse file <path> [OPTIONS]
```

| Argument | Description |
|----------|-------------|
| `path` | Path to the file to parse |

| Option | Description |
|--------|-------------|
| `-l, --language <lang>` | Language to use (auto-detected if not specified) |
| `-i, --item-type <type>` | What to extract: functions, structs, enums, imports, all (default: all) |
| `-p, --pattern <pat>` | Filter by name pattern |
| `--json` | Output as JSON (for agent consumption) |

**Supported Languages:** rust, javascript, typescript, python, go, bash

**Example:**
```bash
# Parse a file, auto-detect language
unispec parse file src/main.rs

# Extract only functions
unispec parse file src/utils.rs --item-type functions

# Find specific function by pattern
unispec parse file src/main.rs --item-type functions --pattern "handle_"

# Output as JSON for agent
unispec parse file src/main.rs --json
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
