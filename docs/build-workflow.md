# Skill: UniSpec BUILD Workflow

## Purpose
This workflow is for implementing specs into code. Use this when converting specifications into working code.

**IMPORTANT: All work starts in Staging. A topic must be listed in the
area's queue (stored in `.unispec/state.toml::queues.<Area>`) to be
pushable. The legacy `queue.md` is not authoritative — `queue_check` /
`queue_add` / `queue_list` operate against `state.toml`.**

---

## KEY RULE: Topics First, Always!

**Before doing ANYTHING, you MUST research topics first!**

1. **ALWAYS start by listing topics** - Never assume you know what's being built
2. **ALWAYS read the topic first** - Understand the scope before building
3. **Topics define scope** - Each topic bounds what specs are being implemented

---

## READINESS SYSTEM - IMPORTANT!

A topic is ONLY ready to push if it is listed in the target area's queue,
stored in `.unispec/state.toml::queues.<Area>`. Use `queue_check`,
`queue_add`, `queue_list`, `queue_remove` — never write `queue.md`
directly.

**Why?**
- Prevents pushing topics that aren't ready
- Ensures all work is tracked in the central to-do list
- The Working queue entry is cleared when moving from Working → Testing

**How to verify:**
```
# Check if topic is in the area queue
queue_check {topic: "<topic-name>", area: "Staging"}
```

If `ready: true`, you can push. If `ready: false`, add it first:
```
queue_add {topic: "<topic-name>", area: "Staging"}
```

---

## The BUILD Workflow

### Step 0: Research Topics FIRST
1. **List all topics in Staging:**
   ```
   topics_list {area: "Staging"}
   ```

2. **Read the topic:**
   ```
   topic_read {topic: "<topic-name>", area: "Staging"}
   ```

### Step 1: Check Readiness FIRST
**Before pushing anything, check if the topic is in the area queue:**

```
queue_check {topic: "<topic-name>", area: "Staging"}
```

If not ready, add it:
```
queue_add {topic: "<topic-name>", area: "Staging"}
```

**This is REQUIRED - you cannot push a topic that's not in the queue!**

### Step 2: Inspect the Staging queue
**List the area queue to know what to build:**
```
queue_list {area: "Staging"}
```

The queue is the CENTRAL TO-DO LIST for the area, stored in
`.unispec/state.toml::queues.Staging`. It contains ALL topics that need
to be built. Use the `queue_*` MCP tools — never read or edit
`queue.md` directly.

### Step 3: Create /src FIRST
**Before writing any code, create /src at project root:**
```
# At project root (same level as spec/)
# Create src/ directory - ALL code goes here
```

### Step 4: Push the Topic to Working
**Push a topic that is listed in the queue:**
```
topics_push {topic: "<topic-name>", area: "Working"}
```

This pushes the topic. The Staging queue entry remains in
`.unispec/state.toml`.

### Step 5: List the Working queue
```
queue_list {area: "Working"}
```

### Step 6: Build Each Topic in Order
For each topic in the queue:

1. **Read spec and task files:**
   ```
   spec_read {topic: "<topic-name>", area: "Working"}
   task_read {topic: "<topic-name>", area: "Working"}
   ```

2. **Create code in /src** (NOT in topic directories)

3. **Link every file:**
   ```
   index_add {topic: "<topic-name>", path: "src/filename.rs", link_type: "implementation", tags: "..."}
   ```

4. **CHECK OFF TASKS IMMEDIATELY - THIS IS REQUIRED!**
   After completing each task, mark it complete via `task_status`:
   ```
   task_status {topic: "<topic-name>", area: "Working", task_id: "1.2", status: "complete"}
   ```
   This mutates the authoritative `tasks.toml` and regenerates the derived
   `task.md`. **Do NOT edit `task.md` checkboxes directly** — that file is
   regenerated and your edits will be lost. **Do NOT use `task_write` to
   flip a single checkbox** — it is an importer, not a per-task mutator,
   and it will refuse with `LegacyFormat:` once the topic is on v1.

   **NEVER skip this step! Always check off completed tasks before moving to the next one.**

### Step 7: Add Testing Tasks Before Pushing to Testing
**Testing tasks are ONLY created here - AFTER all implementation is done.**

Inspect current tasks:
```
task_read {topic: "<topic-name>", area: "Working"}
```

Then append testing tasks. Use the structured task tools to add new tasks
to a Testing phase in `tasks.toml` (the renderer will produce the
corresponding `- [ ]` checkboxes in the derived `task.md`). For example:

```
## Phase 5: Testing
- [ ] **T1** Test the implementation
- [ ] **T2** Verify it works as expected
```

**This is the ONLY place for testing tasks - they are added AFTER
development is complete, right before moving to Testing.**

### Step 8: Push to Testing
When testing tasks are added, push to Testing — the Working queue entry
for this topic is cleared automatically:
```
topics_push {topic: "<topic-name>", area: "Testing"}
```

---

## Key Rules

0. **CREATE /src FIRST** - Before any code

1. **MUST be in queue** - Topic must be listed in the area's queue
   (`.unispec/state.toml::queues.<Area>`) to be pushable

2. **ALL code in /src** - At project root, never in topic directories

3. **Link every file** - Use `index_add`

4. **CHECK OFF TASKS RELIGIOUSLY** - After completing each task, call
   `task_status` to mark it `complete`. NEVER edit `task.md` checkboxes
   directly — `task.md` is derived and will be overwritten.

5. **Add testing tasks last** - Before pushing to Testing, add test tasks

6. **Working queue entry cleared at Testing** - Normal behavior

---

## Queue Storage

The queue lives in `.unispec/state.toml::queues.<Area>`. Treat it as
opaque structured state and only read/mutate it through the `queue_*`
MCP tools:

```
queue_list   {area: "Staging"}
queue_check  {topic, area: "Staging"}
queue_add    {topic, area: "Staging"}
queue_remove {topic, area: "Staging"}
```

---

## Example

```
# Check if topic is in queue
queue_check {topic: "auth", area: "Staging"}
# If not ready, add it:
queue_add {topic: "auth", area: "Staging"}

# List what's in the queue
queue_list {area: "Staging"}

# Push to Working
topics_push {topic: "auth", area: "Working"}

# Build in Working
# ... create files in /src ...
# ... check off tasks ...

# Add testing tasks, then push to Testing
topics_push {topic: "auth", area: "Testing"}
```