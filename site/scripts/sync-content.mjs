#!/usr/bin/env node
/**
 * Copy valuable books/ + docs/ markdown into the VitePress tree.
 *
 * Three jobs:
 *  1. Rewrite in-repo links to public site URLs (or GitHub for source files),
 *     so the published site has no dead internal links. Links to files that do
 *     not exist (internal plans, ledgers) are unwrapped to plain text.
 *  2. Derive `title` / `description` frontmatter from the chapter content so
 *     every page ships a unique <title> and meta description.
 *  3. Escape bare <Type> generics so Vue SFC compilation does not break.
 */
import fs from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const __dirname = path.dirname(fileURLToPath(import.meta.url))
const siteRoot = path.resolve(__dirname, '..')
const repoRoot = path.resolve(siteRoot, '..')
const docsRoot = path.join(siteRoot, 'docs')

const GITHUB_REPO = 'https://github.com/synvek/syx'
const GITHUB_BLOB = `${GITHUB_REPO}/blob/main/`
const GITHUB_TREE = `${GITHUB_REPO}/tree/main/`

function ensureDir(dir) {
  fs.mkdirSync(dir, { recursive: true })
}

/* -------------------------------------------------------------------------- */
/* Link rewriting                                                              */
/* -------------------------------------------------------------------------- */

/** docs/*.md files that have a dedicated page on the site. */
const DOC_PAGES = {
  'docs/9_UsageGuide.md': '/quick-guide/',
  'docs/8_QuickReference.md': '/quick-reference/',
  'docs/1_LexicalAnalysis.md': '/compiler/lexical-analysis',
  'docs/2_SyntaxAnalysis.md': '/compiler/syntax-analysis',
  'docs/3_SemanticAnalysis.md': '/compiler/semantic-analysis',
  'docs/4_InstructionSet.md': '/compiler/instruction-set',
  'docs/5_CompleteTechnicalDocumentation.md': '/compiler/technical-overview',
  'docs/10_BackendParity.md': '/compiler/backend-parity',
  'docs/11_NetworkStdlib.md': '/compiler/network-stdlib',
  'docs/12_BackendDebuggingGuide.md': '/compiler/backend-debugging',
  'docs/env-registry.md': '/compiler/env-registry',
  'docs/README.md': '/compiler/',
}

/** Files published under /about/; the source is shared by both locales. */
const ADR_PAGES = {
  'docs/adr/capabilities-summary.md': '/about/capabilities',
  'docs/adr/architecture-decisions.md': '/about/architecture',
}

const ZH_DOC_PAGES = {
  'docs/zh/9_UsageGuide.zh-CN.md': '/zh/quick-guide/',
  'docs/zh/8_QuickReference.zh-CN.md': '/zh/quick-reference/',
  'docs/zh/1_LexicalAnalysis.zh-CN.md': '/zh/compiler/lexical-analysis',
  'docs/zh/2_SyntaxAnalysis.zh-CN.md': '/zh/compiler/syntax-analysis',
  'docs/zh/3_SemanticAnalysis.zh-CN.md': '/zh/compiler/semantic-analysis',
  'docs/zh/4_InstructionSet.zh-CN.md': '/zh/compiler/instruction-set',
  'docs/zh/5_CompleteTechnicalDocumentation.zh-CN.md': '/zh/compiler/technical-overview',
  'docs/zh/10_BackendParity.zh-CN.md': '/zh/compiler/backend-parity',
  'docs/zh/11_NetworkStdlib.zh-CN.md': '/zh/compiler/network-stdlib',
  'docs/zh/12_BackendDebuggingGuide.zh-CN.md': '/zh/compiler/backend-debugging',
  'docs/zh/env-registry.zh-CN.md': '/zh/compiler/env-registry',
  'docs/zh/README.zh-CN.md': '/zh/compiler/',
}

const BOOK_DIRS = {
  'books/language-guide': { prefix: '', dir: 'guide' },
  'books/language-reference': { prefix: '', dir: 'reference' },
  'books/library-guide': { prefix: '', dir: 'library' },
  'books/zh-cn/language-guide': { prefix: '/zh', dir: 'guide' },
  'books/zh-cn/language-reference': { prefix: '/zh', dir: 'reference' },
  'books/zh-cn/library-guide': { prefix: '/zh', dir: 'library' },
}

const clean = (p) => p.replace(/\\/g, '/')

