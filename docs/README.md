# Aether Documentation Layout

![Aether logo](brand/aether-logo.svg)

## Website Root (GitHub Pages)
- `website/` — React + TailwindCSS + shadcn/ui source
- `docs/brand/aether-logo.svg` — brand/logo asset
- `docs/robots.txt` — crawler directives
- `docs/sitemap.xml` — search discovery map
- `docs/llms.txt` — LLM discovery index
- `docs/.well-known/llms.txt` — well-known LLM pointer
- `docs/.nojekyll` — ensures `/.well-known/*` is published on GitHub Pages
- `.github/workflows/pages.yml` — build/deploy automation

Generated at deploy time (not committed):
- site entrypoint `index.html`
- bundled web assets under `assets/*`

## Specs
- `docs/specs/AetherLanguageSpecification.md`
- `docs/specs/AetherCompilerArchitectureSpec.md`

## Project and Process
- `docs/project/AetherProjectCharter.md`
- `docs/project/AetherImplementationRoadmap.md`
- `docs/project/AetherExecutionBacklog.md`
- `docs/project/AetherConformanceAndQualityPlan.md`
- `docs/project/AetherADR.md`
- `docs/project/AetherDocsIndex.md`
