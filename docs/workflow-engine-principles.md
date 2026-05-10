
# Workflow Engine Principles

**See also:** [Validator Execution Architecture](validator-execution-architecture.md), [Concrete Validator Implementation Design](concrete-validator-architecture.md)

## 1. What This Document Is

This document specifies the invariants that the Phase 2 workflow engine must satisfy. It is the specification that tests must validate, that implementation must conform to, and that reviewers must check against.

Sections marked **[CONSTITUTIONAL]** cannot be altered by implementation decision. Changing them requires a governance decision documented in `docs/phase2-philosophy.md` first.

Where this document conflicts with source code, the source code is the candidate for correction.

## 2. State Model [CONSTITUTIONAL]

There is one canonical state model. `tasks.toml` is the canonical content state. `.unispec/state.toml` is the canonical runtime/operational state. The workflow engine reads from and writes to these stores exclusively. It does not introduce a competing state representation, a derived-but-authoritative cache, or state held in memory across invocations.

`topic_status::compute` is a derived summary. It is computed from the canonical stores. It is not a state store. Writing to derived state instead of canonical state is always wrong.

The current area of a topic is authoritative in `.unispec/state.toml`. If a topic lacks an explicit area field, the engine derives it from `topic_status::compute` and persists it on the first successful move. After that, the persisted value is authoritative.

## 3. Transition Graph

### 3.1 Permitted Transitions

The primary forward path is: Staging → Working → Testing → Build.

The failure/remediation loop is: Testing → Fixing → Testing. This loop may repeat until the Testing exit conditions are satisfied.

Permitted backward transitions, each requiring human approval: Working → Staging; Testing → Working; Fixing → Working; Build → Working.

### 3.2 Forbidden Transitions [CONSTITUTIONAL]

The following transitions are forbidden. They do not exist in the graph. Override flags do not apply to forbidden transitions. No configuration can permit them.

Staging → Testing. Staging → Fixing. Staging → Build. Working → Fixing. Working → Build. Build → Testing. Build → Fixing. Build → Staging. Any transition not explicitly listed as permitted.

The rationale for hard-forbidding `Working → Build` and similar skip-Testing transitions: Testing is not a bureaucratic gate. It is a required quality checkpoint. A system that permits skipping it under any configuration does not have a Testing gate; it has a Testing suggestion. The gate must be architectural to be meaningful.

### 3.3 Transition Semantics

A transition is atomic. Either all state changes for a transition are committed or none are. There is no "partially transitioned" state.

A transition has exactly one actor. Transitions initiated by policy-triggered automation (e.g. auto Testing → Fixing on failed tests) record the triggering policy rule as the actor context, alongside the agent class. The audit record always shows who triggered the transition and what authority class was used.

Same-state "retry" is not a transition. Running `workflow validate` repeatedly on a topic in the same area is permitted. Attempting `workflow move` to the current area is a no-op error, not a transition.

## 4. Validator Model

### 4.1 Severity Classification [CONSTITUTIONAL]

**Hard validators** encode invariants. A hard validator failure means the system's integrity guarantee cannot be maintained if the transition proceeds. Hard validators cannot be overridden. Hard validators should be few. Adding a new hard validator is an architectural decision that requires documentation in this file before implementation.

**Overridable validators** encode policy that is correct under normal conditions but where human judgment may legitimately differ in specific circumstances. The validator is right by default. The override is a justified exception. If a validator's result can never be legitimately overridden, it should be hard. If a validator's result can always be safely ignored, it should be soft.

**Soft validators** encode hygiene and advisory checks. They do not block transitions. They are surfaced in `workflow status` and `workflow explain` output. They produce no friction. Soft validators should be used for checks where the right answer depends on context the engine cannot know.

### 4.2 Validator Catalog

The following catalog is authoritative for Phase 2. Validators not in this catalog do not exist. Adding a validator requires: a documented rationale, a severity classification with justification, assignment to one or more transition scopes, and a human-readable explanation message that must be tested alongside the validator itself.

*(Insert full validator catalog here as per design refinement. This placeholder is intentional and must be replaced with the authoritative catalog when finalized.)*

### 4.3 Execution Order

