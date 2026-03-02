# Contributing to Aether

Thanks for contributing to Aether.

## Before You Start
- Read `docs/specs/AetherLanguageSpecification.md` and `docs/specs/AetherCompilerArchitectureSpec.md`.
- Review `AGENTS.md` for project invariants.
- Check `docs/project/AetherExecutionBacklog.md` for open priorities.
- Follow `.github/copilot-instructions.md` for decision citation requirements.

## Contribution Types
- Specification clarifications and consistency fixes
- Compiler/runtime implementation work
- Conformance and regression tests
- Documentation and onboarding improvements

## Workflow
1. Open an issue (bug/feature/proposal).
2. Describe impacted invariants and expected behavior.
3. Submit a PR with focused scope and test evidence.
4. Ensure docs are updated when contracts or behavior change.

## PR Expectations
- Keep changes minimal and coherent.
- Include tests for positive and negative paths where relevant.
- Preserve effect/capability/ownership/replay semantics.
- Include migration notes for user-visible changes.

## Commit and Review Guidance
- Use clear, imperative commit messages.
- Link issues in PR descriptions.
- Be explicit about unresolved questions and trade-offs.

## Development Environment
Toolchain setup instructions will be expanded as compiler modules land. For now, start from the docs and proposed milestone scope.
