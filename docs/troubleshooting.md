# Troubleshooting

Common errors you'll hit when using `unispec`, with the fix for each. Symptoms come from real test runs.

## Pipeline & CLI

### `Error: ❌ Topic '<name>' is not ready to push. It must be listed in spec/Staging/queue.md`

**Cause.** The source area (`Staging` or `Fixing` by default) is **queue-gated** by `mode.toml`'s `[readiness].areas`. Push won't proceed until the topic is in `spec/<source>/queue.md`.

**Fix.**
```bash
unispec queue add <topic>                       # current area (Staging)
unispec queue add <topic> --area Fixing         # explicit area
```

Then retry the push. The TUI shortcut is `q` while highlighting the topic.

### `Error: ❌ Source area '<name>' not found.` or `❌ Target area '<name>' not found.`

**Cause for source.** The named area directory under `spec/` is missing entirely. Likely an old project initialised before the 5-area default mode landed.

**Cause for target.** As of `everything`, push **auto-creates** missing target area directories. If you still see this error, the binary on `PATH` is older than the auto-create fix — rebuild.

**Fix.**
```bash
cargo build --release        # or `cargo install --path . --force`
which unispec                # confirm PATH points at the new binary
unispec --version            # 0.0.8 on this branch
```

If you want to materialise the missing area manually:
```bash
mkdir -p spec/Testing
cat > spec/Testing/area.md <<'EOF'
---
area: Testing
short: Testing area
---

# Testing
EOF
```

### `Error: ❌ Source and target areas are the same: <area>`

**Cause.** Both `--area` and `--from` resolved to the same area. With the new defaults that resolve via config, this happens when neither flag is passed and the config's `area` field matches the target.

**Fix.** Pass `--from <source>` explicitly.

### `Error: Spec file '<topic>_spec.md' not found for topic '<name>' in area '<area>'`

**Cause.** `unispec_read_spec` / `read_asset` was called for a topic whose spec hasn't been written yet, OR the spec file is on disk under the old `spec.md` name (legacy migration not run).

**Fix.**
```bash
# Confirm what's on disk
ls spec/<area>/<topic>/

# If empty/missing — write the spec
unispec spec add --topic <name> --short "..." --spec-content "..." --task-content "..."

# If you see `spec.md` instead of `<topic>_spec.md`, this is a project from
# before the filename fix. Rename it:
mv spec/<area>/<topic>/spec.md spec/<area>/<topic>/<topic>_spec.md
mv spec/<area>/<topic>/task.md spec/<area>/<topic>/<topic>_task.md
```

### `error: unexpected argument '- ' found` (when running `spec add`)

**Cause.** Clap is parsing the value of `--task-content` (or `--spec-content`) as a sequence of flags because each line starts with `-`. The fix is `allow_hyphen_values = true`, which is set on `everything`.

**Symptom you'll see if running an older binary.** The error fires on the first bullet line and the spec is never written.

**Fix.** Rebuild from the `everything` branch (`cargo install --path . --force`). If you can't rebuild, the workaround is the `=` form which clap handles differently:

```bash
unispec spec add --topic=x --short=x \
  --spec-content="..." \
  --task-content=$'- [ ] one\n- [ ] two'
```

### `Spec and task created` but the bottom row says nothing

The success banner is the ASCII platypus; the `✅ Spec and task created for '<name>' in <area>/` line is printed by main.rs immediately before it. If your terminal is short, scroll up by one line.

## Change management

### `Error: Change '<name>' already exists for topic '<topic>' in area '<area>'`

**Cause.** `change_add` (CLI `unispec change add`) refuses to overwrite an existing change folder under `spec/<area>/<topic>/changes/<change>/`. The folder might be there from a previous successful run, or from a half-completed prior call.

**Fix.** Check what's on disk:

```bash
ls spec/<area>/<topic>/changes/
```

Then either:

