/**
 * Shared SEO / GEO constants and helpers.
 *
 * Single source of truth for the production origin, locale mapping, section
 * naming and the JSON-LD payloads injected on every page.
 *
 * Override the origin per environment with `SITE_URL` (no trailing slash), e.g.
 * `SITE_URL=https://www.syx-lang.org npm run build`.
 */
import { readFileSync } from 'node:fs'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const __seoDir = dirname(fileURLToPath(import.meta.url))

/** Public crate / release semver — never the retired marketing “117.0” banner. */
function readSoftwareVersion(): string {
  if (process.env.SYX_VERSION) return process.env.SYX_VERSION.trim()
  try {
    const cargo = readFileSync(resolve(__seoDir, '../../../Cargo.toml'), 'utf8')
    const m = cargo.match(/^version\s*=\s*"([^"]+)"/m)
    if (m) return m[1]
  } catch {
    /* fall through */
  }
  return '0.2.1'
}

export const SITE_URL = (process.env.SITE_URL || 'https://www.syx-lang.org').replace(/\/+$/, '')
export const SITE_NAME = 'SYX'
export const GITHUB_URL = 'https://github.com/synvek/syx'
export const REPO_NAME = 'synvek/syx'
export const OG_IMAGE = '/og-image.png'
export const OG_IMAGE_WIDTH = 1200
export const OG_IMAGE_HEIGHT = 630
export const OG_IMAGE_ALT =
  'SYX language card — Swift-inspired syntax, Rust toolchain, VM / LLVM / Wasm backends'
export const AUTHOR = 'SYX Language Project'
export const LICENSE_URL = 'https://www.apache.org/licenses/LICENSE-2.0'
export const SOFTWARE_VERSION = readSoftwareVersion()

/**
 * Search-engine ownership proofs, emitted as <meta> tags in `head`.
 *
 * Baidu Search Resource Platform (百度搜索资源平台) HTML-tag verification.
 * Baidu fetches the site root, so the tag ships on every page — that way a
 * homepage rewrite cannot silently drop the verification.
 * Override with `BAIDU_VERIFICATION` if the code is ever reissued.
 */
export const BAIDU_VERIFICATION = process.env.BAIDU_VERIFICATION || 'codeva-x5Cx3nLF6w'

/** Bing Webmaster Tools verification. Empty by default (tag then omitted). */
export const BING_VERIFICATION = process.env.BING_VERIFICATION || ''

/** Google Search Console HTML-tag verification. Empty → tag omitted. */
export const GOOGLE_SITE_VERIFICATION = process.env.GOOGLE_SITE_VERIFICATION || ''

/** Absolute URL helper. `path` must start with "/". */
export const abs = (path: string) => SITE_URL + path

/** Derive the locale of a docs-relative page path (e.g. "zh/guide/01-x.md"). */
export const localeOf = (relativePath: string) =>
  relativePath === 'zh' || relativePath.startsWith('zh/') ? 'zh' : 'root'

/** Docs-relative page path -> public URL path, matching `cleanUrls: true`. */
export const urlPathOf = (relativePath: string) => {
  if (!relativePath) return '/'
  const clean = relativePath.replace(/(^|\/)index\.md$/, '$1').replace(/\.md$/, '')
  return '/' + clean
}

/** Public URL path -> the same page in the other locale, for hreflang. */
export const alternateOf = (urlPath: string) => {
  if (urlPath === '/') return '/zh/'
  if (urlPath === '/zh' || urlPath === '/zh/') return '/'
  return urlPath.startsWith('/zh/') ? urlPath.slice(3) : '/zh' + urlPath
}

