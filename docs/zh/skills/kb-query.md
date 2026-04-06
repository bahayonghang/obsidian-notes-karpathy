# kb-query

从编译后的 wiki 中搜索、回答、归档实质性答案，并生成对外内容。

## 模式

| 模式 | 适用场景 | 主要写入位置 |
| --- | --- | --- |
| Search | 用户只想快速找到相关笔记和证据 | 通常不落盘 |
| Research | 用户想得到可溯源的结论 | `outputs/qa/` |
| Publish | 用户想生成报告、文章、推文串、幻灯片、图表或分享提纲 | `outputs/reports/`、`outputs/slides/`、`outputs/charts/` 或 `outputs/content/` |

## 研究顺序

1. 从 `wiki/index.md` 开始
2. 查看 `INDEX.md`、`CONCEPTS.md`、`SOURCES.md` 等派生索引
3. 如果 Backlinks、未链接提及或 Properties 搜索更快，就优先用它们找线索
4. 先在 `outputs/qa/` 里找是否已有相关答案，避免重复推导
5. 再读最相关的概念页、实体页和摘要页
6. 用带 provenance 的方式输出答案，并在必要时显式标注不确定性

## 默认沉淀规则

只要答案是实质性的，就默认归档到 `outputs/qa/`。这样研究结果会变成 Vault 的一部分，而不是一次性聊天残留。

## 常见输出目录

- `outputs/qa/`
- `outputs/reports/`
- `outputs/slides/`
- `outputs/charts/`
- `outputs/content/articles/`
- `outputs/content/threads/`
- `outputs/content/talks/`

## 对 wiki 的反哺

一次有价值的回答或发布动作之后，这个技能还可能：

- 新建概念页
- 在启用实体层时创建或补强实体页
- 给现有概念页或实体页补证据
- 补上缺失的页面连接
- 向 `wiki/log.md` 追加 `query` 或 `publish` 事件