```bash
# the existing folder is what you wanted — you're done
unispec change list --topic <topic>

# you want to start over: archive the old one, then add the new one
unispec change archive --topic <topic> --change <name>
unispec change add     --topic <topic> --change <name> ...

# the existing folder is garbage; delete it by hand
rm -rf spec/<area>/<topic>/changes/<name>
unispec change add --topic <topic> --change <name> ...
```

### `Error: Topic '<name>' does not exist in area '<area>'` (from change_add)

**Cause.** `change_add` requires the topic directory to exist first; it doesn't create the topic for you.

**Fix.**

```bash
# Confirm what areas have the topic
unispec topic show <name> --all

# If no area has it: create it
unispec topic add <name> --short "..." --content "..."

# If it's in a different area: pass --area explicitly
unispec change add --topic <name> --area <correct-area> --change ... --proposal ...
```

The resolver tries the area name as given and then lowercased, so `--area staging` and `--area Staging` both work. But it does not search across areas — pass the right one.

### `Error: Change '<name>' does not exist for topic '<topic>' in area '<area>'` (from change_archive)

**Cause.** `change_archive` couldn't find a directory at `spec/<area>/<topic>/changes/<name>/`. Either it was typo'd, already archived, or you're looking in the wrong area.

**Fix.**

```bash
unispec change list --topic <topic> --archived
```

If it's already in `[archived]`, you're done — it was archived earlier. If it's nowhere, the name is wrong: `ls spec/<area>/<topic>/changes/` to see the actual directory names.

### `Error: Archived change '<name>' already exists; refusing to overwrite` (from change_archive)

**Cause.** There's already a `changes/archive/<name>/` from a previous archive. Refusing to overwrite is intentional — the previous archive could be the real history.

**Fix.** Rename one of them before archiving:

```bash
mv spec/<area>/<topic>/changes/<name> spec/<area>/<topic>/changes/<name>-v2
unispec change archive --topic <topic> --change <name>-v2
```

Or, if the old archive really is garbage, delete it manually and retry the archive.

### `Error: 'proposal' must be at least 11 characters of actual text` (or `'spec_content'` / `'task_content'`)

**Cause.** All three content fields are trimmed and checked for length > 10. An empty or near-empty value falls foul of this.

**Fix.** Supply real content. The validation is intentional — a single-character proposal is never what the caller actually meant.

### "I called `spec_add` on a topic that already had a spec — what happened?"

**Reality.** `spec_add` does **not** error on overwrite. It silently writes new `<topic>_spec.md` and `<topic>_task.md` files, replacing whatever was there.

**That's almost never what you wanted.** If the topic already has a spec and you want to add a feature, use `change_add` (or CLI `unispec change add`) instead. It writes under `spec/<area>/<topic>/changes/<change>/` and leaves the original spec intact.

**To recover** if you've already overwritten:

```bash
git diff -- spec/<area>/<topic>/   # see what was lost
git restore -- spec/<area>/<topic>/<topic>_spec.md spec/<area>/<topic>/<topic>_task.md
```

Then redo as a change:

```bash
unispec change add --topic <name> --change <feature-id> \
  --proposal "..." --spec-content "..." --task-content "..."
```

See [change-management.md](change-management.md) for the full guide.

## TUI

### TUI shows old content after `Enter` returns from `nano`

**Cause.** Ratatui's frame diffing didn't realise nano clobbered the alt screen.

**Fix.** This is fixed on `everything` — the suspend/restore path issues `Clear(All) + MoveTo(0,0)` and flags the next frame for a full repaint. If you still see stale content, you're on an older binary. Rebuild.

### TUI on Enter: `Failed to open: No such file or directory`

**Cause.** Old binary using `open::that()` which delegates to `xdg-open` (Linux), which isn't installed on WSL/headless setups.

**Fix.** Rebuild from `everything`. The Enter handler now resolves `$EDITOR` → `nano` → `vi` → print-and-wait fallback.