/** Trim to a display width search engines and LLM snippets render fully. */
export const clip = (text: string, max = 158) => {
  const value = String(text || '')
    .replace(/\s+/g, ' ')
    .trim()
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

/** True when a string contains CJK characters. */
export const hasCJK = (text: string) => /[\u3400-\u4dbf\u4e00-\u9fff]/.test(text)

/**
 * Character budget that treats one CJK glyph as roughly two Latin characters,
 * matching how search engines measure snippet width.
 */
export const widthOf = (text: string) =>
  [...text].reduce((n, ch) => n + (hasCJK(ch) ? 2 : 1), 0)

export interface SectionInfo {
  /** Path segment, e.g. "guide" → /guide/ */
  segment: string
  en: string
  zh: string
}

/** Section metadata keyed by the first path segment. */
export const SECTIONS: Record<string, SectionInfo> = {
  'quick-guide': { segment: 'quick-guide', en: 'Quick Guide', zh: '快速指南' },
  'quick-reference': { segment: 'quick-reference', en: 'Quick Reference', zh: '语法速查' },
  editors: { segment: 'editors', en: 'Editors', zh: '编辑器' },
  guide: { segment: 'guide', en: 'Language Guide', zh: '语言指南' },
  reference: { segment: 'reference', en: 'Language Reference', zh: '语言参考' },
  library: { segment: 'library', en: 'Standard Library', zh: '标准库' },
  compiler: { segment: 'compiler', en: 'Compiler Internals', zh: '编译器内部' },
  about: { segment: 'about', en: 'About', zh: '关于' },
}

/** Strip the locale prefix from a docs-relative path. */
export const bareOf = (relativePath: string) => relativePath.replace(/^zh\//, '')

/** Resolve the section of a docs-relative path, ignoring the locale prefix. */
export function sectionOf(relativePath: string): SectionInfo | null {
  const first = bareOf(relativePath).split('/')[0]
  return SECTIONS[first] ?? null
}

/** True for locale root and section landing pages (e.g. "/", "/guide/"). */
export function isHubPage(urlPath: string) {
  const bare = urlPath.replace(/^\/zh/, '')
  return bare === '/' || /^\/[a-z-]+\/$/.test(bare)
}

/**
 * Compose the document <title>.
 *
 * Adds the section name so chapters that share a heading ("Preface", "System")
 * stay unique within a locale, and appends the brand when the width budget
 * allows (one CJK glyph counts as two Latin characters).
 */
export function composeTitle(relativePath: string, rawTitle: string, locale: 'root' | 'zh') {
  const base = (rawTitle || SITE_NAME).trim()
  if (isHubPage(urlPathOf(relativePath))) {
    return base.includes(SITE_NAME) ? base : `${base} | ${SITE_NAME}`
  }
  const section = sectionOf(relativePath)
  const sectionName = section ? (locale === 'zh' ? section.zh : section.en) : ''
  const withSection =
    sectionName && !base.includes(sectionName) ? `${base} — ${sectionName}` : base
  const branded = withSection.includes(SITE_NAME) ? withSection : `${withSection} | ${SITE_NAME}`
  return widthOf(branded) <= 66 ? branded : base.includes(SITE_NAME) ? base : `${base} | ${SITE_NAME}`
}

/** Build a BreadcrumbList JSON-LD node from URL path segments. */
export function breadcrumbList(relativePath: string, urlPath: string, locale: 'root' | 'zh', label: string) {
  const prefix = locale === 'zh' ? '/zh' : ''
  const trail: Array<{ name: string; item: string }> = [
    { name: locale === 'zh' ? '首页' : 'Home', item: abs(prefix || '/') },
  ]
  const section = sectionOf(relativePath)
  if (section) {
    trail.push({
      name: locale === 'zh' ? section.zh : section.en,
      item: abs(`${prefix}/${section.segment}/`),
    })
  }
  const isSectionRoot = section && urlPath === `${prefix}/${section.segment}/`
  if (label && !isSectionRoot) trail.push({ name: label, item: abs(urlPath) })

  return {
    '@type': 'BreadcrumbList',
    itemListElement: trail.map((entry, index) => ({
      '@type': 'ListItem',
      position: index + 1,
      name: entry.name,
      item: entry.item,
    })),
  }
}

/** Definition-style Q&A mirrored from the homepage FAQ section. */
export interface QA {
  q: string
  a: string
}

export const HOME_FAQ: Record<'root' | 'zh', QA[]> = {
  root: [
    {
      q: 'Is SYX the same as Swift?',
      a: 'No. SYX borrows Swift surface syntax but is an independent language with its own semantics, standard library and toolchain. It is explicitly neither a subset nor a superset of Swift.',
    },
    {
      q: 'What runtime backends does SYX have?',
      a: 'Three, all fed by one MIR pipeline: a stack bytecode VM (syx run), LLVM-based native AOT compilation (syx build --native), and WebAssembly (syx build --wasm or syx run --wasm). LLVM is the only native AOT backend.',
    },
    {
      q: 'What file extensions and project layout does SYX use?',
      a: 'Source files end in .sy. A project declares itself with a syx.toml manifest containing fields such as name, version and main, and modules are pulled in with import.',
    },
    {
      q: 'Do I need Rust to use SYX?',
      a: 'Not to run it — releases ship prebuilt binaries. Building the toolchain from source requires Rust 1.85 or newer; the full native toolchain additionally needs a local LLVM installation.',
    },
    {
      q: 'How do I get editor support for SYX?',
      a: 'Install the SYX Language VS Code extension for TextMate highlighting plus LSP completion, hover, go-to-definition, inlay hints, format on save, and DAP debugging through syx lsp. Neovim and Helix configurations are also provided.',
    },
  ],
  zh: [
    {
      q: 'SYX 和 Swift 是一样的吗？',
      a: '不是。SYX 借用了 Swift 的表层语法，但是一门独立语言，拥有自己的语义、标准库与工具链。它明确既不是 Swift 的子集也不是超集。',
    },
    {
      q: 'SYX 有哪几条后端？',
      a: '三条，全部由同一条 MIR 流水线驱动：栈式字节码 VM（syx run）、基于 LLVM 的原生 AOT（syx build --native），以及 WebAssembly（syx build --wasm 或 syx run --wasm）。LLVM 是唯一的原生 AOT 后端。',
    },
    {
      q: 'SYX 的源文件后缀与项目结构是什么？',
      a: '源文件后缀为 .sy。项目通过根目录的 syx.toml 声明，包含 name、version、main 等字段；模块通过 import 引入。',
    },
    {
      q: '使用 SYX 必须先装 Rust 吗？',
      a: '运行不必 —— 发布版提供预编译二进制。从源码构建工具链需要 Rust 1.85 或更高版本；构建完整原生工具链还需要本地 LLVM。',
    },
    {
      q: '如何获得编辑器支持？',
      a: '安装 SYX Language VS Code 扩展即可获得 TextMate 语法高亮以及 LSP 补全、悬停、跳转定义、行内类型提示、保存格式化，并通过 syx lsp 支持 DAP 调试。仓库同时提供 Neovim 与 Helix 配置。',
    },
  ],
}

/** FAQPage node for the homepage, mirroring the visible FAQ section. */
export function faqPage(locale: 'root' | 'zh') {
  const urlPath = locale === 'zh' ? '/zh/' : '/'
  return {
    '@type': 'FAQPage',
    '@id': `${abs(urlPath)}#faq`,
    inLanguage: locale === 'zh' ? 'zh-CN' : 'en-US',
    mainEntity: HOME_FAQ[locale].map(({ q, a }) => ({
      '@type': 'Question',
      name: q,
      acceptedAnswer: { '@type': 'Answer', text: a },
    })),
  }
}

/** SoftwareApplication node describing the SYX toolchain. */
export function softwareApplication(locale: 'root' | 'zh') {
  const urlPath = locale === 'zh' ? '/zh/' : '/'
  return {
    '@type': 'SoftwareApplication',
    '@id': `${abs(urlPath)}#software`,
    name: SITE_NAME,
    applicationCategory: 'DeveloperApplication',
    applicationSubCategory: 'Programming language compiler toolchain',
    operatingSystem: 'Windows, macOS, Linux',
    softwareVersion: SOFTWARE_VERSION,
    programmingLanguage: 'Rust',
    downloadUrl: `${GITHUB_URL}/releases/latest`,
    codeRepository: GITHUB_URL,
    license: LICENSE_URL,
    description:
      locale === 'zh'
        ? 'SYX 编译器工具链：一条 MIR 流水线支持栈式 VM、LLVM 原生 AOT 与 Wasm 三条后端。'
        : 'The SYX compiler toolchain: one MIR pipeline feeding a stack VM, LLVM native AOT, and a Wasm backend.',
    offers: { '@type': 'Offer', price: '0', priceCurrency: 'USD' },
    publisher: { '@id': `${SITE_URL}/#organization` },
  }
}

/**
 * WebSite SearchAction — exposes a sitelinks-style search target.
 * VitePress local search is client-side; the template lands on the locale hub
 * with `?q=` so crawlers have a stable EntryPoint URL.
 */
export function websiteSearchAction(locale: 'root' | 'zh') {
  const hub = locale === 'zh' ? abs('/zh/') : `${SITE_URL}/`
  return {
    '@type': 'SearchAction',
    target: {
      '@type': 'EntryPoint',
      urlTemplate: `${hub}?q={search_term_string}`,
    },
    'query-input': 'required name=search_term_string',
  }
}
