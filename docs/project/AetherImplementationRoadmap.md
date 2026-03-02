# Aether Implementation Roadmap

## Milestone Stack (Spec-Aligned)

### M1 — Frontend Foundation
- Lexer with UTF-8 recovery, nested block comments, stable spans.
- Parser (recursive descent + Pratt) producing CST + AST.
- Diagnostic families: `LEX*`, `PAR*` with fix hints.

### M2 — Semantic Core
- Two-pass resolver and symbol tables.
- Type checker: generics, exhaustive match, `Probabilistic<T>` constraints.
- Capability/effect checker with transitive superset validation.

### M3 — Executable Typed IR
- Canonical typed TIR with effect/capability/value-category annotations.
- Safe optimization subset (no barrier-crossing reorder).
- Python fast-interpret backend with trace/schema instrumentation.

### M4 — Safety Runtime
- Ownership/borrow checker completion.
- Runtime policy engine and enforced `human_gate` path.
- Deterministic replay engine with divergence rejection.

### M5 — Production Backends + Interop
- MLIR/LLVM lowering path.
- Python and Rust bridges with typed boundary mapping.
- Full conformance and regression suite automation.

## Sequencing Rules
- Do not start backend-specific optimization before TIR invariants are complete.
- Do not declare runtime-complete until replay + policy checks pass negative tests.
- Every milestone must include conformance tests for new invariants.
