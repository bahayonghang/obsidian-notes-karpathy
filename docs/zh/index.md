---
layout: home

hero:
  name: "Obsidian Notes Karpathy"
  text: "面向 Obsidian 的 LLM 编译式知识库"
  tagline: 先判断当前处于哪个生命周期，再保持 raw 不可变，并把问答沉淀成长期可复用的研究资产。
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
      text: English Docs
      link: /guide/introduction

features:
  - title: 按生命周期自动路由
    details: 包级入口技能会先判断当前应该初始化、编译、查询，还是做健康检查。
    link: /zh/skills/obsidian-notes-karpathy
  - title: 先编译，不靠临场即兴
    details: 原始资料保持不动，LLM 负责更新摘要、概念页、索引、日志和可选实体层。
    link: /zh/skills/kb-compile
  - title: 持久化研究记忆
    details: 有价值的回答默认写入 outputs/qa，而不是消失在一次性对话里。
    link: /zh/skills/kb-query
  - title: 从已落地笔记产出内容
    details: 报告、文章草稿、推文串和分享提纲都从 wiki 证据层出发，而不是凭空生成。
    link: /zh/workflow/overview
  - title: Immutable Raw
    details: raw 层是人类维护的输入，不是存放编译状态或生成内容的地方。
    link: /zh/guide/directory-structure
  - title: index.md + log.md
    details: vault 同时保留内容入口和 append-only 运行时间线。
    link: /zh/workflow/overview
  - title: 深度健康检查
    details: kb-health 会对完整性、一致性、连通性、新鲜度和溯源质量打分，并区分可自动修复项与需要人工判断的问题。
    link: /zh/skills/kb-health
  - title: 先 markdown 后搜索升级
    details: 先用索引页、Backlinks、未链接提及和 Properties 搜索，规模真的上来后再升级检索基础设施。
    link: /zh/workflow/query
---
