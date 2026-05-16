# CLI Reference

Complete reference for all unispec CLI commands.

## topic

### topic add
Create a new topic in an area.

    unispec topic add <name> --short <description> --content <body> [--area <area>]

- area defaults to the value in .agent/config.toml, or Staging if not set
- short is a one-line description shown in the TUI
- content must be at least 20 characters

### topic list
List all topics in an area.

    unispec topic list [--area <area>]

- area defaults to config, or Staging

### topic push
Move a topic from one area to another.

    unispec topic push <name> --area <target> --from <source>

- both --area and --from default to config area if not provided
- topics in Staging and Fixing must be queued before pushing (see queue add)

### topic pull
Pull a topic back from another area.

    unispec topic pull <name> [--area <area>]

### topic remove
Delete a topic from an area.

    unispec topic remove <name> [--area <area>] [--force]

## spec

### spec add
Create a spec and task file for a topic.

    unispec spec add --topic <name> --short <description> --spec-content <body> --task-content <tasks> [--area <area>]

- area defaults to config area
- task-content supports markdown bullet lists (- [ ] task)
- spec-content and task-content must be at least 11 characters

### spec show
Show the master spec file.

    unispec spec show

## change

Change management for topics — see [change-management.md](change-management.md) for the full guide. Lets you propose features for an existing topic without overwriting its original spec.

### change add
Create a new change folder under `spec/<area>/<topic>/changes/<change>/`. The topic must already exist; the original `<topic>_spec.md` is not touched.

    unispec change add --topic <name> --change <id> --proposal <text> --spec-content <body> --task-content <tasks> [--design <text>] [--area <area>]

- `--topic, -t` — existing topic name (required)
- `--change, -c` — change identifier, kebab-case recommended (e.g. `add-2fa`) (required)
- `--proposal` — why this change exists; must be at least 11 characters (required)
- `--spec-content` — new requirements introduced by this change; at least 11 characters; supports markdown bullets (required)
- `--task-content` — new tasks introduced by this change; at least 11 characters; supports markdown bullets (required)
- `--design` — optional technical-approach body
- `--area, -a` — area the topic lives in; defaults to `.agent/config.toml`'s `area`, then `Staging`

Files written:

    spec/<Area>/<topic>/changes/<change>/
    ├── proposal.md         # always written
    ├── design.md           # only if --design supplied
    ├── <change>_spec.md
    └── <change>_task.md

Example:

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

### change list
List all changes for a topic with their status (`proposed` / `in-progress` / `complete` / `archived`) and which files each one contains.

    unispec change list --topic <name> [--area <area>] [--archived]

- `--topic, -t` — topic name (required)
- `--area, -a` — area name (defaults to config, then Staging)
- `--archived` — also include changes under `changes/archive/`

Example output:

    Changes for 'auth' in Staging/:
      - add-2fa [in-progress] (proposal, design, spec, task)
      - add-oauth [proposed] (proposal, spec, task)

### change archive
Move a completed change into `changes/archive/<change>/`, **merging any delta sections into the canonical `<topic>_spec.md` first**. Fails if the change doesn't exist or if a directory with the same name already exists under `archive/`.

    unispec change archive --topic <name> --change <id> [--area <area>]

- `--topic, -t` — topic name (required)
- `--change, -c` — change name to archive (required)
- `--area, -a` — area name (defaults to config, then Staging)

**Delta merge.** If the change's `<change>_spec.md` contains any of these sections:

- `## ADDED Requirements` — new `### Requirement: <name>` blocks appended to the canonical spec
- `## MODIFIED Requirements` — replace matching `### Requirement:` blocks in place
- `## REMOVED Requirements` — delete matching blocks
- `## RENAMED Requirements` — `- FROM: ### Requirement: Old` / `- TO: ### Requirement: New`

then the canonical spec is rewritten with those edits applied (in order RENAMED → REMOVED → MODIFIED → ADDED) before the change directory is moved to archive. A change spec with no delta sections is archived without touching the canonical spec.

Example:

    unispec change archive --topic auth --change add-2fa

See [change-management.md](change-management.md) for the full delta grammar.

## next

Get a structured next-action payload for a topic — the recommended entry point for any agent driving the pipeline.

    unispec next --topic <name> [--area <area>] [--json]

- `--topic, -t` — topic name (required)
- `--area, -a` — area name (defaults to config, then Staging)
- `--json` — emit the full payload as JSON instead of the human summary

The payload contains:

