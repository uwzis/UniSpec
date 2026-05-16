# Getting Started with UniSpec

UniSpec is a spec-driven workflow tool. You write a spec for each feature, then move it through a fixed pipeline of areas (default: `Staging → Working → Testing → Fixing → Build`). Code is linked back to specs, so AI agents and humans both know exactly what's being built and why.

This guide walks through the basics.

---

## What is UniSpec?

For each feature, you produce three files under `spec/<Area>/<topic>/`:

| File | Purpose |
|------|---------|
| `topic.md` | What this topic is — short description, sub-topics, notes. |
| `<topic>_spec.md` | **What** you're building: requirements, examples, data model. |
| `<topic>_task.md` | The implementation tasks, each tracked with `- [ ]` / `- [x]`. |

UniSpec's MCP server exposes tools that an AI agent (Claude, Cursor, Windsurf, Cline, etc.) calls directly — so the agent can create topics, write specs, flip checkboxes, and link code without you copy-pasting commands.

---

## Installation

```bash
# Linux / macOS (Cargo)
cargo install unispec

# Arch Linux (AUR)
yay -S unispec
```

Verify:

```bash
unispec --version
```

---

## First project

### 1. Initialize

```bash
mkdir my-first-project
cd my-first-project
unispec init
```

This creates:

```
my-first-project/
├── AGENTS.md                       # universal AI-tool entry point
├── spec/
│   ├── Staging/area.md
│   ├── Working/area.md
│   ├── Testing/area.md
│   ├── Fixing/area.md
│   └── Build/area.md
└── .agent/
    ├── config.toml
    ├── constitution.md             # project non-negotiables (5 default principles)
    ├── skill.md
    ├── modes/default/
    └── workflows/
```

The areas come from the default mode (`.agent/modes/default/mode.toml`). `AGENTS.md` is a universal fallback any AI agent will read; `.agent/constitution.md` carries the project's non-negotiable rules — edit it to reflect your team's actual constraints.

### 2. Launch the TUI

```bash
unispec
```

Areas appear on the left; topics show up inside each area once you create them. The platypus mascot is off by default — toggle with `\`.

### 3. Create a topic

In the TUI:
1. `↓` to highlight `Staging`, `→` to enter.
2. `n` to create a topic.
3. Type a name (kebab-case recommended, e.g., `user-login`).
4. `Enter`.

That creates `spec/Staging/user-login/topic.md`. The spec file (`user-login_spec.md`) and task file (`user-login_task.md`) are written by `unispec spec add` (or by an agent via the MCP `spec_add` tool):

```bash
unispec spec add \
  --topic user-login \
  --short "Email/password login with JWT" \
  --spec-content "Users submit email/password to POST /login. ..." \
  --task-content "- [ ] Implement POST /login
- [ ] Add JWT signing
- [ ] Write tests"
```

### Adding more features to an existing topic — use `change add`, not `spec add`

Once a topic has a spec, **don't** call `unispec spec add` again to layer on more requirements — it will silently overwrite `user-login_spec.md` and `user-login_task.md`, losing your original design. Instead, propose a *change*:

```bash
unispec change add \
  --topic user-login \
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

This writes a new folder `spec/Staging/user-login/changes/add-2fa/` containing `proposal.md`, `design.md`, `add-2fa_spec.md`, and `add-2fa_task.md`. The original `user-login_spec.md` and `user-login_task.md` are untouched.

To see what changes are pending:

```bash
unispec change list --topic user-login
```

When the change ships, archive it:

```bash
unispec change archive --topic user-login --change add-2fa
```

See [change-management.md](change-management.md) for the full guide.

---

## File layout the tools create

```
spec/
└── <Area>/
    └── <topic>/
        ├── topic.md
        ├── <topic>_spec.md      # slashes/spaces in <topic> become '-'
        └── <topic>_task.md
```

For a nested topic `auth/login`, the files are `auth-login_spec.md` and `auth-login_task.md` inside `spec/<Area>/auth/login/`.

---

## The pipeline

| Area | Stage |
|------|-------|
| Staging | Writing the spec. |
| Working | Implementing code in `src/`. |
| Testing | Running build/test pipelines. |
| Fixing | Repairing issues found in Testing. |
| Build | Done. Treated as immutable. |

To move a topic between areas:

```bash
unispec topic push <topic> --area <target> --from <source>
```

Both `--area` and `--from` default to the area set in `.agent/config.toml` when omitted (falling back to `Staging`). Or in the TUI, highlight the topic and press `p`.

`Staging` and `Fixing` are gated by a **readiness queue**: a topic must appear in `spec/<area>/queue.md` before it can be pushed out. Use the CLI directly:

```bash
unispec queue add <topic>                        # add to current area's queue
unispec queue add <topic> --area Fixing          # explicit area override
```

