# SYX documentation site

VitePress static site that publishes valuable content from `books/` and `docs/` for promoting the SYX language.

- **Default locale:** English (`/`)
- **Alternate locale:** 简体中文 (`/zh/`)

## Develop

```bash
cd site
npm install
npm run dev
# → http://127.0.0.1:5174/
```

`predev` / `prebuild` run `scripts/sync-content.mjs`, which copies markdown from the repo into `docs/` (source of truth stays in `books/` / `docs/`). While copying it also:

- rewrites in-repo links to public site URLs (or GitHub for source files), so the published site has **no dead internal links**;
- derives a unique `title` and `description` frontmatter field from the chapter, so every page ships its own `<title>` and meta description.

Dev/preview bind **IPv4 only** on **5174 / 4174** so they do not collide with other tools that also use Vite’s default `4173` (often on `[::1]`).

## Build

```bash
npm run build
# output: docs/.vitepress/dist
npm run preview
# → http://127.0.0.1:4174/
```

`postbuild` rewrites `robots.txt` / `llms.txt` in `dist/` to use the configured `SITE_URL`.

## Verify

```bash
npm run check:links  # fails on broken internal links
npm run check:seo    # fails on missing canonical / hreflang / JSON-LD / description
npm run verify       # build + both checks
```

`check:seo` also warns about duplicate, too-short or too-wide titles/descriptions and about reused descriptions.

## SEO / GEO

Search engines (Google, Bing, Baidu, Yandex…) and AI answer engines are both first-class targets. Everything below is generated at build time — edit the sources, not `dist/`.

| Concern | Where |
|---------|-------|
| Origin used in canonical/OG/sitemap URLs | `SITE_URL` env var, default `https://www.syx-lang.org` |
| Canonical, hreflang, OG/Twitter, JSON-LD | `docs/.vitepress/config.mts` (`transformHead`) |
| Per-page `<title>` composition | `docs/.vitepress/config.mts` (`transformPageData` → `composeTitle`) |
| Locale/section/keyword tables, FAQ + SoftwareApplication | `docs/.vitepress/seo.ts` |
| Crawler policy (incl. AI bots) | `docs/public/robots.txt` |
| AI content index | `docs/public/llms.txt` |
| Social card (1200×630) | `docs/public/og-image.png`, regenerate with `npm run og-image` |
| Sitemap `x-default` + priorities | `sitemap.transformItems` in the config |

What ships on every page:

- absolute `rel=canonical`, reciprocal `hreflang` (`en`, `zh-CN`, `x-default`);
- unique `<title>` (section-qualified, brand-appended) and a unique meta description;
- `og:*` + `twitter:*` cards with the generated 1200×630 image;
- JSON-LD `@graph` with `Organization`, `WebSite`, and per page `TechArticle` + `BreadcrumbList` (homepage adds `SoftwareApplication` + `FAQPage`).

`robots.txt` allows classical crawlers and AI answer engines (`GPTBot`, `OAI-SearchBot`, `ClaudeBot`, `PerplexityBot`, …) with live retrieval, so SYX facts stay current inside AI answers.

## Deploy (Cloudflare Pages)

Manual publish via GitHub Actions: **Actions → Site Cloudflare Pages → Run workflow**.

Repository secrets (Settings → Secrets and variables → Actions):

| Name | Value |
|------|-------|
| `CLOUDFLARE_API_TOKEN` | API token with **Account → Cloudflare Pages → Edit** |
| `CLOUDFLARE_ACCOUNT_ID` | Cloudflare account ID |

Set the public origin as a repository variable so canonical URLs are correct:

| Variable | Value |
|----------|-------|
| `SITE_URL` | `https://www.syx-lang.org` (no trailing slash) — also the built-in default |

The production domain is **`https://www.syx-lang.org`**. Because it differs from the Pages subdomain, the domain must be attached to the Pages project and served over HTTPS before search engines index the canonical URLs:

1. In Cloudflare → **Pages → your project → Custom domains**, add `www.syx-lang.org` (and root `syx-lang.org`, which should redirect to `www`).
2. Point the DNS record at the Pages project (Cloudflare flattens the `www` CNAME automatically).
3. Set a redirect rule so `syx-lang.org` → `https://www.syx-lang.org` with a **301**, so only one canonical host is reachable.

Optional: set `SITE_CNAME=www.syx-lang.org` to emit a `CNAME` file into the build output.

In the Cloudflare dashboard, create a **Pages** project (Direct Upload is enough; no Git integration required). Default project name in the workflow is `syx-lang`. Production deploys use `--branch=main`. The workflow runs `check:links` and `check:seo` before deploying, so a broken link graph or missing metadata fails the run.

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

Hand-written pages (`docs/index.md`, `docs/zh/index.md`, the section `index.md` files, `editors/*`) carry their own `title` and `description` frontmatter — keep those unique.
