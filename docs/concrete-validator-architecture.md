# UniSpec Phase 2 — Chunk B2: Concrete Validator Implementation Design

## Preliminary: What B2 Actually Is

B2 is the chunk where the validator machinery from B1 meets the real world. B1 built the execution loop, the result types, the severity model, and the override separation. B2 fills that machinery with functions that read actual files and actual state and return actual verdicts.

This sounds straightforward. It is not, for one specific reason: **the boundary between "reading state to check it" and "doing something to state" is conceptually clear but implementationally easy to blur.** A validator that reads `tasks.toml` to check task completion is correct. A validator that reads `tasks.toml`, notices something looks off, and writes a correction is not a validator — it is an operation wearing a validator's clothing. B2's design must make this blur structurally impossible, not just conventionally discouraged.

Everything in this design document is oriented around that boundary.

---

## 1. Validator Implementation Philosophy

### The Validator Is A Question With A Typed Answer

Every concrete validator asks exactly one question about the current state of a topic. The question is decidable from information that is present in the canonical stores at the time of the call. The answer is one of three things: yes (Pass), no with explanation (Fail), or cannot determine because a prerequisite is not met (Skipped).

The question must be formulated before implementation begins. If the question cannot be stated in one sentence, the validator is asking more than one question and should be split. "Are all implementation tasks marked complete?" is one question. "Is the topic in a valid state to proceed to Testing?" is not — it is a summary of multiple questions, each of which is its own validator.

This philosophy has a practical consequence: **validators are not general health checks.** They are not responsible for catching everything that might be wrong. They are responsible for answering their specific question accurately. If something is wrong that no validator asks about, the answer is to add a validator with a clear question, not to make an existing validator broader.

### Validators Observe, Never Influence

A validator function's only output is its return value. It does not write files. It does not modify in-memory shared state. It does not enqueue operations. It does not log to a shared log. It does not emit events. It does not communicate with any other part of the system except through its return value.

This is the purity requirement. It is not a style preference. A validator that logs to a shared logger has a side effect — the log is different after the validator runs than before. For systems where logging is a background concern, this is acceptable. For UniSpec validators, which are explicitly designed to be reproducible and side-effect-free, even logging through a shared logger is a violation of the purity model.

The solution for diagnostics is: the result value contains the information. If the developer needs to know what the validator observed, that information is in the `current_state` field of the `ExplainableReason`. The result is the diagnostic.

### Small Is Correct

Each validator function should be small enough that its entire logic is visible in one screen of source code. If a validator requires complex logic to determine its answer, the complexity is in the question — which means the question is too complex and should be decomposed. The question "does `tasks.toml` parse correctly?" requires parsing TOML and checking for parse errors. That is appropriately scoped. The question "is the topic fully ready for Testing?" is not a single validator's question.

---

## 2. Validator Purity Rules

These are structural rules, not style guidelines. Each rule is stated as a prohibition because the corresponding temptation is real.

**Rule 1: No writes.** A validator function must not write to any file, any field in any struct, any channel, any database, or any other persistent or shared store. The filesystem access pattern is read-only at the OS level if possible, or read-only by convention enforced through the input model.

**Rule 2: No retained state.** A validator function must not retain state between calls. It must not use static mutable variables, thread-local storage, or any other mechanism that would cause its behavior to differ between the first and second call with identical arguments.

**Rule 3: No external calls.** A validator function must not make network calls, spawn processes, or consult external systems. It reads from the files provided to it, applies its logic, and returns.

**Rule 4: No time-based behavior.** A validator function must not use the current wall-clock time to make its determination, except when reading a timestamp from the canonical state that was already written by a prior operation. "Is this lock older than 30 minutes?" is not a valid validator question because the answer changes with time. "Does this lock have an acquisition timestamp?" is valid.

**Rule 5: No randomness.** Explicit, but worth stating: no random number generation, no UUID generation, no entropy consumption.

**Rule 6: No observable output other than the return value.** No println, no logging to shared loggers, no metric emission, no event publishing. The return value is the complete output.

**Rule 7: No calling other validators.** A validator function must not call another validator function. The execution engine is responsible for calling validators in sequence. A validator that calls another validator bypasses the execution engine's result collection, override handling, and audit recording for the inner call.

---

## 3. Filesystem Access Philosophy

### The Central Question: Direct Access Or Abstracted Access?

The B1 design document deferred this question to B2. The answer must be made here.

