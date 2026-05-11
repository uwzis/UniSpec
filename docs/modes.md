# Creating Modes

Modes define how you work with UniSpec. The default "Simple Mode" uses Staging → Building → Ship. But you can create custom modes for different workflows.

## What is a Mode?

A mode is a complete workflow configuration containing:

- **Workflows** - Step-by-step guides for your process
- **Areas** - Where topics live at each stage
- **Skill** - How AI agents should behave
- **Templates** - Starting points for specs and tasks
- **Connectors** - Commands to run

## Mode Directory Structure

```
.agent/modes/<mode-name>/
├── mode.toml          # Mode metadata and configuration
├── skill.md           # Agent persona
├── workflows/         # Workflow definitions
│   ├── spec.md
│   ├── build.md
│   └── verify.md
├── areas/            # Area-specific area-description templates
│   ├── staging/
│   │   └── area.md      # Template for area description in staging
│   ├── working/
│   │   └── area.md
│   └── build/
│       └── area.md
└── templates/        # Topic-content templates
    ├── topic.md
    ├── spec.md
    ├── task.md
    └── area.md
```

**Filenames are infrastructure-level constants.**
In Phase 1, every topic uses the canonical layout
(`spec.md`, `tasks.toml`, derived `task.md`, optional `notes.md`)
regardless of mode or area. Modes can vary their *content templates*
(what goes inside `spec.md` and the seed shape of `task.md`) and their
area metadata, but they cannot rename the files themselves.

**Template lookup order:**
1. First: `.agent/modes/<mode>/areas/<area>/area.md` (area-specific
   description)
2. Fallback: `.agent/modes/<mode>/templates/<file>.md` (mode-wide
   topic / spec / task / area templates)

## Creating a Mode

### 1. Create the directory

```bash
mkdir -p .agent/modes/sprint
```

### 2. Create mode.toml

```toml
[mode]
name = "sprint"
display_name = "Sprint Mode"
version = "1.0.0"

[author]
name = "Your Name"
email = "you@example.com"

[mode.description]
short = "Two-week sprint workflow"
long = """
Sprint Mode follows a two-week sprint cycle:
1. Sprint planning creates specs
2. Daily standups update progress
3. Sprint review moves to done
"""

[areas]
default = ["Backlog", "In Sprint", "Review", "Done"]
protected = ["In Sprint", "Review"]

[capabilities]
spec_writing = true
building = true
verification = true
connectors = true
custom_workflows = true

[templates]
# Canonical filenames are infrastructure-level constants in Phase 1.
# Every topic gets `spec.md`, `tasks.toml`, derived `task.md`, and the
# area gets `area.md`. These names are fixed across modes and areas;
# the values below exist for legacy compatibility only and should not
# be changed.
spec_file = "spec.md"
task_file = "task.md"
area_file = "area.md"

# Use the per-area `area.md` template if present, otherwise fall back
# to the mode-wide template under `templates/`.
use_area_templates = true
```

### Template Configuration

The `[templates]` section controls:

1. **Canonical filenames** - `spec.md` / `task.md` / `area.md` are fixed
   and the same in every mode and every area. Modes customize *content*,
   not filenames.
2. **`use_area_templates`** - Whether to use area-specific templates from
   `.agent/modes/<mode>/areas/<area>/area.md`. Topic content templates
   (`topic.md`, `spec.md`, `task.md`) live under
   `.agent/modes/<mode>/templates/`.

**How it works:**
- Area description templates are read from
  `.agent/modes/<mode>/areas/<area>/area.md`.
- Topic content templates (`topic.md`, `spec.md`, `task.md`) are read
  from `.agent/modes/<mode>/templates/`.
- Topics are always created using the canonical filenames
  (`spec.md`, `tasks.toml`, derived `task.md`).
- When pushing between areas, the source area copy is kept and the
  target area copy uses the same canonical filenames; `tasks.toml`
  remains the authoritative task store per-area-copy.

### 3. Create skill.md

```markdown
# Sprint Mode Agent Persona

You are a sprint-focused developer who values:
- Incremental progress
- Clear sprint goals
- Daily momentum

Your approach:
1. Check sprint backlog first
2. Update task status daily
3. Flag blockers immediately
4. Focus on completing, not starting
```

### 4. Create workflows

Create `.agent/modes/sprint/workflows/sprint:plan.md`:

```markdown
# Sprint Planning

1. Review backlog items
2. Estimate effort (t-shirt sizes or story points)
3. Select items for sprint
4. Break into tasks
5. Assign to team members
6. Set sprint goal
```

Create `.agent/modes/sprint/workflows/sprint:standup.md`:

```markdown
# Daily Standup

1. What did I complete yesterday?
2. What will I do today?
3. Any blockers?
```

### 5. Create area templates

Create `.agent/modes/sprint/areas/backlog/area.md`:

```markdown
# Backlog

Items ready for prioritization but not yet in a sprint.

## Criteria for Backlog
- Has a spec
- Has acceptance criteria
- Is prioritized by product owner
```

Create `.agent/modes/sprint/areas/in-sprint/area.md`:

```markdown
# In Sprint

Currently being worked on in this sprint.

## Rules
- Each task has an owner
- Blockers must be flagged
- Daily updates required
```

### 6. Create templates

Create `.agent/modes/sprint/templates/spec.md`:

```markdown
# [Feature Name]

## Sprint Goal
[One sentence about what we're achieving]

## From Backlog
- Original spec: [link]
- Priority: [high/medium/low]
- Estimate: [XS/S/M/L/XL]

## This Sprint
- What we're building: [specific scope]
- What we're NOT building: [out of scope]

## Tasks
- [ ] ...
```

## Activating a Mode

```bash
# List available modes
unispec mode list

# Activate your mode
unispec mode activate sprint

# Check current mode
unispec mode current
```

## Installing Modes

### Local Modes

Modes in your project's `.agent/modes/` directory are available only to that project.

```bash
# Add a local mode (project-specific)
unispec mode add ./modes/my-custom-mode
```

### Global Modes

Modes in your user config directory are available across all projects.

```bash
# Add a mode globally (available to all projects)
unispec mode add /path/to/mymode --global

# Remove a global mode
unispec mode remove mymode --global
```

Global modes are stored in `~/.config/unispec/.agent/modes/` and searched when:
- Listing modes (`unispec mode list`)
- Activating a mode (`unispec mode activate`)
- Running the MCP server

### Mode Search Order

Modes are searched in this order:
1. Local: `./.agent/modes/`
2. Global: `~/.config/unispec/.agent/modes/`
3. System: `/usr/share/unispec/.agent/modes/`

The first match wins.

## Installing Modes from Repository

You can install pre-built modes from the community package repository:

```bash
# List available packages
unispec pkg list

# Search for modes
unispec pkg search sprint

# Install a mode package
unispec pkg install sprint-mode

# Install globally (available to all projects)
unispec pkg install sprint-mode --global

# List installed packages
unispec pkg installed
```