Validators execute in deterministic order within each severity class: hard validators first, then overridable, then soft. Within each class, execution order is stable across invocations and is determined by validator ID. The execution order is recorded in the audit event.

The engine surfaces the first hard blocking condition as the primary error. Overridable failures and soft warnings are listed as secondary context. This is an ergonomic decision: presenting all failures simultaneously is less useful than presenting the most fundamental one first. However, all validator results are recorded regardless of presentation order.

### 4.4 Probabilistic Validators [CONSTITUTIONAL, PROHIBITED]

Probabilistic validators are prohibited. A validator whose outcome can vary between invocations on identical state is not a validator; it is a random event with a name.

AI model assessments of topic state (e.g. "does this implementation seem complete?") are not validators. They may be surfaced as soft advisories in a future phase, but they will have zero blocking authority and their outputs will be recorded verbatim at evaluation time, not re-evaluated on demand. Any component that calls an AI model to make a blocking determination is a defect.

## 5. Transaction and Rollback Model

### 5.1 Transaction Phases

Every transition executes in five phases in order. No phase may be skipped.

**Preflight:** Graph validation, authority check, all hard validators, all overridable validators (checking for present overrides), all soft validators. No state changes in this phase.

**Snapshot:** Capture the current state of all artifacts that will be touched by the transition. The snapshot must be sufficient to fully restore prior state if rollback is required.

**Apply:** Write all state changes. This phase must be as short as possible. All decisions about what to write must be made in Preflight.

**Verify:** Confirm that postconditions hold after the write. If postcondition verification fails, proceed to rollback.

**Commit or Rollback:** On success, emit the `transition_committed` audit event. On any failure in Apply or Verify, restore the snapshot completely and emit `transition_rolled_back`.

### 5.2 Rollback Requirements [CONSTITUTIONAL]

Rollback must be complete. There is no partial rollback. Either all touched artifacts are restored to their snapshotted state or the rollback has failed, which is itself a catastrophic error that must be surfaced loudly.

A failed rollback is not handled gracefully. It is reported as a catastrophic system error requiring operator intervention. The audit log records the rollback failure. The operator is told explicitly that the system state may be inconsistent and must be manually inspected.

The engine does not attempt to self-repair after a failed rollback. Attempting automated recovery from an unknown inconsistent state would create worse inconsistency. The correct response is loud failure, explicit operator intervention, and `unispec doctor` to assess state integrity.

### 5.3 Interrupted Execution

If the engine process is terminated between Apply and Verify, the snapshot restore does not happen. This is the most dangerous failure mode and cannot be fully prevented in Phase 2.

Phase 2 mitigation: `unispec doctor` checks for post-interruption inconsistencies as part of its existing invariant checks. The audit log will contain a `transition_attempted` event with no corresponding `transition_committed` or `transition_rolled_back`, which is a detectable signal of interrupted execution. Operators encountering this pattern must run `unispec doctor` and perform explicit recovery.

A write-ahead log that would enable formal crash recovery is deferred to a later phase. See `docs/phase2-philosophy.md §Deferred Decisions`.

## 6. Locking and Contention

### 6.1 Lock Semantics

Locks are an attribute of topic state, not a separate system. A lock is acquired as a side-effect of transitioning into Working or Fixing — it is written in the same atomic operation as the area change. It is released as a side-effect of transitioning out of those areas. Locks and area state are always consistent because they are always written together.

A lock records: the topic, the holding actor, the actor class, and the timestamp of acquisition. This information is required for the lock holder to be identifiable in contention errors.

### 6.2 Contention Behavior [CONSTITUTIONAL]

When a caller attempts to move a topic that is locked by another actor, the engine rejects the attempt immediately. It does not queue the attempt. It does not wait. It does not retry. The rejection includes the topic, the current lock holder identity, and the lock acquisition timestamp.

This is not a limitation. Queuing implies the engine makes an ordering decision, which is an orchestration choice that belongs to the human operator. The caller that receives a contention rejection is responsible for handling its own retry logic in its own layer.

Concurrent `workflow validate` and `workflow status` calls on a locked topic are always permitted. These are read-only operations.

### 6.3 Lock Expiry

