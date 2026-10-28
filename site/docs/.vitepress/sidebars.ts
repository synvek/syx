/** Shared sidebar definitions for EN (root) and ZH locales. */

export const guideSidebarEn = [
  {
    text: 'Getting Started',
    collapsed: false,
    items: [
      { text: 'Overview', link: '/guide/' },
      { text: 'Preface', link: '/guide/00-preface' },
      { text: 'Getting Started', link: '/guide/01-getting-started' },
      { text: 'The Basics', link: '/guide/02-the-basics' },
      { text: 'Basic Operators', link: '/guide/03-basic-operators' },
    ],
  },
  {
    text: 'Language Basics',
    collapsed: false,
    items: [
      { text: 'Strings and Characters', link: '/guide/04-strings-and-characters' },
      { text: 'Collection Types', link: '/guide/05-collection-types' },
      { text: 'Control Flow', link: '/guide/06-control-flow' },
      { text: 'Functions', link: '/guide/07-functions' },
      { text: 'Closures', link: '/guide/08-closures' },
    ],
  },
  {
    text: 'Types',
    collapsed: true,
    items: [
      { text: 'Enumerations', link: '/guide/09-enumerations' },
      { text: 'Structures and Classes', link: '/guide/10-structures-and-classes' },
      { text: 'Properties', link: '/guide/11-properties' },
      { text: 'Methods', link: '/guide/12-methods' },
      { text: 'Subscripts', link: '/guide/13-subscripts' },
      { text: 'Inheritance', link: '/guide/14-inheritance' },
      { text: 'Initialization', link: '/guide/15-initialization' },
      { text: 'Deinitialization', link: '/guide/16-deinitialization' },
    ],
  },
  {
    text: 'Advanced',
    collapsed: true,
    items: [
      { text: 'Error Handling', link: '/guide/17-error-handling' },
      { text: 'Concurrency', link: '/guide/18-concurrency' },
      { text: 'Macros', link: '/guide/19-macros' },
      { text: 'Type Casting', link: '/guide/20-type-casting' },
      { text: 'Nested Types', link: '/guide/21-nested-types' },
      { text: 'Extensions', link: '/guide/22-extensions' },
      { text: 'Protocols', link: '/guide/23-protocols' },
      { text: 'Generics', link: '/guide/24-generics' },
      { text: 'Opaque Types', link: '/guide/25-opaque-types' },
      { text: 'Memory Management', link: '/guide/26-memory-management' },
      { text: 'Memory Safety', link: '/guide/27-memory-safety' },
      { text: 'Access Control', link: '/guide/28-access-control' },
      { text: 'Modules and Packages', link: '/guide/29-modules-and-packages' },
      { text: 'Advanced Operators', link: '/guide/30-advanced-operators' },
      { text: 'SIMD', link: '/guide/31-simd' },
      { text: 'Operators vs Host', link: '/guide/32-operators-vs-host' },
      { text: 'Runtime Extensions', link: '/guide/33-runtime-extensions' },
      { text: 'Unit Testing', link: '/guide/34-testing' },
    ],
  },
  {
    text: 'Appendix',
    collapsed: true,
    items: [
      { text: 'Implementation Matrix', link: '/guide/appendix-a-implementation-matrix' },
      { text: 'Diagnostic Index', link: '/guide/appendix-b-diagnostic-index' },
    ],
  },
]

