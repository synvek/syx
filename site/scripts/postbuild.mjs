#!/usr/bin/env node
/**
 * Post-build fixups for files that live in `docs/public/` but must reflect the
 * deploy origin (which is configurable via `SITE_URL`).
 *
 * Runs automatically as npm's `postbuild` hook. `docs/public/*` stays the single
 * source of truth; only the built copies in `dist/` are rewritten.
 */
import fs from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const __dirname = path.dirname(fileURLToPath(import.meta.url))
const siteRoot = path.resolve(__dirname, '..')
const dist = path.join(siteRoot, 'docs/.vitepress/dist')

const SITE_URL = (process.env.SITE_URL || 'https://www.syx-lang.org').replace(/\/+$/, '')
// The origin written into the authored copies under docs/public/.
const PLACEHOLDER = 'https://www.syx-lang.org'

if (!fs.existsSync(dist)) {
  console.error('[postbuild] no dist directory — did the build fail?')
  process.exit(1)
}

let changed = 0
for (const name of ['robots.txt', 'llms.txt']) {
  const file = path.join(dist, name)
  if (!fs.existsSync(file)) {
    console.warn(`[postbuild] missing ${name}`)
    continue
  }
  const before = fs.readFileSync(file, 'utf8')
  const after = before.split(PLACEHOLDER).join(SITE_URL)
  if (after !== before) {
    fs.writeFileSync(file, after, 'utf8')
    changed++
  }
}

// A CNAME file makes Cloudflare Pages bind the custom domain on deploy.
if (process.env.SITE_CNAME) {
  fs.writeFileSync(path.join(dist, 'CNAME'), process.env.SITE_CNAME.trim(), 'utf8')
  changed++
}

console.log(`[postbuild] origin ${SITE_URL} · updated ${changed} file(s)`)
