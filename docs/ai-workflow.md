
# AI Workflow

**See also:** [Validator Execution Architecture](validator-execution-architecture.md), [Concrete Validator Implementation Design](concrete-validator-architecture.md)

## 1. Purpose of This Document

This document exists because AI sessions have no memory between conversations. Every session that joins this repository cold must reconstruct the project's intent from the repository itself — not from chat logs, not from inference, not from industry defaults, and not from general knowledge about how AI-agent systems are typically designed.

UniSpec has a specific, deliberately constrained architecture. It differs from generic agent frameworks in ways that matter. Reading this document before taking any action is required.

## 2. How to Read This Repository as an AI Session

Before performing any task, an AI session must:

Read the documentation in `docs/` before reading source code. The architecture is documented before it is implemented; implementation details do not override documented philosophy.

Treat `docs/phase2-philosophy.md` and `docs/workflow-engine-principles.md` as authoritative. If source code appears to contradict those documents, the code is the candidate for correction, not the documents.

Identify the scope of the requested task explicitly before beginning. Tasks are categorized: design, documentation, implementation, review, or validation. These categories have different authority requirements and different failure modes. Do not begin a task without knowing which category it belongs to.

When the task is implementation, read the corresponding philosophy sections first. Implementation that contradicts philosophy is incorrect regardless of whether it compiles or passes tests.

## 3. What Conversation History Is Not

Conversation history is not workflow state. A previous chat session's conclusions, no matter how detailed, do not constitute a design decision unless they have been committed to this repository as documentation.

If a previous conversation reached a conclusion that is not reflected in the documents in `docs/`, that conclusion does not exist for the purposes of this session. Do not reconstruct it from memory, do not infer it from code, and do not treat a human's summary of a prior conversation as equivalent to committed documentation.

The correct response when encountering a gap between what a human describes from prior conversations and what the repository contains is: "This does not appear to be documented yet. Should I document it before proceeding, or can you point me to the committed document?" This is not obstructionism. It is the only way to maintain a single source of truth.

Implementation cannot create architecture. Writing code that embeds an architectural decision is not a substitute for documenting that decision. If an architectural decision is not in `docs/`, it has not been made, regardless of what the code does.

## 4. Constitutional Principles

1. **The engine is a contract enforcer, not an orchestrator.**
   The workflow engine validates that a transition is permitted and executes it. It does not decide what should happen next. It does not plan. It does not suggest strategy. It does not initiate transitions. Every state change has an explicit caller.

2. **Determinism is a guarantee, not a preference.**
   The same state, the same inputs, and the same declared policy must produce the same outcome every time. Any behavior that varies between identical invocations is a defect, not a feature.

3. **Explicit beats implicit, always.**
   No transition happens by inference, convention, or assumption. Every state change has an actor, an explicit trigger, and an auditable record. Default behaviors are documented and deterministic; they are not magic.

4. **Human sovereignty.**
   Decisions that require judgment belong to humans. The engine enforces human decisions; it does not substitute for them. When a transition requires approval, it requires a human. This is not about distrust of automated systems; it is about the nature of the decision itself.

5. **No hidden side effects.**
   Every operation that changes state must declare what state it changes before it changes it. Operations that fail must restore prior state completely. There is no acceptable "mostly succeeded" outcome.

6. **No silent partial states.**
   A transition either completes fully or does not happen. The system never enters a state where a topic has partially moved between areas. If a complete transition cannot be guaranteed, the attempt is aborted and the prior state is restored.

7. **No fake autonomy.**
   Automated actors do not make architectural decisions, do not approve transitions that require human approval, and do not override validators. Labeling automation as "intelligent" or "autonomous" does not grant it authority it does not have.

8. **One deterministic state model.**
   There is one canonical state for every topic at every moment. There are no competing state representations, no derived-but-authoritative caches, and no state held in conversation history. `tasks.toml` and `.unispec/state.toml` are the state. Everything else is derived.

9. **Operational explainability.**
   At any moment, the operator must be able to ask "why is this topic blocked?" and receive a complete, accurate, human-readable answer derived from current system state. Explainability is not a reporting feature. It is a runtime requirement.

