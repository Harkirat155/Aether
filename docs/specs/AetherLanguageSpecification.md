# Aether Language Specification

**Version:** 1.0 (Complete Draft — March 2026)  
**Status:** Normative reference for implementation and conformance testing

## 1. Scope and Conformance

This specification defines syntax, static semantics, runtime semantics, safety rules, standard library surface, and interoperability model for Aether 1.0.

A conforming compiler **MUST** implement all mandatory grammar productions, type/effect/capability checks, and deterministic execution trace requirements.

A conforming runtime **MUST** enforce capability boundaries and effect constraints at compile time and runtime.

## 2. Core Model

- Fundamental computation unit: `Agent`
- Every agent has native BDI state:
    - **Beliefs:** typed world model
    - **Desires:** prioritized goal set
    - **Intentions:** committed executable plan

Execution is a state transition system over:

`AgentState = (beliefs, desires, intentions, memory, capabilities, trace)`

## 3. Lexical and Parsing Rules

- UTF-8 source
- Comments:
    - line: `// ...`
    - block: `/* ... */` (nestable)
- Identifiers: `[A-Za-z_][A-Za-z0-9_]*`
- Blocks are explicit `begin/end` or braces depending on construct; indentation is non-semantic
- Reserved keywords cannot be used as identifiers

## 4. Type System

- Hybrid nominal + structural typing
- Algebraic data types:
    - sum (enum/variant)
    - product (struct)
- Ownership model:
    - move by default
    - shared borrow: `&`
    - mutable borrow: `&mut`
- `Option` and `Result` are first-class
- `Probabilistic<T>` is built-in and carries confidence metadata
- Exhaustive `match` required for all closed sum types
- No implicit `any`/dynamic in strict mode (default)

## 5. Effects and Capabilities

- Effects are explicit and checked
    - Example classes: `Network`, `DiskIO`, `MoneyTransfer`, `HumanGate`, `ToolInvoke`, `AuditLog`
- Capabilities are declared and statically verified
    - A call requiring capability `C` is illegal if caller capability set does not include `C`
- Effect subtyping:
    - `Pure ⊑ ReadOnly ⊑ SideEffecting`
- Function effect set must be a superset of all transitive callees unless wrapped in an approved sandbox combinator

## 6. Declarations and Scoping

- Top-level declarations:
    - `agent`, `team`, `tool`, `workflow`, `struct`, `enum`, `trait`, `impl`, `fn`, `const`, `use`
- Lexical scope with shadowing allowed; warnings are recommended
- Closures capture by move unless explicitly borrowed
- Module imports are capability-checked

## 7. Agent Runtime Semantics

Default cycle:

1. Perceive (observe input/events)
2. Believe (belief revision)
3. Deliberate (plan/reason)
4. Intend (commit action)
5. Act (effectful execution)
6. Reflect (outcome evaluation + memory consolidation)

Determinism rules:

- Given identical input stream, model seed, and tool responses in replay mode, trace **MUST** be reproducible
- All nondeterministic events **MUST** be recorded in `ExecutionTrace`

## 8. Memory Model

Memory tiers:

- `short`: bounded conversational state
- `long`: vector/indexed episodic memory
- `skill`: procedural policy memory

- Consolidation is transactional: either full commit or no commit
- Memory operations are effectful and require explicit memory capability

## 9. Teams and Workflows

- Team = typed multi-agent composition with role contracts
- Workflow = static graph with typed node I/O and bounded loops

Runtime must detect:

- cycle without progress
- `max_loop` violations
- schema mismatch on edge transfer

## 10. Reasoning Primitives

- `reason(prompt, context, output_schema) -> T`
- `plan(goal, context) -> Plan`
- `reflect(outcome) -> Reflection`
- `believe(fact) -> Unit`
- `intend(action) -> Unit`

Requirements:

- `output_schema` is mandatory for `reason` in strict mode
- Runtime **MUST** validate output against schema before value materialization
- Validation failure returns typed error, never unchecked object

## 11. Safety and Human Gates

- High-risk effects **MAY** be guarded by policy to require `human_gate`
- `human_gate` is the only sanctioned declassification path from protected intent to irreversible action
- Policies are declarative and auditable in trace

## 12. Interoperability

Python interop:

- typed wrappers generated at compile time
- marshaling errors are typed runtime errors

Rust interop:

- ABI-safe bindings via generated FFI layer
- ownership boundary rules enforced at bridge

All foreign calls are capability and effect annotated.

## 13. Diagnostics and Tooling

Compiler diagnostics **MUST** include:

- stable error code
- source span
- one-line cause
- minimal patch-oriented fix hint

Language server **SHOULD** provide:

- effect/capability lens
- trace-aware debugging
- schema contract previews

## 14. Standard Library (Normative Surface)

- `std::agent`
- `std::team`
- `std::workflow`
- `std::memory`
- `std::trace`
- `std::prob`
- `std::llm`
- `std::policy`
- `std::cap`

