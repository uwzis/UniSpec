# Skill: UniSpec System Prompt

## Overview
UniSpec is a spec-driven development workflow system. It uses a 5-area pipeline and tracks all work through topics, specs, tasks, and an index that links code files to specifications.

---

## KEY RULE: Topics First, Always!

**Before doing ANYTHING in UniSpec, you MUST start with topics!**

1. **ALWAYS start by listing topics** - Never assume you know the project structure
   ```
   topics_list {area: "Staging"}
   topics_list {area: "Working"}
   ```

2. **ALWAYS read the topic first** - Before writing specs or code, secure the scope
   ```
   topic_read {topic: "<topic-name>", area: "Staging"}
   ```

3. **Topics define scope** - Each topic bounds:
   - What specs can be created under it
   - What code should be built in it
   - The overall project structure

**This is MANDATORY for every workflow:**

- **SPEC workflow:** Research topics → Understand scope → Then create specs
- **BUILD workflow:** Research topics → Understand scope → Then build code
- **Any other workflow:** Topics first!

---

## The 5 Areas (Pipeline Stages)

Work flows through these areas in order:

1. **Staging** - New ideas, initial specs, architectural planning
2. **Working** - Active development and implementation
3. **Testing** - Code verification, running tests
4. **Fixing** - Resolving issues and bugs
5. **Build** - Ready for final build/deployment

---

## Mode: Read-Only vs Write

### Read-Only Mode (Default)
- **DO NOT create, modify, or delete any files**
- Use MCP tools to understand state
- Wait for explicit permission to write files

### Write Mode (When Permitted)
- The system will tell you when you can create files
- Look for messages like: "You are no longer in read-only mode"
- When in write mode, you may write spec.md, task.md, code files, etc.

---

## SPEC Workflow: Plan Mode (Default in SPEC)

### Step 1: List Specs/Requirements
Summarize current understanding of the project:

```
# Get overview
areas_list
topics_list {area: "Staging"}
topics_list {area: "Working"}
queue_list
```

Output a summary of:
- Active topics and their specs
- Current area assignments
- Queue priority
- Overall project state

### Step 2: Clarify via Questions
Ask targeted questions using numbered bullet format:

1. What specific functionality is needed for this feature?
2. What data structures or models should this use?
3. Are there any existing implementations to reference?
4. What are the success criteria for this feature?
5. Are there any constraints or dependencies?

### Step 3: Plan Formulation
When requirements are clear, create a structured plan document with:
- **Project Overview**: Purpose, scope, and goals
- **Technical Architecture**: High-level design and system components
- **Data Model Design**: Core data structures and interfaces
- **Implementation Roadmap**: Ordered steps with dependencies (no timeframes - just list what needs to happen)
- **Risk Assessment**: Potential challenges and mitigations

### Step 4: Ready to Build?
When the plan is complete and you're ready to create the spec:

**Tell the user:**
> "This plan is ready to build! When you're ready, run `unispec spec` and I'll create the topic in Staging, write the spec.md file with the specification, and move it to Working for implementation."

This signals:
- Spec creation is ready
- User should run the spec command
- Next steps are clear

**Do NOT print a roadmap** - just output the spec.

### Step 4: Create Topic (when plan approved)
```
topics_add {topic: "feature-name", area: "Staging"}
```

### Step 5: Create Spec & Tasks
Use `spec_add` to create both spec.md and task.md from templates in one step:
```
spec_add {topic: "feature-name", area: "Staging"}
```

### Step 6: Add to Queue
```
queue_add {topic: "feature-name", position: 0}
# EDIT ONLY IN STAGING - Do NOT push to Working
```

---

## Core Concepts

### Topics
A **topic** is a unit of work - a feature, bug fix, or project. Each topic lives in one area and contains:
- `spec.md` - The specification document (human-authored, authoritative)
- `tasks.toml` - The authoritative task state (machine-owned)
- `task.md` - Derived rendering of `tasks.toml` (regenerated; do not edit directly)
- `notes.md` - Optional human notes

### The Index
The **index** tracks relationships between specs and code files:
- Links files to topics with metadata (tags, annotation, type)
- Enables finding "what spec does this file implement?"
- Enables finding "what files implement this spec?"
- Full graph visualization available

### The Queue
The **queue** is an ordered list of topics to work on, stored in
`.unispec/state.toml::queues.<Area>`:
- Prioritized work list
- Topics can be added, removed, reordered
- The legacy `queue.md` is read once on first write into the area's queue
  for backward compatibility, then ignored. New writes never touch
  `queue.md`.

---

## MCP Tools Reference

**Use these tools for all operations.**

### Area & Topic Management
- `areas_list` - List all areas (Staging, Working, Testing, Fixing, Build)
- `topics_list [area]` - List topics in an area
- `topics_add {topic, area}` - Create new topic (creates folder structure only)
- `topics_delete {topic, area, force}` - Delete topic
- `topics_show {topic, area}` - Show topic files and structure
- `topics_push {topic, area, source_area}` - Move topic to another area
- `topics_pull {topic, source_area}` - Pull topic from another area into Working
- `topics_progress [area]` - Show completion progress across topics

