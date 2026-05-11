# Workflow: /verify

**Goal:** Audit whether the implementation of a topic matches its
specification. This is a **read-only audit workflow** — no code changes,
no task mutations happen here. Findings get recorded as notes for a
human or a follow-up `/fix` pass to act on.

---

## What This Workflow Is NOT

- There is no `unispec auto verify` or `unispec_auto_verify` MCP tool
  that does this automatically.
- This workflow is not a one-shot tool call — it is a structured manual
  audit driven by the AI through MCP reads and shell reads.

---

## Tools Used (read-only)

| Tool | Purpose |
| --- | --- |
| `unispec_read_spec` | Reads `spec.md` + `task.md` |
| `index_list` | Lists files bound to the topic |
| `index_backlinks` | Shows what other topics depend on this one |
| `tasks_list` | Lists task statuses |
| File-read tools (Read, `cat`) | Reads actual code files |
| `notes_add` | Records audit findings on the topic |

---

## Steps

### 1. Load the spec contract

```
unispec_read_spec { topic: "<name>", area: "Working" }
```

Extract the acceptance criteria into a numbered list. Each criterion
becomes one row in your audit.

### 2. Enumerate bound code

```
index_list { topic: "<name>" }
```

Read every file path returned. If a file in the index does not exist on
disk, that's a finding. If a file that obviously implements the spec
isn't in the index, that's also a finding (`index_add` was skipped).

### 3. For each acceptance criterion, find the implementing code

Walk the bound files and locate the function / module / route that
satisfies each criterion. Use file-read tools to inspect actual code.

For each criterion, classify:

- **Implemented** — concrete code exists and visibly addresses the
  criterion. Cite file path + line range.
- **Partially implemented** — some path exists but is incomplete or
  has unreachable branches.
- **Missing** — no code addresses this criterion.
- **Drifted** — code exists but does the opposite of, or differently
  than, the spec.

Do **not** classify based on test results. That is `/test`'s job.
`/verify` checks code-against-spec, not code-against-runtime.

### 4. Check task / spec consistency

```
tasks_list { topic: "<name>" }
```

For each task marked complete, find the corresponding code. A complete
task with no implementing code is a finding (false completion). An
incomplete task with implementing code is a finding (stale task list).

### 5. Check the index for false positives

For each file in `index_list`, confirm it still exists and that its
content still relates to this topic. Stale index entries are a finding.

### 6. Record findings

Build one consolidated note and submit it via `notes_add`. Use this
shape (no placeholders — fill in real values or omit the row):

```
notes_add {
  topic: "<name>",
  note: "## /verify audit\n\nSpec ↔ Code:\n- AC1: Implemented (src/auth/login.rs:42-87)\n- AC2: Missing — no module handles refresh tokens\n- AC3: Drifted — implementation uses HMAC, spec specifies RS256\n\nTask ↔ Code:\n- Task 3 marked complete but no implementing file found\n\nIndex:\n- src/legacy/old_login.rs is in index but file no longer exists"
}
```

### 7. Recommend, do not act

Tell the user what was found and what to do next (push to `Fixing`,
update spec, run `tasks_incomplete` on stale task entries, etc.). Do
not perform fixes from this workflow.

---

## Definition of Done

- Every acceptance criterion in `spec.md` has a classification
  (Implemented / Partial / Missing / Drifted) with a code citation
  where applicable.
- Every complete-marked task has been checked against actual code.
- Every entry returned by `index_list` has been checked for existence
  and relevance.
- A single consolidated audit note is in the topic's notes.

---

## Forbidden in this workflow

- Modifying code under `/src`.
- Modifying any UniSpec artifact (`topic.md`, `spec.md`, `task.md`).
- Calling `tasks_complete` or `tasks_incomplete` (that's `/test`'s job).
- Pushing the topic to another area.
- Calling tools that do not exist on this server (e.g.
  `unispec_query_relations`, `unispec auto verify`).
