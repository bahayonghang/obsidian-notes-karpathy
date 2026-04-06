# 快速开始

快速跑通最小可用的 Karpathy 式知识库。

## 1. 初始化 vault 契约

调用包级入口技能或直接运行 `kb-init`。

会创建：

```text
raw/{articles,papers,podcasts,assets}
wiki/{concepts,summaries,indices}
wiki/index.md
wiki/log.md
outputs/{qa,health,reports,slides,charts,content/{articles,threads,talks}}
AGENTS.md
CLAUDE.md
```

## 2. 添加第一份 raw 资料

在 `raw/articles/`、`raw/papers/` 或 `raw/podcasts/` 下创建 markdown 文件。

只放来源元数据：

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

不要把编译状态写回 raw。

## 3. 编译 wiki

运行 `kb-compile`。

预期结果：

- `wiki/summaries/` 下有摘要页
- `wiki/concepts/` 下有概念页
- `wiki/index.md`、`wiki/log.md` 和 `wiki/indices/*` 被更新

## 4. 提一个实质性问题

使用 `kb-query`。

有价值的答案默认写入 `outputs/qa/`。

## 5. 先产出一份内容草稿

再次使用 `kb-query`，但这次走 publish mode。

常见目标目录：

- `outputs/content/articles/`
- `outputs/content/threads/`
- `outputs/content/talks/`

## 6. 跑一次健康检查

用 `kb-health` 在 `outputs/health/` 里生成第一份体检报告。