/** Does this repo-relative path exist as a file or directory? */
const repoPathKind = (repoPath) => {
  try {
    const st = fs.statSync(path.join(repoRoot, repoPath))
    return st.isDirectory() ? 'dir' : 'file'
  } catch {
    return null
  }
}

/** Map a repo-relative path to a public site URL, or null when unpublished. */
function siteUrlForRepoPath(repoPath, destLocale) {
  const p = clean(repoPath)

  if (DOC_PAGES[p]) return DOC_PAGES[p]
  if (ZH_DOC_PAGES[p]) return ZH_DOC_PAGES[p]
  if (ADR_PAGES[p]) return (destLocale === 'zh' ? '/zh' : '') + ADR_PAGES[p]

  for (const [dir, { prefix, dir: section }] of Object.entries(BOOK_DIRS)) {
    if (!p.startsWith(dir + '/')) continue
    const rest = p.slice(dir.length + 1)
    if (!rest.endsWith('.md')) return null
    // Only publish chapters that really exist; renamed/removed chapters fall
    // through to the GitHub fallback instead of producing a dead site link.
    if (repoPathKind(p) !== 'file') return null
    const name = rest.slice(0, -3)
    if (name === 'SUMMARY' || name === 'README') return `${prefix}/${section}/`
    return `${prefix}/${section}/${name}`
  }

  return null
}

/** Fallback target on GitHub for repository files that are not published. */
function githubUrlForRepoPath(repoPath) {
  const kind = repoPathKind(repoPath)
  if (!kind) return null
  const p = clean(repoPath).replace(/\/+$/, '')
  return (kind === 'dir' ? GITHUB_TREE : GITHUB_BLOB) + p
}

/**
 * Rewrite one link target from the source file at `srcRel` (repo-relative).
 * Returns the replacement target, or null to unwrap the link into plain text.
 */
