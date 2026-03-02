# Aether Reference Compiler Architecture Specification

**Version:** 1.0 Draft  
**Date:** March 2026  
**Status:** Reference architecture for the first conforming implementation

## 1. Purpose

This document defines the reference compiler and runtime architecture for Aether 1.0. It is implementation-oriented and complements the language specification by prescribing phase boundaries, internal representations, invariants, diagnostics contracts, and runtime interfaces.

## 2. Compiler Pipeline Overview

```text
Source Files
  -> Lexer
  -> Parser
  -> AST + CST spans
  -> Name Resolution
  -> Type Inference/Checking
  -> Capability & Effect Checker
  -> Ownership/Borrow Checker
  -> Typed IR (TIR)
  -> Mid-level Optimizations
  -> Lowering (Python transpile mode | MLIR mode)
  -> Codegen + Link
  -> Runtime Bootstrap + Metadata
```

### 2.1 Mandatory pipeline outputs

- Rich diagnostics with source spans and fix hints
- Module symbol index
- Typed IR per module
- Capability/effect manifest
- Replay metadata schema for runtime tracing

## 3. Frontend: Lexer Specification

### 3.1 Responsibilities

- Tokenize UTF-8 source into stable token stream
- Preserve trivia (comments/whitespace) for IDE and formatter modes
- Produce byte and line/column spans
- Recover from invalid UTF-8 and malformed literals

### 3.2 Token categories

- Keywords (`agent`, `tool`, `effects`, `requires`, etc.)
- Identifiers
- Literals (int, float, string, bool, none)
- Operators (`->`, `=>`, `::`, `&&`, `||`, `<=`, `>=`, ...)
- Delimiters (`()[]{},:;`)
- Contextual tokens (`&mut`, `begin`, `end`)

### 3.3 Lexer invariants

- Longest-match rule for operators
- Nested block comments are supported
- String escape decoding is deferred; lexer stores raw + cooked when valid
- Every emitted token has a non-empty span except EOF

### 3.4 Error model

- `LEX001`: invalid UTF-8 sequence
- `LEX002`: unterminated string literal
- `LEX003`: unterminated block comment
- `LEX004`: invalid numeric literal suffix/format

Errors are recoverable where possible; lexer emits placeholder tokens to maintain stream progress.

## 4. Parser Architecture

### 4.1 Strategy

- Hybrid recursive-descent parser with Pratt expression parser
- Single token lookahead minimum; bounded k-lookahead for ambiguous generic/type contexts
- Build both:
  - CST (lossless for tooling)
  - AST (normalized for semantic phases)

### 4.2 AST design goals

- Lossless source mapping via node spans
- Explicit node variants for agent/team/tool/workflow constructs
- Distinguish declarations from executable expressions
- Preserve effect and requires clauses as first-class nodes

### 4.3 Parsing recovery

- Synchronization tokens: `;`, `}`, `end`
- Error productions for common mistakes:
  - missing `:` in type annotations
  - malformed `effects {}` lists
  - dangling `else`

### 4.4 Parser diagnostics

- `PAR001`: unexpected token
- `PAR002`: expected token not found
- `PAR003`: malformed declaration header
- `PAR004`: invalid expression continuation

## 5. Semantic Analysis

### 5.1 Name resolution

- Build module scopes and lexical scopes
- Resolve symbols in two passes:
  - pass A: declarations/signatures
  - pass B: bodies and expressions
- Detect duplicate declarations and illegal keyword identifiers

### 5.2 Type checking

- Constraint-based local inference with explicit annotation preference
- Enforce:
  - exhaustive match on closed sums
  - borrow/move consistency
  - generic bounds and where clauses
  - `Probabilistic<T>` operation constraints

### 5.3 Ownership and borrow checking

- Move-by-default semantics
- Region-based borrow graph per function body
- Disallow:
  - use-after-move
  - mutable aliasing
  - escaping short-lived borrows

### 5.4 Capability checker

- For each callable item, compute required capability set
- Validate caller capability declaration includes transitive requirements
- Support policy override only via sanctioned wrappers

### 5.5 Effect checker

- Effects are explicit and not inferred for public signatures
- Internal functions may infer candidate effects, but signature must still be explicit in strict mode
- Verify effect lattice constraints:

$$
Pure \sqsubseteq ReadOnly \sqsubseteq SideEffecting
$$

- Validate effect set superset rule across call graph edges
- Enforce `human_gate` for policy-tagged high-risk effects

### 5.6 Semantic diagnostic families

- `SEM1xx`: symbol/scope errors
- `SEM2xx`: type errors
- `SEM3xx`: ownership/borrow errors
- `SEM4xx`: capability violations
- `SEM5xx`: effect violations

Each diagnostic must include `code`, `span`, `message`, and optional `fixIt`.

## 6. Typed IR (TIR) Design

### 6.1 Goals

- Canonical, typed, effect-annotated SSA-capable IR
- Independent of source syntax sugar
- Suitable for both transpilation and native backend lowering

### 6.2 Core entities

