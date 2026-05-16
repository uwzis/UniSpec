# MCP Tools Reference

Every MCP tool the UniSpec server publishes via `tools/list`, with required args, optional args, an example JSON-RPC `tools/call` request, and a representative response.

This list is generated against `src/mcp/mod.rs::get_tools()` on the `feature/change-management` branch. There are **39 built-in tools** plus one dynamic `unispec_<name>` tool per `[[connector]]` entry in `.agent/config.toml`.

> Filename convention: `spec_add` writes `<topic-safe>_spec.md` and `<topic-safe>_task.md`, where `<topic-safe>` is the topic name with `/` and ` ` replaced by `-`. Every read tool (`unispec_read_spec`, `read_asset`, `topics_show`) uses the same names.

> Default area: every tool that takes an optional `area` defaults to `"Staging"` if omitted. (The server reads `Staging` directly, not the project config, so set the area explicitly when in doubt.)

## Areas

### `areas_list`

List every area present in `spec/`.

**Args:** none.

**Request:**
```json
{ "jsonrpc": "2.0", "id": 1, "method": "tools/call",
  "params": { "name": "areas_list", "arguments": {} } }
```

**Response:**
```json
{ "success": true, "areas": ["Build","Fixing","Staging","Testing","Working"] }
```

## Topics

### `topics_list`

List topics in an area, with their roll-up status.

**Args:** `area?` (default `"Staging"`).

**Request:**
```json
{ "name": "topics_list", "arguments": { "area": "Staging" } }
```

**Response:**
```json
{ "success": true, "area": "Staging",
  "topics": [
    { "name": "user-login", "status": "draft" },
    { "name": "payment-flow", "status": "in-progress" }
  ] }
```

### `topics_add`

Create a new topic. Writes `spec/<area>/<topic>/topic.md` with server-managed frontmatter (`title`, `short`, `created`, `author`).

**Required:** `topic`, `area`, `short`, `content`.

**Constraints:** `content` is trimmed and must be > 10 chars after trim. Any leading `---` frontmatter block in the supplied `content` is stripped.

**Request:**
```json
{ "name": "topics_add", "arguments": {
    "topic": "user-login",
    "area": "Staging",
    "short": "Email/password login with JWT",
    "content": "# user-login\n\n## Overview\nAuthentication system with JWT and refresh tokens."
} }
```

**Response:**
```json
{ "success": true, "message": "Topic 'user-login' created in Staging/",
  "topic": "user-login", "area": "Staging" }
```

### `topics_show`

Inspect what files exist in a topic directory.

**Required:** `topic`. **Optional:** `area`, `show_all`, `from`.

```json
{ "name": "topics_show", "arguments": { "topic": "user-login", "area": "Staging" } }
```

### `topics_delete`

Delete a topic directory.

**Required:** `topic`. **Optional:** `area`, `force` (default `true`).

```json
{ "name": "topics_delete", "arguments": { "topic": "user-login", "force": true } }
```

### `topics_push`

Move a topic to another area (real move — source is removed).

**Required:** `topic`, `area` (target). **Optional:** `source_area` (defaults to `"Staging"`).

**Gated by readiness** if `source_area` is in `[readiness].areas` (default `Staging` and `Fixing`) — the topic must appear in `spec/<source_area>/queue.md` first.

```json
{ "name": "topics_push", "arguments": {
    "topic": "user-login", "area": "Working", "source_area": "Staging"
} }
```

### `topics_pull`

Pull a topic back from another area into `Working`.

**Required:** `topic`, `source_area`.

```json
{ "name": "topics_pull", "arguments": { "topic": "user-login", "source_area": "Build" } }
```

### `topics_progress`

Per-topic task counts in an area.

**Optional:** `area` (default `"Staging"`).

```json
{ "name": "topics_progress", "arguments": { "area": "Working" } }
```

Response shape:
```json
{ "success": true, "area": "Working",
  "topics": [
    { "topic": "user-login", "status": "in-progress", "total_tasks": 5, "completed_tasks": 2 }
  ] }
```

## Asset reading

### `read_asset`

Read one of `topic.md`, `<topic>_spec.md`, or `<topic>_task.md` for a topic. Special case: passing `topic: "templates"` reads from `.agent/modes/default/templates/`.

