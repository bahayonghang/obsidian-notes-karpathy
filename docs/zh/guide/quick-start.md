# 快速开始

如果你想在一轮会话里跑通最小可用闭环，就按这个页面走。

## 1. 先创建或修复 Vault 契约

从包级入口技能开始，或者直接调用 `kb-init`。

预期的支撑层结构：

```text
raw/{articles,papers,podcasts,assets}
wiki/{concepts,summaries,indices}
wiki/index.md
wiki/log.md
outputs/{qa,health,reports,slides,charts,content/{articles,threads,talks}}
AGENTS.md
CLAUDE.md
```

只有在确实需要时才启用可选层：

- `raw/repos/`：仓库快照或 repo 伴随笔记
- `wiki/entities/` 和 `wiki/indices/ENTITIES.md`：稳定命名实体层

## 2. 放入 5 到 10 条真实资料

把 markdown 资料放进 `raw/`、`raw/articles/`、`raw/papers/` 或 `raw/podcasts/`。

raw 笔记里只保留来源元数据：

```yaml
---
title: "Attention Is All You Need"
source: "https://arxiv.org/abs/1706.03762"
author: "Vaswani et al."
date: 2017-06-12
type: paper
tags:
  - transformers
  - attention
clipped_at: 2026-04-03T10:00:00
---
```

不要把编译状态写进 raw。

## 3. 小批量编译

每大约 5 条资料跑一次 `kb-compile`。

预期结果：

- `wiki/summaries/` 下生成摘要页
- `wiki/concepts/` 下生成概念页
- 如果需要实体层，则生成 `wiki/entities/`
- `wiki/index.md`、`wiki/log.md` 和 `wiki/indices/*` 被刷新

## 4. 提一个实质性问题

第一次编译完成后，立刻用 `kb-query` 提一个真实研究问题。

这一步应该做到：

- 先检索编译层
- 先看 `outputs/qa/` 里是否已经有相关答案
- 用可溯源证据作答
- 把实质性结果归档到 `outputs/qa/`

## 5. 产出一个衍生交付物

如果你的 Vault 还要服务外部输出，就让 `kb-query` 进入 publish mode。

常见落点：

- `outputs/reports/`
- `outputs/slides/`
- `outputs/charts/`
- `outputs/content/articles/`
- `outputs/content/threads/`
- `outputs/content/talks/`

## 6. 建立第一次体检基线

跑一次 `kb-health`。

报告应写入 `outputs/health/health-check-{date}.md`，并对完整性、一致性、连通性、新鲜度和溯源质量打分。

## 最小启动闭环

1. 用 `kb-init` 建好支撑层
2. 放入一小批资料
3. 用 `kb-compile` 编译
4. 用 `kb-query` 提一个真实问题
5. 如果有外部输出需求，就把其中一个答案转成内容
6. 用 `kb-health` 建立维护基线
