# Areas

The default mode declares five areas. Each represents a stage in a topic's lifecycle. This document explains what each one is for, when a topic should be in it, and what gates (if any) control leaving it.

For the high-level pipeline rules, see [workflow.md](workflow.md).

## The five default areas

```
Staging  →  Working  →  Testing  →  Fixing  →  Build
```

| Area | Stage | Code in `src/`? | Queue gate to leave? |
|------|-------|-----------------|----------------------|
| Staging | Spec authoring | No | **Yes** |
| Working | Implementation | Yes — being actively written | No |
| Testing | Build / test runs | Yes — but not edited | No |
| Fixing | Debugging | Yes — being patched | **Yes** |
| Build | Shipped, immutable | Yes — but treat as read-only | n/a |

## Staging

**Purpose.** Spec authoring. A topic lives here from creation (`unispec topic add`) until the spec is complete enough to start implementing.

**What goes here.**

- `topic.md` with a real Overview, the agent's `short` description, and a placeholder Sub-topics / Notes block.
- `<topic-safe>_spec.md` with every section of the spec template filled with real content (Overview, Purpose, In-Depth Details, Requirements, Examples, Data Model, Out of Scope).
- `<topic-safe>_task.md` with implementation tasks only. **No test tasks here** — those are added during BUILD-phase work in Working.

**What does NOT go here.**

- Code under `src/`. Nothing in `src/` should reference a Staging-only topic.
- Test plans. Test tasks are appended later when the implementation is done.
- `[Placeholder]` strings from the templates. If you see `[Requirement statement]` in a Staging topic, the spec isn't finished.

**Exit gate.** The topic must appear in `spec/Staging/queue.md` to be pushed out. Add via `unispec queue add <topic>`, the MCP `queue_add` tool, or the TUI `q` key while highlighting the topic.

**Definition of done.**

- `topics_show { topic, area: "Staging" }` lists all three files (`topic.md`, `<topic>_spec.md`, `<topic>_task.md`).
- The spec contains at least one `REQ-*` row and one Example.
- The task file has zero placeholders and zero test tasks.
- `queue_check { topic, area: "Staging" }` returns `ready: true`.

When all four hold, `unispec topic push <topic> --area Working --from Staging` will succeed.

## Working

**Purpose.** Implementation. The spec is frozen; code goes into `src/` and tasks get checked off as you complete them.

**What goes here.**

- The same three spec files, carried in from Staging unchanged. Treat the spec as read-only — if requirements change, you noticed too early; pull the topic back to Staging.
- Real source files under `src/` at the project root (NOT inside `spec/`).
- Each new source file should be linked to the topic: `unispec index add --topic <name> --path src/<file> --link-type implementation`.
- Notes captured via `notes_add` for any decision that isn't in the spec (e.g. "Chose argon2id over bcrypt because of …").

**What does NOT go here.**

- Code inside `spec/<Area>/<topic>/`. The spec directory is for spec and task files only. Source code lives at the project root in `src/`.
- New `REQ-*` rows. If you discover a missing requirement, that's signal that the topic isn't ready for Working — pull back.

**Exit gate.** None. Push to Testing freely.

**Definition of done.**

- Every implementation task in `<topic>_task.md` is `- [x]`.
- Every source file you wrote or substantively changed has an `index_add` link.
- A `### Phase 5: Testing` block has been appended to the task file (the test plan for the next stage).

## Testing

**Purpose.** Running build/test/lint pipelines against the implementation. This area is **not** for writing or editing code.

**What goes here.**

- The same three spec files plus the test-tasks block appended in Working.
- `notes_add` entries from the test runs themselves, recording pass/fail and excerpts of failure output.

**What does NOT go here.**