Packages can include modes, connectors, and workflows. See [Commands Reference](commands.md#pkg) for full details.

## Mode Types

### Simple Mode (Default)
- Staging → Building → Ship
- Good for: Solo devs, small teams

### Sprint Mode
- Backlog → In Sprint → Review → Done
- Good for: Agile teams, two-week cycles

### Kanban Mode
- To Do → In Progress → Code Review → Done
- Good for: Continuous delivery, Ops teams

### Docs Mode
- Draft → Review → Published
- Good for: Documentation projects

### RFC Mode
- Proposed → Discussion → Approved/Rejected
- Good for: Architecture decisions

## Sharing Modes

Modes are just directories. Share them by:

1. **Git**: Commit the mode folder
2. **Template**: Create a template repo
3. **Distribution**: Package as a tarball

Example sharing:
```bash
# Export mode
tar -cvzf my-mode.tar.gz .agent/modes/my-mode/

# Import mode
tar -xvzf my-mode.tar.gz -C /path/to/project/.agent/modes/
```

## Mode Configuration Options

### mode.toml Reference

```toml
[mode]
name = "simple"
display_name = "Simple Mode"
description = "Default mode with Spec-Driven Development workflows. Staging: specs being written. Working: specs being built. Build: shippable code."
version = "1.0.0"

[author]
name = "UniSpec Team"
contact = "https://github.com/uwzis/UniSpec"

[requirements]
min_unispec_version = "0.0.7"

[areas]
default = ["Staging", "Working", "Build"]
protected = ["Staging", "Working", "Build"]
default_area = "Working"

[capabilities]
spec_writing = true
building = true
verification = true
connectors = true
custom_workflows = false

[dependencies]
extends = []

[scripts]
pre_activate = ""
post_activate = ""

# Area Types Configuration - Define how each area type is rendered
[area_types.roadmap]
display_type = "roadmap"
spec_file = "spec.md"
parser = "frontmatter"
list_fields = ["title", "impact", "change_type"]
sort_by = "impact"

[area_types.working]
display_type = "working"
spec_file = "spec.md"
task_file = "tasks.md"
parser = "tasks"
list_fields = ["title", "progress", "status"]
sort_by = "status"

[area_types.build]
display_type = "build"
spec_file = "spec.md"
parser = "dates"
list_fields = ["title", "completed_date"]
sort_by = "modified"
```

## Area Types

Area types define how the TUI renders and parses specs for different areas:

### Display Types

| Type | Description | TUI Display |
|------|-------------|-------------|
| `roadmap` | Proposed items with impact levels | `[CRIT] feat Feature Name` |
| `working` | Task-based progress | `[====    ] (3/5)` progress bar |
| `build` | Completed specs with dates | `✅ Feature Name (2026-03-28)` |
| `standard` | Default task-based (backwards compat) | Progress bar |

### Parser Types

| Type | What it reads |
|------|---------------|
| `frontmatter` | YAML frontmatter (impact, change_type, title) |
| `tasks` | Markdown checkboxes (`- [ ]`, `- [-]`, `- [x]`) |
| `dates` | Created/modified/completed dates in frontmatter |
| `standard` | Tasks only (backwards compatible) |

### Area Type Configuration

```toml
[area_types.<name>]
display_type = "roadmap"     # How TUI renders
spec_file = "spec.md"        # File to read for metadata
parser = "frontmatter"       # How to parse metadata
list_fields = ["title", "impact", "change_type"]
sort_by = "impact"           # Default sort order
```

### Spec Frontmatter Format

Specs in roadmap areas should use YAML frontmatter:

```yaml
---
title: User Authentication
impact: high
change_type: feature
status: proposed
created: 2026-03-28
---

# User Authentication

Description...
```

### Custom Classifications

You can define custom impact levels and change types in mode.toml:

```toml
[area_types.roadmap]
display_type = "roadmap"
spec_file = "spec.md"
parser = "frontmatter"

# Custom impact labels - key is frontmatter value, value is displayed
[area_types.roadmap.impact_labels]
critical = "CRITICAL"
high = "HIGH"
medium = "MEDIUM"
low = "LOW"
p0 = "P0"
p1 = "P1"
p2 = "P2"

# Custom change type labels
[area_types.roadmap.change_type_labels]
feature = "feature"
bugfix = "bugfix"
enhancement = "enhancement"
research = "research"
spike = "spike"
```

### Default Impact Levels

| Level | Badge | TUI Color |
|-------|-------|-----------|
| `critical` | `[CRITICAL]` | Red |
| `high` | `[HIGH]` | Yellow |
| `medium` | `[MEDIUM]` | Cyan |
| `low` | `[LOW]` | White |

### Default Change Types

| Type | Badge |
|------|-------|
| `feature` | feature |
| `bugfix` | bugfix |
| `refactor` | refactor |
| `documentation` | docs |
| `security` | security |

## TUI Workflow

### Creating Roadmap Items

When creating a new spec in a Roadmap area, the TUI prompts for:

1. **Name** - Topic name
2. **Impact** - `critical`, `high`, `medium`, or `low` (or custom values from mode.toml)
3. **Type** - `feature`, `bugfix`, `refactor`, `docs`, or `security` (or custom values)

These are saved in the spec's frontmatter automatically.

### Pushing to Build

When pushing a spec to the Build area, UniSpec automatically:

- Updates `status: completed`
- Adds `modified: <date>`
- Adds `completed: <timestamp>`

The completion timestamp is displayed in the Build area TUI view.

### Build Area Display

Build area shows completed specs with their completion date:

```
✅ User Auth (2026-03-28 14:30:00)
✅ API v2 (2026-03-27 09:15:00)
```

## Best Practices

1. **Start simple** - Don't over-engineer your first mode
2. **Document the why** - Your skill.md explains agent behavior
3. **Share templates** - Help teammates get started
4. **Iterate** - Modes evolve with your team

---

## See Also

- [Commands Reference](commands.md) - CLI command documentation
- [Configuration Reference](configuration.md) - Config files, environment variables
- [MCP Documentation](mcp.md) - AI agent integration
- [Getting Started](getting-started.md) - Quick start guide

*Need help? Check commands.md for CLI usage or mcp.md to connect AI agents to your mode.*
