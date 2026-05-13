# `unispec analyze` — Cross-Artifact Consistency Checker

Static analysis for a topic. Reads the spec, task, and every pending change; reports findings with severity. Never writes.

## Usage

```bash
unispec analyze --topic <name> [--area <area>] [--json]
```

| Flag | Description |
|---|---|
| `-t, --topic <name>` | Required. Topic to analyse. |
| `-a, --area <area>` | Area name. Defaults to config, then `Staging`. |
| `--json` | Emit findings as JSON. |

MCP equivalent: `analyze { topic, area? }` returns the same payload.

## What it checks

### 1. Duplication (WARNING)

A `### Requirement: X` row appears in BOTH the canonical `<topic>_spec.md` AND inside a pending change's `<change>_spec.md`, where the change row is NOT under `## MODIFIED Requirements`. Likely means the change author meant to modify the existing requirement but didn't use the delta header.

**Fix:** move the row under `## MODIFIED Requirements` in the change spec (so `change_archive` will merge it correctly), or remove the duplicate.

### 2. Missing task coverage (ERROR)

A `### Requirement:` row in the spec has no task line that references it. Matching is permissive: the task text needs to contain every non-trivial word (length > 2) from the requirement name, case-insensitive — so "Refresh Token" matches "Implement refresh token endpoint" but not "Add logout button".

**Fix:** add a `- [ ]` line to `<topic>_task.md` (or to a relevant change's task file) that names the requirement.

### 3. Ambiguous language (WARNING)

A requirement's body contains one of these words without a numeric metric or unit token:

```
fast, secure, scalable, easy, simple, good, better, best, quick,
securely, quickly, scalably, easily
```

A "measurable metric" means either:

- a digit anywhere in the body, OR
- one of the unit tokens `ms / second / minute / hour / request / user / req/s / rps / % / p50 / p95 / p99 / qps / kb / mb / gb / tb` matched as a whole word.

**Fix:** replace the vague word with a concrete threshold (`"login must complete in < 200ms p95"`, `"locks the account after 5 failures in 5 minutes"`).

### 4. Empty sections (WARNING)

A `## <heading>` section header with no content (just blank lines) before the next `##` or EOF.

**Fix:** fill the section or delete the header.

### 5. Constitution alignment (INFO)

If `.agent/constitution.md` exists, surfaces its `Version:` as an INFO finding so the agent re-evaluates manually. UniSpec does not try to do automated semantic checking here — the constitution is read by the model, not a regex.

**Action:** the agent reads both the constitution and the spec, and decides whether any principle is violated. If yes, flag manually.

### 6. Task completion (INFO)

Computes the `[x] / [ ]` ratio across the main `<topic>_task.md` AND every pending change's `<change>_task.md`. Reported as:

```
INFO: Task completion
  3 of 5 tasks complete (60%).
```

## Example

Given:

```markdown
# spec/Staging/auth/auth_spec.md

### Requirement: Login
Users must be able to login securely with email and password.

### Requirement: Logout
Users must be able to logout.

### Requirement: Refresh Token
Users must be able to refresh their JWT token.
```

```markdown
# spec/Staging/auth/auth_task.md

- [ ] Implement login
- [ ] Implement logout
```

Run:

```bash
$ unispec analyze --topic auth

Analysis for 'auth' in Staging/

ERROR: Missing task coverage
  Requirement 'Refresh Token' has no corresponding task.
  (Add a `- [ ]` task to spec/Staging/auth/auth_task.md that mentions 'Refresh Token'.)

WARNING: Ambiguous language
  Requirement 'Login' contains 'securely' without a measurable metric.
  (Replace with a concrete number or threshold (e.g. '< 200ms p95').)

INFO: Constitution alignment
  Project constitution loaded — verify the spec/tasks do not violate any principle.
  (Constitution version: 1.0.0)

INFO: Task completion
  0 of 2 tasks complete (0%).

Summary: 1 error, 1 warning, 2 info
```

## JSON output

```bash
unispec analyze --topic auth --json
```

```json
{
  "success": true,
  "topic": "auth",
  "area": "Staging",
  "findings": [
    {
      "severity": "ERROR",
      "check": "Missing task coverage",
      "message": "Requirement 'Refresh Token' has no corresponding task.",
      "detail": "Add a `- [ ]` task to spec/Staging/auth/auth_task.md that mentions 'Refresh Token'."
    }
  ],
  "error_count": 1,
  "warning_count": 1,
  "info_count": 2
}
```

## Exit code

Always 0 — even with ERROR findings. `analyze` is reporting, not enforcement. The agent decides whether to push or block based on the findings.

To turn analyze into a hard gate (e.g. in CI), pipe through `jq` and exit non-zero on error count:

```bash
unispec analyze --topic "$T" --json | jq -e '.error_count == 0' > /dev/null
```

## What `analyze` is NOT

- Not a linter for the spec's prose. It doesn't enforce capitalisation, sentence length, etc.
- Not a build / test runner. It doesn't execute code.
- Not a substitute for the constitution. The constitution alignment check is just a reminder; the model still has to read both texts.
- Not retroactive. Only the topic you pass is analysed; siblings and ancestors are not.

## See also

- [cli-reference.md#analyze](cli-reference.md#analyze) — flag reference
- [mcp-tools-reference.md#analyze](mcp-tools-reference.md#analyze) — MCP shape
- [constitution.md](constitution.md) — what Check 5 reports
- [change-management.md](change-management.md) — what Check 1 flags