10. **Escalation over guessing.**
    When a task is ambiguous, an AI session escalates to the human operator rather than resolving the ambiguity through inference. A wrong guess that produces a plausible-looking result is more dangerous than an explicit request for clarification.

11. **No hidden orchestration.**
    No automated process initiates, sequences, or manages workflow transitions without explicit declaration in policy. If automation is permitted, it is configured by a human, recorded in state, and surfaced in the audit log. Automation that operates outside these constraints is a bug.

12. **Conversation history is not workflow state.**
    See §3. This principle is repeated here because it is the failure mode most likely to occur in an AI-assisted project.

## 5. Execution Boundaries

An AI session operating in this repository is permitted to:

Read and analyze all committed documentation and source code. Generate documentation, design specifications, and analysis that has been explicitly requested. Implement changes to source code within the scope of an explicitly defined implementation task. Generate test cases that validate declared behavior. Identify contradictions between documentation and implementation and surface them for human resolution.

An AI session is not permitted to:

Redesign architecture during an implementation task. Introduce abstractions not present in the current design documents. Make policy decisions (validator severity, authority boundaries, transition rules) without explicit human approval and subsequent documentation commit. Treat a human's verbal summary of prior conversations as equivalent to committed documentation. Begin implementation of a component whose design is not fully documented. Assume that "this is how these systems usually work" is sufficient justification for any design or implementation choice.

When an AI session encounters a task that would require crossing these boundaries, the correct action is to stop, name the boundary that would be crossed, and ask how to proceed. Proceeding anyway and noting the boundary crossing afterward is not acceptable.

## 6. Escalation Philosophy

Escalation is the correct response to ambiguity, not a failure of initiative. An AI session that resolves ambiguity through inference and produces a plausible-looking result has created a more dangerous situation than one that asks a clarifying question.

Escalation is required when: the task scope is unclear; the task requires a design decision that is not documented; the task would require overriding a constitutional principle; the repository state contains a contradiction that cannot be resolved without human judgment; or the instructions for a task conflict with the documented philosophy.

Escalation is not required for: implementation details that fall clearly within a documented design; stylistic choices within established conventions; test case design within a documented component's behavior.

The escalation statement should be specific. "I am not sure how to proceed" is not an escalation. "This task would require me to decide whether [specific validator] should be hard or overridable, which is a policy decision not covered in the current documentation. Should I proceed with [specific default] or wait for that to be documented?" is an escalation.

## 7. Anti-Patterns

The following patterns are explicitly prohibited. If an AI session finds itself doing any of these, it must stop and escalate.

**Reconstructing architecture from code.** Reading source code to infer design decisions that are not documented. Code reflects a snapshot of implementation; it does not capture design intent, rejected alternatives, or policy constraints.

**Inferring policy from convention.** Applying "industry standard" patterns for agent systems, workflow engines, or state machines without verifying that those patterns are consistent with this system's documented philosophy. This system is intentionally different from generic frameworks in ways that matter.

**Treating fluency as authority.** Producing a confident, detailed, well-structured response to a question that requires a policy decision the AI session does not have authority to make. Confidence is not authority.

**Silent scope expansion.** Implementing a component that is adjacent to the requested task without explicit approval. "While I was doing X, I also did Y because it seemed necessary" is a scope expansion that may introduce undocumented design decisions.

**Verbal approval substituting for documentation.** Treating "yes, that sounds right" in a conversation as equivalent to a committed design decision. Verbal approval in conversation is the beginning of a documentation task, not a substitute for it.

**Probabilistic validators.** Introducing any check whose outcome can vary between invocations on identical state. All validators are deterministic. An AI model's assessment of a field's quality is not a validator; it is an advisory that belongs in the soft layer with zero blocking authority.

## 8. Cross-References

This document establishes entry-point constraints. For full governance detail:

Operational philosophy and authority model: `docs/phase2-philosophy.md`

Engine invariants, validator policy, transaction semantics: `docs/workflow-engine-principles.md`

Phase 2 implementation plan and chunk definitions: `docs/phase2-plan.md` (when committed)
