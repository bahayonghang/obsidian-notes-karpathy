# kb-query

统一的读侧入口：只从已批准知识层做检索、回答、归档、派生产物和静态网站导出。

默认读取：

- `wiki/live/**`
- `wiki/live/indices/**`
- `wiki/live/topics/**`
- 相关 `wiki/briefings/**`
- 历史 `outputs/qa/**`

不把 `raw/` 或 `wiki/drafts/` 当作默认检索真相层。

`MEMORY.md` 也不属于默认专题检索面。它只用于协作与编辑上下文。

## 主要模式

- search：先做本地优先候选排序
- research：生成并归档到 `outputs/qa/**` 的有依据回答
- publish：保存到 `outputs/content/**` 或 `outputs/{slides,reports,charts}/**` 的确定性派生产物
- web：导出静态知识站到 `outputs/web/**`
- reflect-lite：先停在 live 外的综合、问题分解或 gap 记录

如果用户还在说 `kb-search`，直接把它路由到这里的 search mode。

如果 Q&A 或发布产物产生了值得长期处理的后续工作，应该显式记录 `writeback_candidates`、`open_questions_touched`、`source_live_pages` 和 `followup_route`，而不是把下一步埋在聊天里。

在起草新的对外交付物之前，优先检查是否已有已批准的 live 页面覆盖，或已有归档 Q&A 可以直接复用，避免把同一段背景解释反复重写。

## 检索梯度

默认检索顺序：

1. `wiki/index.md`
2. `wiki/live/indices/*`
3. `wiki/live/topics/*`
4. `QUESTIONS.md`、`GAPS.md`、`ALIASES.md` 等治理视图（如果存在）
5. 相关 `wiki/briefings/{role}.md`
6. 历史 `outputs/qa/`
7. 本地结构化或元数据驱动检索
8. 只在必要时使用语义检索做候选补充

整个梯度里，已批准的 live 页面始终是真相来源。

## 回写合同

实质性的 Q&A 或 publish 输出应该记录：

- `source_live_pages`：哪些批准页真正支撑了输出
- `open_questions_touched`：输出推进了哪些长期问题
- `writeback_candidates`：哪些长期知识应重新进入 wiki
- `followup_route`：`none | draft | review | health`

其中 `draft` 表示要重新走 draft -> review -> live，`review` 表示下一步是明确的人类裁决，`health` 表示这是治理、漂移、backlog 一类维护工作，而不是直接改真相层。


