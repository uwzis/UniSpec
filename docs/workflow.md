# Workflow

UniSpec's default mode defines a five-area pipeline. Topics enter at one end and leave at the other; each area has a clear purpose, and two of the five are gated by a readiness queue.

```
Staging  →  Working  →  Testing  →  Fixing  →  Build
   ▲                                  │
   │                                  │
   └──── (pull back if needed) ◄──────┘
```

## The five areas

| Area | When a topic should be here | Gated by readiness queue? |
|------|-----------------------------|---------------------------|
| **Staging** | Spec is being written. No code in `src/` yet. | **Yes** — must be in `spec/Staging/queue.md` to push out |
| **Working** | Spec frozen, code is being implemented. | No |
| **Testing** | Build and test scripts are running against the implementation. | No |
| **Fixing** | A defect was found in Testing; the topic is being repaired before being re-tested. | **Yes** — must be in `spec/Fixing/queue.md` to push out |
| **Build** | Topic is shipped. Treated as immutable; do not edit in place. | n/a (no further push) |

Each area is a directory under `spec/`. Each contains a single per-area `area.md` (describing what that area means in your project) and zero or more topic subdirectories.

The set of areas and which ones are queue-gated come from `.agent/modes/default/mode.toml`:

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

If you change the mode, those rules change with it. See [modes.md](modes.md) for custom mode authoring.

## What pushing does

`unispec topic push <topic> --area <target> --from <source>`:

1. Verifies the source area exists.
2. **Creates the target area on the fly** if it doesn't exist yet (writes a minimal `area.md` stub). This is a backstop in case `init` ran with fewer areas than the current mode declares.
3. Confirms the topic exists in the source area.
4. If the source area is queue-gated (`Staging` or `Fixing` by default), checks that the topic appears in `spec/<source>/queue.md`. Errors out with a clear message if not.
5. Copies every file from `spec/<source>/<topic>/` to `spec/<target>/<topic>/`. (No legacy `specs.md`/`tasks.md` duplicates are synthesised.)
6. Removes the source directory. **Push is a move, not a copy.**
7. If pushing into `Build` and the topic carries ownership metadata (someone has it "checked out" in `<topic>_spec.md` frontmatter), the auto-checkin step clears the metadata so Build artefacts are clean.

## The readiness queue

Each gated area carries a `queue.md` at the area root:

```
spec/Staging/queue.md     ← lists topics ready to push out of Staging
spec/Fixing/queue.md      ← lists topics ready to push out of Fixing
```

The file is plain markdown:

```markdown
# Task Queue

Ordered list of topics to work on:
- user-login
- payment-flow
```

Add to it three ways:

| Surface | Command |
|---------|---------|
| CLI | `unispec queue add <topic> [--area <area>] [--position <n>]` |
| MCP | `queue_add { "topic": "user-login" }` |
| TUI | Highlight the topic in a TopicList view and press `q` |

The queue is intentionally append-only by default; remove with `queue_remove` (MCP) or by editing `queue.md` directly. Reorder via the MCP `queue_reorder` tool.

### Why a queue gate?

Two reasons:

1. **Explicit readiness signal.** Anyone who has worked spec-first knows the failure mode: someone writes half a spec and immediately starts coding. The queue gate forces an explicit "I'm done with the spec, this is ready to implement" before push.
2. **AI-agent guardrail.** Without the gate, an MCP-driven agent could ping-pong a topic through every area in seconds. With the gate, an actor (human or agent) has to deliberately enqueue the topic before the next stage will accept it.

`Working`, `Testing`, and `Build` are NOT gated by default — once a topic is past the spec, transitions between implementation stages don't need an extra approval beat. You can change which areas are gated by editing `[readiness].areas` in your mode's `mode.toml`.

## Filenames in a topic directory

```
spec/<Area>/<topic>/
├── topic.md                  ← written by `topic add`
├── <topic-safe>_spec.md      ← written by `spec add`
└── <topic-safe>_task.md      ← written by `spec add`
```

`<topic-safe>` is `<topic>` with `/` and ` ` replaced by `-`. So `auth/login` becomes `auth-login_spec.md` inside `spec/<Area>/auth/login/`.

Nested topics are real directories. Pushing the parent does **not** push the children — each child is its own pipeline traversal.

## Change management — adding features without rewriting the spec

When a topic already exists and someone wants to add a new feature on top of it, the answer is **not** to rewrite `<topic>_spec.md`. Instead, create a *change* — a sibling folder under the topic that contains its own proposal, optional design note, and dedicated spec / task pair. The original spec stays the source of truth.