**Recommendation: Validators receive pre-read, parsed state values — not filesystem handles, not abstraction layers, not paths.**

The reasoning is as follows.

If validators receive filesystem paths and read files themselves, each validator is independently responsible for handling I/O errors, parse errors, and missing-file conditions. This produces twelve validators each with their own error handling for the same set of possible failures. The error handling is duplicated, and the question "what happens if `tasks.toml` is missing?" has twelve different answers depending on which validator happens to observe it first.

If validators receive a filesystem abstraction layer (an interface/trait that wraps filesystem operations), the validators are testable without real files, but the abstraction layer is now a dependency that must be designed, maintained, and kept in sync with actual filesystem behavior. For a system that values simplicity and explicitness, a filesystem abstraction layer is complexity in service of testability, when there is a simpler path to testability.

If validators receive pre-read, parsed state values, the state reading and parsing happen once, before any validator runs. I/O errors and parse errors are handled at the reading stage and produce a typed error that prevents validator execution entirely (because if `tasks.toml` cannot be parsed, no task-level validator can produce a meaningful result). Each validator receives clean, typed data and applies its logic. Validators are testable by constructing the typed data directly — no filesystem, no abstraction layer, no mocking.

This is the correct approach for this system.

### What "Pre-Read Parsed State" Means In Practice

Before the validator sequence runs for a transition, the engine performs a state collection step: read and parse `tasks.toml`, read and parse `.unispec/state.toml`, collect the relevant fields for the topic being transitioned. This produces a typed `TopicState` value (or a read error, which aborts the transition before validators run).

The `TopicState` is passed to every validator in the sequence as part of the `ValidatorInput`. Validators read from this value; they do not perform additional filesystem reads.

This means:
- Filesystem I/O happens once per transition evaluation, not once per validator.
- All validators see a consistent snapshot of state — the state at the moment the `TopicState` was collected, not the state at the moment each individual validator runs.
- Test code constructs `TopicState` values directly and passes them to validator functions without touching the filesystem.
- The behavior of "what happens if `tasks.toml` is missing" is handled entirely at the state collection layer and is not a concern for individual validators.

### Filesystem Access Rules For Validators

**Validators do not open files.** File opening is the state collection layer's responsibility.

**Validators do not access the filesystem through any path, handle, or abstraction.** If a validator needs information that is in a file, that information must be in the `TopicState` that the state collection layer provides.

**If a validator needs information that is not currently in `TopicState`, the solution is to add that information to `TopicState`, not to give the validator direct filesystem access.** The `TopicState` is the defined interface between the filesystem and the validators. Expanding it is a bounded change; giving validators ad hoc filesystem access is an unbounded one.

**The state collection step is not part of the validator framework.** It is a separate, independently testable concern. The state collection step can fail (producing a typed read error); the validator framework receives either a valid `TopicState` or does not run. There is no scenario where validators run against partially-collected state.

---

## 4. Validator Input Model

### The ValidatorInput Structure

Each validator function receives a single `ValidatorInput` value. This is not the same as `TransitionContext`. The `TransitionContext` contains the full transition request including overrides, actor class, and approval flags. The `ValidatorInput` contains only what the validator needs to observe state.

**`ValidatorInput` contains:**

- The topic identifier
- The source area (where the topic currently is)
- The target area (where the transition intends to move it)
- The `TopicState` — the pre-read, parsed state snapshot

**`ValidatorInput` does not contain:**

- The actor class. Validators must not produce different results based on who is asking.
- Override flags. Override handling is the execution engine's responsibility.
- Approval flags. Same reason as actor class.
- The transition request metadata. Validators check topic state, not transition metadata.

### The TopicState Structure

`TopicState` is a typed value that contains everything the validators need, organized by source:

**From `tasks.toml`:**
- The list of tasks with their IDs, categories, and completion status
- Whether the file was present and parseable (the state collection step guarantees this is true if validators run, but the structure carries the information for completeness)

**From `.unispec/state.toml`:**
- The current area of the topic
- Lock state: holder identity, acquisition timestamp (or None if unlocked)
- Queue readiness entry: present or absent, with its relevant fields
- Completion state markers

**From the filesystem structure (checked by state collection):**
- Whether the canonical files exist at their expected paths
- The modification timestamps of derived files (for staleness checks)

**Not in TopicState:**
- File contents beyond what is described above. Validators check properties of state, not raw file bytes.
- Any computed or derived values beyond what is directly readable. Derivation that happens inside `TopicState` construction is acceptable if it is deterministic and simple.

