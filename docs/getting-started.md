# Getting Started with UniSpec

UniSpec is a tool that helps you build better software by organizing your work around specifications (specs). Instead of jumping straight into code, you define what you're building first, then track your progress as you implement it.

This guide walks you through the basics.

---

## What is UniSpec?

Think of UniSpec as a task manager that revolves around **specs** instead of generic todos. Each feature or project gets a spec that describes:

- **What** you're building (requirements)
- **Why** you're building it (problem statement)
- **How** you'll know it's done (acceptance criteria)
- **What** tasks need to be done

Your specs move through **areas** (like Staging → Working → Build) as you progress from planning to implementation to done.

---

## Installation

### Quick Install

```bash
# Linux/macOS (using Cargo)
cargo install unispec

# Or download a release from GitHub
```

### Verify It Works

```bash
unispec --version
```

---

## Your First Project

### Step 1: Create a Project Directory

```bash
mkdir my-first-project
cd my-first-project
```

### Step 2: Initialize UniSpec

```bash
unispec init
```

This creates the basic structure:

```
my-project/
├── spec/              # Your specs live here
│   ├── Staging/       # New specs start here
│   ├── Working/       # Specs you're actively working on
│   └── Build/         # Completed specs ready to deploy
├── .agent/            # UniSpec configuration
└── .unispec/          # Runtime state (queues, ownership, locks)
    └── state.toml
```

### Step 3: Launch the Interactive Interface

```bash
unispec
```

This opens the **TUI** (Terminal User Interface) - a visual way to navigate your specs. You'll see:

- Areas on the left (Staging, Working, Build)
- Topics in each area
- Paddy the platypus (toggle with `\`)

### Step 4: Create Your First Topic

With the TUI open:

1. Press `↓` to select "Staging"
2. Press `→` to enter Staging
3. Press `n` to create a new topic
4. Type a name like "User Login"
5. Press `Enter`

Now you've created a topic!

---

## Understanding Topics

A **topic** is a unit of work - a feature, bug fix, or project. Each topic contains:

### `spec.md` - The Specification

This describes **what** you're building. Here's a simple template:

```markdown
# Feature Name

## Problem Statement
What problem does this solve?

## User Stories
- As a [user], I want [action] so that [benefit]

## Requirements
- [ ] Must have feature
- [ ] Another must have

## Acceptance Criteria
1. Criterion 1
2. Criterion 2
```

### `tasks.toml` + `task.md` - The Tasks

`tasks.toml` is the authoritative task state for a topic (machine-owned).
`task.md` is a human-readable rendering of `tasks.toml` and is regenerated
automatically — never edit it by hand.

You normally don't author `tasks.toml` directly. Use the MCP tools
(`spec_add` to seed it, `task_status` to mutate one task at a time) and
the renderer produces a `task.md` that looks like:

```markdown
# Tasks - Feature Name

## Phase 1: Foundation
- [ ] **1.1** Task 1
- [ ] **1.2** Task 2

## Phase 5: Testing
- [ ] **T1** Test 1
- [ ] **T2** Test 2
```

---

## The Basic Workflow

### 1. Create a Spec in Staging

Start by creating a topic in Staging. This is where new ideas go before they're ready for work.

### 2. Move to Working When Ready

When you've written your spec and it's ready to implement:

```bash
# From command line
unispec topic push "Your Topic" Working
```

Or in the TUI:
- Select your topic
- Press `p`
- Type "Working"

### 3. Implement and Track Progress

Now you're in Working! This is where you:
- Write code
- Check off tasks as you complete them
- Update the spec if requirements change

### 4. Move to Build When Done

When everything is implemented and tested:

```bash
unispec topic push "Your Topic" Build
```

Build is for completed work - ready to deploy or ship.

### 5. Done!

Your spec has moved through the full workflow: Staging → Working → Build

---

## Using the TUI

The TUI is your primary interface. Here's what you need to know:

### Navigation

| Key | Action |
|-----|--------|
| `↑` / `↓` | Move between topics/areas |
| `→` | Enter selected area or topic |
| `←` | Go back |
| `Enter` | Open a topic file |

### Actions

| Key | Action |
|-----|--------|
| `n` | Create new topic |
| `r` | Remove topic (with confirmation) |
| `p` | Push topic to another area |
| `f` | Find files linked to this topic |
| `\` | Toggle Paddy the platypus |
| `q` | Quit |

### Tips

- Press `/` to search/filter topics
- Your specs are just regular Markdown files - you can edit them in your favorite editor too
- Use `unispec topic list` to see all topics from the command line

---

## Quick Command Reference

```bash
# Start the TUI
unispec

# Initialize a new project
unispec init

# Create a topic
unispec topic add "Feature Name"

# List topics
unispec topic list

# Show progress
unispec topic progress

# Move a topic to another area
unispec topic push "Feature Name" Working

# List areas
unispec area list

# Show area health
unispec area health
```

---

## Why Bother with Specs?

You might be thinking "this seems like extra work". Here's why it matters:

1. **Clarity** - Writing down what you're building forces you to think it through
2. **Communication** - Specs make it easy to share what you're working on
3. **Tracking** - You always know what stage each feature is in
4. **Completion** - Acceptance criteria make it clear when you're actually done

---

## What's Next?

If you just want to use UniSpec, you now know enough! The basic workflow is:

1. `unispec init` - Start a project
2. `unispec` - Open the TUI
3. Create topics → Write specs → Implement → Move through areas

But UniSpec can do much more. Read on for advanced features...

---

## Advanced Features

### Installing Modes from the Repository

UniSpec has a package repository with pre-built modes for different workflows:

```bash
# See what's available
unispec pkg list

# Search for something specific
unispec pkg search sprint

# Install a mode
unispec pkg install sprint-mode

# Install globally (available to all your projects)
unispec pkg install sprint-mode --global
```

Different modes give you different area structures. For example:
- **Simple Mode** - Staging → Working → Build
- **Sprint Mode** - Backlog → In Sprint → Review → Done
- **Kanban Mode** - To Do → In Progress → Done

### Creating Custom Modes

You can create your own mode with custom areas and workflows:

```bash
# Create mode directory structure
mkdir -p .agent/modes/my-mode
```

Then add:
- `mode.toml` - Mode metadata
- `skill.md` - How AI agents should behave
- `workflows/` - Step-by-step guides
- `areas/` - Your custom areas

See [Creating Modes](modes.md) for the full guide.

### Connecting AI Editors

Want AI assistants like Claude, Cursor, or Windsurf to understand your specs?

```bash
# Add editor integrations
unispec init --cursor --cline --windsurf --claude-code
```

This creates workflow files in your editor's config folder. Now your AI can:
- Read your specs
- See what tasks need doing
- Run your connectors

See [MCP Integration](mcp.md) for details.

### Using Connectors

Connectors are custom commands that become MCP tools. Define them in `.agent/config.toml`:

```toml
[[connector]]
name = "test"
description = "Run the test suite"
command = "pytest"
args = ["tests/", "-v"]
```

Now AI can run "run the tests" and it'll execute your test command.

### Indexing Code to Specs

Link your code files to topics so AI knows which code implements which feature:

```bash
# Link a file to a topic
unispec index add --topic "user-login" --path src/auth/login.rs

# Find what's linked
unispec index find "user-login" --by topic
```

See [Indexing](indexing.md) for patterns and best practices.

### Configuring Your Setup

UniSpec uses a three-level config hierarchy:

1. **Local** - `./.agent/config.toml` (project-specific)
2. **Global** - `~/.config/unispec/` (user-wide)
3. **System** - `/usr/share/unispec/` (install-wide)

Local overrides global, which overrides system.

See [Configuration Reference](configuration.md) for all options.

---

## Summary

You now know how to:

**Beginner:**
- Initialize a project
- Create and manage topics
- Write specs and tasks
- Move topics through areas (Staging → Working → Build)
- Use the TUI

**Advanced:**
- Install modes from the repository
- Create custom modes
- Connect AI editors
- Use connectors
- Index code to specs
- Configure your setup

---

## Learn More

- [Commands Reference](commands.md) - Complete CLI documentation
- [Configuration Reference](configuration.md) - Config files and settings
- [Creating Modes](modes.md) - Build custom workflows
- [MCP Integration](mcp.md) - Connect AI agents
- [Indexing](indexing.md) - Link code to specs

---

*Write the spec first. Code second. Paddy believes in you.* 🦫