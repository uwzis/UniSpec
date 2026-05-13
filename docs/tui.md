# TUI Guide

The UniSpec TUI is a ratatui-based interactive interface for browsing and managing topics. Launch with `unispec` (no arguments).

## Launch

```bash
cd /path/to/your/project
unispec
```

If the current directory has no `spec/` directory, the binary exits with:

```
No spec folder found. Run unispec init to initialize a project.
```

Run `unispec init` first.

## Screens

The TUI moves between three navigation states. The current state determines what the list shows and what keys do.

### 1. Area selection

The opening screen. Shows every area declared by your active mode:

```
┌─ Status ──────────────────────────┐
│ Mode: default                     │
└───────────────────────────────────┘
┌─ Areas ───────────────────────────┐
│ → Staging                         │
│   Working                         │
│   Testing                         │
│   Fixing                          │
│   Build                           │
└───────────────────────────────────┘
┌─ Short ───────────────────────────┐
│ <one-line description from area.md>│
└───────────────────────────────────┘
┌─ Help ────────────────────────────┐
│  🡙 Move | 🡘  Navigate | ↵ Open | n: New | r: Remove | p: Push | f: Find | q: Quit │
└───────────────────────────────────┘
```

### 2. Topic list (inside an area)

Reached by pressing `→` while highlighting an area. Shows every topic in that area:

```
┌─ Status: Staging ─────────────────┐
┌─ Topics in Staging ───────────────┐
│ → ✅ user-login    (draft)        │
│   ⚙ payment-flow  (in-progress: 2/5)│
│   ✓ profile-page  (complete: 4/4) │
└───────────────────────────────────┘
```

The status badge:

| Status | Meaning |
|--------|---------|
| `draft` | Topic has no spec file yet (or `spec.md` is missing). |
| `in-progress: M/N` | M of N tasks are `[x]` checked. |
| `complete: N/N` | All tasks done. |

In the TopicList view, `q` adds the highlighted topic to that area's `queue.md`. To quit the TUI from here, press `←` first to back out to area selection, then `q`.

### Changes (when a topic has a `changes/` subtree)

If a topic has any changes (proposed via `unispec change add` or the MCP `change_add` tool), they appear in the same list as nested topics, with distinctive prefixes:

```
┌─ Topics in Staging ───────────────┐
│ → auth                            │
│     ✅ auth     (spec)            │
│     📁🔀 changes                  │
│       🔀 add-2fa                  │
│         🔀 add-2fa  (spec, 1/3)   │
│       🔀 add-oauth                │
│         🔀 add-oauth (spec)       │
└───────────────────────────────────┘
```

| Prefix | What it marks |
|--------|----------------|
| `📁🔀` | The `changes/` directory itself — the container holding every change for this topic. |
| `🔀` | A change folder (e.g. `add-2fa`), or a spec/task entry living inside a change folder. |

Navigation is identical to nested topics — press `→` to enter the `changes/` container, `→` again to enter a specific change folder, and `Enter` on any markdown file to open it in `$EDITOR`. The archive (`changes/archive/`) shows up the same way; archived changes still render with the `🔀` prefix so you can tell they're change-related.

### 3. Nested specs

When a topic has child topics (e.g. `auth/` containing `auth/login` and `auth/logout`), `→` from the parent drills into the nested view. Same key layout as the topic list.

### 4. Find results

Pressing `f` from a topic list runs a finder for files linked to the highlighted topic in the index. Results are shown as a flat list; `Enter` opens the file.

### 5. Input mode

Triggered by `n` (new), `r` (remove), `p` (push), or `/` (filter). The help row turns into a prompt:

```
┌─ Help ────────────────────────────┐
│ Name for new topic: │
└───────────────────────────────────┘
```

Type your response, press `Enter` to confirm, or `Esc` to cancel.

## Keybindings

### Universal

| Key | Action |
|-----|--------|
| `↑` / `↓` | Move between items |
| `→` | Enter the highlighted area or topic |
| `←` | Go back |
| `Enter` | Open the highlighted file in `$EDITOR`, then `nano`, then `vi`. If none are on `PATH`, the file contents print to the terminal and you press Enter to return. |
| `n` | Start "new" input prompt (varies by context). |
| `r` | Start "remove" input prompt (varies by context). |
| `p` | Start "push" input prompt — type the target area. |
| `f` | Find files linked to the highlighted topic. |
| `/` | Filter the current list. |
| `\` | Toggle the platypus mascot on/off. |

### Context-sensitive

| Key | In TopicList view | In every other view |
|-----|-------------------|---------------------|
| `q` (or `Q`) | Add the highlighted topic to that area's `queue.md` (success/error shown in the help row, platypus flips Happy/Sad). | Quit the TUI. |

The help row at the bottom reflects this: in a TopicList you see `q: Queue`; everywhere else `q: Quit`.

## Editor handoff

When you press `Enter` on a file, the TUI:

1. Resolves an editor: `$EDITOR` → `nano` → `vi`. `$EDITOR` is split on whitespace, so values like `vim -O` work (first token is the binary, the rest forward as args, the file path is appended).
2. Suspends itself: leaves the alt screen, disables raw mode.
3. Spawns the editor blocking.
4. On editor exit, re-enables raw mode, re-enters the alt screen, issues a `Clear` to the terminal, and flags the next render frame to force a full repaint (otherwise ratatui's diff renderer would skip cells it thinks are unchanged but that the editor visibly clobbered).

If no editor is on `PATH`, the TUI dumps the file contents to the terminal, prints `--- end of file (press Enter to return) ---`, and waits for one line on stdin before returning. The same suspend/restore-and-repaint sequence applies.

## The platypus

Paddy the platypus runs as an optional sidebar animation. Toggle visibility with `\`. State machine:

| State | Trigger |
|-------|---------|
| Idle | Default |
| Happy | Success message (e.g. `q` queued a topic) |
| Sad | Error message (e.g. `q` failed because no topic selected) |
| Love | A topic was pushed to another area |
| Searching | `f` find started |
| Working | TopicList shows topics with `in-progress: M/N > 0` |
| Celebrating | TopicList shows topics where all tasks complete |

Disable globally via `.agent/config.toml`:

```toml
paddy_enabled = false
```

Or per-launch via the TUI itself (`\`).

## TUI vs. CLI

Everything the TUI does is also available as a CLI subcommand. The TUI is convenience; the CLI and MCP are the canonical surfaces. There is no behaviour exclusive to the TUI.

| TUI action | CLI equivalent |
|------------|----------------|
| `n` in area-list view | `unispec area add <name>` |
| `n` in topic-list view | `unispec topic add <name> --short "..." --content "..."` |
| `r` | `unispec area remove <name>` or `unispec topic remove <name>` |
| `p` | `unispec topic push <name> --area <target> --from <source>` |
| `q` in topic-list | `unispec queue add <name>` |
| `Enter` | open the file in `$EDITOR` from the shell |
| `f` | `unispec index find <query> --by topic` |

## See also

- [CLI Reference](cli-reference.md) — full subcommand surface.
- [Workflow](workflow.md) — pipeline rules and queue gates.
- [Quickstart](quickstart.md) — end-to-end walkthrough.