### Why Actor Class Is Absent From ValidatorInput

This deserves explicit treatment because it is the natural instinct to include it.

The question a validator asks is about the topic's state: "Are the implementation tasks complete?" The answer to this question is the same regardless of whether a human or an agent initiated the transition check. If the answer differs based on actor class, it is not a validator — it is an authority check, and authority checks belong in the authority layer (A3), not in the validator framework.

If there is a check that should only run for certain actor classes (e.g., "if the actor is an agent, check that a specific flag is set"), that check is either an authority policy rule or a policy-specific validator that the engine includes in the sequence only for certain actor classes. Either way, the validator function itself does not observe the actor class.

---

## 5. Validator Failure vs. Error Semantics

### Three Outcome Categories, Not Two

The B1 design established `Pass`, `Fail`, and `Skipped` as the outcome categories. For B2, the distinction between `Fail` and an error condition requires precise definition.

**Pass:** The validator's question has a "yes" answer. The check succeeded.

**Fail:** The validator's question has a "no" answer. The check found a specific, describable condition that does not satisfy the required state. The validator knows what it found and can explain it.

**Skipped:** The validator's question cannot be answered because a prerequisite is not met. The skip is honest: "I cannot check this because X." The skip is not a failure and not a success.

**The fourth category that B2 must handle: Error.** An error occurs when the validator encounters an unexpected condition during its logic that is neither a clean pass nor a describable failure. The validator cannot produce a `Fail` with a clean explanation because it does not know what happened.

### Error Semantics

An error in a validator is not the same as a failed check. A failed check means "I checked, and the state does not satisfy the requirement." An error means "I could not complete the check." The distinction matters for the audit record and for `workflow explain` output.

**How errors arise in B2 validators:**

The state collection step handles most error cases before validators run. If `tasks.toml` is unparseable, state collection fails and validators do not run. However, within the `TopicState`, some fields may carry optional or fallback values for conditions that are not complete failures at the collection level but that individual validators may find unacceptable.

For example: if the modification timestamp of a derived file cannot be read (because the file does not exist), the state collection step may record `derived_task_md_timestamp: None` rather than failing entirely. The staleness validator, when it receives `None`, must decide: is this a `Skipped` (cannot check staleness without a timestamp) or a `Fail` (the absence of the file is itself the problem)?

The validator's question determines the answer. If the question is "is the derived task.md stale?" and the file does not exist, the answer is Skipped with `PreconditionNotMet` because staleness requires a file to exist. The absence of the file is a separate question answered by a different validator.

**Error cases that genuinely cannot be classified:**

If a validator function encounters a condition in the `TopicState` that should be impossible — a value that the type system should have prevented but did not, an invariant violation in the data that the state collection step did not detect — the correct response is not to silently produce a `Pass` or a generic `Fail`. The correct response is a distinct error result that surfaces the impossible condition for investigation.

**Recommendation:** Add an `Error { description: String }` variant to the outcome enum for B2. An `Error` result from any validator always blocks the transition (treated as Hard, regardless of the declared severity of that sequence entry). It is recorded in the audit log with the description. It surfaces in `workflow explain` as a system condition, not as a policy failure. It signals that something unexpected occurred and the operator should investigate before retrying.

This is not the same as a panic. An `Error` result is a controlled, typed outcome. A panic is uncontrolled failure propagation.

### Panic Philosophy

Validator functions must not panic. A panic in a validator propagates through the execution engine and is not catchable in a structured way at the result collection level. The types should make panic-inducing conditions impossible: if a field might be None, the validator receives an `Option` and must handle both branches. If a value has range constraints, the state collection step enforces them before the validator runs.

If a validator function genuinely cannot avoid a panic (because it has reached code that should be unreachable given valid input), the appropriate response is to return `Error` rather than panic. The `Error` variant is exactly for this situation.

---

## 6. Validator Organization Strategy

### By Question, Not By Source

Validators are organized by the question they ask, not by where they read their data from. This is a naming and grouping convention that affects readability.

Grouping by source ("all `tasks.toml` validators together") produces a group that mixes Hard structural checks with Overridable policy checks. A reader looking for "what blocks this transition?" must scan multiple groups.

Grouping by question type — structural integrity, completion state, operational readiness — keeps related questions together and makes the validator sequence readable as a coherent checklist.

### Naming Convention

