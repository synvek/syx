#!/usr/bin/env node
/**
 * Copy valuable books/ + docs/ markdown into the VitePress tree.
 * Escapes bare <Type> generics so Vue SFC compilation does not break.
 */
import fs from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const __dirname = path.dirname(fileURLToPath(import.meta.url))
const siteRoot = path.resolve(__dirname, '..')
const repoRoot = path.resolve(siteRoot, '..')
const docsRoot = path.join(siteRoot, 'docs')

function ensureDir(dir) {
  fs.mkdirSync(dir, { recursive: true })
}

/**
 * Escape <Ident...> outside fenced/inline code so Vue does not treat them as tags.
 */
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
  // After replacing `<String`, leftover `>` that closes a type arg in prose tables:
  text = text.replace(
    /&lt;([A-Za-z_][A-Za-z0-9_.:\s,&|]*)>/g,
    '&lt;$1&gt;',
  )

  text = text.replace(/\u0000INLINE(\d+)\u0000/g, (_, i) => inlines[Number(i)])
  text = text.replace(/\u0000FENCE(\d+)\u0000/g, (_, i) => fences[Number(i)])
  return text
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
  const raw = fs.readFileSync(srcAbs, 'utf8')
  fs.writeFileSync(destAbs, sanitizeMarkdown(raw), 'utf8')
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
  'zh/guide',
  'zh/reference',
  'zh/library',
  'zh/compiler',
  'zh/about',
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
  'lexical-analysis.md': 'docs/1_LexicalAnalysis.md',
  'syntax-analysis.md': 'docs/2_SyntaxAnalysis.md',
  'semantic-analysis.md': 'docs/3_SemanticAnalysis.md',
  'instruction-set.md': 'docs/4_InstructionSet.md',
  'technical-overview.md': 'docs/5_CompleteTechnicalDocumentation.md',
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
const aboutZh = {
  'capabilities.md': 'docs/adr/capabilities-summary.md',
  'architecture.md': 'docs/adr/architecture-decisions.md',
}

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
