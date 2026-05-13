# Quickstart

Five minutes from install to a topic in Build, plus the Claude Code MCP wiring. Every command in this file is copy-pasteable.

## 1. Install

```bash
git clone https://github.com/uwzis/UniSpec.git
cd UniSpec
git checkout everything           # the branch with all current fixes
cargo install --path .
```

Verify:

```bash
unispec --help | head
```

## 2. Initialise a project

```bash
mkdir ~/my-app && cd ~/my-app
unispec init
```

What you get:

```
~/my-app/
├── AGENTS.md                                ← universal AI-tool entry point
├── .agent/
│   ├── config.toml
│   ├── constitution.md                      ← project non-negotiables
│   ├── skill.md
│   ├── modes/default/{mode.toml, skill.md, templates/, areas/, workflows/, system_prompts/}
│   ├── modes/README.md
│   └── workflows/{build.md, ingest.md, test.md, unispec:spec.md, verify.md}
└── spec/
    ├── Staging/area.md
    ├── Working/area.md
    ├── Testing/area.md
    ├── Fixing/area.md
    └── Build/area.md
```

All five pipeline areas are present. Default mode is shipped from the binary; no system install required. `AGENTS.md` is the universal fallback any AI agent honours; `.agent/constitution.md` carries the project's non-negotiable principles.

## 3. Create a topic

```bash
unispec topic add user-login \
  --short "Email/password login with JWT" \
  --content "Authentication system for the customer portal. \
Users submit email/password to POST /login and receive a JWT with a 30-minute expiry. \
Refresh tokens live in Redis with a 7-day expiry."
```

Result: `spec/Staging/user-login/topic.md` with frontmatter prepended by the server.

## 4. Write the spec and tasks

```bash
unispec spec add \
  --topic user-login \
  --short "Auth design" \
  --spec-content "POST /login takes {email, password} and returns {jwt} on success. \
Bad credentials return 401 within 200ms. After 5 failed attempts in 5 minutes, lock the account." \
  --task-content "- [ ] Add user table + migration
- [ ] Implement POST /login route
- [ ] Add JWT signing helper
- [ ] Wire rate-limit middleware
- [ ] Write integration tests"
```

Result: `spec/Staging/user-login/user-login_spec.md` and `user-login_task.md`. The leading-`- ` task lines are tolerated thanks to `allow_hyphen_values` on `--task-content`.

## 4a. Sanity-check the spec with `next` and `analyze`

Two read-only commands give the agent a structured view of where the topic stands.

```bash
# What should the agent do next?
unispec next --topic user-login

# Cross-artifact consistency report
unispec analyze --topic user-login
```

`next` returns status, open tasks, pending changes, area-specific rules, and a one-sentence `next_action`. `analyze` reports missing task coverage, ambiguous language, empty sections, and task completion. Use them at every stage transition — and any time the agent isn't sure what to do.

Full guides: [next.md](next.md), [analyze.md](analyze.md).

## 4b. Add a feature to an existing topic (without overwriting the spec)

Once a topic has a spec, **don't rerun `spec add`** to layer on more requirements — it refuses to overwrite, and you'd lose history anyway. Use `change add` instead:

```bash
unispec change add \
  --topic user-login \
  --change add-2fa \
  --proposal "Protect high-value accounts with a second factor." \
  --design "TOTP via authenticator apps; encrypted seed at rest." \
  --spec-content "## 2FA requirements
- TOTP enrolment per user
- 8 recovery codes per user" \
  --task-content "- [ ] Generate TOTP seeds
- [ ] Verify TOTP codes on login
- [ ] Issue and store recovery codes"
```

Result: `spec/Staging/user-login/changes/add-2fa/` containing `proposal.md`, `design.md`, `add-2fa_spec.md`, `add-2fa_task.md`. The original `user-login_spec.md` / `user-login_task.md` are untouched. When the change ships, archive it:

```bash
unispec change list    --topic user-login
unispec change archive --topic user-login --change add-2fa
```

See [change-management.md](change-management.md) for the full guide.

## 5. Walk the topic through the pipeline

```bash
# Queue it (required to push out of Staging or Fixing)
unispec queue add user-login

# Staging → Working
unispec topic push user-login --area Working --from Staging

# Working → Testing (no queue gate)
unispec topic push user-login --area Testing --from Working

# Testing → Fixing (no queue gate)
unispec topic push user-login --area Fixing --from Testing

# Re-queue for the Fixing → Build push (Fixing is also queue-gated)
unispec queue add user-login --area Fixing

# Fixing → Build
unispec topic push user-login --area Build --from Fixing
```

Verify:

```bash
ls spec/Build/user-login/
# topic.md  user-login_spec.md  user-login_task.md
```

Three files, no duplicates.

## 6. Wire Claude Code (or any MCP-aware editor)

Add this to `~/.config/Claude/claude_desktop_config.json` (Linux/macOS path; adjust per-OS):

```json
{
  "mcpServers": {
    "unispec": {
      "command": "unispec",
      "args": ["mcp"],
      "env": {
        "UNISPEC_ROOT": "/home/<you>/my-app"
      }
    }
  }
}
```

Restart Claude Code. The 39 built-in tools become available immediately. From inside Claude Code you can now:

```
> next { "topic": "user-login" }                  ← always start here
> topics_list { "area": "Staging" }
> spec_add { "topic": "...", "area": "Staging", "short": "...", "spec_content": "...", "task_content": "..." }
> analyze { "topic": "user-login" }
> tasks_complete { "topic": "user-login", "task_index": 0 }
```

For per-editor configs (Cursor, Windsurf, Cline, Zed), see [mcp-integration.md](mcp-integration.md). For the full tool list with JSON-RPC examples, see [mcp-tools-reference.md](mcp-tools-reference.md).

## 7. Launch the TUI any time

```bash
unispec
```

In a TopicList view, `q` adds the highlighted topic to that area's queue (so you can satisfy the readiness gate without leaving the TUI). `Enter` opens the highlighted file in `$EDITOR`, then `nano`, then `vi`. Full keybinding reference: [tui.md](tui.md).

## Next steps

- [Workflow](workflow.md) — why the queue gate exists and what each area is for.
- [Areas](areas.md) — per-area conventions.
- [CLI Reference](cli-reference.md) — every flag for every subcommand.
- [Architecture](architecture.md) — how the codebase is laid out.
- [Troubleshooting](troubleshooting.md) — common errors and fixes.
