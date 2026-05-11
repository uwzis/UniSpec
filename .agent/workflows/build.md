# Workflow: /build

## Purpose
Build topics from Staging. A topic MUST be listed in the target area's
queue (stored in `.unispec/state.toml::queues.<Area>`) to be pushable.

## Key Requirements
1. **CREATE /src FIRST** - Before writing any code, create /src at project root
2. **ALL source files in /src** - At project root, NOT in topic directories
3. **MUST be in queue** - Topic must be listed in the area's queue
   (`.unispec/state.toml`) to be pushable. Use `queue_*` MCP tools.
4. **Check off tasks via `task_status`** - `task.md` is a derived
   rendering of `tasks.toml`; never edit its checkboxes directly.
5. **Add testing tasks last** - Add test tasks AFTER building, before Testing
6. **Working queue entry cleared at Testing** - Normal behavior
7. **Use `read_asset`** to read `topic.md`, `spec.md`, or `task.md`
   during build

## Read Asset Tool

Use `read_asset` to read any file:

```
read_asset {
  topic: "myproject/auth",
  asset_type: "spec",   // "topic", "spec", or "task"
  area: "Working"
}
```

## Queue Storage

The queue lives in `.unispec/state.toml::queues.<Area>`. Treat it as
opaque structured state and only read/mutate it through the `queue_*`
MCP tools — never write `queue.md` directly. Legacy `queue.md` files
are imported once per area on first queue mutation and then ignored.

## Readiness Rule

**Only topics listed in the target area's queue can be pushed.**

Check:
```
queue_check {topic: "<topic-name>", area: "Staging"}
```

## Steps

### 1. Check Readiness - MUST be in queue
- Topic must be listed in the Staging queue (`state.toml::queues.Staging`)
- Add to queue if not: `queue_add {topic, area: "Staging"}`

### 2. Create /src First
- System creates this automatically when pushing to Working
- All code goes in /src at project root

### 3. Push Topic to Working
- Push topic that is listed in queue
- Do this ONE TOPIC AT A TIME

### 4. Build in Working
- Work through queue in ORDER
- Create code in /src
- Link every file to spec via `index_add`
- **CHECK OFF EVERY TASK** via `task_status` (mutates `tasks.toml`,
  regenerates `task.md`). Never hand-edit `task.md`.

### 5. Add Testing Tasks Before Testing
- **ONLY add testing tasks here** - AFTER all implementation is done
- This is the LAST step before Testing
- Testing tasks are created in the BUILD phase only, not during SPEC phase

### 6. Push to Testing
- When done, push to Testing
- The Working queue entry for this topic is cleared automatically

## File Placement

```
PROJECT ROOT/
├── src/                    <-- ALL CODE HERE
└── spec/                   <-- Specs here (NOT code)
```

## Important Notes

- **Check off each task** - Don't skip! Use `task_status` immediately;
  never edit `task.md` checkboxes directly (it's a derived file)
- **Queue lives in `.unispec/state.toml`** - Not in topic folders, not
  in `queue.md`
- **Testing comes last** - Add test tasks after all implementation done
- **NO testing in spec phase** - Only development tasks during SPEC workflow