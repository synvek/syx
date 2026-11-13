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
| Baidu / Bing verification tags | `BAIDU_VERIFICATION` / `BING_VERIFICATION` in `docs/.vitepress/seo.ts` |
| AI content index | `docs/public/llms.txt` |
| Social card (1200×630) | `docs/public/og-image.png`, regenerate with `npm run og-image` |
| Sitemap `x-default` + priorities | `sitemap.transformItems` in the config |

What ships on every page:

- absolute `rel=canonical`, reciprocal `hreflang` (`en`, `zh-CN`, `x-default`);
- unique `<title>` (section-qualified, brand-appended) and a unique meta description;
- `og:*` + `twitter:*` cards with the generated 1200×630 image;
- JSON-LD `@graph` with `Organization`, `WebSite`, and per page `TechArticle` + `BreadcrumbList` (homepage adds `SoftwareApplication` + `FAQPage`);
- search-engine ownership proof — `<meta name="baidu-site-verification">` on every page, so a homepage rewrite cannot drop 百度 verification. `check:seo` fails the build if the tag disappears from the site root.

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

The production domain is **`https://www.syx-lang.org`**. The apex `syx-lang.org` should answer with a `301` to the `www` host, so only one hostname is indexable.

Order matters here: create and verify the redirect first, then tidy up. A Redirect Rule is evaluated at the edge *before* the Pages origin, so it takes precedence even while the apex is still attached to the project — which means you can get a correct redirect with no downtime.

1. **Create the redirect rule** (below). The apex must already have a proxied DNS record, which adding it as a Pages custom domain will have created.
2. **Verify** with `curl` that the apex returns `301` and keeps the path.
3. **Only then tidy up**: remove `syx-lang.org` from **Workers & Pages → your project → Custom domains**, leaving just `www.syx-lang.org`. Removing it may remove the DNS record it created, so replace it with a proxied placeholder: `A 192.0.2.1` or `AAAA 100::`. These are documented discard addresses — the edge answers before the connection is ever dialled.

   You can also leave the apex attached. It works, but two things then bind the apex (a certificate and a Pages binding), and if the rule is ever disabled or its host filter edited, the apex silently starts serving the whole site with `200` — the exact duplicate-content situation the redirect exists to prevent.

4. The Pages subdomain `<project>.pages.dev` lives in Cloudflare's own `pages.dev` zone, so a redirect rule in your zone cannot match it. Leave it as is — the canonical tags already point at `www`, which is how search engines consolidate the duplicate.

### Apex → www redirect (Cloudflare)

Two equivalent ways to express this. Both are Free-plan features; no Workers needed.

**Option A — wildcard (simplest in the dashboard)**

- When incoming requests match: **Wildcard pattern** → Request URL `https://syx-lang.org/*`
- Then: **Target URL** `https://www.syx-lang.org/${1}`, **301**, *Preserve query string* on

**Option B — expression (what the API uses)**

- When incoming requests match: **Custom filter expression** → `http.host eq "syx-lang.org"`
- Then: **Dynamic** redirect
  - Expression: `concat("https://www.syx-lang.org", http.request.uri.path)`
  - **301**, *Preserve query string* on

Do not set a static target (`https://www.syx-lang.org` with no variable) — every deep link would land on the homepage, which looks like a mass soft-redirect to search engines.

Then:

1. **SSL/TLS → Edge Certificates → Always Use HTTPS**: on. `http://` is upgraded before the rule is evaluated, so `http://syx-lang.org/x` costs two hops but always ends on the right host.
2. Verify:

   ```bash
   curl -sI https://syx-lang.org/guide/01-getting-started
   # expect: HTTP/2 301
   #         location: https://www.syx-lang.org/guide/01-getting-started

   curl -sI https://syx-lang.org/guide/01-getting-started?x=1
   # expect: location keeps ?x=1
   ```

   Check in a private window too — a cached 301 or an HSTS pin from an earlier attempt will hide a broken rule.

**Evaluation order** at the edge: Always Use HTTPS → Bulk Redirects → Single Redirects → cache → origin. That ordering is why the apex redirect wins over the Pages binding, and why the apex DNS record must be **proxied** (grey-clouded records bypass the rule entirely and return NXDOMAIN or the origin directly).

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