| Field | Meaning |
|---|---|
| `status` | `not-started` / `in-progress` / `complete` / `blocked` |
| `open_tasks`, `completed_tasks` | each task line with `index`, `text`, `from_change` (`None` = main task file, `Some(name)` = inside a change) |
| `pending_changes`, `archived_changes` | per-change status and which files exist |
| `context_files` | relative paths the agent should read (topic.md, spec, task, every pending change's proposal/design/spec/task) |
| `rules` | area-specific constraints (e.g. Staging: "Do not write code yet"; Working: "Spec is frozen"; Build: "Do not modify") |
| `next_action` | one sentence telling the agent exactly what to do next |
| `blockers` | reasons the agent cannot proceed (e.g. topic not in queue when leaving Staging) |

Example (human form):

    Topic: auth
    Area:  Working
    Status: in-progress
    Open tasks (2):
      [ ] Verify TOTP codes (change: add-2fa, idx 0)
      [ ] Issue recovery codes (change: add-2fa, idx 1)
    Next action:
      → Work on task 0 of change 'add-2fa': Verify TOTP codes.

## analyze

Cross-artifact consistency checker. Runs read-only — no files are mutated.

    unispec analyze --topic <name> [--area <area>] [--json]

- `--topic, -t` — topic to analyze (required)
- `--area, -a` — area name (defaults to config, then Staging)
- `--json` — emit findings as JSON

Six checks:

1. **Duplication** — a `### Requirement:` name appears in both the canonical spec and a pending change (outside `## MODIFIED Requirements`) → WARNING
2. **Missing task coverage** — a `### Requirement:` row with no task line referencing it → ERROR
3. **Ambiguous language** — requirements containing `fast / secure / scalable / easy / simple / good / better / best / quick` without a numeric metric or unit token → WARNING
4. **Empty sections** — `## <heading>` with no content before the next `##` → WARNING
5. **Constitution alignment** — surfaces the constitution version as INFO so the agent re-evaluates manually
6. **Task completion** — `[x] / [ ]` ratio across main + pending change task files → INFO

Example output:

    Analysis for 'auth' in Staging/

    ERROR: Missing task coverage
      Requirement 'Refresh Token' has no corresponding task.

    WARNING: Ambiguous language
      Requirement 'Login' contains 'securely' without a measurable metric.

    Summary: 1 error, 1 warning, 2 info

## workspace

Multi-repo coordination across linked UniSpec projects. State lives in `.unispec-workspace/workspace.yaml` in the workspace root.

### workspace init
Create the workspace file in the current directory.

    unispec workspace init <name>

Writes `.unispec-workspace/workspace.yaml`:

    name: <name>
    version: 1
    links: {}

### workspace link
Add a named pointer to another UniSpec project.

    unispec workspace link <name> <path>

The path is recorded as absolute. Re-running with the same `<name>` updates the path.

### workspace list
List linked repos and their status (does the path exist; does it have a `spec/` and `.agent/` dir).

    unispec workspace list

### workspace status
Show every topic across every linked repo.

    unispec workspace status [--json]

Without `--json` prints one row per topic prefixed by area; with `--json` returns a structured payload (matches the `workspace_status` MCP tool).

## queue

### queue add
Add a topic to an area's readiness queue. Required before pushing from Staging or Fixing.

    unispec queue add <topic> [--area <area>] [--position <n>]

- area defaults to config area
- position defaults to -1 (append to end)

## index

### index add
Link a file to a topic.

    unispec index add --topic <name> --path <file> [--link-type <type>]

### index list
List all indexed files for a topic.

    unispec index list --topic <name>

## mcp

Start the MCP server for AI editor integration.

    unispec mcp [--path <project-dir>]

The server speaks JSON-RPC over stdio. Configure your editor to run this command as an MCP server pointed at your project directory.

## TUI

Launch the interactive terminal UI.

    unispec

### Keybindings

    Arrow keys   Navigate
    →            Enter an area or topic
    ←            Go back
    Enter        Open topic file in editor ($EDITOR, nano, or vi)
    n            Create new topic
    r            Remove topic
    p            Push topic to next area
    f            Find topic
    q            Add topic to queue (when inside an area)
    q            Quit (when at the area selection screen)
    \            Toggle Paddy the platypus

## Pipeline

The full spec-driven workflow:

    Staging → Working → Testing → Fixing → Build

Topics must be queued before leaving Staging or Fixing:

    unispec queue add <topic>
    unispec topic push <topic> --area Working --from Staging

Working, Testing, and Fixing can be pushed freely without queuing.
