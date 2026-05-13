# Architecture

How the UniSpec codebase is laid out, what each module does, and how a single CLI invocation or MCP tool call flows through the binary.

## High-level shape

```
unispec (binary)
├── CLI dispatch        src/main.rs
├── CLI parser          src/cli/mod.rs                 (clap)
├── Commands            src/commands/*.rs              (one file per top-level CLI subcommand)
├── Filesystem helpers  src/fs/*.rs                    (paths, areas, config, index, spec)
├── Agent / mode        src/agent/*.rs                 (mode resolution, connectors, auto)
├── MCP server          src/mcp/{mod,server}.rs        (JSON-RPC over stdio)
├── TUI                 src/tui/*.rs                   (ratatui)
└── Mascot              src/platypus.rs
```

There are two top-level user-facing surfaces, **CLI** and **MCP**. Both call into shared per-feature modules under `src/commands/`. The TUI is a third surface but it shells out to the same commands rather than holding its own logic.

## Single source of truth for shared logic

Four shared command modules expose pure functions that both the CLI and the MCP server call:

| Module | Public functions | Called by |
|--------|------------------|-----------|
| `src/commands/topic.rs` | `run_new`, `run_push`, `run_pull`, `run_show`, `run_list`, `run_delete`, `run_progress`, `auto_checkin`, `auto_checkout` | CLI `topic *`, MCP `topics_*` |
| `src/commands/spec.rs` | `run_spec_add` | CLI `spec add`, MCP `spec_add` |
| `src/commands/queue.rs` | `run_queue_add` (returns `QueueAddOutput`) | CLI `queue add`, MCP `queue_add` |
| `src/commands/change.rs` | `run_change_add`, `run_change_list`, `run_change_archive` | CLI `change *`, MCP `change_*` |

This means a behaviour change to (e.g.) `run_spec_add` automatically applies to both CLI and MCP — there's no duplication to keep in sync. Earlier branches had inline copies in `server.rs`; PR4 / PR7 refactored them out.

## CLI flow

1. `main.rs` parses argv via clap (`Cli::parse`) defined in `src/cli/mod.rs`.
2. Top-level subcommand match in `main.rs` dispatches to the appropriate handler.
3. Subcommand-specific flag matching happens in the inner match (e.g., `match topic_cmd { TopicCommands::Add { … } => …, TopicCommands::Push { … } => … }`).
4. Each arm resolves area defaults via `resolve_area_from_config(area)` (helper in `main.rs`) — which reads `.agent/config.toml::area`, falling back to `"Staging"`.
5. The handler calls into `src/commands/<feature>.rs` and prints the result.

Side effects (platypus mascot triggers, success banners) happen in `main.rs` after the command returns `Ok(_)`.

## MCP flow

