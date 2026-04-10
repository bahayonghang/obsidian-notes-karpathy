# 回写与搜索

这个 bundle 把高质量输出当成资产，但它必须通过 review-gated 闭环回流。

## 回写姿态

当前方向是：

- `kb-query` 把高价值回答归档到 `outputs/qa/`
- 对外内容放到 `outputs/content/`
- 高价值输出生成结构化 `writeback_candidates`、`source_live_pages` 和 `followup_route`
- 在新写一篇 publish artifact 之前，优先显式复用已批准 coverage 与 archived Q&A，而不是把同样的背景重新写一遍
- 回写必须重新走 draft -> review -> live，不能直接写 live
- archived outputs 可以提供 backlog 与维护信号，但不会自动变成 approved truth

## 为什么不能直接改 live

如果 query 输出可以直接进 `wiki/live/`，那生产层和裁决层就重新塌成一团，整个 bundle 最重要的保护机制就失效了。

治理索引也是同一条规则：`QUESTIONS.md` 这类批准层视图必须以 approved live pages 为基础，不能把未审的 archived outputs 直接抬升进去。

## 搜索姿态

默认检索顺序仍然是：

1. `wiki/index.md`
2. `wiki/live/indices/*`
3. `QUESTIONS.md`、`GAPS.md`、`ALIASES.md` 等治理视图
4. 相关 briefings
5. 历史 `outputs/qa/`
6. 真的不够时再上本地结构化或 metadata 驱动检索
7. 当前面层级都不够时，才把语义检索作为 candidate surfacing

整个梯度里，approved live pages 始终是真相来源。

## Planned / evolving search upgrades

- 更强的 alias map
- 更丰富的派生索引，例如 `RECENT.md`
- 更清楚的 query / health follow-up routing
- 可选的本地工具，例如 qmd 或 DuckDB

Hosted vector 基础设施刻意不是默认答案。这里优先选择成本最低、可追溯性最强的检索层。
