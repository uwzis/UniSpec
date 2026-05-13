# Workflow: /build (default mode)

Implement a topic — write code into `src/`, link every file, flip task checkboxes, push to Testing.

Mirrors `.agent/workflows/build.md`; this file is the per-mode copy.

---

## Preconditions

- Topic has `<topic>_spec.md` and `<topic>_task.md` in `spec/Staging/<topic>/`.
- Topic appears in `spec/Staging/queue.md` (verify via `queue_check`).
- `src/` exists at the project root.

If any precondition fails, run `/spec` and `queue_add` first.

---

## Tools

| Tool | Purpose |
|------|---------|
| `topics_list`, `queue_list`, `queue_check` | Orient. |
| `queue_add` | Add topic to `Staging/queue.md` if missing. |
| `topics_push` | Move between areas. |
| `unispec_read_spec`, `read_asset` | Load spec / re-read task. |
| `change_list` | **Run this before starting.** Lists pending feature additions for the topic; each non-archived change has its own spec/task pair that needs implementing alongside the topic's main tasks. |
| `tasks_list` | Get task indices. |
| `task_status` | Update `status:` to `working` / `complete`. |
| `tasks_complete`, `tasks_incomplete` | Flip checkbox at 0-based index. |
| `index_add` | Link each new file with `link_type: "implementation"`. |
| `notes_add` | Capture decisions not in the spec. |
| `task_write` | Append test tasks at the end of BUILD. |
| `change_archive` | Archive a change once every box in its `<change>_task.md` is `[x]`. |

---

## Steps

0. **Call `next` first.**
   ```
   next { topic: "<topic>", area: "Working" }
   ```
   Read the full output. Follow `next_action` verbatim. If `blockers` is non-empty, resolve every blocker before proceeding (the blocker text names the tool to call). Treat `rules` as binding for this build pass. Re-call `next` after every meaningful state change (task completed, change archived, push) to get the updated next action.

   **Then** read the constitution and verify your planned actions do not violate any principle:
   ```
   constitution_read {}
   ```
   If any planned action conflicts with a principle, stop and revise the plan before writing code.

1. **Orient.**
   ```
   topics_list  { area: "Staging" }
   queue_check  { topic: "<topic>", area: "Staging" }
   ```
   If `ready: false`, `queue_add { topic: "<topic>", area: "Staging" }`, then re-check until `ready: true`.

   CLI equivalent: `unispec queue add <topic>` (defaults area from `.agent/config.toml`).

2. **Push to Working.**
   ```
   topics_push { topic: "<topic>", area: "Working", source_area: "Staging" }
   ```

   CLI equivalent: `unispec topic push <topic> --area Working --from Staging`.

   Push is a real move — the source directory is removed after the copy. If the target area doesn't exist yet, push auto-creates it with a minimal `area.md`.

3. **Load context.**
   ```
   unispec_read_spec { topic: "<topic>", area: "Working" }
   tasks_list        { topic: "<topic>", area: "Working" }
   change_list       { topic: "<topic>", area: "Working" }
   task_status       { topic: "<topic>", area: "Working", status: "working" }
   ```

   **Pending changes.** `change_list` returns every non-archived feature
   addition proposed against this topic. Each one carries its own
   `proposal.md`, optional `design.md`, `<change>_spec.md`, and
   `<change>_task.md` under `spec/<area>/<topic>/changes/<change>/`. Read
   each change's proposal and spec before implementing the topic's main
   tasks — the change might add requirements that affect how you build the
   core feature. Treat each change's task list the same as the topic's
   own: flip boxes as you go, and archive the change (`change_archive`)
   once every box is `[x]`.

4. **For each open task at index `N`**, in order:
   1. Write code into `src/<file>`.
   2. `index_add { topic, path: "src/<file>", link_type: "implementation", tags: "<…>", annotation: "<…>" }`.
   3. `tasks_complete { topic, task_index: N }`.
   4. If a decision wasn't in the spec, `notes_add { topic, note: "<decision + reason>" }`.

5. **Append test tasks.**
   ```
   read_asset { topic: "<topic>", asset_type: "task", area: "Working" }
   task_write {
     topic: "<topic>",
     area: "Working",
     content: "<existing body + ### Phase 5: Testing block of `- [ ]` items>"
   }
   ```

6. **Promote.**
   ```
   task_status { topic: "<topic>", area: "Working", status: "complete" }
   topics_push { topic: "<topic>", area: "Testing", source_area: "Working" }
   ```

   CLI equivalent: `unispec topic push <topic> --area Testing --from Working`.

   Working → Testing is **not** queue-gated by the default mode (only Staging and Fixing are), so no `queue_add` is required for this transition.

---

## File layout

```
<project-root>/
├── src/                     # all code
└── spec/
    └── <Area>/<topic>/{topic.md, <topic>_spec.md, <topic>_task.md}
```

Nested topic `auth/login` → `spec/<Area>/auth/login/auth-login_spec.md` and `auth-login_task.md`.

---

## Definition of done

- Every implementation task is `- [x]`.
- `task_status` is `complete`.
- Every file you wrote/changed in `src/` has an `index_add` entry.
- Decisions outside the spec are recorded via `notes_add`.
- A test-tasks section has been appended via `task_write`.
- Every pending change reported by `change_list` is either fully implemented (every box in its `<change>_task.md` is `[x]`) and archived via `change_archive`, or — if intentionally deferred — explicitly noted via `notes_add` so it doesn't get lost downstream.
- `topics_push { area: "Testing" }` succeeded.

---

## Failure modes

- **`topics_push` rejected** — topic not in `Staging/queue.md`. Run `queue_add`.
- **`task_write` rejected** — no spec exists. Run `/spec` first.
- **`tasks_complete` says index missing** — re-run `tasks_list`; indices recompute after any edit.
- **Wrote code outside `src/`** — move it, or document via `notes_add` why it lives elsewhere.
