# 回写与搜索

这个 bundle 把高质量输出当成资产，但它必须通过 review-gated 闭环回流。

## 回写姿态

当前方向是：

- `kb-query` 把高价值回答归档到 `outputs/qa/`
- 对外内容放到 `outputs/content/`
- 高价值输出生成结构化 `writeback_candidates`
- 回写必须重新走 draft -> review -> live，不能直接写 live

## 为什么不能直接改 live

如果 query 输出可以直接进 `wiki/live/`，那生产层和裁决层就重新塌成一团，整个 bundle 最重要的保护机制就失效了。

## 搜索姿态

默认检索顺序仍然是：

1. `wiki/index.md`
2. `wiki/live/indices/*`
3. backlinks、aliases 和 metadata
4. 真的不够时再上本地结构化搜索

## Planned / evolving search upgrades

- 更强的 alias map
- 更丰富的派生索引，例如 `RECENT.md`
- 可选的本地工具，例如 qmd 或 DuckDB

Hosted vector 基础设施刻意不是默认答案。这里优先选择成本最低、可追溯性最强的检索层。
