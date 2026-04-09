# kb-query

只从 approved live brain 做检索、回答、归档和发布。

默认读取：

- `wiki/live/**`
- `wiki/live/indices/**`
- 相关 `wiki/briefings/**`
- 历史 `outputs/qa/**`

不把 `raw/` 或 `wiki/drafts/` 当作 query truth。

`MEMORY.md` 也不属于默认专题检索面。它只用于协作与编辑上下文。

## 主要模式

- search：快速定位批准页
- research：生成并归档到 `outputs/qa/**` 的 grounded answer
- publish：保存到 `outputs/content/**` 的报告、线程、演讲稿或幻灯片
- reflect-lite：先停在 live 外的综合、问题分解或 gap 记录

如果 Q&A 或 publish artifact 暴露出值得长期保留的结论，应该显式记录 `writeback_candidates` 和 `open_questions_touched`，而不是把下一步埋在聊天里。
