# Changelog

All notable changes to UniSpec are tracked here. Format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/); this project uses [Semantic Versioning](https://semver.org/).

The current line of work lives on the `everything` branch. PR0 through PR7 are the commit-by-commit history; this changelog rolls them up into a single release note for **0.0.8**.

---

## [Unreleased] — `feature/change-management`

### Added

#### Change management (OpenSpec-inspired)

A new way to propose features for an existing topic without overwriting its original spec. Changes live in a child folder under the topic and travel with it through the pipeline.

On-disk layout:

```
spec/<Area>/<topic>/
├── topic.md
├── <topic>_spec.md         ← original, never modified
├── <topic>_task.md
└── changes/
    ├── <change>/
    │   ├── proposal.md
    │   ├── design.md           # optional
    │   ├── <change>_spec.md
    │   └── <change>_task.md
    └── archive/<archived-change>/
```

- **CLI:** `unispec change add` / `unispec change list` / `unispec change archive`. All three accept `--area` (defaulting via config to `Staging`); `add` takes `--topic`, `--change`, `--proposal`, `--design` (optional), `--spec-content`, `--task-content`; `list` takes `--topic` and `--archived`; `archive` takes `--topic` and `--change`.
- **MCP:** Three new tools — `change_add`, `change_list`, `change_archive` — bringing the static tool count to **34**. Required/optional arg shapes match the CLI surface.
- **Shared module:** `src/commands/change.rs` exposes `run_change_add` / `run_change_list` / `run_change_archive`, so CLI and MCP call into the same code (same pattern as `commands/spec.rs` and `commands/queue.rs`).
- **TUI:** Topics that live under `changes/` now render with a `🔀` prefix; the `changes/` directory itself renders with `📁🔀`. The standard `→` / `←` navigation enters/exits a change folder the same way as any nested topic. Implemented via `TopicNode::is_change` and `TopicNode::is_changes_container` plus a recursion-aware loader.
- **Docs:** New `docs/change-management.md` (full guide). Updated `cli-reference.md`, `commands.md`, `mcp-tools-reference.md`, `mcp-integration.md`, `architecture.md`, `workflow.md`, `quickstart.md`, `getting-started.md`, `areas.md`, `tui.md`, `troubleshooting.md`, and the `default` mode's `unispec:spec.md`, `unispec_basics.md`, `build.md`, `verify.md`.

The original `<topic>_spec.md` is never read or written by any of the new code paths — it remains the source of truth. `spec_add` itself doesn't enforce "first call only" (it will silently overwrite if rerun), so the convention is operational: use `spec_add` exactly once per topic, then reach for `change_add` for every subsequent feature.

---

## [0.0.8] — current `everything` branch

This release consolidates seven topic branches that closed end-to-end usability and correctness gaps in the CLI, init, MCP server, and TUI. After this, an AI agent or human can drive a topic from creation through the five-area pipeline to Build using either the CLI or MCP, without hitting silent failures.

### Added

#### CLI
- **`unispec spec add`** — CLI subcommand to create `<topic>_spec.md` and `<topic>_task.md` for a topic. Mirrors the MCP `spec_add` tool. Both `--spec-content` and `--task-content` accept multi-line markdown bullets via `allow_hyphen_values = true` — no `=` workaround needed.
- **`unispec spec show`** — shows the master spec at `spec/master.md` (preserved behaviour from the previous single-command `unispec spec`).
- **`unispec queue add`** — CLI subcommand to add a topic to a readiness queue. Mirrors the MCP `queue_add` tool. Accepts `--area` and `--position`; defaults area from config.
- Shared command modules `src/commands/spec.rs` and `src/commands/queue.rs` — both CLI and MCP call into the same function, so behaviour changes apply uniformly to both surfaces.

#### MCP
- **Six previously-unimplemented handlers now exist** in `src/mcp/server.rs::call_tool`:
  - `tasks_list` — list every `- [ ]` / `- [x]` task line in `<topic>_task.md` with index, line, status, text.
  - `tasks_complete` — flip the Nth task (0-based) to `[x]`.
  - `tasks_incomplete` — flip back to `[ ]`.
  - `notes_read` — read the `## Notes` section of the task file.
  - `notes_add` — append a dated note. Creates the `## Notes` section if missing.
  - `queue_reorder` — move a topic to a new 0-based slot within its area's queue.
- All six tools were advertised in `mod.rs::get_tools()` before this release but rejected at runtime with `Error: Unknown tool: <name>`.

#### TUI
- `q` key in a TopicList view now **adds the highlighted topic to that area's `queue.md`**. In every other navigation state, `q` keeps its long-standing "quit the TUI" meaning. The help row shows the active meaning (`q: Queue` vs. `q: Quit`).
- Enter on a topic file now resolves an editor in order: `$EDITOR` → `nano` → `vi`. If none are on `PATH`, the file contents are printed and the TUI waits for Enter before returning. Replaces the previous `xdg-open` call that failed on WSL/headless setups.

#### Docs
- New: `docs/quickstart.md`, `docs/workflow.md`, `docs/tui.md`, `docs/areas.md`, `docs/connectors.md`, `docs/architecture.md`, `docs/mcp-integration.md`, `docs/mcp-tools-reference.md`, `docs/cli-reference.md`, `docs/troubleshooting.md`, `CHANGELOG.md`.
- Updated: `README.md`, `CONTRIBUTING.md`, `docs/mcp.md`, `docs/commands.md`, `docs/modes.md`, `docs/getting-started.md`.

### Changed

#### Init
- `unispec init` now creates **all 5 pipeline areas**: `Staging`, `Working`, `Testing`, `Fixing`, `Build`. Previously only created 3 (`Staging`, `Working`, `Build`).
- The default mode is now **embedded in the binary** via `include_dir`. `unispec init` extracts `.agent/modes/default/` (templates, area docs, workflows, skill, mode.toml, system prompts) into the project on first run — no system install required, `cargo install unispec` is now self-contained.
- Init looks up the default mode from three sources, in order: `~/.config/unispec/.agent/modes/default/`, `/usr/share/unispec/.agent/modes/default/`, then the embedded copy. The init success line reports which source supplied the mode.
- Replaced the hardcoded `simple` mode lookup (which never existed in the source) with the canonical `default` mode lookup.
- `.agent/workflows/` is populated with every workflow file from the active mode (`build.md`, `ingest.md`, `test.md`, `unispec:spec.md`, `verify.md`), not just three osdd-named files.
- `.agent/skill.md` is copied from the active mode.
- Init banner now says "default mode" instead of "Simple Mode".

#### CLI
- `unispec topic add` — `--area` is now optional (was a required `default_value = "Working"`). Resolves to `.agent/config.toml`'s `area` field, then `Staging`.
- `unispec topic push` — `<area>` positional argument replaced by `--area <target>` named flag. `--from <source>` is also optional and resolves the same way.
- `unispec topic list`, `topic pull`, `topic remove` — `--area` is now `Option<String>` and resolves via the same config-default-then-Staging path.
- `unispec topic remove` — gained an `--area` flag.
- `unispec topic push` — when the target area directory doesn't exist, push **auto-creates** it on the fly with a minimal `area.md` stub. Useful when init ran with fewer areas than the active mode declares.
- `unispec topic show` — falls back to the configured area when neither `--from` nor `--all` is given.
- `unispec topic push` is now a **real move**: source directory is removed after the copy. Was previously a copy that left both source and destination populated.
- The `unispec spec` single command became `unispec spec <subcommand>` (with `show` and `add`).

#### MCP
- `unispec_read_spec` and `read_asset { asset_type: "spec" | "task" }` now resolve files using the **`<topic-safe>_spec.md` / `<topic-safe>_task.md`** convention that `spec_add` writes. Previously read from `spec.md` / `task.md` and returned empty strings on a project written by `spec_add`.
- `unispec_read_spec` returns a clear error if the spec or task file is missing instead of silently returning empty strings.
- `spec_add` MCP handler refactored to call the shared `crate::commands::spec::run_spec_add`. Identical externally-observable behaviour; deduplicates ~125 lines.
- `queue_add` MCP handler refactored to call the shared `crate::commands::queue::run_queue_add`.

#### Topic push
- The post-copy filename loop that synthesised legacy `specs.md` and `tasks.md` files alongside the `<topic-safe>_spec.md` / `<topic-safe>_task.md` files has been removed. The destination directory now contains exactly the files that were in the source, byte-for-byte. No duplicate file with double-wrapped frontmatter.
- `auto_checkin` (the post-push ownership clear when pushing into Build) now uses the `<topic-safe>_spec.md` filename. Was previously looking for `spec.md` and failing the push with `❌ Spec file not found for topic '<name>'` even after the files had already been moved.
- `auto_checkin` returns a silent informational message when no spec exists, rather than erroring. This is the correct behaviour for topics that don't carry checkout metadata.

### Fixed

#### Preflight (carried across every branch)
- `src/main.rs`: `TopicCommands::Add` pattern destructured only `topic` and `area`, even though the struct also carries `short` and `content`. Caused `cargo build` to fail on stock `main`. Now destructures all four fields and forwards them.
- `src/main.rs`: `crate::agent::auto::build::run_auto_build` was called with `&topic` (a `&Option<String>`) where `Option<&str>` was expected. Now uses `topic.as_deref()`.

#### CLI
- `topic push` no longer creates duplicate legacy `specs.md` / `tasks.md` files in the destination (PR5 Bug 2).
- `auto_checkin` no longer surfaces "Spec file not found" after a successful push to Build (PR6 Fix 1).
- Push to a missing target area no longer errors with "Target area not found" — the area is auto-created (PR6 Fix 2).

#### MCP
- `tasks_list` / `tasks_complete` / `tasks_incomplete` / `notes_read` / `notes_add` / `queue_reorder` no longer return "Unknown tool" (PR2 missing handlers).
- `unispec_read_spec` no longer reads from `spec.md` / `task.md` instead of `<topic>_spec.md` / `<topic>_task.md` (PR2 filename fix).
- `read_asset` with `asset_type: "spec"` or `"task"` no longer fails with "Spec file not found" / "Task file not found" against projects written by `spec_add` (PR2 filename fix).

#### TUI
- Pressing Enter on a topic no longer fails with `Failed to open: No such file or directory` in WSL or other environments without `xdg-open`. Resolves via `$EDITOR` → `nano` → `vi` → print-and-wait fallback.
- After returning from an editor, the TUI now fully repaints rather than showing stale content. Fixed by issuing `terminal.clear()` on the ratatui handle (invalidates its diff buffer) and a crossterm `Clear(All) + MoveTo(0,0)` (clears the visible screen immediately).

#### Docs
- `docs/getting-started.md`: replaced positional `topic push <name> <area>` examples with `--area`/`--from` named-flag examples.
- `docs/getting-started.md`: replaced the "*no CLI command for queue add yet*" disclaimer with the actual `unispec queue add` invocation.
- `docs/getting-started.md`: TUI keybind table reflects context-sensitive `q` and the editor-resolution chain.
- `docs/getting-started.md`: index example uses `--link-type` (kebab-case) instead of the incorrect `--link_type`.

### Branch / merge structure

The seven topic branches that fed into `everything`:

| Branch | Scope |
|--------|-------|
| `pr1-prompt-improvements` | Doc / agent-prompt rewrites across `.agent/` and `docs/` |
| `pr2-engine-fixes` | Init 5 areas; push moves; `unispec_read_spec` / `read_asset` filename fix; six missing MCP handlers |
| `pr3-init-templates` | `include_dir`-embedded default mode + init rewrite |
| `pr4-cli-fixes` | `unispec spec add` CLI; config-default-area for `topic add` |
| `pr5-cli-polish` | `spec add` markdown-bullet handling; remove duplicate `specs.md` writes; all topic subcommands respect config area; `topic push --area/--from` named flags |
| `pr6-final-fixes` | `auto_checkin` filename fix; auto-create missing target areas during push |
| `pr7-queue-add` | `unispec queue add` CLI subcommand; shared `commands::queue::run_queue_add` |

All seven were cherry-picked onto `everything` from `main` in order, with one trivial textual conflict in `init.rs` (PR3 vs PR2 both edited the comment above the area array; PR3's wording was kept).

### Test status

- `cargo build` is clean (no errors).
- The repo ships no unit tests (`cargo test` reports `0 passed; 0 failed`), so green E2E behaviour is verified manually via the smoke-test pattern in `CONTRIBUTING.md`.

---

## Earlier history

Pre-`everything` history (from `main`) is recorded in `git log` and is not enumerated here. The shipped binary versions before this release were untagged work-in-progress states; the README claimed 0.0.4, `Cargo.toml` said 0.0.7, the AUR PKGBUILD said 0.0.3. `Cargo.toml` is the source of truth — this release brings everything to **0.0.8**.

---

[0.0.8]: https://github.com/itsramananshul/UniSpec/tree/everything
