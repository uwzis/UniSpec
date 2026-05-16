# Change Management

A complete guide to the change management system in UniSpec. Use this whenever a topic that already has a spec needs new features added to it.

> TL;DR: don't rewrite an existing `<topic>_spec.md`. Run `unispec change add` (or `change_add` via MCP). It writes the new feature as a child folder under the topic.

## Why this exists

UniSpec is spec-first. Once a topic has been spec'd and shipped, its `<topic>_spec.md` and `<topic>_task.md` are evidence of what was built and why — they are intentionally hard to overwrite.

But real features grow. `auth` ships with email/password login on day one; six weeks later you want to add 2FA. The bad options are:

- **Rewrite `auth_spec.md`.** Loses the day-one design. `spec_add` will silently overwrite if rerun — convenient when you really want to start over, but disastrous as a way of layering new requirements onto a topic that's already shipped.
- **Create a new top-level topic** `auth-2fa`. Splits the feature in two places; future readers have to stitch them back together.
- **Edit the file by hand.** Drifts from frontmatter conventions and bypasses the tooling.

The change-management system — inspired by [OpenSpec](https://github.com/Fission-AI/OpenSpec) — gives you a fourth option:

- Keep the original `<topic>_spec.md` exactly as it is.
- Write the new feature as a *change* inside the topic. The change has its own proposal, optional design note, spec, and tasks.
- When the change ships, archive it. The folder moves to `changes/archive/` and is no longer surfaced as live work, but it's still diffable and traceable.

## Folder structure

A topic with two changes — one proposed, one archived — looks like:

```
spec/Staging/auth/
├── topic.md
├── auth_spec.md             ← original requirements, never modified
├── auth_task.md             ← original tasks
└── changes/
    ├── add-2fa/             ← pending change (in-progress)
    │   ├── proposal.md      ← why this change exists
    │   ├── design.md        ← technical approach (optional)
    │   ├── add-2fa_spec.md  ← new requirements only
    │   └── add-2fa_task.md  ← new tasks only
    ├── add-oauth/           ← pending change (just proposed)
    │   ├── proposal.md
    │   ├── add-oauth_spec.md
    │   └── add-oauth_task.md
    └── archive/
        └── add-magic-link/  ← shipped, archived
            ├── proposal.md
            ├── design.md
            ├── add-magic-link_spec.md
            └── add-magic-link_task.md
```

Inside each change folder:

| File | Required? | Written when |
|------|-----------|--------------|
| `proposal.md` | yes | always — by `change add` |
| `design.md` | optional | only when `--design` is supplied |
| `<change>_spec.md` | yes | always — by `change add` |
| `<change>_task.md` | yes | always — by `change add` |

The change name has `/` and ` ` replaced with `-` when forming filenames, the same convention as topic names. So `change: "add 2fa"` becomes `add-2fa_spec.md`.

## Lifecycle

```
   ┌────────┐    change_add    ┌──────────┐    tasks land   ┌──────────┐    change_archive    ┌──────────┐
   │  none  │ ───────────────▶ │ proposed │ ──────────────▶ │ in-prog  │ ──────────────────▶  │ archived │
   └────────┘                  └──────────┘                 │ complete │                       └──────────┘
                                                            └──────────┘
```

Statuses (as reported by `change_list` / `unispec change list`):

| Status | Meaning |
|--------|---------|
| `proposed` | The change exists but its task file has no checked boxes yet. |
| `in-progress` | Some `- [x]` boxes in the task file, but not all. |
| `complete` | All `- [ ]` / `- [x]` lines in the task file are `[x]`. |
| `archived` | The change has been moved into `changes/archive/<name>/`. Only surfaced when `include_archived: true` (MCP) or `--archived` (CLI). |

The status is derived from the task file every time; there's no separate state column to keep in sync.

## Delta sections — the archive merge

`change_archive` reads the change's `<change>_spec.md` for delta sections and **merges them into the canonical `<topic>_spec.md`** before moving the change to `changes/archive/`. A change spec with no delta sections is archived as a plain move (backward compatible with older changes that just stored a parallel spec).

Four section names are recognised (case-insensitive on the name):

```markdown
## ADDED Requirements
### Requirement: <new name>
<body of the new requirement>

## MODIFIED Requirements
### Requirement: <existing name>
<body that REPLACES the existing block — must be the full block, not a diff>

## REMOVED Requirements
### Requirement: <existing name>
**Reason:** <why it was removed>

## RENAMED Requirements
- FROM: ### Requirement: Old Name
- TO: ### Requirement: New Name
```

### How the merge runs

Operations apply in this order so name lookups stay stable:

1. **RENAMED** — every `FROM: → TO:` pair renames the matching `### Requirement: Old Name` header in the canonical spec.
2. **REMOVED** — every named requirement is deleted from the canonical spec.
3. **MODIFIED** — every block replaces the matching `### Requirement: X` block in place.
4. **ADDED** — every new block is appended to the end of the canonical spec.

If a MODIFIED / REMOVED / RENAMED FROM name doesn't match anything in the canonical spec, that operation is silently skipped. ADDED names that already exist are skipped to keep the merge idempotent.

The frontmatter on `<topic>_spec.md` is preserved verbatim; only the body is rewritten.

### Worked example

Canonical spec before archive:

```markdown
---
title: auth
...
---

### Requirement: Login
Users must be able to login with email and password.

### Requirement: Logout
Users must be able to logout.
```

Change spec (`changes/add-2fa/add-2fa_spec.md`):

```markdown
## ADDED Requirements
### Requirement: Two Factor Auth
Users must be able to enable TOTP based two factor authentication.

## MODIFIED Requirements
### Requirement: Login
Users must be able to login with email and password. If 2FA is enabled they must also provide a TOTP code.
```

After `unispec change archive --topic auth --change add-2fa`:

```markdown
---
title: auth
...
---

### Requirement: Login
Users must be able to login with email and password. If 2FA is enabled they must also provide a TOTP code.

### Requirement: Logout
Users must be able to logout.

### Requirement: Two Factor Auth
Users must be able to enable TOTP based two factor authentication.
```

And `changes/add-2fa/` is now at `changes/archive/add-2fa/`.

### Authoring tips

- **Each MODIFIED block must contain the ENTIRE replacement body**, not a diff. The merger does header-name matching, not line-by-line patching.
- **RENAMED runs before REMOVED / MODIFIED / ADDED**, so a `MODIFIED Requirements` block can target the new name set by RENAMED in the same change.
- **A new requirement that appears in both ADDED and MODIFIED** counts as a hard conflict against the canonical spec — keep one section per requirement name.
- **`change_archive` does not validate the merged spec.** Run `unispec analyze --topic <t>` after archiving if you want to confirm the merged spec still looks right.

## Quick recipe

```bash
# 1. propose
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

# 2. inspect
unispec change list --topic auth

# 3. work — open the change_task.md and tick boxes as you implement
$EDITOR spec/Staging/auth/changes/add-2fa/add-2fa_task.md

# 4. ship & archive
unispec change archive --topic auth --change add-2fa
```

## CLI

Three subcommands. All take `--area, -a` defaulting to `.agent/config.toml`'s `area`, then `Staging`.

### `unispec change add`

```
unispec change add \
  --topic <name>            # existing topic (required)
  --change <id>             # change identifier, e.g. add-2fa (required)
  --proposal <text>         # why this change exists, ≥ 11 chars (required)
  --spec-content <body>     # new requirements only, ≥ 11 chars (required)
  --task-content <tasks>    # new tasks only, ≥ 11 chars (required)
  [--design <text>]         # optional technical approach
  [--area <area>]           # default: from config, then Staging
```

- Errors if the topic doesn't exist.
- Errors if the change folder already exists.
- `--spec-content` and `--task-content` accept lines starting with `- ` (clap `allow_hyphen_values = true`).

### `unispec change list`

```
unispec change list \
  --topic <name>            # required
  [--area <area>]           # default: from config
  [--archived]              # also include changes/archive/*
```

Output:

```
Changes for 'auth' in Staging/:
  - add-2fa [in-progress] (proposal, design, spec, task)
  - add-oauth [proposed] (proposal, spec, task)
```

With `--archived`, archived changes are listed below non-archived ones with `[archived]` status.

### `unispec change archive`

```
unispec change archive \
  --topic <name>            # required
  --change <id>             # required
  [--area <area>]           # default: from config
```

Moves `changes/<change>/` to `changes/archive/<change>/`. Errors if either:

- the source change doesn't exist (typo, wrong topic, already archived), or
- a directory named `<change>` already lives under `archive/` (refuses to overwrite).

## MCP tools

Three tools, one per CLI subcommand. They call into the same `src/commands/change.rs` functions as the CLI — behaviour is identical.

### `change_add`

```json
{
  "name": "change_add",
  "arguments": {
    "topic": "auth",
    "area": "Staging",
    "change": "add-2fa",
    "proposal": "Protect high-value accounts with a second factor.",
    "design": "TOTP via authenticator apps; encrypted seed at rest.",
    "spec_content": "## 2FA requirements\n- TOTP enrolment per user\n- 8 recovery codes per user",
    "task_content": "- [ ] Generate TOTP seeds\n- [ ] Verify TOTP codes on login\n- [ ] Issue and store recovery codes"
  }
}
```

Required: `topic`, `change`, `proposal`, `spec_content`, `task_content`. Optional: `area`, `design`.

### `change_list`

```json
{
  "name": "change_list",
  "arguments": {
    "topic": "auth",
    "area": "Staging",
    "include_archived": true
  }
}
```

Required: `topic`. Optional: `area`, `include_archived` (default `false`).

Response includes one entry per change with `name`, `status`, `has_proposal`, `has_design`, `has_spec`, `has_task`.

### `change_archive`

```json
{
  "name": "change_archive",
  "arguments": {
    "topic": "auth",
    "area": "Staging",
    "change": "add-2fa"
  }
}
```

Required: `topic`, `change`. Optional: `area`.

For full request/response shapes and constraint details, see [mcp-tools-reference.md](mcp-tools-reference.md#change-management).

## How agents should use it

1. **Before authoring a spec, check whether the topic already exists.** `topics_show` or `read_asset { topic, asset_type: "spec" }` tells you. If a `<topic>_spec.md` is already on disk, **do not call `spec_add`** — it will error. Use `change_add` instead.
2. **Before building a topic, run `change_list`** to see if there are pending changes that need to be implemented as part of this build. The change folder's task file lives at `spec/<area>/<topic>/changes/<change>/<change>_task.md` — read it the same way you'd read the topic's main task file.
3. **Implement one change at a time.** Each change has its own `<change>_task.md`; flip its checkboxes via `task_write` (note: `tasks_complete` / `tasks_incomplete` target the topic's main task file, not the change's task file — for changes, write the file directly or shell out to an editor).
4. **Archive when done.** Once every checkbox is `[x]` in `<change>_task.md`, call `change_archive`. The folder moves to `changes/archive/`; further `change_list` calls (without `include_archived`) will not surface it.
5. **Never modify `proposal.md` or `<change>_spec.md` after the change is in progress.** They are the change's contract. If the requirements turn out to be wrong, archive the change and propose a new one.

## Comparison with `spec_add`

| | `spec_add` (or `unispec spec add`) | `change_add` (or `unispec change add`) |
|---|---|---|
| Writes to | `spec/<area>/<topic>/<topic>_spec.md` / `_task.md` | `spec/<area>/<topic>/changes/<change>/<change>_spec.md` / `_task.md` |
| Topic must already exist | Yes (`topics_add` first) | Yes (with a real spec already in place) |
| Behaviour on second call | Silently overwrites — by convention you only call it once per topic | Errors with `Change '<X>' already exists` — refuses to clobber |
| Writes a proposal | No | Yes (`proposal.md`) |
| Writes a design note | No | Optional (`design.md`) |
| Status field | Frontmatter `status: draft` | Computed from task checkboxes (`proposed` / `in-progress` / `complete` / `archived`) |
| Travels with topic on `topics_push` | Yes | Yes (whole `changes/` tree is copied byte-for-byte) |

The two are complementary. The first spec for a topic comes from `spec_add`; every subsequent feature on that topic comes from `change_add`.

## Common mistakes

### "I'm getting 'Topic 'X' does not exist in area 'Staging''."

`change_add` requires the topic directory to exist. Create the topic first with `topics_add` (or `unispec topic add`) — the topic doesn't need a spec yet, but the directory has to be there. The case of `--area` doesn't have to match the directory's case; the resolver tries the upper-cased and lower-cased variants.

### "I'm getting 'Change 'X' already exists for topic 'Y' in area 'Z''."

Either:

- You ran `change_add` with the same change name twice. Pick a different name, or `unispec change archive --topic <name> --change <change>` the first attempt and try again.
- A previous run *did* succeed; you're seeing it on disk and assuming it failed. `ls spec/<Area>/<topic>/changes/<change>/` to confirm.

### "I tried `change archive` and it said 'Change 'X' does not exist'."

The change name passed to `archive` must match the directory name under `changes/`. If you typo'd `--change`, no folder matches. Run `unispec change list --topic <name>` to see the canonical names.

### "I tried `change archive` and it said 'Archived change 'X' already exists; refusing to overwrite'."

There's already a `changes/archive/<change>/` from a previous run. Either:

- The work is genuinely done twice (rare). Rename one of them before archiving (`mv changes/<X> changes/<X>-v2 && unispec change archive --change X-v2`).
- The previous archive was a mistake. Delete `changes/archive/<X>/` by hand, then re-run `change archive`.

### "I called `spec_add` again on a topic and it overwrote my spec."

`spec_add` does not enforce "first call only" — it will silently overwrite `<topic>_spec.md` and `<topic>_task.md` if rerun. The convention is operational:

```
spec_add { topic, area, short, spec_content, task_content }     ← call once per topic
change_add { topic, area, change, proposal, spec_content, task_content [, design] }   ← every later feature
```

If you genuinely need to revise the topic's foundational spec (rare — usually means the topic was wrong), that's the case where re-running `spec_add` is acceptable; just be aware you're throwing away the previous version (and any `git diff` is your only audit trail). Most of the time, prefer `change_add` so both versions are preserved.

### "Where do I check off boxes in a change's task file?"

The change's task file lives at `spec/<area>/<topic>/changes/<change>/<change>_task.md`. It uses the same `- [ ]` / `- [x]` markdown convention as the topic's main task file. The `tasks_list` / `tasks_complete` / `tasks_incomplete` MCP tools currently operate on the topic's *main* task file, not the change task file — for the change task file, either edit it directly via the host editor's Write tool or shell out to `$EDITOR` from the TUI (`Enter` on the task file in the changes tree opens it).

## The TUI

When you drill into a topic that has changes, the TUI shows:

- `📁🔀 changes` — the changes container directory. Press `→` to enter.
- `🔀 add-2fa` (and similar) — individual change folders. Press `→` to enter.
- Inside a change folder, the spec file is rendered as a regular spec entry and tagged with `🔀` so you can see at a glance you're in a change.

Navigation is identical to nested topics — `→` enters, `←` goes back, `Enter` opens files in `$EDITOR`. See [tui.md](tui.md) for the full keybinding chart.

## Travels with the topic

Because `changes/` is just a subdirectory of the topic, `topics_push` carries it (including `changes/archive/`) into every downstream area without special-casing. If `add-2fa` is proposed in Staging, then `auth` is pushed to Working, the change comes along; agents in Working can pick it up and implement it as part of the build.

There is no separate "push the change" operation. The change is part of the topic.

## See also

- [workflow.md](workflow.md) — pipeline rules including where changes live in the lifecycle
- [cli-reference.md](cli-reference.md#change) — flag-level reference for the CLI subcommands
- [mcp-tools-reference.md](mcp-tools-reference.md#change-management) — JSON-RPC examples for the MCP tools
- [tui.md](tui.md) — the `🔀` and `📁🔀` prefixes
- [troubleshooting.md](troubleshooting.md#change-management) — what to do when something goes wrong
