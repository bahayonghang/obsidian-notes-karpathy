---
layout: home

hero:
  name: "Obsidian Notes Karpathy"
  text: "面向 Obsidian 的带审校与批准流程知识库"
  tagline: 把捕获、草稿编译、批准和角色简报分开，让多 Agent 只在已批准知识层上持续复用，而不是在未审草稿上滚雪球。
  image:
    src: /hero-image.svg
    alt: Obsidian Notes Karpathy
  actions:
    - theme: brand
      text: 快速开始
      link: /zh/guide/quick-start
    - theme: alt
      text: 选择技能
      link: /zh/skills/overview
    - theme: alt
      text: 架构说明
      link: /zh/architecture/overview
    - theme: alt
      text: 英文文档
      link: /guide/introduction

features:
  - title: 按生命周期自动路由
    details: 包级入口技能会先判断当前应该初始化、登记来源、编译、审校、查询、渲染，还是做健康检查。
    link: /zh/skills/obsidian-notes-karpathy
  - title: 来源登记表
    details: "`kb-ingest` 会在编译开始前，把 `raw/_manifest.yaml` 与 raw captures 对齐。"
    link: /zh/skills/kb-ingest
  - title: 先落草稿，再进 live
    details: "`kb-compile` 只把 raw 编译到 `wiki/drafts/`，只有 `kb-review` 才能提升到 `wiki/live/`。"
    link: /zh/skills/kb-compile
  - title: 独立审校与批准
    details: "`kb-review` 给草稿打分、写审校记录、批准进入 `wiki/live/`，并重建角色简报。"
    link: /zh/skills/kb-review
  - title: 持久化研究记忆
    details: "有价值的回答默认写入 `outputs/qa/`，但 `kb-query` 只能基于已批准的 live 页面和角色简报做综合。"
    link: /zh/skills/kb-query
  - title: 从已落地笔记产出内容
    details: "报告、文章草稿、推文串和分享提纲都从 `wiki/live/` 证据层出发，而不是凭空生成。"
    link: /zh/workflow/overview
  - title: 统一读侧入口
    details: "`kb-query` 负责本地优先检索、有依据的综合回答、历史答案复用和静态网站导出。"
    link: /zh/skills/kb-query
  - title: 不可变 Raw
    details: "`raw/human/**` 和 `raw/agents/{role}/**` 只是证据输入，不是最终真相层。"
    link: /zh/guide/directory-structure
  - title: 已批准知识层与角色简报
    details: "`wiki/live/` 是已批准知识层，`wiki/briefings/` 会把它整理成各角色可直接读取的上下文。"
    link: /zh/workflow/overview
  - title: 治理入口
    details: "`kb-review` 统一负责即时审校，以及维护模式下的已批准知识层治理、积压和溯源审计。"
    link: /zh/skills/kb-review
  - title: 先用 Markdown，再升级搜索
    details: 先用已批准 live 索引、Backlinks 和角色简报；只有规模真的上来后，再升级检索基础设施。
    link: /zh/workflow/query
  - title: 先画边界，再谈检索
    details: 把协作记忆、草稿知识、已批准知识层和回写候选分开，知识库才会增长，而不是慢慢腐烂。
    link: /zh/architecture/boundaries
---


