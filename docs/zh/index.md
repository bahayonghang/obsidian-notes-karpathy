---
layout: home

hero:
  name: "Obsidian Notes Karpathy"
  text: "面向 Obsidian 的带审校门知识库"
  tagline: 把捕获、草稿编译、批准和 briefing 分开，让多 Agent 只在 approved truth 上复利，而不是在未审草稿上滚雪球。
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
      text: English Docs
      link: /guide/introduction

features:
  - title: 按生命周期自动路由
    details: 包级入口技能会先判断当前应该初始化、编译、审校、查询，还是做健康检查。
    link: /zh/skills/obsidian-notes-karpathy
  - title: 先落草稿，再进 live
    details: "`kb-compile` 只把 raw 编译到 `wiki/drafts/`，只有 `kb-review` 才能提升到 `wiki/live/`。"
    link: /zh/skills/kb-compile
  - title: 独立审校门
    details: "`kb-review` 给草稿打分、写 review 记录、批准进入 `wiki/live/`，并重建 role-specific briefings。"
    link: /zh/skills/kb-review
  - title: 持久化研究记忆
    details: "有价值的回答默认写入 `outputs/qa/`，但 query 只能基于 approved live 和 briefing 做综合。"
    link: /zh/skills/kb-query
  - title: 从已落地笔记产出内容
    details: "报告、文章草稿、推文串和分享提纲都从 `wiki/live/` 证据层出发，而不是凭空生成。"
    link: /zh/workflow/overview
  - title: Immutable Raw
    details: "`raw/human/**` 和 `raw/agents/{role}/**` 只是证据 intake，不是最终真相层。"
    link: /zh/guide/directory-structure
  - title: Live Brain + Briefings
    details: "`wiki/live/` 是批准后的知识脑，`wiki/briefings/` 把它变成各角色可直接读取的上下文。"
    link: /zh/workflow/overview
  - title: 深度健康检查
    details: "`kb-health` 会审计 approved knowledge、stale briefings、review backlog、渲染完整性和溯源质量。"
    link: /zh/skills/kb-health
  - title: 先 markdown 后搜索升级
    details: 先用 approved live 索引、Backlinks 和 briefings，规模真的上来后再升级检索基础设施。
    link: /zh/workflow/query
  - title: 先画边界，再谈检索
    details: 把协作记忆、草稿知识、批准层和回写候选分开，知识库才会增长，而不是慢慢腐烂。
    link: /zh/architecture/boundaries
---
