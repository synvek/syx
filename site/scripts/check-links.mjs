#!/usr/bin/env node
/**
 * Post-build link audit: fails when the built site contains internal links
 * pointing at pages that do not exist.
 *
 *   node scripts/check-links.mjs            # audit docs/.vitepress/dist
 *
 * Exit code 1 when broken internal links are found, so CI and local builds
 * catch regressions in the books/ → site/ link rewriting.
 */
import fs from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const __dirname = path.dirname(fileURLToPath(import.meta.url))
const siteRoot = path.resolve(__dirname, '..')
const dist = path.join(siteRoot, 'docs/.vitepress/dist')

if (!fs.existsSync(dist)) {
  console.error('[links] no build found at docs/.vitepress/dist — run `npm run build` first')
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
    } else if (name.endsWith('.html') && name !== '404.html') {
      pages.push(p)
    }
  }
})(dist)

const routeOf = (file) => {
  const rel = path.relative(dist, file).split(path.sep).join('/')
  return '/' + rel.replace(/index\.html$/, '').replace(/\.html$/, '')
}

const isFile = (p) => {
  try {
    return fs.statSync(p).isFile()
  } catch {
    return false
  }
}

const targetExists = (urlPath) => {
  let p = urlPath.split('#')[0].split('?')[0]
  if (!p) return true
  p = decodeURIComponent(p)
  const candidates = p.endsWith('/')
    ? [path.join(dist, p, 'index.html')]
    : [path.join(dist, p), path.join(dist, p + '.html'), path.join(dist, p, 'index.html')]
  return candidates.some(isFile)
}

const broken = new Map()
let checked = 0
for (const page of pages) {
  const route = routeOf(page)
  const baseDir = route.endsWith('/') ? route : path.posix.dirname(route) + '/'
  const html = fs.readFileSync(page, 'utf8')
  for (const m of html.matchAll(/<a [^>]*href="([^"]*)"/g)) {
    const href = m[1]
    if (/^(https?:|mailto:|tel:|#|javascript:)/i.test(href)) continue
    if (href.startsWith('/assets') || href.startsWith('/vp-icons') || href.startsWith('/@')) continue
    const target = href.startsWith('/') ? href : path.posix.resolve(baseDir, href)
    checked++
    if (targetExists(target)) continue
    if (!broken.has(href)) broken.set(href, new Set())
    broken.get(href).add(route)
  }
}

console.log(`[links] ${pages.length} pages · ${checked} internal links`)

if (broken.size === 0) {
  console.log('[links] OK — no broken internal links')
  process.exit(0)
}

const rows = [...broken.entries()]
  .map(([href, from]) => ({ href, from: [...from] }))
  .sort((a, b) => b.from.length - a.from.length)

console.error(`[links] ${rows.length} broken target(s):`)
for (const { href, from } of rows.slice(0, 40)) {
  console.error(`  ${href}\n      from ${from.slice(0, 3).join(', ')}${from.length > 3 ? ` (+${from.length - 3})` : ''}`)
}
if (rows.length > 40) console.error(`  … and ${rows.length - 40} more`)
process.exit(1)
