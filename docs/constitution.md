# Project Constitution

A short file of non-negotiable principles every agent action must respect. Written by `unispec init` to `.agent/constitution.md`; read at the start of every build / verify cycle.

## Why a constitution

Specs describe **what** the project is building. The constitution describes **how** the project is built. Examples of constitutional rules vs. spec rules:

| Constitution | Spec |
|---|---|
| "No code may be written for a topic until it has a spec." | "POST /login returns a JWT." |
| "Errors must be surfaced; no swallowing exceptions." | "Bad credentials return 401 within 200ms." |
| "Breaking API changes require a MODIFIED Requirements delta." | "Add a `refresh_token` field to the login response." |

The constitution survives across topics; specs don't. Both spec-kit (`.specify/memory/constitution.md`) and OpenSpec (`openspec/config.yaml::context` + `rules`) ship the same pattern under different names.

## What ships

`unispec init` writes a starter constitution to `.agent/constitution.md` if the file doesn't exist:

```markdown
# Project Constitution

This file defines the non-negotiable principles for this project. Every
agent action must respect these principles. Violations block progression
through the pipeline.

## Core Principles

### Principle 1: Spec Before Code
No code may be written for a topic until it has a spec file in Working area.

### Principle 2: Tests Required
No topic may be pushed to Build without passing tests documented in the Testing area.

### Principle 3: No Silent Failures
All errors must be surfaced explicitly. No swallowing exceptions without logging.

### Principle 4: Backward Compatibility
No breaking changes to existing APIs without a MODIFIED Requirements delta in a change file.

### Principle 5: One Topic One Concern
Each topic must have a single clear responsibility. If a spec spans more than one concern, split it into multiple topics.

## Governance

Version: 1.0.0
Ratified: <DATE>
Last Amended: <DATE>
```

`<DATE>` is filled with the date `unispec init` ran.

## Editing it

The constitution is meant to be edited per project. Rewrite or replace the five default principles with what your team actually enforces. Bump `Version:` and update `Last Amended:` when you change anything substantive — `analyze` reports the current version to the agent on every run.

There is no `unispec constitution edit` command — it's a plain markdown file, edit with whatever editor you use.

## How agents see it

Two MCP tools:

### `constitution_read`

Returns the file's contents verbatim.

```json
{ "name": "constitution_read", "arguments": {} }
```

Response:

```json
{
  "success": true,
  "path": "/abs/path/.agent/constitution.md",
  "content": "# Project Constitution\n\n..."
}
```

Errors if the file doesn't exist (`unispec init` writes it; if missing, run init).

### `constitution_check`

Pairs the constitution with a proposed action so the agent can self-evaluate. The check is intentionally simple — UniSpec does not try to do semantic enforcement in a regex; the model reads both texts and decides.

```json
{ "name": "constitution_check", "arguments": {
    "action": "Push topic to Build without running tests"
} }
```

Response:

```json
{
  "success": true,
  "action": "Push topic to Build without running tests",
  "constitution": "# Project Constitution\n\n...",
  "note": "Read each principle and confirm the proposed action does not violate any. If a principle is violated, do not proceed."
}
```

## Workflow integration

Two workflow files reference the constitution as a gate:

- **`.agent/modes/default/workflows/build.md`** — Step 0 reads the constitution alongside `next`, and refuses to start coding if a planned action would violate a principle.
- **`.agent/modes/default/workflows/verify.md`** — Step 0 reads the constitution and treats any conflict as an ERROR finding blocking push to Build.

The system prompt `.agent/modes/default/system_prompts/unispec_basics.md` lists both tools in the MCP table.

## What `unispec analyze` does with it

The `analyze` command (and the `analyze` MCP tool) surfaces the constitution version as an INFO finding on every run:

```
INFO: Constitution alignment
  Project constitution loaded — verify the spec/tasks do not violate any principle.
  (Constitution version: 1.0.0)
```

This is a reminder, not an automated check — the agent's job is to read the spec/tasks against the constitution and report findings manually. The reminder exists so agents don't silently skip the constitution.

## Versioning

The Governance section is hand-maintained. Recommended pattern:

| Change | Bump |
|---|---|
| Editorial / typo fix | none |
| Tightened an existing principle | PATCH |
| Added or removed a principle | MINOR |
| Reversed a long-standing rule | MAJOR |

Update `Last Amended:` on every edit; update `Version:` per the table above.

## What it is NOT

- Not a substitute for code review. An agent confirming "this action does not violate Principle 3" is a one-pass sanity check, not a security audit.
- Not enforced by the binary. UniSpec exposes the file; it does not refuse to build or push based on its contents. The enforcement happens via prompts + agent judgement.
- Not a spec. Don't put feature requirements here.

## See also

- [`unispec init`](commands.md#init) — writes `.agent/constitution.md` on first run
- [analyze](cli-reference.md#analyze) — reports constitution version
- [workflow.md](workflow.md) — pipeline rules
