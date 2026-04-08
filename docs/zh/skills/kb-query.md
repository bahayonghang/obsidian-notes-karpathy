# kb-query

只从 approved live brain 做检索、回答、归档和发布。

默认读取：

- `wiki/live/**`
- 相关 `wiki/briefings/**`
- 历史 `outputs/qa/**`

不把 `raw/` 或 `wiki/drafts/` 当作 query truth。

`MEMORY.md` 也不属于默认专题检索面。它只用于协作与编辑上下文。

如果 Q&A 或 publish artifact 暴露出值得长期保留的结论，应该显式记录 writeback 候选，而不是把下一步埋在聊天里。
