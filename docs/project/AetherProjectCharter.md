# Aether Project Charter

## Vision
Aether is a frontier language for agentic coding: safe, deterministic, capability-governed, and built for multi-agent reasoning workflows.

## Product Thesis
Aether should make powerful autonomous behavior **easier to express** while making unsafe behavior **hard to compile** and **impossible to execute silently**.

## Primary Goals (v1)
- First-class `agent`, `team`, `workflow`, and reasoning primitives.
- Statically checked effect/capability model with runtime defense-in-depth.
- Deterministic replayable execution trace for debugging, audit, and trust.
- Interop with Python and Rust through typed boundaries.
- Conformance-grade diagnostics and test matrix.

## Non-Goals (v1)
- General-purpose implicit dynamic typing.
- Effect/capability inference for public APIs in strict mode.
- Unsafely bypassing policy/human approval flows.

## Success Criteria
- Reference compiler passes conformance requirements in language and architecture specs.
- Replay divergence is rejected reliably in deterministic mode.
- Negative tests for effect/capability/ownership/human_gate violations are comprehensive.
