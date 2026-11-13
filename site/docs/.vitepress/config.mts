import { defineConfig } from 'vitepress'
import type { HeadConfig, TransformContext } from 'vitepress'
import {
  aboutSidebarEn,
  aboutSidebarZh,
  compilerSidebarEn,
  compilerSidebarZh,
  editorsSidebarEn,
  editorsSidebarZh,
  guideSidebarEn,
  guideSidebarZh,
  librarySidebarEn,
  librarySidebarZh,
  referenceSidebarEn,
  referenceSidebarZh,
} from './sidebars'
import {
  AUTHOR,
  BAIDU_VERIFICATION,
  BING_VERIFICATION,
  GITHUB_URL,
  GOOGLE_SITE_VERIFICATION,
  LICENSE_URL,
  OG_IMAGE,
  OG_IMAGE_ALT,
  OG_IMAGE_HEIGHT,
  OG_IMAGE_WIDTH,
  REPO_NAME,
  SITE_NAME,
  SITE_URL,
  abs,
  alternateOf,
  breadcrumbList,
  clip,
  composeTitle,
  faqPage,
  localeOf,
  sectionOf,
  softwareApplication,
  urlPathOf,
  websiteSearchAction,
} from './seo'

const navEn = [
  { text: 'Quick Guide', link: '/quick-guide/' },
  { text: 'Quick Reference', link: '/quick-reference/' },
  { text: 'Editors', link: '/editors/' },
  { text: 'Guide', link: '/guide/00-preface' },
  { text: 'Reference', link: '/reference/00-about-this-reference' },
  { text: 'Library', link: '/library/00-preface' },
  { text: 'Compiler', link: '/compiler/' },
  { text: 'About', link: '/about/' },
]

const navZh = [
  { text: '快速指南', link: '/zh/quick-guide/' },
  { text: '语法速查', link: '/zh/quick-reference/' },
  { text: '编辑器', link: '/zh/editors/' },
  { text: '语言指南', link: '/zh/guide/00-preface' },
  { text: '语言参考', link: '/zh/reference/00-about-this-reference' },
  { text: '标准库', link: '/zh/library/00-preface' },
  { text: '编译器', link: '/zh/compiler/' },
  { text: '关于', link: '/zh/about/' },
]

/** Keyword sets per section, used to enrich WebPage entities for AI/GEO. */
const SECTION_KEYWORDS: Record<string, string[]> = {
  'quick-guide': [
    'SYX install',
    'SYX CLI',
    'syx build',
    'syx run',
    'syx.toml',
    'LLVM native AOT',
    'WebAssembly',
  ],
  'quick-reference': ['SYX syntax', 'SYX cheat sheet', 'syx standard library', 'SYX operators'],
  guide: ['learn SYX', 'Swift-like language', 'SYX tutorial', 'SYX book', 'protocols', 'generics'],
  reference: ['SYX grammar', 'SYX lexical structure', 'SYX type system', 'SYX declarations'],
  library: ['SYX standard library', 'import Math', 'import Networking', 'SYX Crypto', 'SYX Logging'],
  compiler: [
    'SYX compiler',
    'Rust compiler',
    'lexer',
    'parser',
    'MIR',
    'bytecode VM',
    'LLVM backend',
    'Wasm backend',
  ],
  editors: ['SYX VS Code extension', 'SYX LSP', 'TextMate grammar', 'DAP debugging'],
  about: [
    'what is SYX',
    'SYX vs Swift',
    'install SYX',
    '.sy file extension',
    'SYX backends',
    'SYX capabilities',
    'SYX architecture',
    'Apache-2.0 language',
  ],
}

const BASE_KEYWORDS = [
  'SYX',
  'SYX language',
  'SYX programming language',
  'Swift-inspired language',
  'Rust-implemented compiler',
  '.sy file',
  'syx.toml',
  'LLVM',
  'WebAssembly',
  'VM',
  'programming language',
]

function keywordsFor(relativePath: string, locale: 'root' | 'zh') {
  const section = sectionOf(relativePath)
  const extra = section ? SECTION_KEYWORDS[section.segment] ?? [] : []
  const list = [...new Set([...BASE_KEYWORDS, ...extra])]
  if (locale === 'zh') return [...list, 'SYX 编程语言', '受 Swift 启发', 'Rust 编译器']
  return list
}