**Required:** `topic`, `asset_type` ∈ `{"topic", "spec", "task"}`. **Optional:** `area`.

```json
{ "name": "read_asset", "arguments": {
    "topic": "user-login", "asset_type": "spec", "area": "Staging"
} }
```

```json
{ "name": "read_asset", "arguments": {
    "topic": "templates", "asset_type": "spec"
} }
```

### `unispec_read_spec`

Read both the spec and task content for a topic in a single call.

**Required:** `topic`. **Optional:** `area`.

```json
{ "name": "unispec_read_spec", "arguments": { "topic": "user-login", "area": "Working" } }
```

**Response:**
```json
{ "success": true,
  "spec": "---\ntitle: user-login\n...---\n\nPOST /login takes ...",
  "tasks": "---\nspec: user-login\n...---\n\n- [ ] Implement POST /login\n- [ ] ...",
  "spec_file": "user-login_spec.md",
  "task_file": "user-login_task.md" }
```

## Spec & task writing

### `spec_add`

Create `<topic-safe>_spec.md` and `<topic-safe>_task.md` for a topic. The topic itself (the directory and `topic.md`) must already exist (via `topics_add`).

**Required:** `topic`, `area`, `short`, `spec_content`, `task_content`.

**Constraints:** both content fields are trimmed and must be > 10 chars. Any leading `---` block in either content field is stripped before the server prepends its own frontmatter.

```json
{ "name": "spec_add", "arguments": {
    "topic": "user-login", "area": "Staging", "short": "Auth design",
    "spec_content": "POST /login takes {email, password} and returns {jwt} on success.",
    "task_content": "- [ ] Implement POST /login\n- [ ] Add JWT signing\n- [ ] Write tests"
} }
```

### `spec_write`

Overwrite an existing `<topic-safe>_spec.md`. Strips any caller-supplied frontmatter.

**Required:** `topic`, `content`. **Optional:** `area`.

### `task_write`

Overwrite an existing `<topic-safe>_task.md`. **Errors** if the spec file doesn't exist for that topic — use `spec_add` first.

**Required:** `topic`, `content`. **Optional:** `area`.

### `task_status`

Update the `status:` line in `<topic-safe>_task.md`'s frontmatter. Accepted values: `"pending"`, `"working"`, `"complete"`. Does **not** touch the `- [ ]` checkboxes in the body — for that, use `tasks_complete` / `tasks_incomplete`.

**Required:** `topic`, `status`. **Optional:** `area`.

```json
{ "name": "task_status", "arguments": { "topic": "user-login", "status": "working" } }
```

### `tasks_list`

List every `- [ ]` / `- [x]` line in the task file with index, line number, status, and text.

**Required:** `topic`. **Optional:** `area`.

```json
{ "name": "tasks_list", "arguments": { "topic": "user-login", "area": "Working" } }
```

**Response:**
```json
{ "success": true, "topic": "user-login", "area": "Working", "count": 3,
  "tasks": [
    { "index": 0, "line": 7, "status": "pending",   "text": "Implement POST /login" },
    { "index": 1, "line": 8, "status": "complete",  "text": "Add JWT signing helper" },
    { "index": 2, "line": 9, "status": "pending",   "text": "Write tests" }
  ] }
```

### `tasks_complete`

Flip the 0-indexed task in the task file to `[x]`. Index is among task lines, not raw file lines.

**Required:** `topic`, `task_index`. **Optional:** `note`, `area`.

```json
{ "name": "tasks_complete", "arguments": { "topic": "user-login", "task_index": 0 } }
```

### `tasks_incomplete`

Inverse of `tasks_complete` — flip `[x]` back to `[ ]`.

**Required:** `topic`, `task_index`. **Optional:** `note`, `area`.

## Notes

### `notes_read`

Read the `## Notes` section of `<topic-safe>_task.md`.

**Required:** `topic`. **Optional:** `area`.

### `notes_add`

Append a dated note (`- **YYYY-MM-DD**: <text>`) to the `## Notes` section. Creates the section if it doesn't exist.

**Required:** `topic`, `note`. **Optional:** `area`.

```json
{ "name": "notes_add", "arguments": {
    "topic": "user-login", "note": "Chose argon2id over bcrypt — see issue #42"
} }
```

