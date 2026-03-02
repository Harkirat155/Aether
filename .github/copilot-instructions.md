# Copilot Instructions for Aether Repository

## Mandatory Document Reference Protocol
For every non-trivial design or implementation decision, agents must:
1. Identify the governing source documents used.
2. Cite exact file paths (and section names when available).
3. State why the chosen approach is consistent with those sources.
4. Call out any source conflicts and route them to `docs/project/AetherADR.md`.

## Source-of-Truth Priority
1. `docs/specs/AetherLanguageSpecification.md`
2. `docs/specs/AetherCompilerArchitectureSpec.md`
3. `AGENTS.md`
4. `docs/project/AetherConformanceAndQualityPlan.md`
5. `docs/project/AetherImplementationRoadmap.md`
6. `docs/project/AetherExecutionBacklog.md`
7. `docs/project/AetherProjectCharter.md`
8. `docs/project/AetherADR.md`

## Required Decision Checklist
Before finalizing changes, agents must confirm:
- Effect/capability safety constraints are preserved.
- Ownership/borrow and exhaustive-match semantics remain sound.
- Trace/replay determinism requirements are upheld.
- `human_gate` policy paths are not bypassed.
- Tests/docs are updated for changed behavior.

## Escalation Rule
If requirements are ambiguous or conflicting, stop implementation, document the conflict, and propose/record an ADR.