- Code edits. If a test reveals a bug, push to Fixing — don't patch in place.
- Test fixtures. Those belong in `tests/` (or your framework's convention) under the project root, linked via `index add`.

**Exit gate.** None.

**Routing.**

- All tests green → push to Build.
- Any test red → push to Fixing.

**Definition of done.** Every test task in `<topic>_task.md` has a recorded pass/fail. The `notes_add` entries make it clear which run is the most recent.

## Fixing

**Purpose.** Repair. A topic ends up here when Testing surfaced a defect. The job is to reproduce, fix, and return to Testing.

**What goes here.**

- The same three spec files plus the test run that flagged the bug.
- Source-code fixes in `src/` (linked via `index add` if you added a new file).
- A `notes_add` entry that captures: root cause, fix summary, follow-up (if any).

**What does NOT go here.**

- Speculative refactors unrelated to the failing test. Stay scoped.
- Spec changes. If the bug means the spec was wrong, you have a bigger problem — pull back to Staging.

**Exit gate.** Same as Staging — `spec/Fixing/queue.md` must list the topic before pushing out. Use `unispec queue add <topic> --area Fixing`.

**Routing.** Push back to Testing for re-verification. Once that pass is clean, Testing pushes to Build.

## Build

**Purpose.** Done. Topics in Build represent shipped work and are treated as immutable.

**What goes here.**

- All three spec files, in their final form.
- `notes_add` entries summarising the verification run that produced this Build entry.

**What does NOT go here.**

- Edits. To revise a Build topic, `unispec topic pull <topic> --area Build` brings it back into the current default area, then take the normal pipeline again.
- New tasks. The task file should be all `- [x]` by the time the topic arrives.

**Mode configuration.** The default mode marks `Build` as `protected`:

```toml
[areas]
protected = ["Build"]
```

This blocks `unispec area remove Build` (you can't delete a protected area).

## The `area.md` file

Every area directory has an `area.md` that describes the area's role:

```
spec/Staging/area.md
spec/Working/area.md
spec/Testing/area.md
spec/Fixing/area.md
spec/Build/area.md
```

The shipped default mode populates these from `.agent/modes/default/areas/<area>/area.md`. You can edit them per-project to add team-specific conventions — they're plain markdown.

Format:

```markdown
---
area: Staging
short: Make changes to your specs before you write code.
---

# Staging

## Purpose
The Staging area is where specs, topics, and tasks are planned, refined, and formalized before development begins.

## Guidelines
- Ensure all specs are bound to relevant files.
- Verify that every topic has a clear "one-liner" description.
- Tasks must be specific, actionable, and have clear acceptance criteria.
```

The TUI reads the `short:` frontmatter field for the bottom-row hint when an area is highlighted. The `# <Area>` heading and body are shown when an area's `area.md` is opened.

## The readiness queue, in detail

`spec/<gated-area>/queue.md` is a plain markdown file:

```markdown
# Task Queue

Ordered list of topics to work on:
- user-login
- payment-flow
- profile-page
```

`topic push` checks for the topic on any line that matches `^- ` and contains the topic name (substring match). Order in the file is informational — `push` doesn't require the topic to be at position 0.

**Why is the queue plain markdown?** Two reasons:

1. **It's diffable.** A `git diff` on `queue.md` is the audit trail for "who promoted what, when". A SQLite db wouldn't be.
2. **It's hand-editable.** If you fat-finger a queue entry, you can fix it in any text editor without involving the CLI.

The four queue MCP tools (`queue_list`, `queue_add`, `queue_remove`, `queue_check`, `queue_reorder`) and the CLI `queue add` subcommand all read/write this same file. There is no separate index.

## Changes live inside topics, in every area

A topic can carry its own `changes/` subtree — proposed feature additions written via `change_add` without touching the original `<topic>_spec.md`. The directory layout is:

```
spec/<Area>/<topic>/
├── topic.md
├── <topic>_spec.md
├── <topic>_task.md
└── changes/
    ├── <change>/
    │   ├── proposal.md
    │   ├── design.md          # optional
    │   ├── <change>_spec.md
    │   └── <change>_task.md
    └── archive/<archived-change>/
```

`changes/` is always inside the topic, never at the area root. There is no `spec/<Area>/changes/`. Each area can host changes attached to any of its topics.

### How changes move through the pipeline

`topics_push` copies the *entire* topic directory — including `changes/` and `changes/archive/` — to the target area, then removes the source. So:

- A change proposed in **Staging** rides into **Working** when the topic is pushed there. Agents in Working can read its `<change>_spec.md` and tick off `<change>_task.md`.
- A change implemented in **Working** travels through **Testing** and **Fixing** untouched. The work belongs to the topic, not to any one area.
- An archived change (`changes/archive/<name>/`) follows the same path. It stays diffable in `git` but is filtered out of `change_list` (unless `include_archived: true`).

There is no separate "push the change" command. Changes are part of the topic; they move when the topic moves.

### When to spawn a change vs. push the topic back

| Situation | Right answer |
|-----------|--------------|
| Topic is in Working, you want to add a new feature that isn't in the spec yet | `change_add` while the topic is in Working — write the proposal, design, spec, task. Implement during the same Working pass. |
| Topic is in Build (shipped), you want to add a feature | `topic pull` to bring it back; then `change_add` in the current default area, then push it forward again. |
| Spec turned out to be wrong (not "missing a feature" — actually wrong) | `topic pull` to Staging; either re-`spec_add` (acknowledging you're discarding the original) or fix manually. Changes are for *additions*, not retractions. |

See [change-management.md](change-management.md) for the full guide.

## Custom areas

You can declare additional areas in a custom mode. For example, a 7-area "RFC" mode might be:

```toml
[areas]
default = ["Proposed", "Discussion", "Approved", "Staging", "Working", "Build", "Archived"]
protected = ["Build", "Archived"]
default_area = "Proposed"

[readiness]
queue_file = "queue.md"
required_for_push = true
areas = ["Proposed", "Approved"]
```

`unispec init` will lay down `spec/<each>/area.md` for every declared area. `topic push` enforces readiness for the listed gated areas. See [modes.md](modes.md) for the full mode authoring contract.

## See also

- [Workflow](workflow.md) — pipeline rules at the meta level.
- [Quickstart](quickstart.md) — copy-pasteable end-to-end walkthrough.
- [Modes](modes.md) — how to build a custom area set.
