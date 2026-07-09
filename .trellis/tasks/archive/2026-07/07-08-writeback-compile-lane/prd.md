# 打通 writeback 候选的 compile 车道

父任务：`07-08-llm-wiki-gap-optimization`（P2）。证据档案：父任务 `research/llm-wiki-gap-analysis.md`。

## Goal

让 `outputs/qa/**` / `outputs/content/**` 中 `writeback_status: pending` 的候选拥有一条有属主、可操作的车道进入 `wiki/drafts/`，使 Karpathy 模式的另一半复利（"good answers filed back into the wiki"）在本仓库真正闭环，而不仅存在于契约 prose 中。

## 问题

- 契约完备：`references/query-writeback-lifecycle.md` 定义 6 状态生命周期（`none → pending → triaged → drafted → reviewed → rejected`），health 有 `writeback_backlog_issues` 检测积压。
- 通路断裂：`kb-compile` source discovery 只接受 `raw/**` 捕获（`skills/kb-compile/SKILL.md:67-77`），`outputs/qa/**` 不是合法 compile 输入；`kb-query` 是只读车道；`kb-review` 只允许对归档面做机械修复。`pending → drafted` 这一跳没有任何 skill 或命令是属主。
- 后果：写回候选永远停在 `pending`，健康检查能看见积压却没有标准处置动作，回答无法复利进 wiki。

## Requirements

- 指定 `pending → drafted` 的唯一属主车道（方案在 design.md 定夺，候选：扩展 `kb-compile` 接受带 `writeback_status: pending` 的归档产物作为 compile 输入；或新增 `onkb compile writeback` 子命令 + 对应契约小节）。
- 写回草稿的 provenance 必须回指原始依据（`source_live_pages` 指向的批准页 / raw 证据），不得把 QA 归档产物本身当作真值来源——防止社区点名的自引用污染（summary-of-summary）。`compiled_from` / `capture_sources` 字段如何承载此链路在 design.md 定夺。
- 车道执行时自动推进 `writeback_status`（`pending → drafted`），并通过 automation harness 在 `outputs/audit/operations.jsonl` 留下审计事件。
- `draft -> review -> live` 真值边界不变：写回草稿仍走正常评审门。
- 契约措辞同步：`kb-compile`（source discovery 与 posture 小节）、`kb-query`（写回契约的下一跳指引）、`kb-review`（maintenance 模式对积压的处置动作）、`references/query-writeback-lifecycle.md`；保持 `README.md` / `README_CN.md` / `CLAUDE.md` 对齐。

## 约束

- 不得使 `raw/**` 可变；写回车道不产生新的 raw 捕获（除非 design 阶段论证例外并明确记录）。
- 归档产物仍是 artifact archive，不因可作为 compile 输入而升格为真值。
- `rejected` / `triaged` 状态的候选不得被车道自动拾取。

## Acceptance Criteria

- [x] fixture vault 上：一个 `writeback_status: pending` 的归档 QA 可通过文档化车道驱动到 `drafted`，产出的草稿位于 `wiki/drafts/**` 且 provenance 回指批准页/raw 证据
- [x] 状态推进写入归档产物 frontmatter，审计事件落盘 `outputs/audit/operations.jsonl`
- [x] 产出草稿满足 `references/draft-schema.md` 必填字段，能被 `kb-review` 正常评审
- [x] `onkb --json dev contract-validate` 与既有 trigger evals 通过
- [x] 新增 `tests/*.rs` 固定用例；`just ci` 全绿

## Notes

- 复杂任务：`task.py start` 前需补 `design.md`（车道形态、provenance 字段设计、状态机接线）与 `implement.md`。
