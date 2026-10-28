# SYX documentation site

VitePress static site that publishes valuable content from `books/` and `docs/` for promoting the SYX language.

- **Default locale:** English (`/`)
- **Alternate locale:** 简体中文 (`/zh/`)

## Develop

```bash
cd site
npm install
npm run dev
```

`predev` / `prebuild` run `scripts/sync-content.mjs`, which copies markdown from the repo into `docs/` (source of truth stays in `books/` / `docs/`).

## Build

```bash
npm run build
# output: docs/.vitepress/dist
npm run preview
```

## Content map

| Site section | Source |
|--------------|--------|
| Quick Guide | `docs/9_UsageGuide.md` (+ `docs/zh/`) |
| Quick Reference | `docs/8_QuickReference.md` (+ `docs/zh/`) |
| Editors | Hand-written (`editors/vscode-syx` install + LSP) |
| Guide | `books/language-guide` · `books/zh-cn/language-guide` |
| Reference | `books/language-reference` · `books/zh-cn/language-reference` |
| Library | `books/library-guide` · `books/zh-cn/library-guide` |
| Compiler | `docs/1–5`, `10–12`, `env-registry` (+ `docs/zh/`) |
| About | `docs/adr/capabilities-summary.md`, `architecture-decisions.md` |
