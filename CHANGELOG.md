# Changelog

All notable project-level changes will be documented in this file.

## [Unreleased]
### Added
- Initial specification and architecture documents.
- Agentic execution protocol and implementation planning docs.
- Open-source logistics docs (`README`, `LICENSE`, contribution, conduct, security, support, governance, templates).
- Repository logistics metadata: `/.github/FUNDING.yml`, `/.github/CODEOWNERS`, and `/.github/copilot-instructions.md`.
- Documentation layout guide at `/docs/README.md`.
- Discovery metadata: `/docs/llms.txt`, `/docs/.well-known/llms.txt`, `/docs/sitemap.xml`, and `/docs/robots.txt`.
- GitHub Pages publish control file: `/docs/.nojekyll`.
- Repository discovery pointers: `/llms.txt` and `/.well-known/llms.txt`.
- React + TailwindCSS + shadcn/ui website source at `/website` with automated Pages deployment workflow.
- Aether logo asset at `/docs/brand/aether-logo.svg` and website favicon/branding usage.
- Rust workspace bootstrap for the reference compiler, including shared diagnostics/span infrastructure and an initial lexer implementation.

### Changed
- Reorganized technical and project docs into `/docs/specs` and `/docs/project`.
- Updated contributor and governance references to new documentation paths.
- Migrated landing site implementation from static HTML/CSS to React with CI-generated Pages artifacts (`website/dist` -> deploy artifact).