function rewriteTarget(target, srcRel, destLocale) {
  if (!target || /^(https?:|mailto:|tel:|#|\/\/)/i.test(target)) return target

  const [bare, hash] = target.split('#')
  if (!bare) return target // pure in-page anchor

  const suffix = hash ? `#${hash}` : ''
  if (bare.startsWith('/')) return target // already a site path

  const abs = clean(path.posix.normalize(path.posix.join(path.posix.dirname(srcRel), bare)))

  if (/\.md$/i.test(abs)) {
    const siteUrl = siteUrlForRepoPath(abs, destLocale)
    if (siteUrl) return siteUrl + suffix
    const gh = githubUrlForRepoPath(abs)
    return gh ? gh + suffix : null
  }

  // Non-markdown targets (source files, directories, LICENSE, …)
  const gh = githubUrlForRepoPath(abs)
  if (gh) return gh + suffix
  return null
}

/** Rewrite every markdown link/image, leaving fenced and inline code untouched. */
function rewriteLinks(src, srcRel, destLocale) {
  const fences = []
  let text = src.replace(/```[\s\S]*?```/g, (m) => {
    fences.push(m)
    return `\u0000FENCE${fences.length - 1}\u0000`
  })
  const inlines = []
  text = text.replace(/`[^`\n]+`/g, (m) => {
    inlines.push(m)
    return `\u0000INLINE${inlines.length - 1}\u0000`
  })

  text = text.replace(
    /(!?)\[([^\]]*)\]\(\s*([^\s)]+)(\s+"[^"]*")?\s*\)/g,
    (_match, bang, label, target, title) => {
      const replaced = rewriteTarget(target, srcRel, destLocale)
      if (replaced === null) return bang ? '' : label // unwrap: keep text, drop the link
      return `${bang}[${label}](${replaced}${title ? ' ' + title : ''})`
    },
  )

  text = text.replace(/\u0000INLINE(\d+)\u0000/g, (_, i) => inlines[Number(i)])
  text = text.replace(/\u0000FENCE(\d+)\u0000/g, (_, i) => fences[Number(i)])
  return text
}

/* -------------------------------------------------------------------------- */
/* Frontmatter derivation                                                      */
/* -------------------------------------------------------------------------- */

const LANG_ALIASES = {
  sy: 'swift',
  syx: 'swift',
  syb: 'swift',
  bnf: 'text',
  plaintext: 'text',
}

function rewriteFenceLang(fence) {
  return fence.replace(/^```(\w+)/, (_, lang) => {
    const mapped = LANG_ALIASES[lang.toLowerCase()]
    return mapped ? '```' + mapped : '```' + lang
  })
}

const stripInline = (s) =>
  s
    .replace(/!\[[^\]]*\]\([^)]*\)/g, '')
    .replace(/\[([^\]]*)\]\([^)]*\)/g, '$1')
    .replace(/[*_`~]/g, '')
    .replace(/<[^>]+>/g, '')
    .replace(/\s+/g, ' ')
    .trim()

/** Strip a leading YAML frontmatter block. */
const stripFrontmatter = (md) => md.replace(/^---\r?\n[\s\S]*?\r?\n---\r?\n?/, '')

/** First H1 of the document, used as the page title. */
function extractTitle(md) {
  const m = stripFrontmatter(md).match(/^#\s+(.+)$/m)
  return m ? stripInline(m[1]) : ''
}

/** Boilerplate paragraphs that make poor meta descriptions. */
function isBoilerplate(block) {
  // "**Implementation status:** **Implemented** …", "**实现状态：** …"
  if (/^\*\*[^*]{1,40}[:：]/.test(block)) return true
  if (/^\*\*[^*]{1,60}\*\*\s*$/.test(block)) return true
  if (/^(implementation status|this edition covers|实现状态|本版覆盖)/i.test(stripInline(block))) return true
  return false
}

/** Prose paragraphs that are safe to quote in a meta description. */
function proseBlocks(md) {
  const body = stripFrontmatter(md).replace(/^(```|~~~)[\s\S]*?^\1.*$/gm, '')
  const out = []
  for (const raw of body.split(/\n{2,}/)) {
    if (!raw.trim()) continue
    for (const part of raw.split(/\n(?=#{1,6}\s)/)) {
      const block = part.replace(/^#{1,6}[^\n]*\n?/, '').trim()
      if (!block) continue
      if (/^#{1,6}\s/.test(block)) continue
      if (/^[>|]/.test(block)) continue
      if (/^[-*+]\s|^\d+\.\s/.test(block)) continue
      if (/^(---+|\*\*\*+)$/.test(block)) continue
      // A paragraph ending in a colon introduces a code sample or list.
      if (/[:：]\s*$/.test(block)) continue
      if (isBoilerplate(block)) continue
      const plain = stripInline(block)
      if (widthOf(plain) < 30) continue
      out.push(plain)
    }
  }
  return out
}

/**
 * First substantial prose, extended with following paragraphs while the result
 * is too short to be a useful snippet.
 */
function extractDescription(md) {
  const blocks = proseBlocks(md)
  if (blocks.length === 0) return ''
  let text = blocks[0]
  for (const next of blocks.slice(1, 4)) {
    if (widthOf(text) >= 110) break
    text = `${text} ${next}`
  }
  return clipWidth(text, 158)
}

/** True when a string contains CJK characters. */
const hasCJK = (s) => /[\u3400-\u4dbf\u4e00-\u9fff]/.test(s)

/** Width in "Latin character" units: one CJK glyph counts as two. */
const widthOf = (s) => [...s].reduce((n, ch) => n + (hasCJK(ch) ? 2 : 1), 0)

/** Clip to a display width, appending an ellipsis when truncated. */
function clipWidth(text, max = 158) {
  const value = String(text || '').replace(/\s+/g, ' ').trim()
  if (widthOf(value) <= max) return value
  let out = ''
  let used = 0
  for (const ch of value) {
    const w = hasCJK(ch) ? 2 : 1
    if (used + w > max - 1) break
    out += ch
    used += w
  }
  const space = out.lastIndexOf(' ')
  if (space > out.length * 0.6) out = out.slice(0, space)
  return out.replace(/[,.;:\-–—，。；：、]$/, '') + '…'
}

const yamlString = (s) => JSON.stringify(s.replace(/[\u0000-\u001f]/g, ' '))

/**
 * VitePress injects frontmatter descriptions into `content="…"` without
 * escaping, so an ASCII double quote would truncate the attribute. Normalize
 * prose punctuation to typographic forms and drop angle brackets (which would
 * otherwise be re-escaped by the Vue-safety pass).
 */
const htmlSafe = (s) =>
  s
    .replace(/\\"/g, '"')
    .replace(/"/g, '\u201d')
    .replace(/'/g, '\u2019')
    .replace(/[<>]/g, '')
    .replace(/\s+/g, ' ')
    .trim()

function withFrontmatter(md) {
  const hasFrontmatter = /^---\r?\n[\s\S]*?\r?\n---\r?\n/.test(md)
  if (hasFrontmatter) return md

  const title = extractTitle(md)
  const description = htmlSafe(extractDescription(md))
  const lines = ['---']
  if (title) lines.push(`title: ${yamlString(title)}`)
  if (description) lines.push(`description: ${yamlString(description)}`)
  lines.push('---', '')
  if (lines.length === 3) return md
  return lines.join('\n') + md
}

/* -------------------------------------------------------------------------- */
/* Vue safety                                                                  */
/* -------------------------------------------------------------------------- */

/**
 * Escape <Ident...> outside fenced/inline code so Vue does not treat them as tags.
 */
function sanitizeMarkdown(src) {
  const fences = []
  let text = src.replace(/```[\s\S]*?```/g, (m) => {
    const i = fences.length
    fences.push(rewriteFenceLang(m))
    return `\u0000FENCE${i}\u0000`
  })

  const inlines = []
  text = text.replace(/`[^`\n]+`/g, (m) => {
    const i = inlines.length
    inlines.push(m)
    return `\u0000INLINE${i}\u0000`
  })

  // Bare generics / HTML-like identifiers: Array<String>, <T>, </foo>
  text = text.replace(/<\/?[A-Za-z_][A-Za-z0-9_.:-]*/g, (m) =>
    m.replaceAll('<', '&lt;').replaceAll('>', '&gt;'),
  )
  // Closing remainder of tags that started above, e.g. String> from Array<String>
  text = text.replace(/&lt;([A-Za-z_][A-Za-z0-9_.:\s,&|]*)>/g, '&lt;$1&gt;')

  text = text.replace(/\u0000INLINE(\d+)\u0000/g, (_, i) => inlines[Number(i)])
  text = text.replace(/\u0000FENCE(\d+)\u0000/g, (_, i) => fences[Number(i)])
  return text
}

const localeOfDest = (destAbs) => {
  const rel = clean(path.relative(docsRoot, destAbs))
  return rel === 'zh' || rel.startsWith('zh/') ? 'zh' : 'root'
}

function writeSynced(srcAbs, destAbs) {
  if (!fs.existsSync(srcAbs)) {
    console.warn(`[sync] missing source: ${srcAbs}`)
    return false
  }
  ensureDir(path.dirname(destAbs))
  try {
    fs.lstatSync(destAbs)
    fs.unlinkSync(destAbs)
  } catch {
    /* dest does not exist */
  }
  const srcRel = clean(path.relative(repoRoot, srcAbs))
  const destLocale = localeOfDest(destAbs)
  const raw = fs.readFileSync(srcAbs, 'utf8')
  const linked = rewriteLinks(raw, srcRel, destLocale)
  fs.writeFileSync(destAbs, sanitizeMarkdown(withFrontmatter(linked)), 'utf8')
  return true
}

function syncDirMarkdown(srcDir, destDir, { exclude = [] } = {}) {
  if (!fs.existsSync(srcDir)) {
    console.warn(`[sync] skip missing: ${srcDir}`)
    return 0
  }
  ensureDir(destDir)
  let n = 0
  for (const name of fs.readdirSync(srcDir)) {
    if (!name.endsWith('.md')) continue
    if (exclude.includes(name)) continue
    if (name === 'SUMMARY.md') continue
    if (writeSynced(path.join(srcDir, name), path.join(destDir, name))) n++
  }
  return n
}

function clearSyncedDirs(dirs) {
  for (const dir of dirs) {
    const abs = path.join(docsRoot, dir)
    if (!fs.existsSync(abs)) continue
    for (const name of fs.readdirSync(abs)) {
      if (!name.endsWith('.md')) continue
      if (name === 'index.md') continue
      const p = path.join(abs, name)
      try {
        fs.unlinkSync(p)
      } catch {
        /* ignore */
      }
    }
  }
}

const bookExclude = []

clearSyncedDirs([
  'guide',
  'reference',
  'library',
  'compiler',
  'about',
  'quick-guide',
  'quick-reference',
  'zh/guide',
  'zh/reference',
  'zh/library',
  'zh/compiler',
  'zh/about',
  'zh/quick-guide',
  'zh/quick-reference',
])

const counts = {}

counts.guide = syncDirMarkdown(
  path.join(repoRoot, 'books/language-guide'),
  path.join(docsRoot, 'guide'),
  { exclude: bookExclude },
)
counts.reference = syncDirMarkdown(
  path.join(repoRoot, 'books/language-reference'),
  path.join(docsRoot, 'reference'),
  { exclude: bookExclude },
)
counts.library = syncDirMarkdown(
  path.join(repoRoot, 'books/library-guide'),
  path.join(docsRoot, 'library'),
  { exclude: bookExclude },
)

counts.zhGuide = syncDirMarkdown(
  path.join(repoRoot, 'books/zh-cn/language-guide'),
  path.join(docsRoot, 'zh/guide'),
  { exclude: bookExclude },
)
counts.zhReference = syncDirMarkdown(
  path.join(repoRoot, 'books/zh-cn/language-reference'),
  path.join(docsRoot, 'zh/reference'),
  { exclude: bookExclude },
)
counts.zhLibrary = syncDirMarkdown(
  path.join(repoRoot, 'books/zh-cn/library-guide'),
  path.join(docsRoot, 'zh/library'),
  { exclude: bookExclude },
)

// Standalone top-level pages (Usage Guide → quick-guide, Quick Reference → quick-reference)
const quickPages = [
  ['quick-guide/index.md', 'docs/9_UsageGuide.md'],
  ['quick-reference/index.md', 'docs/8_QuickReference.md'],
  ['zh/quick-guide/index.md', 'docs/zh/9_UsageGuide.zh-CN.md'],
  ['zh/quick-reference/index.md', 'docs/zh/8_QuickReference.zh-CN.md'],
]
counts.quick = 0
for (const [dest, src] of quickPages) {
  if (writeSynced(path.join(repoRoot, src), path.join(docsRoot, dest))) counts.quick++
}

const compilerEn = {
  'technical-overview.md': 'docs/5_CompleteTechnicalDocumentation.md',
  'lexical-analysis.md': 'docs/1_LexicalAnalysis.md',
  'syntax-analysis.md': 'docs/2_SyntaxAnalysis.md',
  'semantic-analysis.md': 'docs/3_SemanticAnalysis.md',
  'instruction-set.md': 'docs/4_InstructionSet.md',
  'backend-parity.md': 'docs/10_BackendParity.md',
  'network-stdlib.md': 'docs/11_NetworkStdlib.md',
  'backend-debugging.md': 'docs/12_BackendDebuggingGuide.md',
  'env-registry.md': 'docs/env-registry.md',
}
const compilerZh = {
  'lexical-analysis.md': 'docs/zh/1_LexicalAnalysis.zh-CN.md',
  'syntax-analysis.md': 'docs/zh/2_SyntaxAnalysis.zh-CN.md',
  'semantic-analysis.md': 'docs/zh/3_SemanticAnalysis.zh-CN.md',
  'instruction-set.md': 'docs/zh/4_InstructionSet.zh-CN.md',
  'technical-overview.md': 'docs/zh/5_CompleteTechnicalDocumentation.zh-CN.md',
  'backend-parity.md': 'docs/zh/10_BackendParity.zh-CN.md',
  'network-stdlib.md': 'docs/zh/11_NetworkStdlib.zh-CN.md',
  'backend-debugging.md': 'docs/zh/12_BackendDebuggingGuide.zh-CN.md',
  'env-registry.md': 'docs/zh/env-registry.zh-CN.md',
}

counts.compiler = 0
for (const [dest, src] of Object.entries(compilerEn)) {
  if (writeSynced(path.join(repoRoot, src), path.join(docsRoot, 'compiler', dest)))
    counts.compiler++
}
counts.zhCompiler = 0
for (const [dest, src] of Object.entries(compilerZh)) {
  if (writeSynced(path.join(repoRoot, src), path.join(docsRoot, 'zh/compiler', dest)))
    counts.zhCompiler++
}

const aboutEn = {
  'capabilities.md': 'docs/adr/capabilities-summary.md',
  'architecture.md': 'docs/adr/architecture-decisions.md',
}
const aboutZh = aboutEn

counts.about = 0
for (const [dest, src] of Object.entries(aboutEn)) {
  if (writeSynced(path.join(repoRoot, src), path.join(docsRoot, 'about', dest)))
    counts.about++
}
counts.zhAbout = 0
for (const [dest, src] of Object.entries(aboutZh)) {
  if (writeSynced(path.join(repoRoot, src), path.join(docsRoot, 'zh/about', dest)))
    counts.zhAbout++
}

console.log('[sync] synced content:', counts)