## Next — structured agent feed

### `next`

The recommended first call for any agent working on a topic. Returns a structured payload composing spec/task state, pending changes, queue gating, area conventions, and a one-sentence `next_action`.

**Required:** `topic`. **Optional:** `area` (default `"Staging"`).

**Request:**
```json
{ "name": "next", "arguments": { "topic": "auth", "area": "Working" } }
```

**Response:**
```json
{
  "success": true,
  "topic": "auth",
  "area": "Working",
  "status": "in-progress",
  "open_tasks": [
    { "index": 2, "text": "Verify TOTP codes", "completed": false, "from_change": "add-2fa" }
  ],
  "completed_tasks": [
    { "index": 0, "text": "Implement POST /login", "completed": true, "from_change": null }
  ],
  "pending_changes": [
    { "name": "add-2fa", "status": "in-progress", "has_proposal": true, "has_design": true, "has_spec": true, "has_task": true }
  ],
  "archived_changes": [],
  "context_files": [
    "spec/Working/auth/topic.md",
    "spec/Working/auth/auth_spec.md",
    "spec/Working/auth/auth_task.md",
    "spec/Working/auth/changes/add-2fa/proposal.md"
  ],
  "rules": [
    "Implementation phase. Write code under src/ and flip task checkboxes using tasks_complete.",
    "The spec is frozen — do not modify <topic>_spec.md."
  ],
  "next_action": "Work on task 2 of change 'add-2fa': Verify TOTP codes.",
  "blockers": []
}
```

`status` values: `not-started` (spec exists, no tasks done), `in-progress` (some `[x]`, some `[ ]`), `complete` (all `[x]`), `blocked` (`blockers` is non-empty — agent must resolve before proceeding).

`from_change: null` means the task lives in the topic's main `<topic>_task.md`; a non-null value means the task lives in `<change>_task.md` inside that change folder. The `index` is 0-based within its source file.

## Analyze

### `analyze`

Cross-artifact consistency checker. Returns `findings[]` with severity counts.

**Required:** `topic`. **Optional:** `area` (default `"Staging"`).

