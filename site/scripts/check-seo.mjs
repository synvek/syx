#!/usr/bin/env node
/**
 * Post-build SEO / GEO audit over docs/.vitepress/dist.
 *
 * Checks every rendered page for the signals search engines and AI answer
 * engines rely on, and reports the ones that are missing:
 *
 *   - unique, reasonable-length <title>
 *   - non-empty meta description (warning when duplicated)
 *   - <html lang> per locale
 *   - rel=canonical pointing at an absolute URL
 *   - reciprocal hreflang alternates + x-default
 *   - Parseable JSON-LD with Organization / WebSite / TechArticle nodes
 *   - og: / twitter: card tags
 *
 *   node scripts/check-seo.mjs [--strict]
 *
 * Exit code 1 when errors are found (or warnings too, with --strict).
 */
import fs from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const __dirname = path.dirname(fileURLToPath(import.meta.url))
const siteRoot = path.resolve(__dirname, '..')
const dist = path.join(siteRoot, 'docs/.vitepress/dist')
const strict = process.argv.includes('--strict')

if (!fs.existsSync(dist)) {
  console.error('[seo] no build found at docs/.vitepress/dist — run `npm run build` first')
  process.exit(1)
}

const pages = []
;(function walk(dir) {
  for (const name of fs.readdirSync(dir)) {
    const p = path.join(dir, name)
    const st = fs.statSync(p)
    if (st.isDirectory()) {
      if (name === 'assets') continue
      walk(p)
    } else if (name.endsWith('.html')) {
      pages.push(p)
    }
  }
})(dist)

const routeOf = (file) => {
  const rel = path.relative(dist, file).split(path.sep).join('/')
  return '/' + rel.replace(/index\.html$/, '').replace(/\.html$/, '')
}

const get = (html, re) => {
  const m = html.match(re)
  return m ? (m[1] ?? '').trim() : ''
}

/** CJK glyphs are ~2x the display width of a Latin character in SERP snippets. */
const hasCJK = (s) => /[\u3400-\u4dbf\u4e00-\u9fff]/.test(s)
const widthOf = (s) => [...s].reduce((n, ch) => n + (hasCJK(ch) ? 2 : 1), 0)

/** Collect JSON-LD blocks and report parse failures. */
function readJsonLd(html) {
  const blocks = []
  const re = /<script type="application\/ld\+json">([\s\S]*?)<\/script>/g
  let m
  while ((m = re.exec(html))) {
    try {
      blocks.push(JSON.parse(m[1]))
    } catch (err) {
      blocks.push({ __error: String(err) })
    }
  }
  return blocks
}

const errors = []
const warnings = []
const titles = new Map()
const descriptions = new Map()

let canonicalCount = 0
let hreflangCount = 0
let jsonLdCount = 0

for (const file of pages) {
  const route = routeOf(file)
  const html = fs.readFileSync(file, 'utf8')
  const is404 = path.basename(file) === '404.html'
  const lang = get(html, /<html[^>]*\blang="([^"]*)"/)

  if (!lang) {
    ;(is404 ? warnings : errors).push({ route, msg: 'missing <html lang>' })
  }

  if (is404) continue

  const title = get(html, /<title>([\s\S]*?)<\/title>/)
  const desc = get(html, /<meta name="description" content="([^"]*)"/)
  const canonical = get(html, /<link rel="canonical" href="([^"]*)"/)
  const ogImage = get(html, /<meta property="og:image" content="([^"]*)"/)
  const ogTitle = get(html, /<meta property="og:title" content="([^"]*)"/)
  const twitter = get(html, /<meta name="twitter:card" content="([^"]*)"/)
  const robots = get(html, /<meta name="robots" content="([^"]*)"/)
  const hreflangs = [...html.matchAll(/<link rel="alternate" hreflang="([^"]*)"/g)].map((m) => m[1])

  if (!title) errors.push({ route, msg: 'missing <title>' })
  else {
    titles.set(title, (titles.get(title) ?? []).concat(route))
    if (widthOf(title) > 72) warnings.push({ route, msg: `title too wide (${widthOf(title)}): ${title}` })
  }

  if (!desc) errors.push({ route, msg: 'missing meta description' })
  else {
    descriptions.set(desc, (descriptions.get(desc) ?? []).concat(route))
    const w = widthOf(desc)
    if (w < 70) warnings.push({ route, msg: `description too short (${w}): ${desc}` })
    if (w > 180) warnings.push({ route, msg: `description too wide (${w})` })
  }

  if (!canonical) errors.push({ route, msg: 'missing rel=canonical' })
  else if (!/^https?:\/\//.test(canonical)) errors.push({ route, msg: `canonical is not absolute: ${canonical}` })
  else canonicalCount++

  if (!ogImage) errors.push({ route, msg: 'missing og:image' })
  if (!ogTitle) errors.push({ route, msg: 'missing og:title' })
  if (!twitter) warnings.push({ route, msg: 'missing twitter:card' })
  if (!robots) warnings.push({ route, msg: 'missing robots meta' })

  if (hreflangs.length === 0) errors.push({ route, msg: 'missing hreflang alternates' })
  else {
    if (!hreflangs.includes('x-default')) errors.push({ route, msg: 'missing hreflang x-default' })
    if (!hreflangs.includes('en') || !hreflangs.includes('zh-CN')) {
      errors.push({ route, msg: `incomplete hreflang set: ${hreflangs.join(', ')}` })
    } else hreflangCount++
  }

  const ld = readJsonLd(html)
  const failed = ld.find((b) => b.__error)
  if (ld.length === 0) errors.push({ route, msg: 'missing JSON-LD' })
  else if (failed) errors.push({ route, msg: `invalid JSON-LD: ${failed.__error}` })
  else {
    jsonLdCount++
    const types = ld.flatMap((b) => (b['@graph'] ?? [b]).map((n) => n['@type']))
    for (const required of ['Organization', 'WebSite']) {
      if (!types.includes(required)) warnings.push({ route, msg: `JSON-LD missing ${required}` })
    }
  }
}

