# Workflow: /spec

**Goal:** Translate a discussed idea into a concrete topic + spec + task list
inside UniSpec. Do not write source code in this workflow.

---

## Hard Rule

Use MCP tools for every artifact under `spec/**`. Do not use `Write`/`Edit`
on `topic.md`, `spec.md`, or `task.md` — MCP tools own those files and
enforce frontmatter, validation, and queue integrity.

---

## Tools Used

| Tool | Purpose |
| --- | --- |
| `topics_add` | Creates a topic directory with `topic.md` |
| `spec_add` | Creates `spec.md` + `task.md` for a topic |
| `read_asset` | Reads `topic.md` / `spec.md` / `task.md` for verification |
| `queue_add` | Adds the topic to an area's readiness queue (required before `topics_push`) |

---

## Steps

### 1. Confirm the active area

Default for new specs is `Staging`. If the user wants a different area,
they must say so explicitly.

### 2. Gather requirements before any tool call

You MUST have the following before calling `topics_add` or `spec_add`. If
any is missing, ask one targeted question per missing item:

- **Functional Goal** — what does this do?
- **Acceptance Criteria** — how do we know it works?
- **Scope Boundaries** — what is explicitly out of scope?
- **One-line Description** — fits in the `short` parameter.

### 3. Create the topic with `topics_add`

`topics_add` requires `topic`, `area`, `short`, and `content`. Pass the
actual gathered information — never literal template placeholders.

Concrete example (illustrative — substitute the user's real values):

```
topics_add {
  topic: "auth-login",
  area: "Staging",
  short: "JWT-based user login with refresh tokens",
  content: "# auth-login\n\nLogin flow that issues short-lived access tokens and longer-lived refresh tokens. Handles password and OAuth providers."
}
```

The MCP server adds `title`, `created`, and `author` frontmatter
automatically. Do not include those fields in `content`.

### 4. Create the spec + initial task list with `spec_add`

`spec_add` requires `topic`, `area`, `short`, `spec_content`, and
`task_content`. Both content fields must contain real, detailed text —
not placeholders.

`spec_content` covers:

- Problem statement
- Functional requirements
- Acceptance criteria
- Out of scope

`task_content` is a checklist of implementation tasks. **No testing
tasks here** — testing tasks are added in the `/build` workflow after
implementation lands.

### 5. Verify

Call `read_asset` for each of `topic`, `spec`, and `task` and confirm
the files contain what you sent. Report any divergence to the user
rather than silently retrying.

### 6. (Optional) Queue the topic

If the user wants the topic ready to push to `Working`, call:

```
queue_add { topic: "auth-login", area: "Staging" }
```

`topics_push` is gated by the area's queue; without `queue_add`, the
push will be rejected.

---

## Definition of Done

- `topics_show { topic: "<name>", area: "Staging" }` returns the topic.
- `read_asset { topic: "<name>", asset_type: "spec" }` returns the spec
  body you sent.
- `read_asset { topic: "<name>", asset_type: "task" }` returns the task
  list you sent.
- No literal `[Fill this in]` / `[Use the template structure]` strings
  exist anywhere in the returned files.

---

## Forbidden in this workflow

- Writing code or modifying anything under `/src`.
- Using `Write` / `Edit` directly on `spec/**`.
- Submitting placeholder strings (`[…]`, `TBD`, `xxx`) as actual
  `content` / `spec_content` / `task_content` parameters.
- Adding testing tasks to `task_content` — those belong to `/build`.