Six checks (see [cli-reference.md#analyze](cli-reference.md#analyze) for the full description):

| # | Check | Severity |
|---|-------|----------|
| 1 | Duplication between canonical spec and pending change | WARNING |
| 2 | Missing task coverage for `### Requirement:` rows | ERROR |
| 3 | Ambiguous language without a metric | WARNING |
| 4 | Empty `## <heading>` sections | WARNING |
| 5 | Constitution alignment (info only) | INFO |
| 6 | Overall task completion ratio | INFO |

**Request:**
```json
{ "name": "analyze", "arguments": { "topic": "auth", "area": "Staging" } }
```

**Response:**
```json
{
  "success": true,
  "topic": "auth",
  "area": "Staging",
  "findings": [
    {
      "severity": "ERROR",
      "check": "Missing task coverage",
      "message": "Requirement 'Refresh Token' has no corresponding task.",
      "detail": "Add a `- [ ]` task to spec/Staging/auth/auth_task.md that mentions 'Refresh Token'."
    },
    {
      "severity": "WARNING",
      "check": "Ambiguous language",
      "message": "Requirement 'Login' contains 'securely' without a measurable metric.",
      "detail": "Replace with a concrete number or threshold (e.g. '< 200ms p95')."
    }
  ],
  "error_count": 1,
  "warning_count": 1,
  "info_count": 2
}
```

## Constitution

### `constitution_read`

Return the contents of `.agent/constitution.md` — the project's non-negotiable principles.

**Args:** none.

```json
{ "name": "constitution_read", "arguments": {} }
```

**Response:**
```json
{
  "success": true,
  "path": "/abs/path/.agent/constitution.md",
  "content": "# Project Constitution\n\n..."
}
```

Errors if the file doesn't exist (run `unispec init` to create it).

### `constitution_check`

Pair the constitution with a proposed action so the agent can self-evaluate whether the action would violate any principle. Intentionally simple: the real semantic evaluation happens in the model, not a regex.

**Required:** `action` (one-sentence description of what the agent is about to do).

```json
{ "name": "constitution_check", "arguments": {
    "action": "Push topic to Build without running tests"
} }
```

**Response:**
```json
{
  "success": true,
  "action": "Push topic to Build without running tests",
  "constitution": "# Project Constitution\n\n...",
  "note": "Read each principle and confirm the proposed action does not violate any. If a principle is violated, do not proceed."
}
```

## Workspace

### `workspace_status`

Combined topic list across every repo linked from `.unispec-workspace/workspace.yaml` in the server's current working directory.

**Args:** none.

**Pre-condition:** the MCP server must be launched inside a workspace root (one with `.unispec-workspace/workspace.yaml`). See [workspaces.md](workspaces.md) for setup.

```json
{ "name": "workspace_status", "arguments": {} }
```

**Response:**
```json
{
  "success": true,
  "workspace": "my-app",
  "repos": [
    {
      "name": "api",
      "path": "/abs/path/api",
      "topics": [
        { "area": "Staging", "topic": "auth" },
        { "area": "Working", "topic": "rate-limit" }
      ],
      "error": null
    },
    {
      "name": "web",
      "path": "/abs/path/web",
      "topics": [{ "area": "Staging", "topic": "login-page" }],
      "error": null
    }
  ]
}
```

Per-repo `error` is non-null when the path doesn't exist, or when the path exists but lacks `spec/` (i.e. it's not a UniSpec project).

## Change management

Three tools manage per-topic change folders (see [change-management.md](change-management.md)). The original `<topic>_spec.md` and `<topic>_task.md` are never touched — these tools only read/write under `spec/<area>/<topic>/changes/`.

### `change_add`

Create a new change folder inside an existing topic. Writes `proposal.md`, optional `design.md`, `<change>_spec.md`, and `<change>_task.md` under `spec/<area>/<topic>/changes/<change>/`.

**Required:** `topic`, `change`, `proposal`, `spec_content`, `task_content`. **Optional:** `area` (default `"Staging"`), `design`.

**Constraints.** `proposal`, `spec_content`, and `task_content` are trimmed and must each be > 10 chars. `change` is normalised by replacing `/` and ` ` with `-`. The topic directory must already exist (case-insensitive area match). Errors if `spec/<area>/<topic>/changes/<change>/` already exists.

**Request:**
```json
{ "name": "change_add", "arguments": {
    "topic": "auth", "area": "Staging", "change": "add-2fa",
    "proposal": "Protect high-value accounts with a second factor.",
    "design": "TOTP via authenticator apps; encrypted seed at rest.",
    "spec_content": "## 2FA requirements\n- TOTP enrolment\n- 8 recovery codes",
    "task_content": "- [ ] Generate TOTP seeds\n- [ ] Verify codes on login"
} }
```

**Response:**
```json
{ "success": true,
  "message": "Change folder created",
  "topic": "auth",
  "area": "Staging",
  "change": "add-2fa",
  "change_dir": "/abs/path/spec/Staging/auth/changes/add-2fa",
  "proposal_file": "/abs/path/spec/Staging/auth/changes/add-2fa/proposal.md",
  "design_file":   "/abs/path/spec/Staging/auth/changes/add-2fa/design.md",
  "spec_file":     "/abs/path/spec/Staging/auth/changes/add-2fa/add-2fa_spec.md",
  "task_file":     "/abs/path/spec/Staging/auth/changes/add-2fa/add-2fa_task.md" }
```

`design_file` is `null` when `design` was omitted.

### `change_list`

List every change directly under `spec/<area>/<topic>/changes/` (and optionally everything under `changes/archive/`). Each entry's status is computed from the task file's `- [ ]` / `- [x]` checkboxes.

**Required:** `topic`. **Optional:** `area` (default `"Staging"`), `include_archived` (default `false`).

Status values:
- `proposed` — task file has no checkboxes yet, or none are completed
- `in-progress` — some boxes checked, some not
- `complete` — all boxes checked
- `archived` — change lives under `changes/archive/` (only returned when `include_archived: true`)

**Request:**
```json
{ "name": "change_list", "arguments": {
    "topic": "auth", "area": "Staging", "include_archived": true
} }
```

**Response:**
```json
{ "success": true,
  "topic": "auth", "area": "Staging", "count": 2,
  "changes": [
    { "name": "add-oauth", "status": "proposed",
      "has_proposal": true, "has_design": false, "has_spec": true, "has_task": true },
    { "name": "add-2fa", "status": "archived",
      "has_proposal": true, "has_design": true, "has_spec": true, "has_task": true }
  ] }
```

### `change_archive`

Move a change directory from `changes/<change>/` to `changes/archive/<change>/`, **merging any delta sections into the canonical `<topic>_spec.md` first**. Supported sections in the change's `<change>_spec.md`: `## ADDED Requirements`, `## MODIFIED Requirements`, `## REMOVED Requirements`, `## RENAMED Requirements`. Operations are applied in order RENAMED → REMOVED → MODIFIED → ADDED. A change spec with no delta sections is archived without touching the canonical spec.

Errors if the source change doesn't exist or if `changes/archive/<change>/` already exists.

**Required:** `topic`, `change`. **Optional:** `area` (default `"Staging"`).

**Request:**
```json
{ "name": "change_archive", "arguments": {
    "topic": "auth", "area": "Staging", "change": "add-2fa"
} }
```

**Response:**
```json
{ "success": true,
  "message": "Change 'add-2fa' archived",
  "topic": "auth", "area": "Staging", "change": "add-2fa",
  "from": "/abs/path/spec/Staging/auth/changes/add-2fa",
  "to":   "/abs/path/spec/Staging/auth/changes/archive/add-2fa" }
```

## Readiness queue

All queue tools operate on `spec/<area>/queue.md`.

### `queue_list`

Show the raw text of the queue file.

**Optional:** `area`.

### `queue_add`

Add a topic to the queue (appended by default, inserted at `position` if provided).

**Required:** `topic`. **Optional:** `position` (default `-1` = append), `area`.

```json
{ "name": "queue_add", "arguments": { "topic": "user-login", "area": "Staging" } }
```

### `queue_remove`

Remove a topic from the queue.

**Required:** `topic`. **Optional:** `area`.

### `queue_check`

Returns `{ "ready": true|false }` depending on whether the topic appears in the queue.

**Required:** `topic`. **Optional:** `area`.

```json
{ "name": "queue_check", "arguments": { "topic": "user-login", "area": "Staging" } }
```

### `queue_reorder`

Move a topic to a new position within its current queue.

**Required:** `topic`, `new_position`. **Optional:** `area`.

```json
{ "name": "queue_reorder", "arguments": {
    "topic": "user-login", "new_position": 0, "area": "Staging"
} }
```

## Index (file ↔ spec linking)

### `index_add`

Add a topic↔path link.

**Required:** `topic`, `path`. **Optional:** `area`, `link_type`, `tags` (comma-separated), `annotation`, `exports`, `descriptions`, `export_types`, `signatures`.

```json
{ "name": "index_add", "arguments": {
    "topic": "user-login", "path": "src/auth/login.rs",
    "link_type": "implementation", "tags": "auth,backend",
    "annotation": "Core POST /login handler"
} }
```

### `unispec_bind_spec`

Bind a code file to a spec file path. Used when you want a stronger "this code implements this spec file" linkage.

**Required:** `spec_path`, `file_path`, `topic`. **Optional:** `area`.

### `index_find`

Find links by topic, path, or tag.

**Required:** `query`. **Optional:** `by` ∈ `{"topic" (default), "path", "tag"}`.

```json
{ "name": "index_find", "arguments": { "query": "user-login", "by": "topic" } }
```

### `index_lookup`

Find an export by full ID, e.g., `user-login:login_user`.

**Required:** `id`.

### `index_list`

List every link, with optional filters.

**Optional:** `topic`, `path`, `tag`.

### `index_graph`

Export the entire link graph as JSON (nodes + edges) for visualization.

**Args:** none.

### `index_backlinks`

Generate a markdown backlinks block for a topic.

**Required:** `topic`.

## Dynamic connector tools

Each `[[connector]]` in `.agent/config.toml` is published as a tool named `unispec_<connector-name>`. Calling it shells out to the configured command and returns combined stdout. See [connectors.md](connectors.md).

## Discovering the live surface

You can verify the exact list of tools a running server publishes:

```bash
printf '%s\n' \
  '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' \
  '{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}' \
  | unispec mcp 2>/dev/null
```

This is the canonical answer when the documentation and the binary diverge — the binary wins.
