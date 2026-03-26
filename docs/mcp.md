# MCP Integration

UniSpec speaks MCP (Model Context Protocol). This means you can connect AI agents like Claude, Cursor, Windsurf, and they understand your specs, workflows, and progress.

## What is MCP?

MCP is a protocol that lets AI tools interact with your project. Instead of just chatting with AI, it can:

- Read your specs
- See your workflows
- Run your connectors
- Query your progress
- Understand your project structure
- Add your own commands & scripts

## Available MCP Tools

UniSpec provides **26 built-in MCP tools** plus dynamic tools for each connector you define:

### Topic Management (7 tools)

| MCP Tool | Description | CLI Equivalent |
|----------|-------------|----------------|
| `topics_list` | List all topics in an area | `unispec topic list` |
| `topics_add` | Create a new topic | `unispec topic add` |
| `topics_show` | Show details of a topic | `unispec topic show` |
| `topics_delete` | Delete a topic | `unispec topic remove` |
| `topics_push` | Move a topic to another area | `unispec topic push` |
| `topics_pull` | Pull a topic from another area | `unispec topic pull` |
| `topics_progress` | Show progress across topics | `unispec topic progress` |

### Area Management (6 tools)

| MCP Tool | Description | CLI Equivalent |
|----------|-------------|----------------|
| `areas_list` | List all areas | `unispec area list` |
| `areas_add` | Add a new area | `unispec area add` |
| `areas_remove` | Remove an area | `unispec area remove` |
| `areas_rename` | Rename an area | `unispec area rename` |
| `areas_default` | Set the default area | `unispec area default` |
| `areas_health` | Show area health statistics | `unispec area health` |

### Index/Link Management (14 tools)

| MCP Tool | Description | CLI Equivalent |
|----------|-------------|----------------|
| `index_list` | List all index links (optional filters) | `unispec index list` |
| `index_add` | Add a link between topic and path | `unispec index add` |
| `index_remove` | Remove a link | `unispec index remove` |
| `index_find` | Find links by topic, path, tag, or annotation | `unispec index find` |
| `index_cleanup` | Remove orphaned links | `unispec index cleanup` |
| `index_tags` | List all unique tags in index | `unispec index tags` |
| `index_graph` | Export graph JSON for visualization | `unispec index graph` |
| `index_backlinks` | Generate backlinks markdown for topic | `unispec index backlinks` |
| `index_exports` | List exports (functions, classes) for a topic | `unispec index exports` |
| `index_query` | Query exports by name, type, description, or ID | `unispec index query` |
| `index_depends` | Find what topics depend on a given topic | `unispec index depends` |
| `index_lookup` | Find export by full ID | `unispec index lookup` |

### Mode Management (4 tools)

| MCP Tool | Description | CLI Equivalent |
|----------|-------------|----------------|
| `mode_list` | List all available modes | `unispec mode list` |
| `mode_info` | Get detailed info about a mode | `unispec mode info` |
| `mode_activate` | Activate an agent mode | `unispec mode activate` |
| `mode_current` | Get the current active mode | `unispec mode current` |

### Connector Tools (2 + dynamic)

| MCP Tool | Description | CLI Equivalent |
|----------|-------------|----------------|
| `connector_list` | List all available connectors | `unispec connector list` |
| `connector_run` | Run a connector command | `unispec connector run` |
| `unispec_<name>` | Dynamic tool per connector | Custom commands |

### Configuration (2 tools)

| MCP Tool | Description | CLI Equivalent |
|----------|-------------|----------------|
| `config_get` | Get current configuration | Internal |
| `config_set` | Set the default area | `unispec set` |

---

### Using MCP Tools in Conversation

Once your AI is connected, you can use natural language:

```
# Topics
"List all topics in Staging"
"Create a new topic called 'Payment API' in Working"
"Show me the details for the User Login topic"
"Push 'Payment API' to Build"

# Areas
"What areas do we have?"
"Add a new area called 'Review'"
"What's the health of each area?"

# Index
"What files are linked to the authentication topic?"
"Link src/auth/login.rs to the User Login topic"
"Find all files linked to the payment topic"

# Exports (Capability Registry)
"What functions does user-login export?"
"Find all functions named login"
"Find exports matching 'authenticate'"
"What topics depend on user-login?"
"Find the export with ID user-login:login_user"

# Modes
"Switch to sprint mode"
"What modes are available?"

# Connectors
"Run the test connector"
"Run lint with extra arguments"
```

## Connector MCP Tools

Create connectors that become AI-runnable commands:

```bash
# Add a test connector
unispec connector new test "Run test suite" "pytest" "tests/" "-v"

# Add a build connector  
unispec connector new build "Build project" "cargo" "build"

# Add a lint connector
unispec connector new lint "Run linter" "ruff" "check" "."
```

Now your AI can run:
- "Run the test connector"
- "Execute build"
- "Run lint and fix errors"

### Connector Structure

Connectors are defined in `.agent/config.toml`:

```toml
[[connector]]
name = "test"
description = "Run test suite"
command = "pytest"
args = ["tests/", "-v"]
timeout = 60
```

### Connector Dynamic Tools

Each connector automatically becomes an MCP tool named `unispec_<name>`:

