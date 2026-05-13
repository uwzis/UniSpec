# Workspaces

Coordinate multiple UniSpec projects from a single root. A workspace is a folder containing `.unispec-workspace/workspace.yaml` and a list of named pointers to other UniSpec project roots.

## When to use a workspace

A workspace is the right answer when:

- Your product spans multiple repos (`api`, `web`, `mobile`) and each is its own UniSpec project.
- You want to ask "what's everyone working on?" without `cd`-ing through every repo.
- You want an AI agent to plan across repos — e.g. spec the API change AND the matching UI change together.

A workspace is NOT a monorepo refactor. Each linked repo keeps its own `spec/` and `.agent/` directory; the workspace just maintains pointers.

## On-disk layout

```
my-app/                            ← workspace root
├── .unispec-workspace/
│   └── workspace.yaml             ← name, version, links
├── api/                           ← linked UniSpec project
│   ├── spec/
│   └── .agent/
└── web/                           ← linked UniSpec project
    ├── spec/
    └── .agent/
```

The linked projects don't have to be subdirectories — they can be anywhere on disk. The workspace YAML stores absolute paths.

## `workspace.yaml`

```yaml
name: my-app
version: 1
links:
  api: /abs/path/to/api
  web: /abs/path/to/web
```

| Field | Meaning |
|---|---|
| `name` | Display name shown by `workspace list / status`. Free-form string. |
| `version` | Schema version. Currently always `1`. |
| `links` | Map of short names to absolute project paths. Short names are free-form. |

The schema is small enough that UniSpec hand-rolls the reader/writer in `src/commands/workspace.rs` — no `serde_yaml` dependency.

## CLI

### `unispec workspace init <name>`

Create `.unispec-workspace/workspace.yaml` in the current directory.

```bash
mkdir my-app && cd my-app
unispec workspace init my-app
```

Errors if the file already exists. Use a text editor for subsequent edits, or `unispec workspace link` to add repos.

### `unispec workspace link <name> <path>`

Add a named pointer to another UniSpec project.

```bash
unispec workspace link api /path/to/api-repo
unispec workspace link web ../web-repo
```

Paths are stored as absolute (relative paths are resolved against the workspace root). Re-running with the same `<name>` updates the existing entry.

There is no `workspace unlink` command — edit `workspace.yaml` directly to remove an entry.

### `unispec workspace list`

```bash
$ unispec workspace list
Workspace: my-app
  - api          /abs/path/to/api [✓ ✓ unispec]
  - web          /abs/path/to/web [✓ ✓ unispec]
```

Per row: `[<path-status> <unispec-status>]`.

- Path-status: `✓` if the path exists, `✗ path missing` otherwise.
- UniSpec-status: `✓ unispec` if the path contains both `spec/` and `.agent/`, `✗ no spec/.agent` otherwise.

### `unispec workspace status`

Combined topic list across every linked repo.

```bash
$ unispec workspace status
Workspace: my-app

[api] /abs/path/to/api
  Staging/auth
  Working/rate-limit

[web] /abs/path/to/web
  Staging/login-page
```

Add `--json` to emit a structured payload:

```bash
unispec workspace status --json
```

```json
{
  "success": true,
  "workspace": "my-app",
  "repos": [
    { "name": "api", "path": "/abs/path/to/api",
      "topics": [
        { "area": "Staging", "topic": "auth" },
        { "area": "Working", "topic": "rate-limit" }
      ],
      "error": null },
    { "name": "web", "path": "/abs/path/to/web",
      "topics": [{ "area": "Staging", "topic": "login-page" }],
      "error": null }
  ]
}
```

Per-repo `error` is non-null when the path is missing or doesn't contain `spec/` (i.e. not a UniSpec project).

## MCP

### `workspace_status`

Equivalent of `unispec workspace status --json`. The MCP server must be launched from inside a workspace root (otherwise `.unispec-workspace/workspace.yaml` won't be found).

```json
{ "name": "workspace_status", "arguments": {} }
```

Same response shape as the CLI's `--json` output.

There are intentionally no `workspace_init` / `workspace_link` MCP tools — workspace setup is a one-time admin step, not something an agent should do programmatically.

## Recommended setup

```bash
# Each project is its own UniSpec init
cd ~/api-repo  && unispec init
cd ~/web-repo  && unispec init

# Workspace lives one level up
mkdir ~/my-app && cd ~/my-app
unispec workspace init my-app
unispec workspace link api ~/api-repo
unispec workspace link web ~/web-repo

unispec workspace status
```

To run the MCP server against the workspace:

```json
{
  "mcpServers": {
    "unispec": {
      "command": "unispec",
      "args": ["mcp"],
      "cwd": "/abs/path/to/my-app"
    }
  }
}
```

Inside Claude Code / Cursor / etc., agents can now call `workspace_status` to see every topic across both repos.

## Workspaces vs. monorepos

If your `api` and `web` already live in the same repo, you don't need a workspace — they can be sibling topics in one UniSpec project (e.g. `spec/Staging/api-auth/` and `spec/Staging/web-login/`).

A workspace is only useful when the projects are physically separate repos / directories with their own `.agent/` config.

## Limitations

- No CLI command to push topics between linked repos. A topic in `api/spec/Working/auth/` cannot be `topics_push`-ed into `web/spec/Working/`. Workspaces are coordination only, not a unified pipeline.
- No cross-repo `change_archive` merge. Each repo's `change_archive` operates on its own `spec/`.
- No global queue. Each linked repo keeps its own `queue.md`.
- `workspace.yaml` stores absolute paths, so workspace files don't travel cleanly between machines. Tracked deliberately — paths are per-machine. Don't commit them to a shared repo without a local override pattern.

## See also

- [`unispec init`](commands.md#init) — each linked repo needs its own init
- [workflow.md](workflow.md) — the 5-area pipeline operates per repo
- [`workspace_status`](mcp-tools-reference.md#workspace_status) — MCP shape
