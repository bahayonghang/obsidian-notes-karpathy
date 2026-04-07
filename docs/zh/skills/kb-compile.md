# kb-compile

把 immutable raw 编译成可维护的 wiki。

## 核心职责

`kb-compile` 应该像编译器一样工作，而不是像源文件编辑器一样工作。它从 `raw/` 读取资料，把结果写入 `wiki/`，并尽量保持溯源关系清楚。

`raw/papers/` 现在是双模目录：既可以放 markdown 论文笔记，也可以直接放 PDF 论文。

## 先读什么

真实技能合同要求先读取：

- 本地 `AGENTS.md`
- 本地 `CLAUDE.md`（如果存在）
- schema、summary、concept、entity、index、log、Obsidian-safe markdown 等共享模板

## 增量模型

每个 raw 源都会和 `wiki/summaries/` 里的对应摘要页做比对。

以下情况需要重新编译：

- 摘要页不存在
- `source_hash` 落后于当前 raw
- `source_mtime` 落后于当前 raw

如果摘要页已经对应当前 raw，就跳过。

## PDF 论文降级链

对 `raw/papers/` 下的 PDF：

1. 先从可选的 `paper-name.source.md` sidecar 或文件名里解析确定的 paper handle
2. 只有 handle 存在时才优先用 `alphaxiv-paper-lookup`
3. 如果没有 handle，就直接降级到 `pdf`
4. 如果两个 companion skill 都没安装，只跳过该 PDF，并明确提示安装什么

编译器不应把提取后的 markdown 回写到 `raw/`。

## 主要输出

- `wiki/summaries/*.md`
- `wiki/concepts/*.md`
- 启用实体层时的 `wiki/entities/*.md`
- `wiki/index.md`
- `wiki/log.md`
- `wiki/indices/INDEX.md`
- `wiki/indices/CONCEPTS.md`
- `wiki/indices/SOURCES.md`
- `wiki/indices/RECENT.md`
- 启用实体层时的 `wiki/indices/ENTITIES.md`

## 冲突处理

如果新证据和既有概念页或实体页冲突，编译器不应该静默覆盖旧结论，而应该把 tension 暴露出来，并保留 provenance。

## 轻量级收尾检查

编译完成后，这个技能可以顺手修掉本轮引入的明显机械问题，比如：

- 新生成的坏链路
- 缺失但可以明确推断出的 reciprocal `related`
- 摘要页元数据缺口
- Markdown 表格单元格里的 alias-style wikilink

如果用户要完整维护报告，再交给 [kb-health](/zh/skills/kb-health)。
