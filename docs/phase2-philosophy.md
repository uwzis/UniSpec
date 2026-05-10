
# UniSpec Phase 2 Philosophy

**See also:** [Validator Execution Architecture](validator-execution-architecture.md), [Concrete Validator Implementation Design](concrete-validator-architecture.md)

## 1. What This Document Is

This document records the governance philosophy for UniSpec Phase 2. It is not a tutorial, a user guide, or a feature description. It exists to preserve the reasoning behind authority constraints, approval requirements, and actor boundaries so that future contributors — human or AI — can understand not just what the rules are but why they exist and why changing them requires explicit deliberation.

Sections marked **[CONSTITUTIONAL]** describe principles that cannot be altered by implementation decisions, configuration, or task-level instructions. Changing a constitutional section requires an explicit, documented governance decision, not an implementation change.

## 2. System Purpose Statement

UniSpec is a workflow management system for structured software development. Its purpose is to make human-directed workflow decisions machine-enforceable, not to replace human judgment with machine judgment.

The system enforces. Humans decide. This distinction is the foundation of every authority and approval rule in Phase 2.

The system is not an autonomous agent. It is not an AI orchestrator. It is not a planning system. It is a contract enforcer: given a declared state, a declared policy, and an explicit human instruction, it either executes the instruction or explains precisely why it cannot.

## 3. Operational Philosophy

**On determinism.** [CONSTITUTIONAL]
The workflow engine produces identical outputs for identical inputs under identical policy. This is not a performance goal or a quality aspiration. It is a design constraint from which there are no exceptions. Behavior that varies between invocations on the same state is always a defect. Probabilistic or AI-judgment-based outcomes are not permitted anywhere in the transition or validation path.

**On explainability.** [CONSTITUTIONAL]
At any moment, any operator must be able to reconstruct the complete reason for any current system state using only current system state and the documented policy. This requires that the audit log be append-only and complete, that validator outcomes be recorded verbatim, and that `workflow explain` derive its output from live state rather than cached summaries. Explainability is not optional and cannot be degraded to improve performance.

**On autonomy.** [CONSTITUTIONAL]
Automated actors (agent-cli and agent-mcp) execute transitions that have been declared permissible by policy. They do not make policy decisions. They do not approve gated transitions. They do not override validators. The boundary between "permitted automation" and "autonomous decision-making" is always: did a human declare, in committed policy, that this specific action is permitted for this specific actor class? If not, a human must perform the action.

**On failure.** [CONSTITUTIONAL]
The system never leaves a topic in a state that cannot be explained by a completed, committed transition. Partial states, transitional states, and "in-progress" states are not permitted. A transition either completes atomically or does not happen. Failed transitions restore prior state completely and record the failure. There is no outcome category between "fully succeeded" and "fully reverted."

**On simplicity.**
The system resists complexity that serves no operational need. Features that are clever but not necessary are not added. Abstractions that are general but not required are not introduced. The scope of Phase 2 is fixed; additions require a documented design decision at the phase level, not an implementation decision within a component.

## 4. Authority Model

### 4.1 Actor Classes

The engine recognizes three actor classes. Classification is declared by the caller at invocation time. If no class is declared, the engine defaults to `agent-mcp`, the most restrictive classification. The engine does not infer actor class from context, session characteristics, or invocation source.

**`human-cli`:** A human operator invoking the CLI with explicit intent. This class has the broadest authority, including approval authority for gated transitions and override authority for overridable validators. Declaring `human-cli` is an assertion of human intent and human accountability.

**`agent-cli`:** An automated or scripted CLI invocation classified as non-human. This class can perform forward transitions that do not require human approval. It cannot perform backward transitions, approve gated transitions, or override validators.

**`agent-mcp`:** Any MCP tool call. MCP is structurally an agent surface. The engine does not distinguish "human using MCP" from "agent using MCP" because this distinction is not enforceable. MCP actor authority is identical to `agent-cli` with additional restrictions: MCP cannot perform Working → Testing even with approval flags present.

There is no `human-mcp` classification. The decision is architectural, not a policy preference: MCP callers are structurally indistinguishable from each other and from automated systems. Any caller can declare itself human. Granting human-level authority to a classification that cannot be verified would mean any automated system with MCP access could bypass human gates by declaring itself human. The conservative position is therefore enforced unconditionally: MCP is an agent surface. Humans requiring human-class authority use the CLI.

