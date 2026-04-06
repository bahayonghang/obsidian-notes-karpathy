# 简介

Obsidian Notes Karpathy 是一套用 LLM 维护个人知识库的 Obsidian 工作流。

## 核心思路

把知识管理当成编译流程：

```text
raw/（immutable 资料）-> kb-compile -> wiki/（编译产物）-> kb-query / kb-health -> outputs/
```

人负责策展资料，LLM 负责维护 wiki 和生成可复用输出。

## 它和普通 RAG 的区别

- wiki 是持久资产
- 摘要和链接会持续积累
- 有价值的 Q&A 会写入 `outputs/qa/`
- 可发布内容可以从编译后的 wiki 生成，并回链到支撑它的 Q&A
- 整个系统可以像代码库一样做 lint 和维护

## 包结构

| 技能 | 角色 |
|------|------|
| `obsidian-notes-karpathy` | 包级入口与路由 |
| `kb-init` | 初始化 vault 契约 |
| `kb-compile` | 把 raw 编译成 wiki |
| `kb-query` | 搜索、问答和生成输出 |
| `kb-health` | 深度体检与维护 |

## 关键产物

- `wiki/index.md`：内容入口
- `wiki/log.md`：append-only 时间线
- `outputs/qa/`：持久化研究问答
- `outputs/health/`：量化健康报告
- `outputs/content/`：文章、推文串和分享提纲
