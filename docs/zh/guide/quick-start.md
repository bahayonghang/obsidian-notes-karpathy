# 快速开始

## 1. 建好 V2 支撑层

需要有：

- `raw/human/**`
- `raw/agents/{role}/**`
- `wiki/drafts/**`
- `wiki/live/**`
- `wiki/briefings/**`
- `outputs/reviews/**`
- `outputs/qa/**`

## 2. 放入捕获内容

- 人类整理的资料放进 `raw/human/**`
- Agent 自动产出的内容放进 `raw/agents/{role}/**`

## 3. 编译成草稿

运行 `kb-compile`，输出应该落在 `wiki/drafts/`。

## 4. 通过审校门

运行 `kb-review`，把合格草稿提升到 `wiki/live/`，并重建 `wiki/briefings/`。

## 5. 基于批准层提问

运行 `kb-query`，只从 `wiki/live/` 和相关 briefing 做检索与生成。
