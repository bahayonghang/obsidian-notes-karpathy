# 查询与输出

`kb-query` 是把编译后的 wiki 变成可用结论的阶段。

## 标准顺序

1. 读 `wiki/index.md`
2. 读 `wiki/indices/INDEX.md` 和 `wiki/indices/CONCEPTS.md`
3. 用 Backlinks、unlinked mentions 或 Properties 搜索补充线索
4. 先查 `outputs/qa/`
5. 再读概念页和摘要页
6. 输出并在需要时归档

## 为什么 outputs/qa 很重要

这个工作流把高价值问答看成可复用知识，而不是聊天残留。

## 常见输出格式

- markdown 报告
- Marp 幻灯片
- Mermaid 图表
- Canvas 文件
- 文章草稿
- 推文串
- 分享提纲