```json
{
  "name": "unispec_test",
  "description": "Run test suite",
  "inputSchema": {
    "type": "object",
    "properties": {
      "args": {
        "type": "array",
        "items": {"type": "string"},
        "description": "Additional arguments to pass to the command"
      }
    }
  }
}
```

### Running Connectors

```bash
# Run a connector
unispec connector run test

# Run with arguments
unispec connector run test -- -k "test_user"

# List all connectors
unispec connector list

# Generate MCP config for connectors
unispec connector mcp
```

### Per-Project Connectors

Create connectors specific to your project:

```bash
# Custom build script
unispec connector new deploy-prod "Deploy to production" "./scripts/deploy.sh" "prod"

# Database migrations
unispec connector new migrate "Run migrations" "alembic" "upgrade" "head"

# Type checking
unispec connector new typecheck "Type check project" "mypy" "src/"
```

These are stored in `./.agent/config.toml` and available to your AI editor.

## Commands Not Available via MCP

Some CLI commands are not exposed as MCP tools because they're used for project setup or configuration rather than runtime operations:

| CLI Command | Reason not in MCP |
|-------------|-------------------|
| `unispec init` | One-time project initialization |
| `unispec mode add` | Mode addition (manual config) |
| `unispec mode remove` | Mode removal (manual config) |
| `unispec connector new` | Connector creation (config-based) |
| `unispec connector delete` | Connector deletion (config-based) |
| `unispec connector edit` | Connector editing (config-based) |
| `unispec pkg list/search/install/remove` | Package management (use CLI) |
| `unispec patty enable/disable/status` | Mascot control (use CLI) |
| `unispec index full` | Index statistics |
| `unispec index watch` | Background watcher |

## MCP Server

Start the UniSpec MCP server for direct integration:

```bash
unispec mcp
```

This starts an MCP server that tools like Claude Desktop can connect to.

### Claude Desktop Config

Add to `~/.config/claude/settings.json`:

```json
{
  "mcpServers": {
    "unispec": {
      "command": "unispec",
      "args": ["mcp", "./"]
    }
  }
}
```

## Workflow Files

When you init with an editor, UniSpec creates workflow files in your project:

### Cursor/Windsurf

Files created in `./.cursor/commands/unispec/`:

```markdown
# unispec:spec.md

## When to use
Use this workflow when starting a new feature.

## Steps
1. Check the spec for existing requirements
2. Create a new topic if needed
3. Write clear acceptance criteria
4. Link relevant files
```

### Claude Code

Files created in `./.claude/commands/unispec/`:

```markdown
# unispec-commands

[TOOL_CALL]
{
  tool => 'unispec_list_topics',
  args => {
    --area Staging
  }
}
[/TOOL_CALL]
```

### Cline

Files created in `./.clinerules/workflows/unispec/`:

```markdown
# unispec-spec

Run when user wants to create or review a spec.

1. Check existing specs in ./spec/
2. Create new topic if needed
3. Use specs to guide implementation
```

### Manual IDE MCP Setup

If you want to connect UniSpec MCP directly to your editor:

#### Cursor
1. Go to Settings → MCP
2. Add new MCP server:
   - Command: `unispec`
   - Args: `mcp`
   - Cwd: `./`

#### Windsurf
1. Go to Settings → Extensions → MCP
2. Add server with same config as Cursor

#### Claude Desktop
Add to `~/.config/claude/settings.json`:

```json
{
  "mcpServers": {
    "unispec": {
      "command": "unispec",
      "args": ["mcp"],
      "env": {
        "UNISPEC_ROOT": "./"
      }
    }
  }
}
```

#### VS Code
Use the MCP extension:
1. Install "MCP" extension
2. Add to settings.json:

```json
{
  "mcpServers": {
    "unispec": {
      "command": "unispec",
      "args": ["mcp"],
      "cwd": "./"
    }
  }
}
```

## Custom MCP Integration

### For Custom Tools

If your tool supports MCP, add UniSpec:

```json
{
  "mcpServers": {
    "unispec": {
      "command": "path/to/unispec",
      "args": ["mcp"],
      "env": {
        "UNISPEC_ROOT": "/path/to/project"
      }
    }
  }
}
```

### Environment Variables

| Variable | Description |
|----------|-------------|
| `UNISPEC_ROOT` | Project root (default: current dir) |
| `UNISPEC_MODE` | Mode to use |

## Tips

1. **Start simple** - Just `unispec init --cursor` to begin
2. **Link everything** - Use `unispec index add` to connect code to specs
3. **Add exports** - Declare what your topic provides using `--exports` when adding links
4. **Query exports** - Use `index_exports` and `index_query` to find available functions without reading entire files
5. **Use references** - When importing from another topic, use `# ref:index:topic:name` comments
6. **Run connectors** - Make AI run your tests and builds
7. **Update specs** - AI works better with accurate specs

---

## See Also

- [Commands Reference](commands.md) - CLI command documentation
- [Configuration Reference](configuration.md) - Config files, environment variables
- [Modes Documentation](modes.md) - Custom workflow configurations
- [Getting Started](getting-started.md) - Quick start guide

*See modes.md to create custom modes with their own MCP workflows, or commands.md for CLI reference.*