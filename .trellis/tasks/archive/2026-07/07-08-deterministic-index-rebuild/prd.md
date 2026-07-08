# 确定性检索索引重建命令

父任务：`07-08-llm-wiki-gap-optimization`（P1）。证据档案：父任务 `research/llm-wiki-gap-analysis.md`。

## Goal

为检索入口索引提供确定性重建/漂移检测能力，使 `wiki/index.md` 与 `wiki/live/indices/{INDEX,CONCEPTS,SOURCES,TOPICS}.md` 不再依赖 LLM 跨会话手工同步。

## 问题

- 这些索引只在 init 脚手架时写入一次（`src/kb/init/legacy_impl.rs:62-67`），之后由 LLM 维护。
- `kb-query` 的第一步导航就是读这些索引（`skills/kb-query/SKILL.md:54-64`），索引陈旧直接毒化检索入口——这是社区（TiddlyWiki 作者）点名的 index drift 失败模式，公认缓解手段是"lint 时确定性全量重建"。
- 现有 `onkb review governance` 已证明"从 vault 状态确定性重建索引"在本仓库可行（QUESTIONS/GAPS/ALIASES/ENTITIES/RELATIONSHIPS），但未覆盖检索入口索引。

## Requirements

- 新增或扩展 `onkb` 命令（形态在 design.md 定夺：扩展 `review governance`，或新增 `review indices <vault-root> [--write]`），从 `wiki/live/**` 页面与 frontmatter 确定性重建检索入口索引。
- 默认 dry-run 报告漂移（on-disk 索引 vs 重建结果的差异），`--write` 才落盘，与现有 automation harness 语义一致。
- 纳入 `scheduled-health` automation 模式，使定时体检自动完成索引防漂移。
- 同一 vault 状态重复运行输出逐字节一致（确定性），产物符合 obsidian-safe markdown 约束。
- 更新契约措辞：`kb-review`（maintenance 模式的索引刷新职责）、`kb-query`（导航入口的可信度说明）、相关 references；保持 `README.md` / `README_CN.md` / `CLAUDE.md` 对齐。
- `wiki/index.md` 若保留人工/LLM 编辑区，需定义受管区块边界（哪些段落是生成的、哪些保留）——边界方案在 design.md 定夺。

## 约束

- 索引是导航层不是真值层：重建只读 `wiki/live/**` 与既有元数据，不得引入 drafts/raw 内容，不得改变真值边界。
- 不得改变 `wiki/log.md` 语义。
- JSON 输出可被测试断言（AGENTS.md 要求）。

## Acceptance Criteria

- [x] fixture vault 上：故意弄脏索引 → dry-run 报告漂移条目 → `--write` 后索引与 live 层一致
- [x] 同一 vault 重复运行 `--write`，第二次报告零漂移且文件无变化
- [x] `scheduled-health` 模式包含索引漂移检查
- [x] 契约/文档措辞更新且 `onkb --json dev contract-validate` 通过
- [x] 新增 `tests/*.rs` 固定用例；`just ci` 全绿

## Notes

- 复杂任务：`task.py start` 前需补 `design.md`（命令形态、受管区块边界、索引生成规则）与 `implement.md`。
