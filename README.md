# Aether

<p align="center">
  <img src="docs/brand/aether-logo.svg" alt="Aether logo" width="120" />
</p>

Aether is a frontier programming language for agentic coding: first-class agents, explicit effects/capabilities, deterministic replay, and policy-governed execution.

## Why Aether
- **Agent-native model**: `agent`, `team`, `workflow`, and reasoning primitives are built into the language.
- **Safety by construction**: static effect/capability checks plus runtime enforcement.
- **Deterministic auditability**: replayable execution traces with recorded nondeterminism.
- **Interop-ready**: typed boundaries for Python and Rust.

## Project Status
Aether is in specification-first development. Core language and reference compiler/runtime architecture are defined; implementation is in active buildout.

## Repository Documents
- `docs/specs/AetherLanguageSpecification.md` — normative language semantics and grammar.
- `docs/specs/AetherCompilerArchitectureSpec.md` — reference compiler/runtime architecture.
- `docs/project/AetherImplementationRoadmap.md` — milestone sequence.
- `docs/project/AetherExecutionBacklog.md` — prioritized execution queue.
- `docs/project/AetherConformanceAndQualityPlan.md` — validation matrix.
- `AGENTS.md` — autonomous engineering protocol.
- `.github/copilot-instructions.md` — agent decision-reference protocol.
- `docs/README.md` — documentation layout map.
- `docs/brand/aether-logo.svg` — Aether logo asset for docs/branding.
- `docs/llms.txt` and `docs/.well-known/llms.txt` — machine-readable discovery for coding agents.

## Discovery Setup
1. In repository settings, set Pages source to **GitHub Actions**.
2. Use `.github/workflows/pages.yml` to build/deploy the React website.
3. Set repository description, homepage (`https://harkirat155.github.io/Aether/`), and topics (for example: `programming-language`, `compiler`, `agentic-ai`, `deterministic-systems`, `formal-methods`).
4. Keep `docs/.nojekyll`, `docs/sitemap.xml`, `docs/robots.txt`, and `docs/llms.txt` current when docs paths change.
5. Do not commit generated `docs/index.html` or `docs/assets/*`; they are produced by the Pages workflow from `website/`.
6. Pin Aether in the GitHub profile and cross-link from related projects/posts.

## Collaboration Channels
- Discussions (design, philosophy, roadmap): `https://github.com/Harkirat155/Aether/discussions`
- Issues (bugs, feature requests, actionable tasks): `https://github.com/Harkirat155/Aether/issues`
- Pull Requests (implementation and docs contributions): `https://github.com/Harkirat155/Aether/pulls`

## Quick Start (Current)
1. Read `docs/specs/AetherLanguageSpecification.md` and `docs/specs/AetherCompilerArchitectureSpec.md`.
2. Review `docs/project/AetherImplementationRoadmap.md` for milestone context.
3. Pick a backlog item in `docs/project/AetherExecutionBacklog.md`.
4. Follow `CONTRIBUTING.md` to propose and ship changes.

## Contributing
We welcome contributors across language design, compiler engineering, runtime systems, docs, and conformance testing.

Start here:
- `CONTRIBUTING.md`
- `CODE_OF_CONDUCT.md`
- `SECURITY.md`

## License
Licensed under Apache-2.0. See `LICENSE`.
