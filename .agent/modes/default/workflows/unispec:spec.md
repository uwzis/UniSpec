# Workflow: unispec:spec (default mode)

Create a topic, spec, and task list in `Staging` using the MCP tools. This is the spec-authoring command for the default mode.

---

## Preconditions

- You have a one-line description of the feature (`short`).
- You have enough information to write real content for the topic, spec, and task list — no `[placeholder]` strings.

If either is missing, ask the user before doing anything.

---

## Tools

| Tool | Required args | Notes |
|------|---------------|-------|
| `read_asset` | `topic, asset_type` | Read templates with `topic: "templates"`, `asset_type ∈ {"topic","spec","task"}`. |
| `topics_add` | `topic, area, short, content` | `content` ≥ 10 chars. Server prepends frontmatter — don't include `---`. |
| `spec_add` | `topic, area, short, spec_content, task_content` | **First spec for a topic only (by convention).** Both content fields ≥ 10 chars. Creates `<topic>_spec.md` and `<topic>_task.md`. Will silently overwrite if rerun — never use it to "add a feature" to an existing topic. |
| `change_add` | `topic, change, proposal, spec_content, task_content` | **Adding features to an existing topic.** Use this whenever `<topic>_spec.md` already exists. Writes under `spec/<area>/<topic>/changes/<change>/` and refuses to overwrite an existing change folder. |
| `queue_add` | `topic, area` | Add to `spec/Staging/queue.md`. |
| `topics_show` | `topic, area` | Verify the files exist. |

---

## Steps

### 1. Read the templates
```
read_asset { topic: "templates", asset_type: "topic" }
read_asset { topic: "templates", asset_type: "spec" }
read_asset { topic: "templates", asset_type: "task" }
```

### 2. Create the topic
```
topics_add {
  topic: "<topic-name>",
  area: "Staging",
  short: "<one-line description>",
  content: "<body mirroring templates/topic.md, with real content>"
}
```

### 3. Create the spec and tasks
```
spec_add {
  topic: "<topic-name>",
  area: "Staging",
  short: "<one-line description>",
  spec_content: "<body mirroring templates/spec.md, every section filled>",
  task_content: "<body mirroring templates/task.md, implementation tasks only>"
}
```

Filenames produced: `<topic>_spec.md`, `<topic>_task.md` (slashes/spaces in `topic` become `-`).

### 4. Add to the readiness queue
```
queue_add { topic: "<topic-name>", area: "Staging" }
```

CLI equivalent:

```bash
unispec queue add <topic-name>
```

Staging is one of two queue-gated areas in the default mode (the other is Fixing). The `topics_push` from Staging will be rejected with `❌ Topic '<name>' is not ready to push. It must be listed in spec/Staging/queue.md.` if this step is skipped.

### 5. Verify
```
topics_show { topic: "<topic-name>", area: "Staging" }
queue_check { topic: "<topic-name>", area: "Staging" }
```
`topics_show` should list `topic.md`, `<topic-safe>_spec.md`, `<topic-safe>_task.md`. `queue_check` should return `{ "ready": true }`.

---

## Content rules

- **Spec = WHAT, not HOW.** Use SHALL/SHOULD; acceptance criteria are checkable.
- **Tasks = implementation steps only.** No test tasks here.
- **No placeholder text in the body.** `[Requirement statement]`, `[Foundation task]`, etc. exist in the template to be replaced.
- **Nested topics use `/`** (e.g., `auth/login`). The parent topic must already exist.

---

## Definition of done

- `spec/Staging/<topic>/topic.md` exists with real content.
- `spec/Staging/<topic>/<topic>_spec.md` exists with every template section filled.
- `spec/Staging/<topic>/<topic>_task.md` exists with concrete implementation tasks and zero test tasks.
- `queue_check { topic, area: "Staging" }` returns `ready: true`.

---

## Adding a feature to an existing topic

`spec_add` refuses to overwrite an existing spec — by design, the topic's foundational requirements are evidence of what shipped. Layer new features on top with `change_add`:

```
change_add {
  topic: "<existing-topic>",
  area: "<area>",
  change: "<change-id>",            // kebab-case, e.g. "add-2fa"
  proposal: "<why this change exists, ≥ 11 chars>",
  design: "<optional technical approach>",
  spec_content: "<new requirements only, ≥ 11 chars>",
  task_content: "<new tasks only, ≥ 11 chars>"
}
```

This writes:

```
spec/<Area>/<topic>/
├── topic.md
├── <topic>_spec.md            ← untouched
├── <topic>_task.md            ← untouched
└── changes/
    └── <change>/
        ├── proposal.md
        ├── design.md          (only if `design` supplied)
        ├── <change>_spec.md
        └── <change>_task.md
```

After implementing the change, archive it:

```
change_list    { topic, area }                     // verify it's complete
change_archive { topic, area, change }             // moves to changes/archive/<change>/
```

See [docs/change-management.md](../../../docs/change-management.md) for the full guide.

## Failure modes

- **`topics_add` errors with "content required"** — `content` was empty or under 10 chars. Write a real body.
- **`spec_add` errors with "spec_content required"** — you passed `content` instead. Use `spec_content`.
- **`spec_add` errors with "parent topic does not exist"** — for a nested topic like `auth/login`, run `topics_add { topic: "auth", ... }` first. The parent must be a real topic directory before the child can be created inside it.
- **Conflict on existing topic** — `topics_add` refuses to overwrite. If you genuinely want to redo it, `topics_delete { topic, force: true }` first.
- **`spec add` CLI errors with `unexpected argument '- ' found`** — you ran an older binary (pre-`everything`) without `allow_hyphen_values = true` on `--task-content`. Rebuild from the `everything` branch.

## CLI form (full workflow in one shell session)

```bash
unispec topic add <topic-name> \
  --short "<one-line>" \
  --content "<body matching templates/topic.md>"

unispec spec add \
  --topic <topic-name> \
  --short "<one-line>" \
  --spec-content "<body matching templates/spec.md>" \
  --task-content "- [ ] <task 1>
- [ ] <task 2>"

unispec queue add <topic-name>
```

`--area` defaults to the `area` field in `.agent/config.toml` (then `Staging` if no config exists), so the explicit `--area Staging` is omitted on a default project.
