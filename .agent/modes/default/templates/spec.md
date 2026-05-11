---
title: <TopicName>
short: <Add a one-liner description here>
created: <DateTime>
modified: <DateTime>
author: <Author>
---

<!--
spec.md is the human-authored specification.

Runtime state (status, ownership, queue, locks) lives in `.unispec/state.toml`
and per-task state lives in this topic's `tasks.toml`. Do NOT add `status:`,
`checked_out:`, or `checked_out_at:` to this frontmatter — `spec_write` will
reject them.
-->

# Design: <TopicName>

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

### Example 2: [Name]
- **Input**: [What goes in]
- **Output**: [What comes out]

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