The on-disk layout becomes:

```
spec/<Area>/<topic>/
├── topic.md
├── <topic>_spec.md         ← original requirements, never modified
├── <topic>_task.md         ← original tasks
└── changes/
    ├── add-2fa/
    │   ├── proposal.md     ← why this change exists
    │   ├── design.md       ← optional technical approach
    │   ├── add-2fa_spec.md ← new requirements only
    │   └── add-2fa_task.md ← new tasks only
    ├── add-oauth/
    │   ├── proposal.md
    │   ├── add-oauth_spec.md
    │   └── add-oauth_task.md
    └── archive/
        └── add-2fa/        ← archived after completion
```

### Lifecycle

1. **Propose** — `change_add` writes `proposal.md` (and optionally `design.md`) plus a fresh spec/task pair under `changes/<change>/`. Status starts as `proposed`.
2. **Implement** — as agents tick off `- [x]` in `<change>_task.md`, `change_list` reports the change as `in-progress`, then `complete` when every box is checked.
3. **Archive** — `change_archive` moves `changes/<change>/` into `changes/archive/<change>/` once the change has landed. Archived changes stay diffable and traceable but are no longer surfaced as live work.

### When to use `change add` vs `spec add`

| You want to… | Use |
|--------------|-----|
| Spec out a brand-new topic for the first time | `spec_add` (or CLI `unispec spec add`) |
| Add a feature to a topic that already has a spec | `change_add` (or CLI `unispec change add`) |
| Rework requirements that were already shipped | Pull the topic back via `topics_pull`, then `change_add` once it's in Staging |

`spec_add` writes to the topic root and refuses to overwrite. `change_add` writes under `changes/<change>/` and leaves the root alone — this is the *only* way to layer additions onto a topic without losing history.

### Changes travel with the topic

`changes/` lives inside the topic directory, so `topics_push` carries it (and `changes/archive/`) into every downstream area unchanged. A change proposed in Staging follows the topic into Working, Testing, Fixing, and Build with no extra work — its files move byte-for-byte.

See [change-management.md](change-management.md) for the full guide with worked examples.

## Backflow: `pull`

If a topic in `Build` is found to have a bug, pull it back:

```bash
unispec topic pull <topic> --area Build
```

The topic returns to the current default area (typically `Staging`). From there it can be re-spec'd, re-implemented, and re-pushed.

## End-to-end shape

```bash
# Spec
unispec topic add  <name> --short "..." --content "..."
unispec spec add   --topic <name> --short "..." \
                   --spec-content "..." --task-content "..."

# Implement
unispec queue add  <name>
unispec topic push <name> --area Working --from Staging
# (code in src/)

# Test
unispec topic push <name> --area Testing --from Working
# (run tests)

# If broken
unispec topic push <name> --area Fixing  --from Testing
# (fix in src/)
unispec queue add  <name> --area Fixing
unispec topic push <name> --area Build   --from Fixing
# else
unispec topic push <name> --area Build   --from Testing   # if no Fixing pass needed
```

The whole pipeline can also be driven via MCP — the tools mirror the CLI surface 1:1. See [mcp-tools-reference.md](mcp-tools-reference.md).

## Working the pipeline with an agent

Three commands shape every agent action at every stage:

| Command | When | Purpose |
|---|---|---|
| `unispec next --topic <t>` | Before every action | Structured payload — `status`, open tasks, pending changes, area rules, one-sentence `next_action`, any `blockers`. The agent follows `next_action` verbatim and resolves every blocker before proceeding. |
| `unispec analyze --topic <t>` | After spec edits, before push | Six static checks — duplication, missing task coverage, ambiguous language, empty sections, constitution alignment, task completion. ERROR findings should be fixed before push. |
| `constitution_read {}` (MCP) | Step 0 of build & verify | Loads `.agent/constitution.md` — non-negotiable principles. Any planned action conflicting with a principle blocks progress. |

`next` and `analyze` both have matching MCP tools with identical payloads. See [next.md](next.md), [analyze.md](analyze.md), [constitution.md](constitution.md).

The default mode's `build.md` and `verify.md` workflows now codify this — Step 0 reads the constitution alongside `next` so every implementation pass starts from the same baseline.

## See also

- [Areas](areas.md) — per-area conventions (what code belongs in Working vs. Fixing, etc.).
- [Quickstart](quickstart.md) — five-minute walkthrough with copy-pasteable commands.
- [Modes](modes.md) — how to customise the pipeline (different areas, different gates).
- [Next](next.md), [Analyze](analyze.md), [Constitution](constitution.md), [Workspaces](workspaces.md).
