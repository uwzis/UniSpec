# Workflow: /verify (default mode)

Confirm that the implementation in `src/` satisfies every requirement in the spec. With `--fix`, repair gaps and return to Testing.

Mirrors `.agent/workflows/verify.md`; per-mode copy.

---

## Tools

MCP:
- `unispec_read_spec { topic, area }`
- `read_asset { topic, asset_type: "spec", area }`
- `change_list { topic, area, include_archived? }` — every non-archived change must be accounted for before push to Build
- `change_archive { topic, change, area }` — archive a complete change so it stops showing as live work
- `index_list { topic }`
- `index_find { query, by }` — `by ∈ {"topic","path","tag"}`
- `index_backlinks { topic }`
- `notes_add { topic, note }`
- `topics_push { topic, area }`
- `task_status { topic, area, status }`

CLI:
- `unispec auto verify <topic>` — runs the configured verifier.

There is no MCP tool named `unispec_auto_verify`, `unispec_query_relations`, `unispec_update_task`, or `unispec_nav`. Use the tools listed above.

---

## Steps

1. **Load context.**
   ```
   unispec_read_spec { topic: "<topic>", area: "<Testing or Working>" }
   index_list        { topic: "<topic>" }
   change_list       { topic: "<topic>", area: "<Testing or Working>" }
   ```

   **Account for every pending change before pushing to Build.** A topic
   that still has unarchived changes is *not* ready to ship — verify each
   one the same way you verify the topic's own requirements:

   - For every non-archived change, read its
     `spec/<area>/<topic>/changes/<change>/<change>_spec.md` and trace its
     requirements the same way as the topic's `REQ-*` rows below.
   - If a change is fully implemented (every box `[x]` in its task file
     AND all its requirements have evidence), `change_archive` it so it
     stops showing as live work.
   - If a change is genuinely deferred (won't ship in this build),
     `notes_add` an explanation. Do not silently leave it unarchived —
     downstream verifiers will treat it as a blocker.

2. **Trace each requirement.** For every `REQ-*` in the spec:
   - Find the linked file(s) from `index_list` (filter `link_type: "implementation"`).
   - Read the file(s) with the host editor's Read tool.
   - Record state: `✓ implemented`, `⚠ partial`, or `✗ missing`. Always cite `<file>:<line>`.

3. **Run the verifier (optional).**
   ```bash
   unispec auto verify <topic>
   ```
   Combine its output with your manual trace.

4. **Scope regressions.**
   ```
   index_backlinks { topic: "<topic>" }
   ```
   For each dependent topic, re-check its critical paths.

5. **Report.**
   ```
   notes_add {
     topic: "<topic>",
     note: "Verification YYYY-MM-DD: <N>/<M> requirements implemented.\n- REQ-001 ✓ src/auth/login.rs:42\n- REQ-002 ✗ not found\n- REQ-003 ⚠ partial — src/auth/login.rs:88 (locks after 10 failures, spec says 5)"
   }
   ```

6. **With `--fix`, if gaps exist:**
   ```
   topics_push { topic: "<topic>", area: "Fixing", source_area: "Testing" }
   task_status { topic: "<topic>", area: "Fixing", status: "working" }
   ```

   CLI: `unispec topic push <topic> --area Fixing --from Testing`.

   Fix in `src/`, flip checkboxes (`tasks_complete`), record decisions (`notes_add`). Then **enqueue for the push back to Testing** — Fixing is queue-gated, so this `queue_add` is mandatory:
   ```
   queue_add   { topic: "<topic>", area: "Fixing" }
   topics_push { topic: "<topic>", area: "Testing", source_area: "Fixing" }
   task_status { topic: "<topic>", area: "Testing", status: "complete" }
   ```

   CLI: `unispec queue add <topic> --area Fixing && unispec topic push <topic> --area Testing --from Fixing`.

---

## Definition of done

- Every `REQ-*` in the spec has a recorded state (`✓` / `⚠` / `✗`) with `<file>:<line>` evidence.
- Every non-archived change reported by `change_list` is either fully verified and `change_archive`-d, or explicitly deferred via `notes_add`. No pending changes are left silently unaccounted for.
- The verification block is appended via `notes_add`.
- With `--fix`: gaps closed, topic back in `Testing`, `task_status` is `complete`.
- Without `--fix`: gaps reported by `REQ-*` ID; the topic stays where it is.

---

## Failure modes

- **`unispec_read_spec` returns empty content** — wrong area, or files were renamed manually. Run `topics_show { topic, show_all: true }` to locate them.
- **`index_list` returns nothing** — code exists but was never linked. Catch up with `index_add` and report the gap as a BUILD-process failure.
- **Verifier exits non-zero with no per-REQ mapping** — don't auto-push to Fixing. Ask the user how to interpret the failure.
