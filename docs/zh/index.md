---
layout: home

hero:
  name: "Obsidian Notes Karpathy"
  text: "面向 Obsidian 的 LLM 编译式知识库"
  tagline: 保持 raw 不可变，把资料编译成持久 wiki，让问答结果持续复用。
  image:
    src: /hero-image.svg
    alt: Obsidian Notes Karpathy
  actions:
    - theme: brand
      text: 快速开始
      link: /zh/guide/introduction
    - theme: alt
      text: 技能概览
      link: /zh/skills/overview
    - theme: alt
      text: English Docs
      link: /guide/introduction

features:
  - title: 包级入口 + 四个操作技能
    details: 一个包级入口技能负责把请求路由到 kb-init、kb-compile、kb-query 或 kb-health。
    link: /zh/skills/obsidian-notes-karpathy
  - title: Immutable Raw
    details: 原始资料不回写，编译状态保存在 wiki 摘要和健康报告里。
    link: /zh/guide/directory-structure
  - title: 持久化 Q&A 沉淀
    details: 有价值的问答默认写入 outputs/qa，并可反哺概念页和后续研究。
    link: /zh/skills/kb-query
  - title: 从 Wiki 产出内容
    details: kb-query 可以把研究结论写成文章草稿、推文串和分享提纲，统一落到 outputs/content。
    link: /zh/workflow/query
  - title: index.md + log.md
    details: wiki 同时有内容入口和 append-only 时间线。
    link: /zh/workflow/overview
  - title: 深度健康检查
    details: kb-health 负责冲突、过时、alias drift、弱连接、孤儿页和 provenance 缺口等体检项。
    link: /zh/skills/kb-health
  - title: 先 markdown 后搜索升级
    details: 默认先用 markdown 索引、Backlinks 和 Properties 搜索。规模上来后再接 DuckDB、Dataview 或更重的检索层。
    link: /zh/workflow/query
---
