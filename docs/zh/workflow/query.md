# 查询与输出

`kb-query` 是把编译后的 wiki 变成可用结论的阶段。

## 标准顺序

1. 读 `wiki/index.md`
2. 读 `wiki/live/indices/INDEX.md`、`CONCEPTS.md`，以及相关 source / question 视图
3. 用 Backlinks、unlinked mentions 或 Properties 搜索补充线索
4. 先查 `outputs/qa/`
5. 再读概念页和摘要页
6. 输出并在需要时归档到 `outputs/qa/` 或 `outputs/content/`

## 为什么 outputs/qa 很重要

这个工作流把高价值问答看成可复用知识，而不是聊天残留。

如果一个归档回答暴露出值得进入长期脑的结论，应该显式生成 writeback 候选，而不是把下一步藏在聊天上下文里。

Archive 仍然是下游工作面：

- `outputs/qa/**` 和 `outputs/content/**` 属于 reusable artifact archive
- 它们让未来复用更快
- 但它们依然不会替代 `wiki/live/**` 作为真相层

## 默认复用顺序

在写新内容之前：

1. 先查 approved live pages
2. 再查 indices 和 briefings
3. 再查历史 archived Q&A
4. 最后才查已经正确复用批准知识的 archived publish artifacts

## 边界提醒

`kb-query` 不应把 `raw/`、`wiki/drafts/` 或 `MEMORY.md` 当作默认专题检索真相层。

## 常见输出格式

- markdown 报告
- Marp 幻灯片
- Mermaid 图表
- Canvas 文件
- 文章草稿
- 推文串
- 分享提纲