### 4.2 Authority Hierarchy

Authority in the engine is not a permission list. It is a hierarchy with a clear principle: the higher the risk of a transition, the higher the required actor class and the more explicit the invocation must be.

Transitions that move a topic into a more committed state (toward Build) have increasing authority requirements. Transitions that move a topic backward have the highest authority requirements because they represent reversal of previously approved decisions. Approval requirements on backward transitions are not bureaucratic friction; they are a record that a human has made an explicit decision to undo prior work.

### 4.3 Capability Matrix

This matrix reflects default policy. It cannot be altered by configuration alone for transitions marked [CONSTITUTIONAL]. Transitions that are architecture-level restrictions (e.g. MCP cannot approve Working → Testing) require a governance decision to change, not a configuration change.

| Transition                | human-cli | agent-cli | agent-mcp |
|---------------------------|-----------|-----------|-----------|
| Staging → Working         | ✅        | ✅        | ✅        |
| Working → Testing         | ✅+approval| ❌        | ❌        |
| Testing → Build           | ✅+approval| ❌        | ❌        |
| Testing → Fixing          | ✅        | ✅        | ✅        |
| Fixing → Testing          | ✅        | ✅        | ✅        |
| Working → Staging         | ✅+approval| ❌        | ❌        |
| Testing → Working         | ✅+approval| ❌        | ❌        |
| Fixing → Working          | ✅+approval| ❌        | ❌        |
| Build → Working           | ✅+approval| ❌        | ❌        |

## 5. Approval and Override Model

### 5.1 Approval Semantics

Approval is an inline assertion, not a workflow step. When a transition requires approval, the invoking actor includes `--approve` and `--reason` in the invocation. The engine validates that both are present, that the actor class has approval authority for this transition, and records the assertion in the audit log.

There is no approval queue. There is no async approval workflow. There is no approval token or delegation mechanism in Phase 2. These mechanisms are intentionally absent because they introduce complexity around token validity, delegation scope, and the question of whether the delegating human had sufficient context at the time of delegation. Inline approval is simpler, more auditable, and more honest about what "approval" means: a human actor, at this moment, is asserting this transition is appropriate.

The absence of an approval token model is a design decision, not a missing feature. If operational experience reveals a legitimate need for delegated approval, that decision will be documented as a governance change, not implemented as a convenience feature.

### 5.2 Override Semantics

Overrides apply only to validators classified as `overridable`. They require: actor class `human-cli`; `--override <validator_id>` naming the specific validator; and `--reason` with a non-empty human-readable justification. Each override is recorded individually in the audit log with the validator ID, reason text, and actor identity.

An override is not a suppression. The validator still runs and its result is recorded. The override instructs the engine to proceed despite the validator's negative result. The audit log contains both the validator's outcome and the override decision, making the complete picture reconstructable.

Overrides exist because overridable validators encode policy that is correct under normal conditions but where human judgment may legitimately differ. The validator is right by default. The override is an exception that must be justified. If an operator finds themselves regularly overriding the same validator, the operational question is whether the validator's classification is correct — but that question is answered through a documented policy change, not through routine overriding.

### 5.3 What Cannot Be Overridden [CONSTITUTIONAL]

Hard validators cannot be overridden by any actor, under any circumstances, with any justification. Hard validators encode invariants. An invariant that can be overridden is not an invariant.

Forbidden transition graph edges cannot be overridden. The transition graph is policy, not a validator. A transition that does not exist in the graph does not become permitted through override flags.

Actor authority constraints cannot be self-granted. An `agent-mcp` caller cannot grant itself `human-cli` authority. An `agent-cli` caller cannot grant itself approval authority for transitions that require `human-cli`.

## 6. Human vs. Agent Boundaries

The boundary between human and agent authority is based on the nature of the decision, not the capability of the actor. Automated systems can often perform transitions mechanically that humans can also perform. The question is not capability; it is accountability and judgment.

Transitions that require approval are gated on human judgment because they represent commitments that require human accountability. Working → Testing commits that implementation is complete enough to validate. Testing → Build commits that the topic is production-ready. These are not mechanical checks that an automated system can verify by running a test suite; they are qualitative judgments about readiness. The approval requirement is the system's way of ensuring a human has made and recorded that judgment.

