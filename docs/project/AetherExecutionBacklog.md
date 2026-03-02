# Aether Execution Backlog (Agent-Ready)

## P0 — Must Complete for Conforming Alpha
1. Token model + span infrastructure.
2. Grammar-complete parser for Appendix B non-terminals.
3. Resolver + type checker + match exhaustiveness.
4. Capability checker + effect lattice enforcement.
5. TIR with mandatory expression annotations.
6. Trace engine with deterministic replay metadata capture.
7. Policy + `human_gate` enforcement path.

## P1 — Stabilization
1. Effect-aware optimization passes (safe subset only).
2. Python interop wrappers and error mapping.
3. Rust FFI bridge with ownership policy checks.
4. Language-server diagnostics and effect/capability lenses.

## P2 — Expansion
1. MLIR backend maturity and performance profile.
2. Additional memory providers via stable traits.
3. Extended effect classes under lattice-extension rules.

## Work Item Template
- **ID**:
- **Subsystem**: lexer | parser | sema | tir | backend | runtime | interop
- **Invariant(s) impacted**:
- **Required tests**:
- **Safety review**: effects/capabilities/human_gate
- **Replay impact**: yes/no + rationale
