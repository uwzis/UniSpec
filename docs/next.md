# `next` — Structured Agent Feed

A single CLI / MCP call that returns the agent's full decision context for a topic. Call it before every action so the agent doesn't have to re-parse markdown to figure out what to do.

## Why it exists

UniSpec used to require agents to read raw `<topic>_spec.md`, `<topic>_task.md`, plus every pending change folder, then reason about queue gating, area conventions, and which task to do next on their own. That's a lot of context for every step.

`next` composes those data sources once and hands the agent a structured payload with a one-sentence `next_action`. Closes the biggest gap UniSpec had vs. OpenSpec's `openspec instructions --json` and spec-kit's `setup-plan.sh --json`.

## CLI

```bash
unispec next --topic <name> [--area <area>] [--json]
```

Without `--json` prints a human summary; with `--json` emits the full payload.

Example:

```
$ unispec next --topic auth --area Working

Topic: auth
Area:  Working
Status: in-progress

Open tasks (2):
  [ ] Verify TOTP codes (change: add-2fa, idx 0)
  [ ] Issue recovery codes (change: add-2fa, idx 1)

Completed tasks (3):
  [x] Implement POST /login (idx 0)
  [x] Implement POST /logout (idx 1)
  [x] Write tests (idx 2)

Pending changes:
  - add-2fa [in-progress]

Context files:
  - spec/Working/auth/topic.md
  - spec/Working/auth/auth_spec.md
  - spec/Working/auth/auth_task.md
  - spec/Working/auth/changes/add-2fa/proposal.md
  - spec/Working/auth/changes/add-2fa/design.md
  - spec/Working/auth/changes/add-2fa/add-2fa_spec.md
  - spec/Working/auth/changes/add-2fa/add-2fa_task.md

Rules:
  - Implementation phase. Write code under src/ and flip task checkboxes using tasks_complete.
  - The spec is frozen — do not modify <topic>_spec.md.
  - Link every new source file to the topic via index_add with link_type "implementation".

Next action:
  → Work on task 0 of change 'add-2fa': Verify TOTP codes.
```

## MCP

```json
{ "name": "next", "arguments": { "topic": "auth", "area": "Working" } }
```

Returns the same shape as `--json` plus `success: true`.

## Payload reference

| Field | Type | Meaning |
|---|---|---|
| `topic`, `area` | string | Echo of the call |
| `status` | string | `not-started` / `in-progress` / `complete` / `blocked` |
| `open_tasks[]` | TaskItem | Tasks with `[ ]` |
| `completed_tasks[]` | TaskItem | Tasks with `[x]` |
| `pending_changes[]` | ChangeItem | Non-archived changes |
| `archived_changes[]` | ChangeItem | Changes under `changes/archive/` |
| `context_files[]` | string | Relative paths the agent should load |
| `rules[]` | string | Area-specific constraints |
| `next_action` | string | One sentence telling the agent exactly what to do |
| `blockers[]` | string | Reasons the agent cannot proceed — must be empty before action |

### TaskItem

```json
{
  "index": 2,
  "text": "Verify TOTP codes",
  "completed": false,
  "from_change": "add-2fa"
}
```

- `index` is 0-based within its source file.
- `from_change` is `null` for the topic's main `<topic>_task.md`, or the change name when the task lives in a `<change>_task.md`.

### ChangeItem

```json
{ "name": "add-2fa", "status": "in-progress", "has_proposal": true, "has_design": true, "has_spec": true, "has_task": true }
```

## Status machine

| Condition | Status |
|---|---|
| `blockers[]` non-empty | `blocked` |
| Total task count = 0 (spec exists, no `- [ ]` lines yet) | `not-started` |
| All tasks `[x]` | `complete` |
| Some `[x]`, some `[ ]` | `in-progress` |
| All `[ ]`, none `[x]` | `not-started` |

## Area rules

The `rules` array is the source of truth for "what does this area mean right now". Excerpts:

- **Staging** — "Spec is being written. Do not write code yet." + queue-gating reminder.
- **Working** — "Implementation phase. Write code under src/ and flip task checkboxes using tasks_complete." + "The spec is frozen".
- **Testing** — "Run tests and report results — do not edit code in this area."
- **Fixing** — "Fix failing tests. Do not change the spec." + queue-gating reminder.
- **Build** — "Topic is shipped. Do not modify."
- **Custom areas** — Generic fallback referencing the project's mode conventions.

When pending changes exist, an extra rule is appended naming the count.

## Blockers

When `area_requires_readiness(area)` is true (Staging and Fixing by default) and the topic isn't in `spec/<area>/queue.md`, `next` returns:

```
blockers: [
  "Topic 'X' is not listed in Staging/queue.md. Add it via queue_add before pushing out of Staging."
]
next_action: "Resolve blocker: ..."
status: "blocked"
```

When the spec file is missing, `next` returns a `No spec file at ...` blocker.

## Recommended agent usage

```
1. next { topic, area }                    ← always first
2. read context_files                      ← agent loads the listed paths
3. confirm rules don't forbid the planned action
4. if blockers non-empty: resolve each blocker (queue_add / spec_add / …), goto 1
5. perform next_action
6. re-call next to see what's next
```

This matches the hard rule in `.agent/modes/default/system_prompts/unispec_basics.md`:

> Before every action on a topic, call `next { topic, area }` and read the full payload. Follow the `next_action` field verbatim. Do not proceed if `blockers` is non-empty.

## See also

- [cli-reference.md#next](cli-reference.md#next) — flag reference
- [mcp-tools-reference.md#next-structured-agent-feed](mcp-tools-reference.md#next-structured-agent-feed) — JSON-RPC shape
- [change-management.md](change-management.md) — what `pending_changes` and the `from_change` field mean
- [workflow.md](workflow.md) — how area rules map to the 5-area pipeline
