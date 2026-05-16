# MCP Integration — Editor Configs

`unispec mcp` runs a JSON-RPC over stdio MCP server that exposes 39 built-in tools (plus dynamic `unispec_<name>` tools for each connector). This document shows how to wire it into five popular MCP-aware editors.

> **Looking for the tool list?** See [mcp-tools-reference.md](mcp-tools-reference.md). For mcp.md's design-level overview, see [mcp.md](mcp.md).

## Common shape

All editors below boot the same command:

```bash
unispec mcp
```

Optional first positional arg: the project path to operate against. If omitted, the server uses its current working directory.

```bash
unispec mcp /path/to/project
```

Each editor configures the command, args, and (optionally) the working directory.

## Claude Code (Desktop)

Edit `~/.config/Claude/claude_desktop_config.json` on Linux, `~/Library/Application Support/Claude/claude_desktop_config.json` on macOS:

```json
{
  "mcpServers": {
    "unispec": {
      "command": "unispec",
      "args": ["mcp"],
      "env": {
        "UNISPEC_ROOT": "/abs/path/to/your/project"
      }
    }
  }
}
```

Restart Claude Code. The 39 tools should appear in the tools dropdown.

If the `unispec` binary isn't on Claude Code's `PATH`, use an absolute path:

```json
{
  "mcpServers": {
    "unispec": {
      "command": "/home/you/.cargo/bin/unispec",
      "args": ["mcp", "/home/you/your-project"]
    }
  }
}
```

## Cursor

Open the Cursor settings → MCP → "Add new MCP server". Or edit `~/.cursor/mcp.json`:

```json
{
  "mcpServers": {
    "unispec": {
      "command": "unispec",
      "args": ["mcp"],
      "cwd": "/abs/path/to/your/project"
    }
  }
}
```

Cursor honours `cwd` instead of `env.UNISPEC_ROOT`, so passing the project via `cwd` works on every recent Cursor build.

## Windsurf

`~/.codeium/windsurf/mcp_config.json`:

```json
{
  "mcpServers": {
    "unispec": {
      "command": "unispec",
      "args": ["mcp"],
      "cwd": "/abs/path/to/your/project"
    }
  }
}
```

Restart Windsurf for it to pick up the new server.

## Cline (VS Code extension)

`~/.config/Code/User/globalStorage/saoudrizwan.claude-dev/settings/cline_mcp_settings.json`:

```json
{
  "mcpServers": {
    "unispec": {
      "command": "unispec",
      "args": ["mcp"],
      "env": {
        "UNISPEC_ROOT": "/abs/path/to/your/project"
      },
      "disabled": false,
      "autoApprove": []
    }
  }
}
```

`autoApprove` is Cline-specific: if you want to skip the per-call approval prompt for safe read-only tools, add them by name, e.g. `["topics_list", "tasks_list", "queue_check"]`.

## Zed

Zed reads MCP servers from its `settings.json` (Cmd/Ctrl+, then "Open Settings JSON"):

```json
{
  "context_servers": {
    "unispec": {
      "command": {
        "path": "unispec",
        "args": ["mcp"],
        "env": {
          "UNISPEC_ROOT": "/abs/path/to/your/project"
        }
      }
    }
  }
}
```

(Field name `context_servers` is Zed's, not the generic `mcpServers`.)

## Verifying the connection

Once an editor is configured, the simplest verification is to ask the agent to list tools. The set should include `topics_list`, `topics_add`, `spec_add`, `next`, `analyze`, `constitution_read`, `constitution_check`, `change_add`, `change_list`, `change_archive`, `workspace_status`, `queue_add`, `tasks_list`, `tasks_complete`, `notes_add`, `index_add`, `unispec_read_spec`. If any of those are missing, the binary on `PATH` is an older revision — see [troubleshooting.md](troubleshooting.md).

A bare smoke test from any shell (no editor needed) — pipe a single JSON-RPC line:

```bash
printf '%s\n' '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' \
  '{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}' \
  | unispec mcp /path/to/project 2>/dev/null \
  | grep -c '"name":"'
```

Should print `39` (plus one per configured connector).

## Project-root resolution

Three ways to point the server at a project, in order of precedence:

1. `unispec mcp /abs/path/to/project` — positional arg on the server command line.
2. `cwd` in the editor MCP config.
3. `UNISPEC_ROOT` environment variable.

If none are set, the server uses its OS-level current directory (likely your home directory). MCP tool calls then look for `spec/` in the wrong place. Always set at least one of the three.

## Dynamic connector tools

Any `[[connector]]` entry in `.agent/config.toml` becomes a dynamic MCP tool named `unispec_<connector-name>`. From the editor's perspective, these look like any other tool. See [connectors.md](connectors.md) for the configuration format and worked examples.

## What the agent can actually do via MCP

Once connected, the agent can call any of the 39 built-in tools. The full list lives in [mcp-tools-reference.md](mcp-tools-reference.md); the high-level groupings are:

| Group | Tools |
|-------|-------|
| **Agent feed** | **`next`** — call before every action |
| **Analysis** | **`analyze`** — cross-artifact consistency checker |
| **Constitution** | **`constitution_read`, `constitution_check`** |
| Areas | `areas_list` |
| Topics | `topics_list`, `topics_add`, `topics_show`, `topics_delete`, `topics_push`, `topics_pull`, `topics_progress` |
| Reading | `read_asset`, `unispec_read_spec` |
| Specs & tasks | `spec_add`, `spec_write`, `task_write`, `task_status`, `tasks_list`, `tasks_complete`, `tasks_incomplete` |
| Change management | `change_add`, `change_list`, `change_archive` (archive merges delta sections into the canonical spec) |
| **Workspace** | **`workspace_status`** — combined topic list across linked repos |
| Notes | `notes_read`, `notes_add` |
| Queue | `queue_list`, `queue_add`, `queue_remove`, `queue_check`, `queue_reorder` |
| Index | `index_add`, `index_find`, `index_lookup`, `index_list`, `index_graph`, `index_backlinks`, `unispec_bind_spec` |
| Dynamic | `unispec_<connector-name>` per `[[connector]]` in `.agent/config.toml` |

The change-management tools (`change_add`, `change_list`, `change_archive`) are the right answer whenever a topic already has a `<topic>_spec.md` and the agent needs to layer on new requirements — `spec_add` silently overwrites, which would destroy history. See [change-management.md](change-management.md) for the full agent workflow.

## See also

- [MCP Tools Reference](mcp-tools-reference.md) — every tool with required args and JSON-RPC examples.
- [Change Management](change-management.md) — when and how to call `change_add` / `change_list` / `change_archive`.
- [Connectors](connectors.md) — how to expose shell commands as MCP tools.
- [Troubleshooting](troubleshooting.md) — "MCP server not connecting" and similar issues.