Each module **MUST** publish stable interfaces and semantic versioning guarantees for 1.x.

## Appendix A: Full Reserved Keywords (1.0)

`agent, team, role, goal, beliefs, desires, intentions, memory, capabilities, tool, workflow, protocol, reason, plan, reflect, believe, intend, effects, requires, policy, await, human_gate, begin, end, fn, struct, enum, trait, impl, match, if, else, while, for, loop, break, continue, return, use, const, let, mut, true, false, none, some, result, ok, err, async, spawn, where, as, in, self, super, module, pub, private, Probabilistic`

## Appendix B: Complete EBNF Grammar (1.0)

```ebnf
Program             ::= Item* EOF ;

Item                ::= ImportDecl
                                            | ConstDecl
                                            | StructDecl
                                            | EnumDecl
                                            | TraitDecl
                                            | ImplDecl
                                            | FnDecl
                                            | ToolDecl
                                            | WorkflowDecl
                                            | TeamDecl
                                            | AgentDecl ;

ImportDecl          ::= "use" ImportSpec ";" ;
ImportSpec          ::= StringLiteral
                                            | Identifier ("::" Identifier)*
                                            | "python" StringLiteral
                                            | "rust" StringLiteral ;

ConstDecl           ::= "const" Identifier ":" Type "=" Expr ";" ;

StructDecl          ::= "struct" Identifier TypeParams? "{" FieldDecl* "}" ;
FieldDecl           ::= Identifier ":" Type ( "=" Expr )? ";" ;

EnumDecl            ::= "enum" Identifier TypeParams? "{" VariantDecl ("," VariantDecl)* ","? "}" ;
VariantDecl         ::= Identifier
                                            | Identifier "(" TypeList? ")"
                                            | Identifier "{" FieldDecl* "}" ;

TraitDecl           ::= "trait" Identifier TypeParams? TraitBounds? "{" TraitItem* "}" ;
TraitItem           ::= FnSig ";" ;

ImplDecl            ::= "impl" TypeParams? Type TraitImplTarget? WhereClause? "{" ImplItem* "}" ;
TraitImplTarget     ::= ":" Type ;
ImplItem            ::= FnDecl | ConstDecl ;

FnDecl              ::= "fn" Identifier TypeParams? "(" ParamList? ")" ReturnType? EffectsClause? RequiresClause? Block ;
FnSig               ::= "fn" Identifier TypeParams? "(" ParamList? ")" ReturnType? EffectsClause? RequiresClause? ;

ToolDecl            ::= "tool" Identifier "(" ParamList? ")" ReturnType? EffectsClause? RequiresClause? Block ;

WorkflowDecl        ::= "workflow" Identifier "{" WorkflowBody "}" ;
WorkflowBody        ::= WorkflowNodeDecl WorkflowEdgeDecl WorkflowLoopDecl? ;
WorkflowNodeDecl    ::= "nodes" ":" "[" IdentifierList? "]" ";" ;
WorkflowEdgeDecl    ::= "edges" ":" EdgeList ";" ;
WorkflowLoopDecl    ::= "max_loops" ":" IntegerLiteral ";" ;
EdgeList            ::= Edge ("," Edge)* ;
Edge                ::= Identifier "->" Identifier
                                            | Identifier "->" "(" Identifier ("|" Identifier)+ ")" ;

TeamDecl            ::= "team" Identifier "{" TeamBody "}" ;
TeamBody            ::= "roles" ":" "[" TypeList? "]" ";" "protocol" ":" Expr ";" ;

AgentDecl           ::= "agent" Identifier TypeParams? "{" AgentBody "}" ;
AgentBody           ::= GoalSection?
                                                BeliefSection?
                                                DesireSection?
                                                IntentionSection?
                                                MemorySection?
                                                CapabilitySection?
                                                AgentMember* ;

GoalSection         ::= "goal" ":" Expr ";" ;
BeliefSection       ::= "beliefs" ":" Type ";" ;
DesireSection       ::= "desires" ":" "[" ExprList? "]" ";" ;
IntentionSection    ::= "intentions" ":" Type ";" ;

MemorySection       ::= "memory" "{" MemoryTier* "}" ;
MemoryTier          ::= Identifier ":" Type "(" ArgList? ")" ";" ;

CapabilitySection   ::= "capabilities" ":" "[" IdentifierList? "]" ";" ;

AgentMember         ::= FnDecl | ConstDecl ;

ParamList           ::= Param ("," Param)* ","? ;
Param               ::= Identifier ":" Type ;

ReturnType          ::= "->" Type ;

EffectsClause       ::= "effects" "{" EffectList? "}" ;
EffectList          ::= Identifier ("," Identifier)* ","? ;

RequiresClause      ::= "requires" "{" PredicateList? "}" ;
PredicateList       ::= Expr ("," Expr)* ","? ;

TypeParams          ::= "<" TypeParam ("," TypeParam)* ">" ;
TypeParam           ::= Identifier TraitBounds? ;
TraitBounds         ::= ":" Type ("+" Type)* ;

WhereClause         ::= "where" WherePred ("," WherePred)* ;
WherePred           ::= Type ":" Type ("+" Type)* ;

Type                ::= TypeUnion ;
TypeUnion           ::= TypePrimary ("|" TypePrimary)* ;
TypePrimary         ::= Identifier TypeArgs?
                                            | "&" Type
                                            | "&mut" Type
                                            | "[" Type "]"
                                            | "(" TypeList? ")"
                                            | "fn" "(" TypeList? ")" "->" Type
                                            | "Probabilistic" "<" Type ">" ;

TypeArgs            ::= "<" TypeList ">" ;
TypeList            ::= Type ("," Type)* ","? ;

Block               ::= "{" Stmt* Expr? "}" | "begin" Stmt* Expr? "end" ;

Stmt                ::= LetStmt
                                            | AssignStmt
                                            | IfStmt
                                            | MatchStmt
                                            | WhileStmt
                                            | ForStmt
                                            | LoopStmt
                                            | ReturnStmt
                                            | BreakStmt
                                            | ContinueStmt
                                            | AwaitStmt
                                            | ExprStmt ;

LetStmt             ::= "let" "mut"? Pattern (":" Type)? ("=" Expr)? ";" ;
AssignStmt          ::= Expr "=" Expr ";" ;
IfStmt              ::= "if" Expr Block ("else" (IfStmt | Block))? ;
MatchStmt           ::= "match" Expr "{" MatchArm+ "}" ;
MatchArm            ::= Pattern "=>" (Block | Expr) ","? ;
WhileStmt           ::= "while" Expr Block ;
ForStmt             ::= "for" Pattern "in" Expr Block ;
LoopStmt            ::= "loop" Block ;
ReturnStmt          ::= "return" Expr? ";" ;
BreakStmt           ::= "break" ";" ;
ContinueStmt        ::= "continue" ";" ;
AwaitStmt           ::= "await" Expr ";" ;
ExprStmt            ::= Expr ";" ;

Pattern             ::= "_"
                                            | Identifier
                                            | Literal
                                            | Identifier "(" PatternList? ")"
                                            | Identifier "{" PatternFieldList? "}" ;
PatternList         ::= Pattern ("," Pattern)* ","? ;
PatternFieldList    ::= PatternField ("," PatternField)* ","? ;
PatternField        ::= Identifier (":" Pattern)? ;

Expr                ::= LambdaExpr | LogicalOr ;
LambdaExpr          ::= "|" ParamList? "|" Expr ;

LogicalOr           ::= LogicalAnd ("||" LogicalAnd)* ;
LogicalAnd          ::= Equality ("&&" Equality)* ;
Equality            ::= Compare (("==" | "!=") Compare)* ;
Compare             ::= Add (("<" | ">" | "<=" | ">=") Add)* ;
Add                 ::= Mul (("+" | "-") Mul)* ;
Mul                 ::= Unary (("*" | "/" | "%") Unary)* ;
Unary               ::= ("!" | "-" | "&" | "&mut" | "*" ) Unary | Postfix ;
Postfix             ::= Primary (CallSuffix | IndexSuffix | FieldSuffix)* ;
CallSuffix          ::= "(" ArgList? ")" ;
IndexSuffix         ::= "[" Expr "]" ;
FieldSuffix         ::= "." Identifier ;

Primary             ::= Literal
                                            | Identifier
                                            | "(" Expr ")"
                                            | ArrayLiteral
                                            | StructLiteral
                                            | IfExpr
                                            | MatchExpr ;

IfExpr              ::= "if" Expr Block "else" Block ;
MatchExpr           ::= "match" Expr "{" MatchArm+ "}" ;

ArgList             ::= Arg ("," Arg)* ","? ;
Arg                 ::= Identifier "=" Expr | Expr ;

ExprList            ::= Expr ("," Expr)* ","? ;
IdentifierList      ::= Identifier ("," Identifier)* ","? ;

ArrayLiteral        ::= "[" ExprList? "]" ;
StructLiteral       ::= Identifier "{" StructInitList? "}" ;
StructInitList      ::= StructInit ("," StructInit)* ","? ;
StructInit          ::= Identifier ":" Expr ;

Literal             ::= IntegerLiteral
                                            | FloatLiteral
                                            | StringLiteral
                                            | "true"
                                            | "false"
                                            | "none" ;
```

## Appendix C: Conformance Test Requirements

- Parser tests for every grammar non-terminal
- Type-check tests:
    - ownership violations
    - non-exhaustive match
    - invalid capability/effect calls
- Runtime tests:
    - deterministic trace replay
    - schema validation failure path
    - human_gate policy enforcement
- Interop tests for Python and Rust boundary safety
