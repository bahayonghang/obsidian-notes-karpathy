# kb-query

搜索编译后的 wiki、回答问题，并生成报告、幻灯片、图表或对外内容草稿。

## 模式

- search mode
- research mode
- publish mode

## 查询顺序

1. 从 `wiki/index.md` 开始
2. 读取派生索引
3. 当 Backlinks、unlinked mentions 或 Properties 搜索更高效时先用它们补线索
4. 先查 `outputs/qa/`
5. 再读概念页和摘要页
6. 输出带 provenance 的答案

## 默认沉淀规则

只要答案是实质性的，就默认归档到 `outputs/qa/`。

这样问答结果会变成知识库的一部分，而不是消失在聊天记录里。

## 常见输出目录

- `outputs/qa/`
- `outputs/reports/`
- `outputs/slides/`
- `outputs/charts/`
- `outputs/content/articles/`
- `outputs/content/threads/`
- `outputs/content/talks/`