Locks do not expire automatically. Automatic expiry introduces race conditions where a lock expires as the legitimate holder is completing work, producing two actors who each believe they have authority over the same topic simultaneously. The cost of a stuck lock — an operator must explicitly clear it using `human-cli` authority — is lower than the cost of silent concurrent corruption.

Explicit lock clearing requires `human-cli` authority and a recorded reason. The clearing is recorded in the audit log.

Configurable lock expiry for CI environments is deferred to a later phase. See `docs/phase2-philosophy.md §Deferred Decisions`.

## 7. Audit and Event Model

### 7.1 Required Events

The following events must be emitted. Absence of any required event for a completed operation is a defect.

`transition_attempted`: emitted before Preflight begins. `validator_evaluated`: emitted for each validator that runs, including result and override status. `transition_committed`: emitted on successful Commit. `transition_failed`: emitted when Preflight blocks the transition. `transition_rolled_back`: emitted when Apply or Verify fails and snapshot is restored.

### 7.2 Required Fields

Each event must include: event ID, timestamp, actor identity, actor class, topic ID, from area, to area, command source, transition attempt correlation ID, final outcome.

### 7.3 Audit Integrity [CONSTITUTIONAL]

The audit log is append-only. No operation may modify, delete, or rewrite audit entries. The audit log is not a debugging convenience; it is the system's accountability record.

A transition that is not in the audit log did not happen as far as the system is concerned. A transition in the audit log that contradicts current state in `.unispec/state.toml` is a bug requiring investigation, not a state to be explained away.

## 8. CLI/MCP Parity Requirements

CLI and MCP tool invocations share one implementation: the `WorkflowFacade`. There is no CLI-specific logic and no MCP-specific logic in the transition engine. All authority and validation rules apply identically to both surfaces; the only difference is the actor class available to each.

The JSON output schema for `workflow status`, `workflow validate`, `workflow move`, and `workflow explain` is versioned. The version is included in every JSON response. CLI and MCP responses are structurally identical for the same operation. A change to the CLI JSON schema requires a corresponding change to the MCP schema and a version increment. Schema changes are not backward-compatible by default unless explicitly designed to be.

## 9. Doctor Integration

`unispec doctor` is an independent health check tool. The workflow engine integrates doctor findings as validators with severity mapped as follows: doctor hard-fail assertions map to hard validators; doctor warnings map to overridable validators; doctor advisories map to soft validators.

The engine does not reinterpret doctor's severity classifications. If doctor classifies a finding as a hard fail, the engine treats it as a hard validator. The engine's mapping of doctor findings is a pass-through, not an editorial decision.

Doctor is run as part of the Preflight phase for transitions that have a doctor gate in their validator set. Doctor is not run on every transition; the validator catalog specifies which transitions include the doctor gate.

## 10. Implementation Discipline Rules

Every validator ships with a tested human-readable explanation message. A validator that produces only a pass/fail result without an explanation message is incomplete. The explanation must describe: what the validator checks, what the current state is that caused failure, and what action would resolve the failure. This is a Definition of Done requirement for the validator framework, not a nice-to-have.

Every transition chunk must keep the test suite green. Implementation chunks are sized to be independently compilable and independently testable. A chunk that breaks existing tests is not mergeable regardless of how much of the new feature it implements.

Configuration does not override constitutional constraints. A configuration file that grants `agent-mcp` approval authority is invalid configuration, not an operator policy choice. The engine must reject it.

`topics_push` as a compatibility wrapper must produce identical transition outcomes to direct `workflow move` invocations. If the wrapper produces a different outcome for the same topic state and the same intended transition, the wrapper is broken.

New abstractions require documented justification. An implementation that introduces a new module, trait, or data structure not present in the current design documents must include documentation of why the abstraction is necessary. "It seemed cleaner" is not sufficient justification.

## 11. Definition of Done Per Area

*(Transplanted directly from the design with minor wording tightening. Each area's DoD is a testable contract. Insert authoritative prose here when finalized.)*

---

## Cross-References

- See `docs/ai-workflow.md` for entry-point constraints and constitutional principles.
- See `docs/phase2-philosophy.md` for operational philosophy and authority model.
- See §4.2 for the validator catalog.
- See §5.2 for rollback requirements.
- See §6.2 for contention behavior.
- See §7.3 for audit integrity.
