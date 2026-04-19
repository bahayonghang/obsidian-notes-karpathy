# 架构概览

这一组页面解释这套 bundle 为什么要这样设计，以及接下来往哪里演化。

## 当前合同

当前已落地的工作流是带审校与批准流程的：

- `raw/` 是不可变证据层
- `wiki/drafts/` 是可审校的编译产物
- `wiki/live/` 是批准后的检索真相层
- `wiki/live/procedures/` 是批准后的流程记忆
- `wiki/briefings/` 是从 live 重建的运行时上下文
- `outputs/reviews/` 是必需的决策账本
- `outputs/qa/`、`outputs/content/`、`outputs/health/` 是按阶段出现的下游产物
- `outputs/episodes/` 是工作链路结晶后的情节记忆
- `outputs/audit/operations.jsonl` 是机器可读的审计轨
- `raw/**` + `raw/_manifest.yaml` 构成 source retention archive
- `outputs/**` 构成 artifact archive
- `raw/` 同时也是长期素材库面，创作者的整理、复用和发布编排则发生在下游工作面
- `MEMORY.md` 是协作记忆层，默认不进入专题检索面
- `wiki/live/indices/EDITORIAL-PRIORITIES.md` 是总编工作面
- `QUESTIONS.md`、`GAPS.md`、`ALIASES.md` 这类治理视图适合成熟 vault，但必须以已批准的 live 页面为基础

路由与职责归属主要由 `skills/obsidian-notes-karpathy/scripts/skill-contract-registry.json` 和 `skills/obsidian-notes-karpathy/references/lifecycle-matrix.md` 共同定义。需要确认生命周期状态、owner skill 或允许写入的层面时，优先读这两个文件。

## 设计主线

- 把生产和裁决拆开，避免草稿错误持续复利。
- 保持 本地优先、可审计的检索姿态。
- 把高价值回答当成可回流资产，而不是聊天残留。
- 把完整工作链当作情节记忆资产，后续再提炼成 semantic / 流程记忆。
- 把协作记忆与专题知识分层，而不是混成一个桶。
- 让归档产物参与维护与 writeback 闭环，但不把它们静默提升成已批准知识层。
- 把来源保留归档和产物归档明确分开，避免 archive 语义漂移。

## 当前演进压力点

目前还在继续强化的主要方向是：

- 更强的 weak evidence 与知识漂移反腐检查
- 更顺滑的开放问题、alias drift 治理刷新流程
- 把 durable outputs 更稳地回写回 drafts 的闭环
- 更强的面向创作者的 prior-coverage reuse、curated hubs 与 planning views，但仍保持在真相边界之外
- 更明确的 retrieval ladder 与 follow-up routing 说明

Guide、Skills、Workflow 页面描述当前合同；Architecture 页面解释设计理由和前进方向。

归档模型详见 [归档模型](/zh/architecture/archive-model)。