Validator function names follow the pattern: `check_<what>_<condition>`.

- `check_topic_exists` — is the topic present in the canonical records?
- `check_canonical_files_exist` — do the required files exist on disk?
- `check_tasks_toml_valid` — does `tasks.toml` parse and conform to the declared schema?
- `check_no_duplicate_task_ids` — are all task IDs unique within the topic?
- `check_no_dangling_queue_entries` — are there orphaned queue records?
- `check_lock_ownership_clean` — is there no conflicting lock holder?
- `check_impl_tasks_complete` — are all implementation-category tasks marked complete?
- `check_testing_tasks_complete` — are all testing-category tasks marked complete?
- `check_stale_derived_task_md` — is the derived task.md out of date?
- `check_queue_readiness_present` — is a queue readiness entry recorded in state?
- `check_index_bindings_present` — are the required index bindings present?

The name is the question. A reader of the sequence table who knows nothing about the implementation can read the names and understand the checklist.

### Module Organization

B2 validators are grouped into modules by their domain, where domain reflects the source of truth they consult:

**structural validators:** topic existence, file existence, TOML parse validity, duplicate IDs — checks that the basic structure of the topic's data is intact

**task validators:** implementation task completion, testing task completion — checks that task state satisfies transition requirements

**operational validators:** lock ownership, queue readiness, queue entry cleanliness — checks that operational state is consistent

**derived-artifact validators:** stale task.md, index bindings — checks that derived artifacts are current

Each module contains validator functions only. No state reading. No shared state. No module-level mutable variables.

---

## 7. Deterministic IO Philosophy

### The State Collection Step Is The IO Boundary

All filesystem and state store reads happen in the state collection step, which runs before the validator sequence begins. The validators do not perform IO. This makes the IO boundary explicit and the validator functions testable without IO.

The state collection step is:

1. A defined operation with a clear input (topic identifier, filesystem root) and a clear output (`TopicState` or `StateCollectionError`).
2. Performed synchronously.
3. Not a part of the validator framework — it is infrastructure that feeds the framework.
4. Independently testable against real filesystem fixtures.

### What "Deterministic" Means For IO

The state collection step reads from the canonical stores at a point in time. The `TopicState` it produces is a snapshot. All validators see this snapshot. If the underlying files change between the state collection step and the completion of the validator sequence, the validators do not observe those changes — they continue to operate on the snapshot.

This is the correct behavior. Validators that might observe different state at different points in their execution are not deterministic in the required sense. The snapshot model guarantees that all validators in a sequence evaluate the same state.

### Filesystem Read Failures

If the state collection step encounters a read failure (file not found, permission denied, IO error), it produces a `StateCollectionError` that describes what could not be read. This error prevents the validator sequence from running. The transition evaluation returns an error result (not a validation result) that surfaces the IO failure to the operator.

This is the correct boundary: IO failures are not validation failures. They are infrastructure failures that must be resolved before validation can meaningfully run.

---

## 8. Explainability Construction Philosophy

### The Three Fields Are Authored Per Validator, Not Generated

Each validator's `ExplainableReason` construction is specific to that validator. There is no shared template system, no format string library, no explanation generator. Each validator knows what it checked and what it found, and it describes those things directly.

The `summary` field is a single present-tense sentence describing the failure condition. It is authored once when the validator is implemented and does not change based on runtime conditions. "4 of 7 implementation tasks are marked incomplete" contains a runtime value (4, 7) but the sentence structure is authored.

The `current_state` field describes what the validator specifically found. It should include identifiers when available. "Tasks [task-42, task-67, task-88, task-91] have status 'pending' in tasks.toml" is correct. This field will vary at runtime based on what was found.

The `resolution` field is the most important field for operator experience. It must describe a concrete action. If the validator is Overridable, it must also describe the override path. The resolution field for `check_impl_tasks_complete` should tell the operator both how to actually complete the tasks and how to override if completion is intentionally deferred.

### Explanation Quality Is A First-Class Requirement

A validator implementation that returns `Fail` with a poor `ExplainableReason` is an incomplete implementation, not a partial one. The explanation quality should be reviewed as rigorously as the check logic during code review.

The test for explanation quality: read the explanation to a developer who has not seen the codebase. Can they determine, from the explanation alone, what went wrong and what to do about it? If not, the explanation is not good enough.

### Dynamic Content In Explanations

