# Workflow: /ingest

**Goal:** Take an existing codebase and reverse-engineer a UniSpec
spec tree so the codebase becomes navigable through Topics → Specs →
Tasks → Bound Files. Useful when adopting UniSpec on a project that
already has code.

---

## What This Workflow Is NOT

- It does not rewrite or refactor your existing code.
- It does not have a single "ingest everything" MCP tool. The CLI
  command `unispec ingest run` does index-level ingestion, but
  generating topics/specs is driven by this workflow through MCP.

---

## Tools Used

| Tool | Purpose |
| --- | --- |
| Shell exec | Runs `unispec ingest run <path>` to populate the code index |
| `topics_add` | Creates a topic per logical module |
| `spec_add` | Creates a spec describing existing functionality + a task list reflecting what is already done |
| `tasks_complete` | Marks ingest-derived tasks complete (the code already exists) |
| `index_add` | Binds source files to their topics |
| `read_asset` | Verifies what was written |
| File-read tools (Read, `cat`) | Inspects existing source to derive the spec body |

---

## Steps

### 1. Run the index ingest pass

In a shell:

```
unispec ingest run <project-root-or-subdir> --area Ingested
```

This populates `spec/code_analysis.toml` with parsed functions /
structs / enums but does **not** create topics or specs yet.

### 2. Decide the topic decomposition

A topic should be a coherent module or feature, not "one topic per
file." Read the directory layout and propose a topic tree to the user
before creating anything. Get explicit confirmation on names before
proceeding.

Typical good shapes:

- One topic per top-level domain module (e.g. `auth`, `billing`,
  `index`).
- Sub-topics for sub-modules (e.g. `auth/login`, `auth/refresh`).

### 3. For each topic, derive content from the actual code

For each topic the user confirmed:

a. Read the relevant source files. Identify the public surface
   (exported functions, public APIs, CLI entry points).

b. Call `topics_add` with real content — a one-paragraph description
   of the module's purpose derived from the code, not placeholder text:

   ```
   topics_add {
     topic: "auth-login",
     area: "Ingested",
     short: "JWT login + session issuance",
     content: "Module that authenticates users via password or OAuth, issues short-lived access tokens, and stores session metadata in Redis."
   }
   ```

c. Call `spec_add` with `spec_content` describing **what the code
   already does** (this is a retrospective spec) and `task_content`
   describing the implementation steps that were obviously taken.

### 4. Mark all ingest-derived tasks complete

The implementation already exists. For each task in the generated
task list:

```
tasks_complete {
  topic: "auth-login",
  task_index: <i>,
  note: "Existing implementation, ingest-derived"
}
```

### 5. Bind source files to the topic

For every source file relevant to the topic:

```
index_add {
  topic: "auth-login",
  path: "src/auth/login.rs",
  link_type: "implementation",
  annotation: "Token issuance and session handling"
}
```

Use `link_type` = `implementation` for source, `test` for tests,
`config` for config files, `docs` for documentation.

### 6. Verify

For each new topic call `read_asset` for `topic`, `spec`, and `task` to
confirm the content landed correctly.

---

## Definition of Done

- `unispec ingest run` completed and `spec/code_analysis.toml` exists.
- Every topic the user agreed to has a `topic.md`, `spec.md`, and
  `task.md` with real (not placeholder) content.
- Every source file in scope is bound to its topic via `index_add`.
- All ingest-derived tasks are marked complete with an explanatory
  note.
- No new code was written under `/src`.

---

## Forbidden in this workflow

- Modifying existing source code.
- Using `Write` / `Edit` to create `topic.md`, `spec.md`, or `task.md`
  directly. Always go through `topics_add` / `spec_add`.
- Creating one topic per file (over-decomposition). Group by module.
- Calling commands or tools that don't exist (e.g.
  `unispec index callers`). Use `index_backlinks` if you need
  relationship info.
- Submitting placeholder strings (`[Describe this module]`) as
  `content` or `spec_content`.