export const guideSidebarZh = [
  {
    text: '入门',
    collapsed: false,
    items: [
      { text: '概览', link: '/zh/guide/' },
      { text: '前言', link: '/zh/guide/00-preface' },
      { text: '快速入门', link: '/zh/guide/01-getting-started' },
      { text: '基础', link: '/zh/guide/02-the-basics' },
      { text: '基本运算符', link: '/zh/guide/03-basic-operators' },
    ],
  },
  {
    text: '语言基础',
    collapsed: false,
    items: [
      { text: '字符串和字符', link: '/zh/guide/04-strings-and-characters' },
      { text: '集合类型', link: '/zh/guide/05-collection-types' },
      { text: '控制流', link: '/zh/guide/06-control-flow' },
      { text: '函数', link: '/zh/guide/07-functions' },
      { text: '闭包', link: '/zh/guide/08-closures' },
    ],
  },
  {
    text: '类型',
    collapsed: true,
    items: [
      { text: '枚举', link: '/zh/guide/09-enumerations' },
      { text: '结构体和类', link: '/zh/guide/10-structures-and-classes' },
      { text: '属性', link: '/zh/guide/11-properties' },
      { text: '方法', link: '/zh/guide/12-methods' },
      { text: '下标', link: '/zh/guide/13-subscripts' },
      { text: '继承', link: '/zh/guide/14-inheritance' },
      { text: '构造过程', link: '/zh/guide/15-initialization' },
      { text: '析构过程', link: '/zh/guide/16-deinitialization' },
    ],
  },
  {
    text: '高级特性',
    collapsed: true,
    items: [
      { text: '错误处理', link: '/zh/guide/17-error-handling' },
      { text: '并发', link: '/zh/guide/18-concurrency' },
      { text: '宏', link: '/zh/guide/19-macros' },
      { text: '类型转换', link: '/zh/guide/20-type-casting' },
      { text: '嵌套类型', link: '/zh/guide/21-nested-types' },
      { text: '扩展', link: '/zh/guide/22-extensions' },
      { text: '协议', link: '/zh/guide/23-protocols' },
      { text: '泛型', link: '/zh/guide/24-generics' },
      { text: '不透明类型', link: '/zh/guide/25-opaque-types' },
      { text: '内存管理', link: '/zh/guide/26-memory-management' },
      { text: '内存安全', link: '/zh/guide/27-memory-safety' },
      { text: '访问控制', link: '/zh/guide/28-access-control' },
      { text: '模块与包', link: '/zh/guide/29-modules-and-packages' },
      { text: '高级运算符', link: '/zh/guide/30-advanced-operators' },
      { text: 'SIMD', link: '/zh/guide/31-simd' },
      { text: '运算符 vs Host', link: '/zh/guide/32-operators-vs-host' },
      { text: '运行时扩展', link: '/zh/guide/33-runtime-extensions' },
      { text: '单元测试', link: '/zh/guide/34-testing' },
    ],
  },
  {
    text: '附录',
    collapsed: true,
    items: [
      { text: '实现状态矩阵', link: '/zh/guide/appendix-a-implementation-matrix' },
      { text: '诊断代码索引', link: '/zh/guide/appendix-b-diagnostic-index' },
    ],
  },
]

export const referenceSidebarEn = [
  {
    text: 'Language Reference',
    items: [
      { text: 'About', link: '/reference/00-about-this-reference' },
      { text: 'Lexical Structure', link: '/reference/01-lexical-structure' },
      { text: 'Types', link: '/reference/02-types' },
      { text: 'Expressions', link: '/reference/03-expressions' },
      { text: 'Statements', link: '/reference/04-statements' },
      { text: 'Declarations', link: '/reference/05-declarations' },
      { text: 'Attributes', link: '/reference/06-attributes' },
      { text: 'Patterns', link: '/reference/07-patterns' },
      { text: 'Generics', link: '/reference/08-generic-parameters-and-arguments' },
      { text: 'Protocols', link: '/reference/09-protocols-and-conformances' },
      { text: 'Grammar Summary', link: '/reference/10-grammar-summary' },
      { text: 'Implementation Matrix', link: '/reference/appendix-implementation-matrix' },
    ],
  },
]

export const referenceSidebarZh = [
  {
    text: '语言参考',
    items: [
      { text: '关于本参考', link: '/zh/reference/00-about-this-reference' },
      { text: '词法结构', link: '/zh/reference/01-lexical-structure' },
      { text: '类型', link: '/zh/reference/02-types' },
      { text: '表达式', link: '/zh/reference/03-expressions' },
      { text: '语句', link: '/zh/reference/04-statements' },
      { text: '声明', link: '/zh/reference/05-declarations' },
      { text: '特性', link: '/zh/reference/06-attributes' },
      { text: '模式', link: '/zh/reference/07-patterns' },
      { text: '泛型', link: '/zh/reference/08-generic-parameters-and-arguments' },
      { text: '协议', link: '/zh/reference/09-protocols-and-conformances' },
      { text: '语法摘要', link: '/zh/reference/10-grammar-summary' },
      { text: '实现状态矩阵', link: '/zh/reference/appendix-implementation-matrix' },
    ],
  },
]