/** Organization + WebSite entities, emitted on every page for entity grounding. */
function siteEntities(locale: 'root' | 'zh') {
  return [
    {
      '@type': 'Organization',
      '@id': `${SITE_URL}/#organization`,
      name: AUTHOR,
      url: `${SITE_URL}/`,
      logo: {
        '@type': 'ImageObject',
        url: abs('/logo.svg'),
        width: 256,
        height: 256,
      },
      sameAs: [GITHUB_URL, `${SITE_URL}/`, `${SITE_URL}/llms.txt`],
    },
    {
      '@type': 'WebSite',
      '@id': `${SITE_URL}/#website`,
      url: `${SITE_URL}/`,
      name: SITE_NAME,
      alternateName: locale === 'zh' ? 'SYX 编程语言' : 'SYX programming language',
      description:
        locale === 'zh'
          ? 'SYX 是一门受 Swift 启发的编程语言，配套 Rust 实现的编译器工具链（VM、LLVM 原生 AOT、Wasm）。'
          : 'SYX is a Swift-inspired programming language with a Rust-implemented compiler toolchain (VM, LLVM native AOT, and Wasm).',
      publisher: { '@id': `${SITE_URL}/#organization` },
      inLanguage: locale === 'zh' ? 'zh-CN' : 'en-US',
      license: LICENSE_URL,
      potentialAction: websiteSearchAction(locale),
    },
  ]
}

/**
 * Per-page head: canonical URL, hreflang alternates, Open Graph / Twitter
 * cards, and JSON-LD (TechArticle + BreadcrumbList + site entities).
 */
const LOCALE_FALLBACK_DESCRIPTION: Record<'root' | 'zh', string> = {
  root: 'SYX documentation — a Swift-inspired programming language with a Rust-implemented compiler toolchain: stack VM, LLVM native AOT and Wasm backends.',
  zh: 'SYX 文档 —— 一门受 Swift 启发、由 Rust 实现编译器工具链的编程语言：栈式 VM、LLVM 原生 AOT 与 Wasm 三条后端。',
}

function seoHead(ctx: TransformContext): HeadConfig[] {
  const { pageData, page } = ctx
  if (pageData.isNotFound || page === '404.md') {
    return [['meta', { name: 'robots', content: 'noindex, follow' }]]
  }

  const relativePath = pageData.relativePath || page || ''
  const locale = localeOf(relativePath)
  const urlPath = urlPathOf(relativePath)
  const canonical = abs(urlPath)
  const other = alternateOf(urlPath)
  const isHome = urlPath === '/' || urlPath === '/zh/'

  // `title` already carries the section + brand composition applied in
  // transformPageData (see composeTitle), so og:/twitter: match <title>.
  const title = ctx.title || pageData.title || SITE_NAME
  const heading = pageData.frontmatter?.title || pageData.title || SITE_NAME
  const section = sectionOf(relativePath)
  // VitePress emits <meta name="description"> from this same value, so the
  // OG/Twitter descriptions stay identical instead of being clipped twice.
  const description = pageData.description || ctx.description || LOCALE_FALLBACK_DESCRIPTION[locale]
  const keywords = keywordsFor(relativePath, locale).join(', ')
  const imageUrl = abs(OG_IMAGE)
  const localeTag = locale === 'zh' ? 'zh_CN' : 'en_US'
  const alternateTag = locale === 'zh' ? 'en_US' : 'zh_CN'

  const head: HeadConfig[] = [
    ['link', { rel: 'canonical', href: canonical }],
    ['link', { rel: 'alternate', hreflang: locale === 'zh' ? 'zh-CN' : 'en', href: canonical }],
    ['link', { rel: 'alternate', hreflang: locale === 'zh' ? 'en' : 'zh-CN', href: abs(other) }],
    ['link', { rel: 'alternate', hreflang: 'x-default', href: locale === 'zh' ? abs(other) : canonical }],
    ['meta', { name: 'robots', content: 'index, follow, max-image-preview:large, max-snippet:-1' }],
    ['meta', { name: 'keywords', content: keywords }],
    ['meta', { name: 'author', content: AUTHOR }],
    ['meta', { property: 'og:site_name', content: SITE_NAME }],
    ['meta', { property: 'og:type', content: urlPath === '/' || urlPath === '/zh/' ? 'website' : 'article' }],
    ['meta', { property: 'og:title', content: title }],
    ['meta', { property: 'og:description', content: description }],
    ['meta', { property: 'og:url', content: canonical }],
    ['meta', { property: 'og:image', content: imageUrl }],
    ['meta', { property: 'og:image:width', content: String(OG_IMAGE_WIDTH) }],
    ['meta', { property: 'og:image:height', content: String(OG_IMAGE_HEIGHT) }],
    ['meta', { property: 'og:image:alt', content: OG_IMAGE_ALT }],
    ['meta', { property: 'og:locale', content: localeTag }],
    ['meta', { property: 'og:locale:alternate', content: alternateTag }],
    ['meta', { name: 'twitter:card', content: 'summary_large_image' }],
    ['meta', { name: 'twitter:title', content: title }],
    ['meta', { name: 'twitter:description', content: description }],
    ['meta', { name: 'twitter:image', content: imageUrl }],
    ['meta', { name: 'twitter:image:alt', content: OG_IMAGE_ALT }],
  ]

  const article = {
    '@type': 'TechArticle',
    '@id': `${canonical}#article`,
    headline: clip(String(heading), 110),
    description,
    url: canonical,
    inLanguage: locale === 'zh' ? 'zh-CN' : 'en-US',
    isPartOf: { '@id': `${SITE_URL}/#website` },
    publisher: { '@id': `${SITE_URL}/#organization` },
    author: { '@id': `${SITE_URL}/#organization` },
    image: imageUrl,
    license: LICENSE_URL,
    ...(section
      ? { articleSection: locale === 'zh' ? section.zh : section.en, keywords }
      : { keywords }),
    ...(pageData.lastUpdated
      ? { dateModified: new Date(pageData.lastUpdated).toISOString().slice(0, 10) }
      : {}),
  }

  const graph: unknown[] = [...siteEntities(locale)]
  if (isHome) {
    graph.push(softwareApplication(locale), faqPage(locale))
  } else {
    graph.push(article)
    if (section) graph.push(breadcrumbList(relativePath, urlPath, locale, String(heading)))
  }

  head.push([
    'script',
    { type: 'application/ld+json' },
    JSON.stringify({ '@context': 'https://schema.org', '@graph': graph }),
  ])

  return head
}