Backward transitions are human-only because they represent the reversal of a previously approved decision. Reversing a decision that a human approved requires a human to make a new decision. Automating the reversal would mean that automated systems can undo human decisions without human involvement.

This boundary is not a technology constraint. It is a governance choice. The engine is capable of executing any transition for any actor. The authority restrictions are policy, not mechanism. Future phases that want to relax these boundaries must make a documented governance decision explaining why the judgment/accountability argument no longer applies.

## 7. Migration Philosophy

Phase 2 introduces the workflow engine as the single decision-maker for topic transitions. The migration from Phase 1 operational patterns to Phase 2 engine-enforced transitions follows a compatibility-first approach.

`topics_push` is retained as a compatibility wrapper that delegates to the engine. Its behavior is preserved for operators who have not migrated. Its internals now use the engine. This dual-path period is explicitly temporary.

The deprecation of `topics_push` is scheduled but not rushed. The timeline: the wrapper is retained and maintained until operational data shows negligible usage. Deprecation is signaled with documentation and `--help` notices before removal. Removal happens in a future phase with appropriate notice.

The migration principle: the engine is the authority from Phase 2 forward. Wrappers exist to maintain operator continuity, not to preserve alternative authority paths. No new functionality will be added to `topics_push`; new functionality is added to the engine and surfaced through `workflow` commands.

Markdown workflow documentation from Phase 1 becomes informational. It describes historical patterns. It is not authoritative for transition rules. The engine's declared policy is authoritative.

## 8. Governance Failure Modes

**Policy drift through implementation.** An implementation detail that embeds a policy decision without a corresponding documentation update. Recognized by: source code behavior that contradicts `docs/` content. Response: treat the code as the candidate for correction.

**Override normalization.** Repeated overriding of the same overridable validator until the override becomes the operational default. Recognized by: audit log patterns showing frequent overrides of a specific validator. Response: documented policy review to determine correct validator classification.

**Actor class inflation.** Gradually granting higher actor class authority to automated systems through configuration changes without governance decisions. Recognized by: configuration that effectively gives `agent-mcp` approval or override authority. Response: revert to defaults; require governance decision.

**Approval formalism without substance.** Human approval becoming a rubber stamp because the `--approve` flag is included by convention in all scripts. Recognized by: scripts that always include `--approve` regardless of context. Response: operational review; `--approve` must be an explicit human decision per invocation.

**Documentation lag.** Implementation outpacing documentation to the point where source code is the de facto specification. Recognized by: contributors asking "what does the code do?" instead of "what does the spec say?" Response: documentation freeze until `docs/` is current.

## 9. Deferred Decisions

The following decisions are explicitly deferred to later phases. They are not missing from Phase 2 by accident; they are absent because Phase 2 scope is fixed and these decisions require operational experience before they can be made well.

**Lock expiry policy.** Automatic lock expiry for CI environments where stuck locks are common. Deferred because: the failure modes of automatic expiry (race conditions, false-expiry during legitimate work) require operational experience to calibrate. The safe default (no automatic expiry) is used until the failure rate of stuck locks justifies the added complexity.

**Override frequency limits.** Policy-level limits on how often a specific validator can be overridden before requiring additional authorization. Deferred because: the override patterns in live operation are not yet known.

**Approval delegation model.** A token-based or role-based approval delegation mechanism for environments where the approving human cannot be present at invocation time. Deferred because: the inline approval model is simpler and sufficient; delegation introduces complexity that should be deferred until there is demonstrated operational need.

**AI advisory layer.** A non-blocking advisory layer that surfaces AI-model analysis of topic state. Deferred because: the determinism guarantee precludes AI assessments from having any blocking authority; the design of a useful advisory layer requires a separate design phase. If implemented, it will be strictly soft (non-blocking) with zero gating authority.

**Write-ahead log for crash recovery.** Formal crash recovery for interrupted transitions. Deferred because: Phase 2 mitigation (detection via `doctor` + explicit operator recovery) is sufficient for Phase 2 scale.

---

## Cross-References

- See `docs/ai-workflow.md` for entry-point constraints and constitutional principles.
- See `docs/workflow-engine-principles.md` for engine invariants, validator policy, and transaction semantics.
- See §9 for deferred decisions.