Explanations that include runtime values (specific task IDs, specific file paths, specific counts) are better than explanations with static text. The `current_state` field should always include the specific values that caused the failure. A validator that knows which tasks are incomplete must list them; it must not say "some tasks are incomplete."

However, explanations must not include speculative content ("this might be because...") or suggestions that are not directly actionable ("consider reviewing your workflow"). Every sentence in the explanation should be a fact or a concrete instruction.

---

## 9. Validator Testability Philosophy

### Test The Question, Not The Implementation

Each validator test is a test of the question the validator asks. The test setup creates a `TopicState` (or `ValidatorInput`) that represents a specific scenario, calls the validator function, and asserts on the result.

Test scenarios for each validator:

**The clean pass:** `TopicState` where the validator's question has a "yes" answer. Assert `Pass`.

**The specific fail:** `TopicState` where the validator's question has a "no" answer in a specific, known way. Assert `Fail`. Assert that the `ExplainableReason` contains the expected content — not just that it is non-empty, but that it contains the specific identifiers or values from the failing state.

**The skip case (if applicable):** `TopicState` where a prerequisite for the validator is absent. Assert `Skipped` with the correct `SkipReason`.

**The error case (if applicable):** `TopicState` containing a value that the validator treats as unexpected. Assert `Error`.

**Edge cases specific to the validator:** Empty task list, all tasks complete, exactly one task incomplete, zero queue entries, single queue entry, etc.

### No Filesystem In Validator Tests

Validator unit tests construct `TopicState` values directly. They do not write files, read files, or use filesystem fixtures. The state collection step has its own tests that use real filesystem fixtures. Validator tests and state collection tests are independent.

This is the primary testability benefit of the pre-read input model. Validator tests are fast, reliable, and do not require test infrastructure setup beyond constructing typed values.

### Testing Explanation Quality

Explanation quality is tested explicitly, not just structurally. For each `Fail` case, the test asserts not just that `ExplainableReason` is present but that `current_state` contains the specific identifiers from the test fixture and that `resolution` contains the expected action description.

This test discipline is what prevents explanation quality from degrading over time. If the test only checks that an explanation exists, explanation quality will drift toward minimum viable content under implementation pressure.

---

## 10. Validator Dependency Philosophy

### No Inter-Validator Dependencies

Validators must not depend on each other's results. One validator must not call another validator to reuse its logic. One validator must not require that another validator has already run.

When two validators share a common logical check (e.g., both need to verify that a task ID exists before checking its status), the shared check is extracted as a pure helper function — not a validator — and called by both validators independently. The helper function is a private implementation detail of the validator module, not a validator that the execution engine knows about.

This rule preserves the execution engine's authority: the engine is the only entity that knows which validators have run and what their results were.

### The PreconditionNotMet Skip Is Not A Dependency

When a validator returns `Skipped { PreconditionNotMet }`, it is expressing that it cannot answer its question given the current state of `ValidatorInput`. This is not a dependency on another validator's result — it is an observation about the input. The validator checks the input, finds a precondition absent, and returns Skipped. It does not check whether a specific other validator passed.

The distinction: "I cannot check task completion because `tasks.toml` has a structural error" is observing the input. "I cannot check task completion because the `check_tasks_toml_valid` validator failed" is a dependency on another validator's result. Only the former is acceptable.

---

## 11. What MUST NOT Be Implemented In B2

**Validators that write any state.** Not even "just a cache." Not even "just a log entry." Not even "just a timestamp update."

**Validators that observe the actor class, override flags, or approval flags.** These are `ValidatorInput` exclusions. The implementation must not smuggle this information in through another path.

**Validators that call other validators.** Even validators that seem related. Even validators that share 90% of their logic. The shared logic is a helper function, not a validator.

**Validators that produce different results on repeated calls with identical input.** The state collection step is the IO boundary. Validators that read additional state on their own are not reproducible.

**Validators that communicate through shared mutable state.** No shared counters, no shared error collections, no shared result buffers that validators write to during execution.

**Composite validators.** A single validator function that asks multiple questions and returns a result for each. The function signature returns one result. One question per function.

**Validators with configurable behavior driven by runtime configuration files.** If a validator's behavior needs to be tunable, the tunable parameter is part of `ValidatorInput` (and therefore declared statically per transition in the sequence table), not read from a config file by the validator itself.

**Explanation templates or shared explanation generators.** Each validator's explanation is authored specifically. A shared `make_explanation(template, args)` function produces formulaic explanations that satisfy the structural requirement without satisfying the quality requirement.