export default defineConfig({
  title: 'SYX',
  description:
    'SYX is a Swift-inspired programming language with a Rust-implemented compiler toolchain — VM, LLVM native AOT, and Wasm backends. Source files use the .sy extension.',
  lang: 'en-US',
  cleanUrls: true,
  lastUpdated: true,
  // Prefer failing the build on dead markdown links; CI also runs check:links
  // over the rendered HTML as a second gate.
  ignoreDeadLinks: false,
  metaChunk: true,

  // Titles are composed per page from the frontmatter `title` plus the section
  // name, so chapters that share a heading ("Preface", "System") stay unique.
  titleTemplate: ':title',

  /**
   * Give each page a unique, section-qualified `title` before VitePress builds
   * the <title> tag from it. The frontmatter heading is preserved for other
   * consumers (breadcrumbs, JSON-LD headline).
   */
  transformPageData(pageData) {
    const relativePath = pageData.relativePath || ''
    const locale = localeOf(relativePath)
    const heading = pageData.frontmatter?.title || pageData.title
    pageData.title = composeTitle(relativePath, heading, locale)
  },

  sitemap: {
    hostname: SITE_URL,
    transformItems: (items) => {
      // At transform time URLs are still site-relative ("about/x" or "/about/x").
      const rel = (url: string) => {
        const stripped = url.startsWith(SITE_URL) ? url.slice(SITE_URL.length) : url
        return '/' + stripped.replace(/^\/+/, '')
      }

      return items.map((item) => {
        const path = rel(item.url)
        // x-default points at the English variant of the same page.
        const englishPath = path === '/zh/' || path === '/zh' ? '/' : path.replace(/^\/zh\//, '/')
        const links = [
          ...(item.links ?? []).map((link) => ({ ...link, url: rel(link.url) })),
          { url: englishPath, lang: 'x-default' },
        ]

        const isHome = path === '/' || path === '/zh/' || path === '/zh'
        const isSection = /^(\/zh)?\/[a-z-]+\/$/.test(path)
        return {
          ...item,
          url: path,
          links,
          changefreq: 'weekly' as const,
          priority: isHome ? 1.0 : isSection ? 0.8 : 0.6,
        }
      })
    },
  },

  markdown: {
    theme: { light: 'github-light', dark: 'github-dark' },
    // Books/docs use bare generics like Array<String> in tables; keep them as text.
    config(md) {
      md.set({ html: false })
    },
    languageAlias: {
      sy: 'swift',
      syx: 'swift',
      syb: 'swift',
      bnf: 'txt',
    },
  },

  head: (() => {
    const tags: HeadConfig[] = [
      ['meta', { name: 'theme-color', content: '#0f766e' }],
      ['link', { rel: 'icon', type: 'image/svg+xml', href: '/logo.svg' }],
      ['link', { rel: 'apple-touch-icon', href: '/logo.svg' }],
      ['link', { rel: 'sitemap', type: 'application/xml', href: '/sitemap.xml' }],
      ['link', { rel: 'author', href: GITHUB_URL }],
      ['meta', { property: 'og:image', content: `${SITE_URL}${OG_IMAGE}` }],
    ]
    // Search-engine ownership proofs. Emitted site-wide so a homepage rewrite
    // cannot silently drop verification; empty codes are skipped.
    if (BAIDU_VERIFICATION) {
      tags.push(['meta', { name: 'baidu-site-verification', content: BAIDU_VERIFICATION }])
    }
    if (BING_VERIFICATION) {
      tags.push(['meta', { name: 'msvalidate.01', content: BING_VERIFICATION }])
    }
    if (GOOGLE_SITE_VERIFICATION) {
      tags.push(['meta', { name: 'google-site-verification', content: GOOGLE_SITE_VERIFICATION }])
    }
    return tags
  })(),

  transformHead: seoHead,

  themeConfig: {
    logo: '/logo.svg',
    siteTitle: 'SYX',
    socialLinks: [{ icon: 'github', link: GITHUB_URL }],
    search: {
      provider: 'local',
      options: {
        locales: {
          zh: {
            translations: {
              button: { buttonText: '搜索', buttonAriaLabel: '搜索' },
              modal: {
                noResultsText: '没有结果',
                resetButtonTitle: '清除',
                footer: {
                  selectText: '选择',
                  navigateText: '导航',
                  closeText: '关闭',
                },
              },
            },
          },
        },
      },
    },
  },

  locales: {
    root: {
      label: 'English',
      lang: 'en-US',
      title: 'SYX',
      description:
        'Swift-inspired programming language with a Rust toolchain — VM, LLVM native AOT, and Wasm backends.',
      themeConfig: {
        nav: navEn,
        sidebar: {
          '/editors/': editorsSidebarEn,
          '/guide/': guideSidebarEn,
          '/reference/': referenceSidebarEn,
          '/library/': librarySidebarEn,
          '/compiler/': compilerSidebarEn,
          '/about/': aboutSidebarEn,
        },
        outline: { label: 'On this page', level: [2, 3] },
        docFooter: { prev: 'Previous', next: 'Next' },
        lastUpdated: { text: 'Last updated' },
        returnToTopLabel: 'Return to top',
        sidebarMenuLabel: 'Menu',
        darkModeSwitchLabel: 'Theme',
        footer: {
          message: 'SYX · Apache-2.0',
          copyright: 'Documentation sourced from books/ and docs/',
        },
      },
    },
    zh: {
      label: '简体中文',
      lang: 'zh-CN',
      title: 'SYX',
      description: '受 Swift 启发的编程语言 · Rust 编译器工具链 · VM / LLVM / Wasm',
      link: '/zh/',
      themeConfig: {
        nav: navZh,
        sidebar: {
          '/zh/editors/': editorsSidebarZh,
          '/zh/guide/': guideSidebarZh,
          '/zh/reference/': referenceSidebarZh,
          '/zh/library/': librarySidebarZh,
          '/zh/compiler/': compilerSidebarZh,
          '/zh/about/': aboutSidebarZh,
        },
        outline: { label: '本页目录', level: [2, 3] },
        docFooter: { prev: '上一页', next: '下一页' },
        lastUpdated: { text: '最后更新' },
        returnToTopLabel: '回到顶部',
        sidebarMenuLabel: '菜单',
        darkModeSwitchLabel: '主题',
        footer: {
          message: 'SYX · Apache-2.0',
          copyright: '内容来源于 books/ 与 docs/',
        },
      },
    },
  },
})