To use a specific editor:
```bash
EDITOR=vim unispec
EDITOR="code --wait" unispec        # VS Code in another window, blocking
```

### `q` quits when I expected it to queue a topic

`q` is context-sensitive:
- In a TopicList view (you've drilled into an area), `q` queues the highlighted topic.
- Anywhere else (area selection, find results, etc.), `q` quits.

The help row at the bottom reflects which meaning is active.

## MCP

### Editor reports "MCP server failed to start" or shows zero tools

**Diagnostic.** Run the smoke test by hand:

```bash
printf '%s\n' \
  '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' \
  '{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{}}' \
  | unispec mcp 2>/dev/null
```

Expected: two JSON lines, the second containing 31 tool descriptors.

If the command fails with `command not found` — the binary isn't on `PATH`. Use an absolute path in the editor MCP config.

If it runs but `tools/list` returns < 31 tools (or is missing `tasks_list`, `tasks_complete`, `notes_add`, `queue_reorder`, etc.) — the binary is older than `everything`. Rebuild.

### MCP `unispec_read_spec` returns `{"spec":"","tasks":""}`

**Cause.** Older binary that still looks for the legacy `spec.md` / `task.md` filenames while the project was created by a newer `spec_add` that writes `<topic>_spec.md` / `<topic>_task.md`.

**Fix.** Rebuild from `everything`. Both the read and write paths use the same `<topic-safe>_spec.md` convention.

### MCP tool returns `Error: Unknown tool: tasks_list` (or `tasks_complete`, `notes_add`, …)

**Cause.** Older binary. Those six handlers were added on the `everything` branch.

**Fix.** Rebuild.

## Build / install

### `cargo install unispec` succeeds but the binary is missing features

**Cause.** crates.io is pinned at an older version. The fixes on `everything` haven't been published to crates.io yet.

**Fix.** Install from the local repo:
```bash
git clone https://github.com/uwzis/UniSpec.git
cd UniSpec
git checkout everything
cargo install --path . --force
```

`--force` overwrites any prior `~/.cargo/bin/unispec`.

### `cargo build` fails with `error[E0027]: pattern does not mention fields short, content`

**Cause.** You checked out `main`, which has three pre-existing compile errors in `src/main.rs`. The `everything` branch fixes them.

**Fix.**
```bash
git checkout everything
cargo build
```

### `error: could not find `Cargo.toml` in /home/theeye or any parent directory`

**Cause.** Running `cargo build` from outside the project root.

**Fix.**
```bash
cd /path/to/UniSpec
cargo build
```

## WSL-specific

### `xdg-open` errors on Enter in the TUI

WSL on Windows has no display server by default, so `xdg-open` fails. This is exactly why the Enter handler now uses `$EDITOR` / `nano` / `vi` instead. If you're on `everything`, this is already fixed. If not, rebuild.

### `nano` runs but the screen is glitched on exit

**Cause.** Older binary that didn't issue the post-editor `Clear` + `terminal.clear()` repaint. Fixed on `everything`.

**Fix.** Rebuild.

### `unispec init` succeeds but `~/.config/unispec/` is empty

That's fine — UniSpec uses `.agent/` inside the project, not `~/.config/`. The global config dir is only consulted when looking for an overridden default mode or area templates. The embedded default mode covers everything for a fresh project.

### Line endings (CRLF vs. LF) in spec files

If you edit `.md` files from a Windows editor that defaults to CRLF, the MCP tools may misparse trailing whitespace in `- [ ]` lines. The recommended setup:

```bash
git config core.autocrlf input          # WSL: keep LF in the working tree
```

In the spec files themselves, prefer `\n` (Unix line endings). The CLI tools all write LF.

## Anything else

- Open an issue: <https://github.com/uwzis/UniSpec/issues>
- The `CHANGELOG.md` lists every fix made in this branch — if your symptom matches an entry, the fix is already in.
- For architectural questions (where does X live? why does Y do Z?), see [architecture.md](architecture.md).
