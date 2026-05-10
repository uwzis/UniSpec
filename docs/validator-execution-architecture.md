# UniSpec Phase 2 — Validator Execution Architecture

> **This document is canonical. It codifies the finalized validator execution architecture for UniSpec Phase 2, Chunk B1.**

**See also:** [Concrete Validator Implementation Design](concrete-validator-architecture.md)

## 1. Mental Model: Checklist, Not Framework

The UniSpec validator system is a **fixed checklist**: a known, ordered set of checks, each producing a typed result. There is no plugin loading, dynamic registration, or runtime discovery. The checklist is static, explicit, and auditable.

- Each transition has a compile-time-known, ordered sequence of validator functions.
- Each validator receives the transition context and returns a typed result.
- All results are collected in order; no validator can skip or short-circuit the sequence.
- No validator can produce side effects or influence the execution of others.

## 2. Validator Registry Structure

- **Per-transition static tables:** Each transition type has a compile-time-constant table: an ordered array of (validator_id, severity, function) triples.
- **No runtime registry:** There is no global mutable registry, no dynamic registration, and no runtime composition.
- **Function sharing:** Validators may appear in multiple transition tables. The function reference is shared; severity is declared per transition.

## 3. Validator Execution Model

- **Ordered static dispatch:** The engine iterates the transition's validator table in declaration order, calling each function with the transition context.
- **No trait objects, no dynamic dispatch:** All dispatch is explicit and static. The source code is the documentation.
- **No inter-validator communication:** Each validator is independent; results are not shared between functions.
- **No async, no retries:** All execution is synchronous and deterministic.

## 4. Validator Result Model

Each validator function returns a `ValidatorResult`:
- **validator_id:** Must match the table entry; mismatch is a programming error.
- **outcome:**
  - `Pass`
  - `Fail { reason: ExplainableReason }`
  - `Skipped { reason: SkipReason }`
- **ExplainableReason:**
  - `summary` (one sentence: what failed)
  - `current_state` (what was observed)
  - `resolution` (concrete, actionable fix)
- **SkipReason:**
  - `NotApplicableForTransition`
  - `PreconditionNotMet`

Severity is not returned by the function; it is declared in the table.

## 5. Deterministic Execution Guarantees

- **Fixed input sequence:** The validator sequence is a compile-time constant.
- **No shared mutable state:** Validators are pure functions over the transition context.
- **No inter-validator communication:** Each validator is isolated.
- **Stable ordering:** Execution order is declaration order, always.

## 6. Explainability Model

- **Mandatory, structured explanations:** Every `Fail` result must include a complete `ExplainableReason` (summary, current_state, resolution).
- **No free-form strings:** The type system enforces explanation quality.
- **Explainability is not separate:** `workflow explain` uses the same results as `workflow validate`.

## 7. Severity Evaluation and Override Semantics

- **Severity is static:** Declared in the table, not returned by the function.
- **Engine handles overrides:** Only Overridable failures can be overridden, and only by the engine.
- **Override does not change result:** The failure is still recorded; the override is an audit event.

## 8. Failure and Ordering Semantics

- **No short-circuiting:** All validators run, even after Hard failures. Skips are explicit and explained.
- **Primary/secondary/override/warning/skip:** The engine organizes results for presentation, not the validators.
- **Declaration order is execution order:** No sorting or reordering.

## 9. Integration and Boundaries

- **Validator execution is one phase:** It produces a `ValidationOutcome` for the transition engine.
- **No dynamic registration, async, or configuration:** All structure is static and explicit.
- **No state writes:** Validators are read-only.
- **No CLI or audit event integration in B1:** Those are later phases.

## 10. Anti-Patterns and Prohibitions

- No trait objects, plugin systems, or dynamic registration.
- No validator-to-validator communication.
- No probabilistic or async execution.
- No severity in function results.
- No aggregate or catch-all validators.
- No configuration-driven validator behavior.
- No explanation templates or free-form reasons.

## 11. Implementation Boundaries for B1

- **Delivers:**
  - Static sequence tables for transitions
  - Execution loop and result model
  - Test harness for the machinery
- **Does NOT deliver:**
  - Concrete validator functions (B2/B3)
  - Transition engine integration (C3)
  - CLI/audit event integration (B4)
  - State writes or audit events

---

**This document is canonical. All validator execution code and documentation must conform to this architecture.**
