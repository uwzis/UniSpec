# UniSpec Phase 2 — PR Change Summary

This document provides a summary of the architectural and implementation changes introduced in the `phase2-workflow-engine` branch. It serves as a guide for reviewers to understand what was changed, why it was changed, what is currently functioning, and what is intentionally deferred.

## 1. Overview

This PR lays the architectural and structural foundation for **UniSpec Phase 2**, transitioning UniSpec from a legacy task-tracking system to a deterministic, human-gated workflow engine. It establishes a formal, strongly-typed transition graph, explicit actor class boundaries, and a purely functional validator execution core.

## 2. Why this work exists

In UniSpec Phase 1, workflow transitions (via `topics_push`) and state definitions were organically derived, leading to primitive obsession, implicit policies, and difficult-to-audit state changes. This work formalizes those workflows into a unified, machine-enforceable engine that guarantees identical outcomes for identical inputs, completely separating human authorization from agent mechanics.

## 3. Current branch / PR scope

**Branch:** `phase2-workflow-engine`
**Scope:** Architecture documentation, workflow typings (Macro-Phase A), and the validator execution core (Chunk B1). 
This PR does *not* replace the existing legacy workflow system yet, nor does it run against real topics yet. It builds the underlying engine that will eventually power those actions.

## 4. Completed work so far

The following work has been successfully completed and tested:
- Phase 0 stabilization
- Phase 1 canonical state migration
- Phase 1.5 / 1.75 operational hardening
- Governance and specification documentation completion
- Macro-Phase A: Typed workflow foundations (primitives, areas, transitions, authority metadata)
- Chunk B1: Validator execution core
- Chunk B1 hardening (fixing primitive obsession, aligning override boundaries)
- Chunk B2: Concrete validator architecture documentation

## 5. Documentation added

The following canonical architectural documents were added or substantially formalized to establish the rules of the engine:
- `docs/ai-workflow.md`
- `docs/phase2-philosophy.md`
- `docs/workflow-engine-principles.md`
- `docs/validator-execution-architecture.md`
- `docs/concrete-validator-architecture.md`

## 6. Architecture added

We introduced several typed abstractions to formally represent the workflow rules:
- **`WorkflowArea`**: Explicit typed states (`Staging`, `Working`, `Testing`, `Fixing`, `Build`).
- **`ActorClass`**: Defined permission classes (`HumanCli`, `AgentCli`, `AgentMcp`).
- **`TransitionRequest` / `TransitionResult`**: Typed execution payloads.
- **Typed Transition Graph**: Explicit, hardcoded list of permissible forward/backward movements.
- **Transition Authority Metadata**: Rules for who can invoke which transition and whether it requires human approval.
- **Validator Metadata Infrastructure**: Definition of `ValidatorId` and severity models (`HardFail`, `Overridable`, `SoftWarning`).
- **Execution Pipeline Types**: `ValidatorResult`, `ValidatorOutcome`, `ExplainableReason`, `SkipReason`, `ValidationOutcome`.
- **Deterministic Execution Loop**: Ordered, non-short-circuiting dispatch of validator functions.
- **CanProceed derivation**: Aggregation of validation outcomes into a boolean permit.

## 7. Runtime behavior changes

**Zero runtime behavior changes** have been introduced to the live tool. The existing `workflow move` and legacy features operate identically. The engine components introduced here are inert infrastructure waiting to be wired into the `WorkflowFacade`.

## 8. Runtime behavior intentionally not changed

The existing `topics_push` wrapper remains intact and unaffected. We have strictly preserved backwards compatibility for current workflows during this dual-path migration period.

## 9. Phase 2 implementation status

This PR covers Macro-Phase A and Chunk B1 of the implementation plan. 
**Important caveats—the following are NOT implemented yet:**
- `WorkflowFacade` is NOT implemented.
- `workflow move` is NOT implemented.
- Real, concrete validators (Chunk B2/B3) are NOT implemented.
- The transaction engine (snapshot/apply/rollback) is NOT implemented.
- Audit persistence is NOT implemented.
- CLI/MCP workflow surfaces are NOT implemented.

## 10. Current test status

- The codebase compiles with zero warnings.
- `cargo test` passes successfully.
- Latest test execution result: **150 passed; 0 failed**.
- Test coverage includes behavior guarantees for severity overrides, ordered execution, explicit fail reasons, and skip behaviors.

## 11. Important safety/invariant guarantees

- **Validator Purity**: The execution loop mandates that validators read inputs and return outputs cleanly. No state mutation, inter-validator communication, or side-effects are permitted inside the loop.
- **Determinism**: Execution order is identical to declaration order. The engine does not utilize async logic, dynamic plugin registries, or runtime configuration.
- **Override Isolation**: The boolean flag for override authority resides in the transition context. The engine dictates how this interacts with the `Overridable` validator severity, entirely preventing validators from parsing their own override states.

## 12. What is intentionally out of scope

- Migration of existing legacy topics to the new state stores.
- AI advisory layers (AI models determining workflow blockages).
- Crash recovery protocols and write-ahead logs (WAH).
- Lock expiry policies.

## 13. Recommended review order

1. **Governance & Philosophy**: Read `docs/phase2-philosophy.md` and `docs/workflow-engine-principles.md` to understand the structural rules.
2. **Validator Architecture**: Read `docs/validator-execution-architecture.md` and `docs/concrete-validator-architecture.md` to understand the engine design.
3. **Primitives**: Review `src/workflow/primitives.rs` and `src/workflow/graph.rs` to see the types in action.
4. **Execution Core**: Review `src/workflow/validator_exec.rs` and `src/workflow/validator_exec_test.rs` to observe the pure execution loop and test cases.

## 14. Next planned work

The next steps following the completion of this PR will involve:
- **Chunk B2/B3**: Implementation of the concrete validator functions referencing the `TopicState` snapshot model.
- **Chunk C1-C3**: Building the transaction engine and the `WorkflowFacade` to route requests.
- **Chunk B4**: Audit persistence and event recording.

## 15. Risks / things to watch

- **Architectural Drift**: Future contributors (human or AI) might attempt to bypass the rigid execution model by adding internal state, dynamic dispatch, or abstracting filesystem reading inside the validators.
- **Authority Inflation**: We must remain vigilant that automated actors (e.g., `AgentMcp`) are never granted approval authority in future configuration PRs.
