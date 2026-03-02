# AGENTS.md — Aether Autonomous Engineering Protocol

## Mission
Build a conforming Aether 1.0 compiler/runtime that implements the language and architecture specs with safety, determinism, and auditable behavior by default.

## Source-of-Truth Order
1. `docs/specs/AetherLanguageSpecification.md` (normative language semantics)
2. `docs/specs/AetherCompilerArchitectureSpec.md` (reference architecture and invariants)
3. This file (execution protocol)
4. Supporting project docs in this repository

If documents conflict, follow the highest-priority source and open an ADR for reconciliation.

## Decision Reference Requirement
For every non-trivial design or implementation decision, agents MUST:
1. Cite governing documents by path (and section when possible).
2. Explain why the chosen approach satisfies those sources.
3. Record unresolved conflicts in `docs/project/AetherADR.md`.

Repository agent behavior also follows `.github/copilot-instructions.md`, layered on top of global Copilot guidance.

## Non-Negotiable Invariants
- Enforce explicit effects and capabilities at compile time and runtime.
- Preserve deterministic replay requirements (`ExecutionTrace`, seeds, tool I/O, schema hashes).
- Never bypass `human_gate` for policy-tagged irreversible actions.
- Keep diagnostics stable: code + span + cause + minimal fix hint.
- Maintain type/ownership safety (move-by-default, borrow rules, exhaustive match).

## Agent Operating Loop
1. **Ingest**: read relevant spec sections before coding.
2. **Plan**: define smallest safe change with validation steps.
3. **Implement**: keep phase boundaries strict (lexer/parser/sema/TIR/backend/runtime).
4. **Validate**: run scoped tests + conformance checks for changed subsystem.
5. **Audit**: confirm effects/capabilities/trace semantics unchanged or intentionally updated.
6. **Document**: update docs when behavior or contracts change.

## Required Work Artifacts per Non-Trivial Change
- Problem statement and impacted invariants
- Design choice and rejected alternatives (ADR when architecture-significant)
- Test evidence (positive + negative)
- Safety impact assessment (effects/capabilities/human_gate)
- Replay impact assessment (trace/determinism)

## Definition of Done
A change is done only when:
- Spec conformance for impacted rules is demonstrated.
- No new unsoundness in type/effect/capability/borrow semantics.
- Diagnostics remain stable and actionable.
- Runtime replay and policy enforcement behavior remain valid.

## Escalation Triggers
Stop and escalate if any of the following is true:
- A required behavior is unspecified or contradictory across specs.
- A proposed optimization risks effect reordering or trace loss.
- Interop boundary requires ownership/capability semantics not yet defined.