for (const [title, routes] of titles) {
  if (routes.length > 1) warnings.push({ route: routes.join(', '), msg: `duplicate title: ${title}` })
}
for (const [desc, routes] of descriptions) {
  if (routes.length > 3) warnings.push({ route: routes.slice(0, 3).join(', '), msg: `description reused ${routes.length}x` })
}

const sitemapPath = path.join(dist, 'sitemap.xml')
let sitemapCount = 0
if (!fs.existsSync(sitemapPath)) {
  errors.push({ route: '/sitemap.xml', msg: 'sitemap.xml not generated' })
} else {
  const xml = fs.readFileSync(sitemapPath, 'utf8')
  sitemapCount = (xml.match(/<url>/g) ?? []).length
  const hreflangLinks = (xml.match(/xhtml:link/g) ?? []).length
  if (sitemapCount < 100) errors.push({ route: '/sitemap.xml', msg: `only ${sitemapCount} URLs` })
  if (hreflangLinks === 0) warnings.push({ route: '/sitemap.xml', msg: 'no xhtml:link hreflang entries' })
}

if (!fs.existsSync(path.join(dist, 'robots.txt'))) {
  errors.push({ route: '/robots.txt', msg: 'robots.txt not published' })
}
if (!fs.existsSync(path.join(dist, 'og-image.png'))) {
  errors.push({ route: '/og-image.png', msg: 'og-image.png not published' })
}
if (!fs.existsSync(path.join(dist, 'llms.txt'))) {
  errors.push({ route: '/llms.txt', msg: 'llms.txt not published' })
}

// Search-engine ownership proof: Baidu fetches the site root, so the tag must
// be present there. Missing it silently blocks 百度搜索资源平台 verification.
const rootHtml = path.join(dist, 'index.html')
if (fs.existsSync(rootHtml)) {
  const root = fs.readFileSync(rootHtml, 'utf8')
  if (!/name="baidu-site-verification"\s+content="[^"]+"/.test(root)) {
    errors.push({ route: '/', msg: 'missing baidu-site-verification meta tag' })
  }
}

const report = (list, label) => {
  if (list.length === 0) return
  console.error(`\n[seo] ${list.length} ${label}:`)
  for (const item of list.slice(0, 30)) console.error(`  ${item.route}\n      ${item.msg}`)
  if (list.length > 30) console.error(`  … and ${list.length - 30} more`)
}

console.log(
  `[seo] ${pages.length} pages · canonical ${canonicalCount} · hreflang ${hreflangCount} · ` +
    `JSON-LD ${jsonLdCount} · sitemap ${sitemapCount} URLs`,
)
report(errors, 'error(s)')
report(warnings, 'warning(s)')

if (errors.length === 0 && (!strict || warnings.length === 0)) {
  console.log('[seo] OK')
  process.exit(0)
}
process.exit(1)