**Validators that treat IO errors as validation failures.** IO errors are infrastructure failures. If a validator encounters an IO error (which should not happen if the state collection step is correct), the result is `Error`, not `Fail`.

---

## 12. Highest-Risk Validator Implementation Mistakes

**Risk 1: Conflating "topic cannot proceed" with "validator fails."**
A validator checks one specific condition. Whether the topic can proceed overall is determined by the execution engine across all validators. A validator that tries to answer "should this topic proceed?" is scope-inflating and will produce confusing results.

**Risk 2: Implementing `check_impl_tasks_complete` using a different definition of "complete" than Phase 1.**
This is the highest-risk specific validator. The definition of "implementation task complete" must be exactly consistent with how Phase 1 determined completion. Any divergence means the engine and the existing operational logic give different answers about the same topic. The validator implementation must be verified against Phase 1's completion logic before the chunk is merged.

**Risk 3: Putting shared logic in a validator instead of a helper.**
When two validators share logic, the temptation is to make one call the other. This creates an implicit execution dependency that the execution engine cannot see or record. The helper function approach is slightly more code but completely explicit.

**Risk 4: Treating `Skipped` as a silent success.**
`Skipped` means "I could not answer the question." It does not mean "the topic passes this check." The audit record must reflect the skip. `workflow explain` must surface the skip with its reason. A `Skipped` result from a Hard-severity sequence entry is a situation that should be examined carefully — it may indicate a gap in the state collection step.

**Risk 5: Building an explanation that is accurate today but breaks as state evolves.**
Explanations that hardcode assumptions about what the state looks like ("check the queue entry in `.unispec/state.toml` under the `[topics.X]` key") will be wrong if the state schema changes. Explanations should describe the condition in terms of the canonical concepts, not the specific storage layout.

**Risk 6: Treating a parse error in state as a validator `Fail` instead of a state collection `Error`.**
If `tasks.toml` contains invalid TOML, this is a state collection failure. The `check_tasks_toml_valid` validator checks schema conformance of an already-parsed structure — it does not re-parse the file. The validator only runs if state collection succeeded, which means parsing succeeded. A validator that re-reads and re-parses the file will produce confusing behavior when the file changes between state collection and validator execution.

**Risk 7: Writing explanations in a voice that assigns blame.**
"You failed to complete the required tasks" is technically accurate but operationally hostile. "4 implementation tasks are marked incomplete" is neutral and more useful. Explanations describe state; they do not evaluate the operator's actions.

---

## 13. Recommended Implementation Boundaries For B2

### What B2 Delivers

**The `TopicState` type and the state collection step.** This is the IO boundary. It is separately testable against real filesystem fixtures. It produces either a populated `TopicState` or a `StateCollectionError`.

**The `ValidatorInput` type.** The typed container that carries `TopicState` to validator functions. Contains topic ID, source area, target area, and `TopicState`. Does not contain actor class, overrides, or approval flags.

**The concrete validator functions.** Organized by module (structural, task, operational, derived-artifact). Each function receives `ValidatorInput` and returns `ValidatorResult`. Each function is tested in isolation with constructed `ValidatorInput` values.

**The `Error` variant addition to the outcome enum.** If this was not added in B1 (it may have been deferred), B2 adds it as part of establishing the error handling semantics for concrete validators.

### What B2 Does Not Deliver

- Wiring validators into the sequence tables. That happens when the sequence tables are populated, which may be B2 or the early part of B4 depending on whether the sequence tables are currently populated with stubs.
- Any CLI surface. B2 is functions and types, not commands.
- Any transition engine integration. B2 validators are callable from tests; they are not yet called from a live transition path.
- Doctor integration. That is Chunk D2.
- Any validator that requires state not yet defined in `TopicState`. If a validator's question requires information not in `TopicState`, either add it to `TopicState` (if it is clearly within the existing canonical stores) or defer the validator (if it requires new state concepts).

### The Deliverable Verification Criteria

B2 is complete when:

Every validator function in the declared catalog has a concrete implementation. Every implementation has tests for pass, fail, and skip cases (where applicable). Every `Fail` result's `ExplainableReason` has been tested for content quality, not just presence. The state collection step has integration tests against real filesystem fixtures. No existing tests have been broken. The validator functions are not yet called from any production path (they are inert infrastructure, as with Macro-Phase A modules before B4 wires them in).
