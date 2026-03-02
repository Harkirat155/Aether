# Aether Conformance and Quality Plan

## Core Principle
Validation must prove both language correctness and operational safety (effect/capability/policy/replay).

## Mandatory Test Families

### 1) Frontend Conformance
- Lexer acceptance/rejection corpus.
- Parser non-terminal coverage for Appendix B grammar.
- Span-accuracy tests for diagnostics.

### 2) Semantic Conformance
- Type errors (`SEM2xx`), borrow errors (`SEM3xx`), capability errors (`SEM4xx`), effect errors (`SEM5xx`).
- Exhaustive-match enforcement on closed sums.
- Public signature explicitness for effects in strict mode.

### 3) Runtime Conformance
- Deterministic replay exactness under fixed seed/tool I/O.
- Divergence rejection behavior.
- `human_gate` requirement for policy-tagged high-risk effects.
- Capability token and effect dispatcher hard-failure paths.

### 4) Interop Conformance
- Python bridge schema validation and typed error mapping.
- Rust bridge ABI safety, ownership transfer, panic isolation.

## Release Gates
- No unresolved conformance failures in affected modules.
- No prohibited optimization transformations introduced.
- Diagnostics retain stable codes/spans/messages for known cases.
- Replay and policy regression suites green.