- `TModule`
- `TItem` (`TAgent`, `TTool`, `TFn`, `TStruct`, `TEnum`, `TWorkflow`)
- `TBlock`, `TStmt`, `TExpr`
- `TType`, `TEffectSet`, `TCapabilitySet`
- `TSpanRef` (for diagnostics mapping)

### 6.3 Required annotations per expression

- Static type
- Value category (owned/shared/mut-borrow)
- Effect contribution
- Capability requirements (if any)
- Purity flag for optimization

### 6.4 TIR example (conceptual)

```text
tfn cycle(obs: MarketObservation) -> Action
  effects {Network, MoneyTransfer, HumanGate}
  capabilities {read_market, execute_trade}
{
  %b1 = believe(obs)              ; effect: MemoryWrite
  %p1 = plan(goal_ctx)            ; effect: LLMInvoke
  %d1 = reason("Analyze", %p1)   ; effect: LLMInvoke
  %r1 = evaluate_risk(%d1)        ; effect: Pure
  br_if %r1 > 0.05 then %gate else %exec
}
```

### 6.5 Lowering contracts

- No unresolved symbols
- No untyped nodes
- No implicit effects/capabilities
- Structured control flow normalized before backend

## 7. Optimization Passes

### 7.1 Required safe passes

- Constant folding (pure expressions only)
- Dead code elimination (effect-aware)
- Common subexpression elimination (purity-gated)
- Inlining for pure/private small functions

### 7.2 Prohibited transformations

- Reordering side-effecting ops across effect barriers
- Eliminating `human_gate` edges
- Dropping trace emission points

## 8. Backends and Code Generation

### 8.1 Fast-interpret mode (transpile)

- Lower TIR to Python-compatible runtime IR
- Preserve semantic checks from compile phase
- Inject trace probes and schema validators

### 8.2 Compiled mode (MLIR/LLVM)

- Lower TIR to MLIR dialect stack:
  - control-flow + func
  - memory/ownership annotations
  - optional accelerator dialects
- Emit native object code and runtime metadata tables

### 8.3 Backend conformance

Both backends must preserve:

- Type soundness
- Effect/capability safety
- Deterministic trace semantics under replay constraints

## 9. Runtime Architecture

### 9.1 Runtime components

- Scheduler (agent cycles and workflow execution)
- Capability manager (tokenized capability grants)
- Effect dispatcher (audited side-effect execution)
- Trace engine (`ExecutionTrace` append-only log)
- Memory subsystem (short/long/skill tiers)
- Policy engine (`human_gate` and risk policies)
- Interop bridge (Python/Rust adapters)

### 9.2 Agent execution state machine

```text
Init -> Perceive -> Believe -> Deliberate -> Intend -> Act -> Reflect -> Idle/NextCycle
```

Transitions are explicit; each transition emits trace events.

### 9.3 Determinism and replay

Runtime MUST record:

- model seed/version
- prompt and validated schema hash
- tool call inputs/outputs
- non-deterministic timestamps/randomness sources

Replay mode uses recorded events to reconstruct outcomes and must reject divergence.

## 10. Effect and Capability Runtime Enforcement

Although primarily compile-time checked, runtime enforces defense-in-depth:

- Missing capability token => hard failure
- Unauthorized effect dispatch => hard failure
- Policy-gated action without `human_gate` approval => hard failure

## 11. Interoperability Layer

### 11.1 Python bridge

- Generated wrappers map Aether types to Python objects
- Runtime schema validation on boundaries
- Exceptions mapped to typed `Result::Err`

### 11.2 Rust bridge

- Generated FFI stubs with ABI-safe marshaling
- Ownership transfer policy explicit at each boundary
- Panic/abort isolation and typed fault mapping

## 12. Artifact Layout (Reference Project)

```text
/compiler
  /lexer
  /parser
  /ast
  /sema
    /resolver
    /types
    /effects
    /capabilities
    /borrowck
  /tir
  /opt
  /backend
    /python
    /mlir
  /diag
/runtime
  /scheduler
  /effects
  /capabilities
  /trace
  /memory
  /policy
  /interop
/tests
  /lexer
  /parser
  /sema
  /tir
  /runtime
```

## 13. Minimal Milestone Plan

- **M1:** Lexer + parser + AST/CST with diagnostics
- **M2:** Name resolution + type checker + effect/capability checker
- **M3:** TIR + interpreter backend + trace runtime
- **M4:** Borrow checker + policy engine + human gate
- **M5:** MLIR backend + interop bridges + conformance suite

## 14. Conformance and Validation Matrix

- Frontend conformance:
  - grammar acceptance/rejection corpus
  - span accuracy checks
- Semantic conformance:
  - capability/effect negative tests
  - ownership violations
  - exhaustive match enforcement
- Runtime conformance:
  - deterministic replay
  - policy/human_gate enforcement
  - interop boundary safety

## 15. Open Extension Points (Non-breaking)

- Additional effect classes under declared lattice extensions
- New backend targets via TIR lowering adapters
- Additional memory providers behind stable runtime traits

All extensions must preserve the mandatory safety and trace guarantees defined above.
