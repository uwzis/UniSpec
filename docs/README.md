# UniSpec Documentation Index

This directory contains the canonical documentation for UniSpec Phase 2 governance, workflow philosophy, and engine principles. All contributors and AI sessions must consult these documents before performing any design, implementation, or validation task.


## Index

- **validator-execution-architecture.md** — Canonical architecture for validator execution, sequencing, results, severity, overrides, and explainability.
- **concrete-validator-architecture.md** — Concrete implementation design for validators, focusing on the boundary between reading state and modifying it, deterministic IO, and error semantics.

- **ai-workflow.md** — Entry-point document for AI sessions. Defines how to read the repository, the role of conversation history, constitutional principles, execution boundaries, escalation philosophy, and prohibited anti-patterns.
- **phase2-philosophy.md** — Governance philosophy and authority model. Explains the rationale for authority constraints, approval and override semantics, actor class boundaries, migration principles, and deferred decisions.
- **workflow-engine-principles.md** — Technical invariants and implementation requirements. Specifies the canonical state model, transition graph, validator model, transaction and rollback semantics, locking, audit model, CLI/MCP parity, and implementation discipline rules.

All documents are cross-referenced and must be read in context. For full governance and workflow detail, consult all three documents.
