# 简介

Obsidian Notes Karpathy 是一组面向 Obsidian 的技能包，用来在 Vault 里运行一套“先编译、再查询、再维护”的知识库工作流。

## 这套包负责什么

```text
raw/     -> 人类整理、保持不可变的原始资料
wiki/    -> 由 LLM 维护的编译层 markdown
outputs/ -> 持久化答案、报告和可发布衍生产物
```

这套工作流的核心不是“每次都临时做一遍检索”，而是“先把知识编译成可维护的结构，再持续更新和复用”。

## 它和普通 RAG 的差别

| 普通聊天或临时 RAG | Obsidian Notes Karpathy |
| --- | --- |
| 回答往往是一次性的 | 高价值回答默认归档到 `outputs/qa/` |
| 每次提问都重新临时检索 | Vault 内有持续维护的 `wiki/` 编译层 |
| 结构主要活在 prompt 里 | 结构落在摘要、概念页、索引和日志里 |
| 维护通常是临时性的 | `kb-health` 会给出评分式维护报告 |
| 发布内容需要另起流程 | `kb-query` 可以直接生成报告和内容草稿 |

## 核心技能

| 技能 | 责任 | 什么时候用 |
| --- | --- | --- |
| `obsidian-notes-karpathy` | 包级入口与生命周期路由 | 用户谈的是整套工作流，或者不知道下一步该做什么 |
| `kb-init` | 创建或修复 Vault 契约 | 支撑层缺失或不完整 |
| `kb-compile` | 把 raw 编译成摘要、概念页、索引和日志 | 有新资料或变更资料等待处理 |
| `kb-query` | 从编译层搜索、回答、归档和生成内容 | Vault 已经有一定知识，用户想要结论或交付物 |
| `kb-health` | 审计漂移、矛盾、弱连接和陈旧输出 | 编译层开始变得不可靠或割裂 |

## 不能破坏的运行规则

- `raw/` 对编译器来说是不可变输入层。
- `wiki/index.md` 和 `wiki/log.md` 是一等导航面。
- `outputs/qa/` 存的是持久研究记忆，不是一次性聊天残留。
- 标准索引目录名是 `wiki/indices/`，但要兼容旧 Vault 的 `wiki/indexes/`。
- 检索要先从 markdown、Backlinks、未链接提及和 Properties 搜索开始，再考虑更重的基础设施。

## 从哪里开始

1. 新 Vault 或支撑层损坏：看 [快速开始](/zh/guide/quick-start)，先走 `kb-init`。
2. `raw/` 下刚放了新资料：直接走 [kb-compile](/zh/skills/kb-compile)。
3. 现在就要答案、报告、推文串或分享提纲：直接走 [kb-query](/zh/skills/kb-query)。
4. 笔记越来越散、越来越旧、互相打架：直接走 [kb-health](/zh/skills/kb-health)。