The MCP `queue_add` tool exposes the same logic for AI agents. From inside the TUI, you can also press `q` while highlighting a topic to add it to the current area's queue.

---

## TUI cheatsheet

| Key | Action |
|-----|--------|
| `↑` / `↓` | Move between items |
| `→` | Enter an area or topic |
| `←` | Go back |
| `Enter` | Open the highlighted file in `$EDITOR`, then `nano`, then `vi`; falls back to printing content if none found |
| `n` | Create new topic |
| `r` | Remove topic (confirm) |
| `p` | Push topic to another area |
| `f` | Find files linked to this topic |
| `/` | Search/filter |
| `\` | Toggle the platypus mascot |
| `q` | Add the highlighted topic to the area queue (when inside a TopicList) |
| `q` | Quit (when on the area-selection screen) |

Your specs are plain Markdown — open them in any editor. `unispec topic list` works from the command line too.

---

## Quick CLI reference

```bash
unispec                                        # launch TUI
unispec init                                   # initialize project
unispec topic add <name> --short "..." \
  --content "..."                              # create a topic in the default area
unispec topic list                             # list topics in the default area
unispec topic progress                         # task progress per area
unispec spec add --topic <name> --short "..." \
  --spec-content "..." --task-content "..."    # write spec + task files (first time only)
unispec change add --topic <name> --change <id> \
  --proposal "..." --spec-content "..." \
  --task-content "..."                          # add a feature to an existing topic
unispec change list --topic <name>             # list pending changes for a topic
unispec change archive --topic <name> \
  --change <id>                                 # mark a change complete; merges deltas into the spec
unispec next --topic <name>                    # structured next-action payload for an agent
unispec analyze --topic <name>                 # cross-artifact consistency checker
unispec workspace init <name>                  # create a multi-repo workspace
unispec workspace link <name> <path>           # link a UniSpec project into the workspace
unispec workspace status                       # combined topic list across linked repos
unispec queue add <name>                       # add to the area readiness queue
unispec topic push <name> --area <target> \
  --from <source>                              # move between areas
unispec area list
unispec area health
unispec index add --topic <name> --path <path>
unispec mode list
unispec mode activate <mode>
unispec mcp                                    # start the MCP server (for agents)
```

For the complete CLI surface, see [cli-reference.md](cli-reference.md) (or the legacy [commands.md](commands.md)).

---

## Connecting an AI agent

UniSpec speaks MCP. To let an agent drive the pipeline:

```bash
unispec init --cursor --cline --windsurf --claude_code
```

Drops workflow files into the right editor folders. Or wire the MCP server directly:

```json
{
  "mcpServers": {
    "unispec": {
      "command": "unispec",
      "args": ["mcp"],
      "cwd": "/abs/path/to/project"
    }
  }
}
```

See [mcp.md](mcp.md) for the full MCP tool surface and per-editor configs.

---

## Indexing code to specs

When an agent (or you) writes a code file, link it to a topic so the spec ↔ code relationship is preserved:

```bash
unispec index add --topic user-login --path src/auth/login.rs \
  --link-type implementation \
  --tags "auth,backend" \
  --annotation "Core login handler"
```

The link is stored in `spec/index.toml`. Agents can query it with `index_find`, `index_list`, `index_backlinks`, and `index_graph`.

See [indexing.md](indexing.md) for the full feature set.

---

## What's next?

- [Quickstart](quickstart.md) — same content, five minutes, copy-pasteable
- [Workflow](workflow.md) — the five-area pipeline rules
- [Next](next.md) — the structured agent feed (call before every action)
- [Analyze](analyze.md) — cross-artifact consistency checker
- [Constitution](constitution.md) — project non-negotiables
- [Workspaces](workspaces.md) — multi-repo coordination
- [Change Management](change-management.md) — adding features to existing topics without overwriting the spec
- [Areas](areas.md) — what each area is for
- [TUI Guide](tui.md) — every keybinding and screen
- [CLI Reference](cli-reference.md) — every subcommand and flag (the up-to-date reference)
- [Commands Reference](commands.md) — older long-form CLI documentation
- [MCP Tools Reference](mcp-tools-reference.md) — every MCP tool, JSON-RPC examples
- [MCP Integration](mcp-integration.md) — editor configs
- [Connectors](connectors.md) — shell commands as MCP tools
- [Architecture](architecture.md) — codebase layout
- [Troubleshooting](troubleshooting.md) — common errors
- [MCP Integration](mcp.md) — every MCP tool the server exposes
- [Modes](modes.md) — customize the pipeline (areas, templates, workflows)
- [Configuration](configuration.md) — `.agent/config.toml`, env vars, exit codes
- [Indexing](indexing.md) — link code to specs

---

*Write the spec first. Code second.*
