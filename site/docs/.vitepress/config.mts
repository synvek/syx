import { defineConfig } from 'vitepress'
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

export default defineConfig({
  title: 'SYX',
  description:
    'SYX is a Swift-inspired programming language with a Rust-implemented compiler toolchain (VM · LLVM · Wasm).',
  lang: 'en-US',
  cleanUrls: true,
  lastUpdated: true,
  ignoreDeadLinks: true,

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

  head: [
    ['meta', { name: 'theme-color', content: '#0f766e' }],
    ['meta', { property: 'og:type', content: 'website' }],
    ['meta', { property: 'og:title', content: 'SYX Language' }],
    [
      'meta',
      {
        property: 'og:description',
        content:
          'A Swift-inspired language with Rust toolchain — VM, LLVM native AOT, and Wasm.',
      },
    ],
  ],

  themeConfig: {
    logo: undefined,
    siteTitle: 'SYX',
    socialLinks: [
      { icon: 'github', link: 'https://github.com/synvek/syx' },
    ],
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
        'Swift-inspired language · Rust toolchain · VM / LLVM / Wasm',
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
      description: '受 Swift 启发的编程语言 · Rust 工具链 · VM / LLVM / Wasm',
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
