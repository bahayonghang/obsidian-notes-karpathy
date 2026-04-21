import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'Obsidian Notes Karpathy',
  description: 'LLM-driven knowledge base skills for Obsidian',
  base: '/obsidian-notes-karpathy/',
  lastUpdated: true,
  cleanUrls: true,
  themeConfig: {
    logo: {
      light: '/logo-light.svg',
      dark: '/logo-dark.svg',
    },
    socialLinks: [
      { icon: 'github', link: 'https://github.com/bahayonghang/obsidian-notes-karpathy' },
    ],
    editLink: {
      pattern: 'https://github.com/bahayonghang/obsidian-notes-karpathy/edit/main/docs/:path',
      text: 'Edit this page on GitHub',
    },
    footer: {
      message: 'Released under the MIT License.',
      copyright: 'Copyright © 2026 Obsidian Notes Karpathy',
    },
    search: {
      provider: 'local',
      options: {
        locales: {
          root: {
            translations: {
              button: {
                buttonText: 'Search',
                buttonAriaLabel: 'Search docs',
              },
              modal: {
                noResultsText: 'No results found',
                resetButtonTitle: 'Reset search',
                footer: {
                  selectText: 'to select',
                  navigateText: 'to navigate',
                  closeText: 'to close',
                },
              },
            },
          },
          zh: {
            translations: {
              button: {
                buttonText: '搜索文档',
                buttonAriaLabel: '搜索文档',
              },
              modal: {
                noResultsText: '无法找到相关结果',
                resetButtonTitle: '清除查询条件',
                footer: {
                  selectText: '选择',
                  navigateText: '切换',
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
      lang: 'en',
      themeConfig: {
        nav: navEn(),
        sidebar: sidebarEn(),
        outline: {
          label: 'On this page',
        },
        docFooter: {
          prev: 'Previous page',
          next: 'Next page',
        },
      },
    },
    zh: {
      label: '简体中文',
      lang: 'zh-CN',
      link: '/zh/',
      themeConfig: {
        nav: navZh(),
        sidebar: sidebarZh(),
        outline: {
          label: '页面导航',
        },
        docFooter: {
          prev: '上一页',
          next: '下一页',
        },
      },
    },
  },
})

function navEn() {
  return [
    { text: 'Guide', link: '/guide/introduction', activeMatch: '/guide/' },
    { text: 'Skills', link: '/skills/overview', activeMatch: '/skills/' },
    { text: 'Workflow', link: '/workflow/overview', activeMatch: '/workflow/' },
    { text: 'Architecture', link: '/architecture/overview', activeMatch: '/architecture/' },
  ]
}

function navZh() {
  return [
    { text: '指南', link: '/zh/guide/introduction', activeMatch: '/zh/guide/' },
    { text: '技能', link: '/zh/skills/overview', activeMatch: '/zh/skills/' },
    { text: '工作流', link: '/zh/workflow/overview', activeMatch: '/zh/workflow/' },
    { text: '架构', link: '/zh/architecture/overview', activeMatch: '/zh/architecture/' },
  ]
}

function sidebarEn() {
  return {
    '/guide/': [
      {
        text: 'Start Here',
        items: [
          { text: 'Introduction', link: '/guide/introduction' },
          { text: 'Quick Start', link: '/guide/quick-start' },
          { text: 'Installation', link: '/guide/installation' },
        ],
      },
      {
        text: 'Operating Model',
        items: [
          { text: 'Karpathy Workflow', link: '/guide/karpathy-workflow' },
          { text: 'Directory Structure', link: '/guide/directory-structure' },
          { text: 'AGENTS.md Schema', link: '/guide/agents-schema' },
        ],
      },
    ],
    '/skills/': [
      {
        text: 'Routing & Core Skills',
        items: [
          { text: 'Overview', link: '/skills/overview' },
          { text: 'obsidian-notes-karpathy', link: '/skills/obsidian-notes-karpathy' },
          { text: 'kb-init', link: '/skills/kb-init' },
          { text: 'kb-ingest', link: '/skills/kb-ingest' },
          { text: 'kb-compile', link: '/skills/kb-compile' },
          { text: 'kb-review', link: '/skills/kb-review' },
          { text: 'kb-query', link: '/skills/kb-query' },
          { text: 'kb-render', link: '/skills/kb-render' },
        ],
      },
      {
        text: 'Companion Skills',
        items: [
          { text: 'Obsidian Markdown', link: '/skills/obsidian-markdown' },
          { text: 'Obsidian CLI', link: '/skills/obsidian-cli' },
          { text: 'Canvas Creator', link: '/skills/obsidian-canvas-creator' },
        ],
      },
    ],
    '/workflow/': [
      {
        text: 'Lifecycle Flow',
        items: [
          { text: 'Overview', link: '/workflow/overview' },
          { text: 'Collect Sources', link: '/workflow/collect' },
          { text: 'Compile Wiki', link: '/workflow/compile' },
          { text: 'Review Gate', link: '/skills/kb-review' },
          { text: 'Query & Output', link: '/workflow/query' },
          { text: 'Health Checks', link: '/workflow/health-checks' },
        ],
      },
    ],
    '/architecture/': [
      {
        text: 'Design Notes',
        items: [
          { text: 'Overview', link: '/architecture/overview' },
          { text: 'Chinese-LLM-Wiki Compatibility', link: '/architecture/chinese-llm-wiki-compat' },
          { text: 'Archive Model', link: '/architecture/archive-model' },
          { text: 'Boundaries', link: '/architecture/boundaries' },
          { text: 'Writeback & Search', link: '/architecture/writeback-search' },
        ],
      },
    ],
  }
}

function sidebarZh() {
  return {
    '/zh/guide/': [
      {
        text: '从这里开始',
        items: [
          { text: '简介', link: '/zh/guide/introduction' },
          { text: '快速开始', link: '/zh/guide/quick-start' },
          { text: '安装', link: '/zh/guide/installation' },
        ],
      },
      {
        text: '工作方式',
        items: [
          { text: 'Karpathy 工作流', link: '/zh/guide/karpathy-workflow' },
          { text: '目录结构', link: '/zh/guide/directory-structure' },
          { text: 'AGENTS.md 规范', link: '/zh/guide/agents-schema' },
        ],
      },
    ],
    '/zh/skills/': [
      {
        text: '路由与核心技能',
        items: [
          { text: '概览', link: '/zh/skills/overview' },
          { text: 'obsidian-notes-karpathy', link: '/zh/skills/obsidian-notes-karpathy' },
          { text: 'kb-init', link: '/zh/skills/kb-init' },
          { text: 'kb-ingest', link: '/zh/skills/kb-ingest' },
          { text: 'kb-compile', link: '/zh/skills/kb-compile' },
          { text: 'kb-review', link: '/zh/skills/kb-review' },
          { text: 'kb-query', link: '/zh/skills/kb-query' },
          { text: 'kb-render', link: '/zh/skills/kb-render' },
        ],
      },
      {
        text: '搭配技能',
        items: [
          { text: 'Obsidian Markdown', link: '/zh/skills/obsidian-markdown' },
          { text: 'Obsidian CLI', link: '/zh/skills/obsidian-cli' },
          { text: 'Canvas 创建器', link: '/zh/skills/obsidian-canvas-creator' },
        ],
      },
    ],
    '/zh/workflow/': [
      {
        text: '生命周期流程',
        items: [
          { text: '概览', link: '/zh/workflow/overview' },
          { text: '收集资料', link: '/zh/workflow/collect' },
          { text: '编译 Wiki', link: '/zh/workflow/compile' },
          { text: '审校与批准', link: '/zh/skills/kb-review' },
          { text: '查询与输出', link: '/zh/workflow/query' },
          { text: '健康检查', link: '/zh/workflow/health-checks' },
        ],
      },
    ],
    '/zh/architecture/': [
      {
        text: '设计说明',
        items: [
          { text: '概览', link: '/zh/architecture/overview' },
          { text: 'Chinese-LLM-Wiki 兼容映射', link: '/zh/architecture/chinese-llm-wiki-compat' },
          { text: '归档模型', link: '/zh/architecture/archive-model' },
          { text: '边界', link: '/zh/architecture/boundaries' },
          { text: '回写与搜索', link: '/zh/architecture/writeback-search' },
        ],
      },
    ],
  }
}

