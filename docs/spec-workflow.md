# Skill: UniSpec SPEC Workflow

**Act as a UniSpec Specification Architect.** Your goal is to create precise, actionable specs and tasks using the system templates.

---

## Purpose
This workflow is for creating, modifying, and arranging topics, specs, and tasks. Use this when planning new features, refining specifications, or organizing work.

---

## KEY RULE: Topics First, Always!

**Constraint: Before doing ANYTHING else, you MUST research topics first!**

---

## CREATE TOPICS → FILL TOPICS → CREATE SPECS → FILL SPECS

**CRITICAL: ALWAYS use MCP tools to create each item one at a time with full content matching the templates!**

### Step 1: Create Topics Using topics_add MCP Tool

**Format: Use the topic.md template structure**

```
topics_add {topic: "auth", area: "Staging", content: "---
title: auth
created: 2026-04-08 10:00:00
author: Your Name
---

# auth

## Overview
[What this topic covers - high-level summary in 1-2 sentences]

## Purpose
[Why this topic exists - what problem it solves, who benefits]

## Specs
- [spec name]: [What this spec covers]

## Sub-topics
- [sub-topic name]
"}
```

**Constraint: Never create an empty topic. Always provide full content matching the template.**

### Step 2: Create Specs Using spec_add MCP Tool

**Format: Use the spec.md template exactly. Include all sections:**

```
spec_add {topic: "auth/login", area: "Staging", content: "---
title: auth/login
created: 2026-04-08 10:00:00
author: Your Name
---

# Design: auth/login

## Overview

> High-level summary: What is this feature in 1-2 sentences?

## Purpose

> Why does this feature exist? What problem does it solve? Who benefits?

## In-Depth Details

> Technical explanation: How does it work? What are the components? What are the key decisions?

## Requirements

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-001 | [Requirement statement] | Must/Should |

## Examples

### Example 1: [Name]
- **Input**: [What goes in]
- **Output**: [What comes out]
- **Flow**:
  1. [Step 1]
  2. [Step 2]

## Data Model

### Entities

| Entity | Fields | Description |
|--------|--------|-------------|
| [Name] | [field]: [type] | [description] |

### Relationships

```
[EntityA] ──1:N──> [EntityB]
```

## Out of Scope

- [What this spec does NOT cover]
- [What belongs in a different spec]

## Acceptance Criteria

- [ ] [Criterion 1]
- [ ] [Criterion 2]
", task_content: "# Tasks: auth/login

## Implementation

### Phase 1: Foundation
- [ ] **1.1** [Foundation task - e.g., Set up project structure]

### Phase 2: Core Features
- [ ] **2.1** [Core feature task - e.g., Implement login form]

### Phase 3: Integration
- [ ] **3.1** [Integration task - e.g., Connect to authentication service]

### Phase 4: Polish
- [ ] **4.1** [Polish task - e.g., Add error handling]
"}
```

**Constraint: Provide markdown task content matching the four phases shown.
`spec_add` parses this markdown into the topic's authoritative `tasks.toml`
and regenerates the derived `task.md` from it. Do NOT include task-status
frontmatter — runtime state lives in `tasks.toml`, not in `task.md`. Only
create IMPLEMENTATION tasks - NO testing tasks!**

---

## CRITICAL: NO TESTING TASKS IN SPEC PHASE!

**Constraint: NEVER create testing tasks during the SPEC workflow!**

Testing tasks are ONLY created in the BUILD phase, right before pushing to Testing.

- ❌ DO NOT add "write tests", "run tests", "test functionality" tasks
- ✅ DO add implementation tasks like "implement login form", "connect to API"

**Remember: Testing comes LATER in the BUILD phase.**

---

## Complete Example Flow

**Use MCP tools ONE AT A TIME with content matching templates:**

```
# 1. Research first
topics_list {area: "Staging"}

# 2. Create topic WITH full content (matching template)
topics_add {topic: "auth", area: "Staging", content: "..."}

# 3. Create spec WITH full content AND task WITH full content (both matching templates)
spec_add {topic: "auth/login", area: "Staging", content: "...", task_content: "..."}
```

---

## Key Rules

1. **ALWAYS work in Staging area**
2. **ALWAYS start with topics**
3. **Use MCP tools** - topics_add, spec_add with content/task_content params
4. **Create + Fill = ONE action** - Never separate them
5. **Follow templates EXACTLY** - Use the `spec.md` and `task.md` template structure. `task.md` is a derived rendering of `tasks.toml`; only its content shape matters here.
6. **One at a time** - Complete one piece before starting the next
7. **Use nested topics with `/`**
8. **NO TESTING TASKS** - Only create IMPLEMENTATION tasks. Testing is handled in the BUILD phase.
9. **Verify against template** - Always ensure your spec/task matches the template format