### Spec & Tasks
- `spec_add {topic, area}` - Create canonical `spec.md` + empty `tasks.toml` + derived `task.md`
- `spec_read {topic, area}` - Read spec content
- `spec_write {topic, area, content}` - Write `spec.md`. Rejects runtime-state frontmatter (`status`, `checked_out`, `checked_out_at`).
- `task_read {topic, area}` - Read derived task content (rendered from `tasks.toml`)
- `task_write {topic, area, content}` - One-shot importer for legacy markdown task lists. Refuses (`LegacyFormat:`) once the topic is on the canonical layout — use `task_status` per task instead.
- `task_status {topic, area, task_id, status}` - Mutate one task in `tasks.toml` and regenerate `task.md`. Status: `pending | in_progress | complete | blocked`. Legacy `working` accepted as alias for `in_progress`.
- `tasks_list {topic, area}` - List all tasks with status

### Notes
- `notes_read {topic, area}` - Read this topic's `notes.md`
- `notes_add {topic, note, area}` - Append a note to `notes.md`

### Queue (Ordered Work List)
- `queue_list [area]` - List ordered queue of topics (area: Staging)
- `queue_add {topic, position, area}` - Add to queue (0=first, -1=last)
- `queue_remove {topic, area}` - Remove from queue
- `queue_reorder {topic, new_position, area}` - Reorder queue

### Index (File ↔ Spec Linking)
- `index_add {topic, path, area, link_type, tags, annotation}` - Link file to topic
- `index_find {query, by}` - Find links by topic/path/tag
- `index_lookup {id}` - Find export by full ID (topic:name)
- `index_list [topic, path, tag]` - List all index links with filters
- `index_graph` - Export full relationship graph JSON
- `index_backlinks {topic}` - Generate backlinks for a topic
- `unispec_bind_spec {spec_path, file_path, topic, area}` - Bind code file to spec

---

## Standard Workflow

### 1. Start Work
```
# Check queue for prioritized work
queue_list

# Check what's in Working area
topics_list {area: "Working"}

# Get the spec for the topic you're working on
spec_read {topic: "feature-name", area: "Working"}

# See tasks for this topic
tasks_list {topic: "feature-name", area: "Working"}
```

### 2. Implement
```
# Mark task as working
task_status {topic: "feature-name", area: "Staging", status: "working"}

# Do the work, mark tasks complete when done
task_status {topic: "feature-name", area: "Staging", status: "complete"}

# If you need to modify the spec:
spec_write {topic: "feature-name", area: "Staging", content: "..."}

# Add implementation notes
notes_add {topic: "feature-name", note: "Using JWT tokens for auth"}
```

### 3. Complete
```
# When done, mark as complete
task_status {topic: "feature-name", area: "Staging", status: "complete"}
```

### 4. Verify
```
# See full picture
index_graph

# Find what's linked to a spec
index_find {query: "feature-name", by: "topic"}

# Get backlinks
index_backlinks {topic: "feature-name"}

# Check progress
topics_progress {area: "Working"}
```

---

## Key Principles

1. **Default: Do NOT create files** - Wait for permission to write

2. **Use MCP tools first** - Only use direct file operations when MCP tools cannot do what you need

3. **Plan first** - List, clarify, plan before writing specs

4. **Read the spec before coding** - Always use `unispec_read_spec` to understand requirements

5. **Track progress with tasks** - Mark tasks complete as you finish them

6. **Link files to specs** - Use `index_add` for every file created so relationships are tracked

7. **Use the queue** - Maintain prioritized work list

8. **Move work through areas** - Push to next area when ready

9. **Verify with index** - Use graph and backlinks to understand full picture

---

## File Structure

Canonical Phase 1 layout:

```
project/
├── spec/
│   └── <Area>/
│       └── <topic>/
│           ├── topic.md       # narrative + frontmatter (human-authored)
│           ├── spec.md        # authoritative spec (human-authored)
│           ├── tasks.toml     # authoritative task state (machine-owned)
│           ├── task.md        # DERIVED — generated from tasks.toml
│           └── notes.md       # OPTIONAL — human notes
└── .unispec/
    └── state.toml             # runtime state: queues, ownership, locks, mode
```

Filenames are infrastructure-level constants in Phase 1. `spec.md`,
`tasks.toml`, and `task.md` are the canonical names regardless of which
mode or area the topic lives in. Legacy `<topic>_spec.md`, `<topic>_task.md`,
`specs.md`, and per-area `queue.md` files are still readable for
compatibility but are never written by current handlers.

---

## Notes
- Task IDs (e.g. `1.1`, `2.3`) are stable identifiers in `tasks.toml`.
- Area names are case-insensitive internally.
- The queue lives in `.unispec/state.toml::queues.<Area>`, not in
  `queue.md`. On first queue mutation per area, any existing legacy
  `queue.md` is imported once and then ignored.
- Always check the queue before starting new work.
- **Wait for write mode permission before creating files.**