export const librarySidebarEn = [
  {
    text: 'Standard Library',
    items: [
      { text: 'Preface', link: '/library/00-preface' },
      { text: 'Package Overview', link: '/library/01-overview' },
      { text: 'Linalg', link: '/library/02-linalg' },
      { text: 'Os', link: '/library/03-os' },
      { text: 'Log', link: '/library/04-log' },
      { text: 'IO and Net', link: '/library/05-io-net' },
      { text: 'Text and Crypto', link: '/library/06-text-crypto' },
    ],
  },
]

export const librarySidebarZh = [
  {
    text: '标准库指南',
    items: [
      { text: '前言', link: '/zh/library/00-preface' },
      { text: '包一览', link: '/zh/library/01-overview' },
      { text: 'Linalg', link: '/zh/library/02-linalg' },
      { text: 'Os', link: '/zh/library/03-os' },
      { text: 'Log', link: '/zh/library/04-log' },
      { text: 'IO 与 Net', link: '/zh/library/05-io-net' },
      { text: 'Text 与 Crypto', link: '/zh/library/06-text-crypto' },
    ],
  },
]

export const editorsSidebarEn = [
  {
    text: 'Editors',
    items: [
      { text: 'Overview', link: '/editors/' },
      { text: 'VS Code', link: '/editors/vscode' },
    ],
  },
]

export const editorsSidebarZh = [
  {
    text: '编辑器',
    items: [
      { text: '概览', link: '/zh/editors/' },
      { text: 'VS Code', link: '/zh/editors/vscode' },
    ],
  },
]

export const compilerSidebarEn = [
  {
    text: 'Compiler Internals',
    items: [
      { text: 'Overview', link: '/compiler/' },
      { text: 'Lexical Analysis', link: '/compiler/lexical-analysis' },
      { text: 'Syntax Analysis', link: '/compiler/syntax-analysis' },
      { text: 'Semantic Analysis', link: '/compiler/semantic-analysis' },
      { text: 'Instruction Set', link: '/compiler/instruction-set' },
      { text: 'Technical Overview', link: '/compiler/technical-overview' },
      { text: 'Backend Parity', link: '/compiler/backend-parity' },
      { text: 'Network Stdlib', link: '/compiler/network-stdlib' },
      { text: 'Backend Debugging', link: '/compiler/backend-debugging' },
      { text: 'Env Registry', link: '/compiler/env-registry' },
    ],
  },
]

export const compilerSidebarZh = [
  {
    text: '编译器内部',
    items: [
      { text: '概览', link: '/zh/compiler/' },
      { text: '词法分析', link: '/zh/compiler/lexical-analysis' },
      { text: '语法分析', link: '/zh/compiler/syntax-analysis' },
      { text: '语义分析', link: '/zh/compiler/semantic-analysis' },
      { text: '指令集', link: '/zh/compiler/instruction-set' },
      { text: '完整技术文档', link: '/zh/compiler/technical-overview' },
      { text: '后端对等', link: '/zh/compiler/backend-parity' },
      { text: '网络标准库', link: '/zh/compiler/network-stdlib' },
      { text: '后端调试', link: '/zh/compiler/backend-debugging' },
    ],
  },
]

export const aboutSidebarEn = [
  {
    text: 'About SYX',
    items: [
      { text: 'Overview', link: '/about/' },
      { text: 'Capabilities', link: '/about/capabilities' },
      { text: 'Architecture Decisions', link: '/about/architecture' },
    ],
  },
]

export const aboutSidebarZh = [
  {
    text: '关于 SYX',
    items: [
      { text: '概览', link: '/zh/about/' },
      { text: '现行能力', link: '/zh/about/capabilities' },
      { text: '架构决议', link: '/zh/about/architecture' },
    ],
  },
]
