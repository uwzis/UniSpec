# Workflow: /test

**Goal:** Run the project's actual build and test commands for a topic in
the `Testing` area, capture results, and update task statuses honestly.

---

## What This Workflow Is NOT

- There is no `unispec auto test` or `unispec_auto_test` MCP tool.
- This workflow does **not** invoke a magic UniSpec runner. It uses the
  project's own build/test commands (e.g. `cargo test`, `pytest`,
  `npm test`) via your shell.

---

## Tools Used

| Tool | Purpose |
| --- | --- |
| `topics_show` | Confirms the topic is in `Testing` |
| `unispec_read_spec` | Reads `spec.md` + `task.md` to understand what should pass |
| `index_list` | Lists files bound to the topic (the code under test) |
| `tasks_list` | Lists tasks for the topic |
| `tasks_complete` / `tasks_incomplete` | Marks individual task indices |
| `notes_add` | Records run output / failure details in the topic's notes section |
| Shell exec (your editor's bash tool) | Runs the actual build/test command |

---

## Steps

### 1. Confirm the topic is in `Testing`

```
topics_show { topic: "<name>", area: "Testing" }
```

If the topic is not in `Testing`, stop and tell the user. Do not test a
topic that hasn't been moved to `Testing` — that bypasses the workflow.

### 2. Load context

```
unispec_read_spec { topic: "<name>", area: "Testing" }
index_list { topic: "<name>" }
tasks_list { topic: "<name>", area: "Testing" }
```

Read the spec's acceptance criteria. Read the file list. Read the task
list. You will reconcile test outcomes against all three.

### 3. Run build

Run the project's build command in the shell (do not invent a UniSpec
build tool). Examples:

- Rust: `cargo build`
- Python: `pip install -e . && python -m compileall src`
- Node: `npm run build` or `pnpm build`

If build fails, **stop**. Do not run tests on a broken build. Add a
note via `notes_add` describing the build failure and report to the
user.

### 4. Run tests

Run the project's test command in the shell.

- Rust: `cargo test`
- Python: `pytest -v`
- Node: `npm test`

Capture stdout + stderr.

### 5. Reconcile against acceptance criteria

For each acceptance criterion in `spec.md`, decide pass / fail / not
exercised based on the test output. **Do not infer success from
"compilation passed."** If a criterion has no corresponding test, that
is a fail, not a pass.

### 6. Update task statuses and notes

For each implementation task confirmed by passing tests, call
`tasks_complete { topic, task_index, note: "verified by <test name>" }`.

For tasks that were marked complete but tests now reveal regressions,
call `tasks_incomplete { topic, task_index, note: "regression: <details>" }`.

Append a run summary via `notes_add { topic, note: "<test report>" }`
including: command run, exit code, count of pass/fail/skipped, and any
new failures.

### 7. Decide next move

- All criteria pass and build is green → tell the user the topic is
  ready to push to `Build`. Do not push it yourself — let the user
  trigger that.
- Any criterion fails → tell the user. Do **not** push the topic to
  `Fixing` yourself unless the user asks.

---

## Definition of Done

- Build command executed; exit code recorded in notes.
- Test command executed; exit code and pass/fail counts recorded in notes.
- Every implementation task has been either confirmed complete via
  `tasks_complete` or returned to incomplete via `tasks_incomplete`,
  each with a one-line justification.
- A run summary exists in the topic's notes via `notes_add`.

---

## Forbidden in this workflow

- Calling tools that don't exist (`unispec auto test`, `unispec_nav`,
  etc.).
- Marking tasks complete without observed test evidence.
- Editing `task.md` directly to flip checkboxes — use `tasks_complete`
  / `tasks_incomplete`.
- Pushing the topic to `Build` or `Fixing` autonomously.
