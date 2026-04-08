# 架构概览

这一组页面解释这套 bundle 为什么要这样设计，以及接下来往哪里演化。

## 当前合同

当前已落地的工作流是 review-gated 的：

- `raw/` 是不可变证据层
- `wiki/drafts/` 是可审校的编译产物
- `wiki/live/` 是批准后的检索真相层
- `wiki/briefings/` 是从 live 重建的运行时上下文
- `outputs/qa/` 和 `outputs/content/` 是持久化下游产物

路由与职责归属主要由 `skills/obsidian-notes-karpathy/scripts/skill-contract-registry.json` 和 `skills/obsidian-notes-karpathy/references/lifecycle-matrix.md` 共同定义。需要确认生命周期状态、owner skill 或 allowed write surfaces 时，优先读这两个文件。

## 设计主线

- 把生产和裁决拆开，避免草稿错误持续复利。
- 保持 local-first、可审计的检索姿态。
- 把高价值回答当成可回流资产，而不是聊天残留。
- 把协作记忆与专题知识分层，而不是混成一个桶。

## Planned / evolving areas

下面这些方向已经进入设计，但在当前合同中可能还只是部分落地：

- 用 `MEMORY.md` 承接协作记忆
- 用 `EDITORIAL-PRIORITIES.md` 承接总编层
- 让 `outputs/qa/` 和 `outputs/content/` 产生结构化回写候选
- 强化 weak evidence 与知识漂移的反腐检查

Guide、Skills、Workflow 页面描述当前合同；Architecture 页面解释设计理由和前进方向。