1. `unispec mcp [path]` lands in `src/mcp/server.rs::run_mcp_server`.
2. The server reads a byte stream from stdin, parsing one complete JSON object per request via a hand-rolled brace-counter (handles `\` escapes and string boundaries).
3. `handle_request` matches `method`:
   - `initialize` → version handshake.
   - `tools/list` → returns the static tool catalog from `src/mcp/mod.rs::get_tools()` (34 entries).
   - `tools/call` → routes to `call_tool(name, args)`.
4. `call_tool` is one large `match name` dispatch. Each arm reads required args, calls into shared `src/commands/<feature>.rs` functions or thin module helpers, and returns a JSON response.
5. The server writes each response back as a single JSON line to stdout (Zed/Claude Code/MCP standard).

Fallback dispatch: if no `match` arm matches AND the tool name starts with `unispec_`, the server treats the suffix as a connector name and shells out via `crate::agent::connector::run_run`. This is how dynamic `unispec_<connector>` tools work.

If neither matches: `Err("Unknown tool: <name>")`. (This was a major source of bugs on `main` before PR2 added the missing six handlers — `tasks_list`, `tasks_complete`, `tasks_incomplete`, `notes_add`, `notes_read`, `queue_reorder`.)

## On-disk layout the binary creates

```
<project-root>/
├── spec/
│   ├── <Area>/
│   │   ├── area.md                       # per-area description (mode-templated)
│   │   ├── queue.md                      # readiness queue (only present where used)
│   │   └── <topic>/
│   │       ├── topic.md                  # written by `topic add`
│   │       ├── <topic-safe>_spec.md      # written by `spec add` / `spec_write`
│   │       ├── <topic-safe>_task.md      # written by `spec add` / `task_write`
│   │       └── changes/                  # optional — written by `change add`
│   │           ├── <change>/
│   │           │   ├── proposal.md
│   │           │   ├── design.md             # only if --design supplied
│   │           │   ├── <change>_spec.md
│   │           │   └── <change>_task.md
│   │           └── archive/<archived-change>/   # written by `change archive`
│   └── index.toml                        # topic ↔ path index (written by `index add`)
└── .agent/
    ├── config.toml                       # active mode, default area, ingest config, connectors
    ├── skill.md                          # agent persona (copied from active mode)
    ├── workflows/                        # workflow prompts (copied from active mode)
    │   ├── build.md
    │   ├── ingest.md
    │   ├── test.md
    │   ├── unispec:spec.md
    │   └── verify.md
    └── modes/
        ├── README.md                     # how to author modes
        └── default/                      # the embedded default mode
            ├── mode.toml
            ├── skill.md
            ├── system_prompts/unispec_basics.md
            ├── templates/{topic,spec,task,area}.md
            ├── areas/{staging,working,testing,fixing,build}/area.md
            └── workflows/{build,ingest,test,unispec:spec,verify}.md
```

`<topic-safe>` is the topic name with `/` and ` ` replaced by `-`. So a nested topic `auth/login` produces `spec/<Area>/auth/login/auth-login_spec.md`.

`<Area>` is one of the five default areas, or a custom one if the active mode declared more.

## Module-by-module

### `src/main.rs` (~700 lines)

Top-level CLI dispatch. Imports every subcommand enum from `cli/mod.rs` and every shared module from `commands/`. Holds `resolve_area_from_config` (the helper that gives `topic_add`, `topic_list`, `topic_push`, etc. their config-default behaviour).

### `src/cli/mod.rs` (~830 lines)

All clap structs. The interesting enums:

- `Commands` — top-level (`Init`, `Topic(...)`, `Spec(...)`, `Queue(...)`, `Index(...)`, `Mcp`, `Mode(...)`, `Connector(...)`, `Pkg(...)`, `Ingest(...)`, `Parse(...)`, `Patty(...)`, `Auto(...)`).
- `TopicCommands` — `Add` / `List` / `Push` / `Pull` / `Remove` / `Show` / `Progress` / `Order`. All take `Option<String>` for area, named via `--area`.
- `SpecCommands` — `Show` / `Add`. `Add` uses `allow_hyphen_values = true` on `--spec-content` and `--task-content` so markdown-bullet content doesn't get parsed as flags.
- `QueueCommands` — `Add` only (plus the MCP exposes `queue_list`/`queue_remove`/`queue_check`/`queue_reorder`).
- `IndexCommands` — 13 subcommands covering link CRUD, exports, queries, graph export.

### `src/commands/*.rs`

| File | Purpose |
|------|---------|
| `area.rs` | `unispec area *` (add/remove/list/rename/default/health/order). |
| `doctor.rs` | `unispec doctor` schema and state checks. *(Engine branch only, not present on main.)* |
| `index.rs` | `unispec index *` (link CRUD, find/list/cleanup/tags/graph/backlinks/exports/query/depends/lookup/callers). |
| `ingest.rs` | `unispec ingest run` (tree-sitter parsing). |
| `init.rs` | `unispec init` — creates `spec/`, `.agent/`, extracts the embedded default mode. |
| `init_editor.rs` | `unispec init --cursor`, `--cline`, etc.; drops integration files. |
| `migrate.rs` | Schema migration v1 + rollback. *(Engine branch only.)* |
| `pull.rs` | Legacy pull helpers. |
| `push.rs` | Legacy push helpers. (Now superseded by `topic.rs::run_push`.) |
| `queue.rs` | `run_queue_add` shared function. |
| `repo.rs` | `unispec pkg *` (community package repo). |
| `set.rs` | `unispec set <area>` (sets `default_area` in config). |
| `spec.rs` | `run_spec_add` shared function. |
| `change.rs` | `run_change_add` / `run_change_list` / `run_change_archive`. Manages `spec/<area>/<topic>/changes/` — proposing, listing, and archiving per-topic feature additions without touching the original `<topic>_spec.md`. Inspired by OpenSpec's change folders. |
| `topic.rs` | Heart of the CLI: `run_new`, `run_push`, `run_pull`, `run_show`, `run_list`, `run_delete`, `run_progress`, plus the auto-checkout/checkin helpers. |

### `src/fs/*.rs`

| File | Purpose |
|------|---------|
| `mod.rs` | Path helpers (`spec_dir`, `topic_path`, `global_config_dir`, `system_install_dir`). |
| `config.rs` | `.agent/config.toml` schema + IO (`load_config`, `save_config`, plus `ExtendedConfig` for ingest/paddy/protected_areas). |
| `index.rs` | `spec/index.toml` schema + queries. |
| `spec.rs` | Frontmatter parsing, completion-metadata helpers. |

### `src/agent/*.rs`

| File | Purpose |
|------|---------|
| `mode.rs` | Mode resolution: which mode is active, what areas it declares, which areas require readiness, what filenames the templates use. |
| `connector.rs` | Connector configuration + execution (`run_run` spawns the connector's command). |
| `code_parser.rs` | Tree-sitter parsing for the `code_analysis` / `code_parse` tools. |
| `auto/*.rs` | Legacy auto-build / auto-test / auto-verify orchestration. |
| `mod.rs` | `load_agent_config`, `current_mode`. |

### `src/mcp/`

| File | Purpose |
|------|---------|
| `mod.rs` | `get_tools()` — the static catalog of 31 `Tool` descriptors used by `tools/list`. |
| `server.rs` | The stdio JSON-RPC loop, `handle_request`, and the giant `call_tool` match. |

### `src/tui/`

| File | Purpose |
|------|---------|
| `mod.rs` | Module re-exports. |
| `main.rs` | TUI entry point (`pub fn main() -> Result<()>`). |
| `app.rs` | The big one — `App` struct, run loop, key handlers, render code, `open_file` (Enter), `queue_selected_topic` (`q` in TopicList). |
| `state.rs` | `AppState`, `NavState`, `TopicNode`, area + topic loading. |
| `platypus.rs` | Animation frames for Paddy. |
| `widgets/` | Custom ratatui widgets. |

## The embedded default mode

`src/commands/init.rs` declares:

```rust
static EMBEDDED_DEFAULT_MODE: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/.agent/modes/default");
```

This is the entire `.agent/modes/default/` directory tree (templates, area docs, workflows, skill, mode.toml, system prompts) compiled into the binary at build time via the `include_dir` crate. On `init`, if no `~/.config/unispec/.agent/modes/default/` and no `/usr/share/unispec/.agent/modes/default/` exist, the embedded copy is extracted into the project. This is why `cargo install unispec` followed by `unispec init` produces a fully-populated project with zero external state.

## The pipeline as code

The five-area pipeline is **not** hard-coded in the binary. It comes from `.agent/modes/default/mode.toml`:

```toml
[areas]
default = ["Staging", "Working", "Testing", "Fixing", "Build"]
protected = ["Build"]
default_area = "Staging"

[readiness]
queue_file = "queue.md"
required_for_push = true
areas = ["Staging", "Fixing"]
```

`run_push` in `topic.rs` calls `crate::agent::mode::area_requires_readiness(source)` — which loads the active mode's `mode.toml` and checks `[readiness].areas`. Custom modes can declare different areas and different gating. See [modes.md](modes.md).

## State files vs. authoritative files

`unispec` is designed so that `git diff` over the working tree is enough to understand a project's full state. There's no SQLite, no hidden cache, no daemon — every piece of state lives in markdown or TOML.

- `topic.md`, `<topic>_spec.md`, `<topic>_task.md` — human-authored, agent-touched.
- `area.md` — per-area description.
- `queue.md` — readiness queue.
- `spec/index.toml` — topic ↔ path links.
- `.agent/config.toml` — active mode, default area, connectors, ingest config.

Task completion lives **in the task file** as `- [ ]` / `- [x]` markdown checkboxes. The `tasks_complete` MCP tool flips them; `tasks_list` reads them.

## Pre-existing complexity that `everything` did not touch

- The `src/agent/auto/*` legacy orchestration is still there but largely unused on this branch — the prompts no longer reference it as a primary path.
- `src/agent/code_parser.rs` (tree-sitter) backs the `code_analysis` / `code_parse` tools and `unispec ingest run`. Untouched.
- `src/tui/widgets/` contains custom ratatui widgets for the topic-list rendering. Untouched.

## Where to start when changing things

| Want to change | Touch |
|----------------|-------|
| Add or rename a CLI subcommand | `src/cli/mod.rs` enum + `src/main.rs` dispatch + the relevant `src/commands/<file>.rs` |
| Add or rename an MCP tool | `src/mcp/mod.rs::get_tools()` (advertise) + `src/mcp/server.rs::call_tool` (handler) — both must agree |
| Change push behaviour | `src/commands/topic.rs::run_push` |
| Change init layout | `src/commands/init.rs::run_init` (and `EMBEDDED_DEFAULT_MODE` content under `.agent/modes/default/`) |
| Change a TUI keybinding | `src/tui/app.rs::handle_key_input` and the help text in the render block |
| Add a new shared function (CLI + MCP) | New `src/commands/<feature>.rs`, register in `src/commands/mod.rs`, call from both `main.rs` and `src/mcp/server.rs` |

For any deeper question, the canonical answer is the source. Memory docs and prompt docs can rot; the binary's behaviour is whatever `src/` says today.
