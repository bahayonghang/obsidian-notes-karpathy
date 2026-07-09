# SKILL 契约上下文瘦身分层

父任务：`07-08-llm-wiki-gap-optimization`（P3）。证据档案：父任务 `research/llm-wiki-gap-analysis.md`。

## Goal

把每次 skill 调用的"执行前必读"上下文成本降到可实际执行的水平，使契约从名义契约回到可被模型完整遵循的契约——对齐 Karpathy 模式中 schema 保持极简、与用户共同演化的原则。

## 问题

- 当前必读清单：`kb-review` 24 个文件（`skills/kb-review/SKILL.md:34-59`）、`kb-compile` 18 个（`SKILL.md:30-51`）、router 14 个（`SKILL.md:31-48`）；`references/` 共 30+ 文件。
- 每次 skill 调用如果诚实执行需要读入数万 token 的 references，实际执行中模型大概率跳读或略读——契约退化为"名义上的"，规则是否被遵循变得不可预期。
- Karpathy 的 schema 刻意是一个文件："You and the LLM co-evolve this over time"。本仓库把 wiki 层的维护负担消灭了，却在契约层重建了同等负担。

## Requirements

- 将每个 skill 的 references 分为两层（具体分法在 design.md 定夺）：
  - **core（必读）**：执行该 skill 缺了会出错的最小集合，目标每 skill ≤ 6 个文件（或等效的单一合并简报）。
  - **on-demand（按需）**：按触发条件加载（如"出现 paper PDF 时读 paper-ingestion-lifecycle.md"、"涉及别名冲突时读 provenance-and-alias-policy.md"），在 SKILL.md 中写明触发条件而不是无条件罗列。
- 语义无损：现行每条契约规则在瘦身后仍必须可达（core 直接覆盖，或 on-demand 带明确触发条件）。合并/删除 references 文件时在 design.md 留映射表。
- `scripts/skill-contract-registry.json` 保持 canonical：注册表结构需能表达 core/on-demand 分层，`onkb --json dev contract-validate` 与 `dev audit-skills` 更新为按新分层校验（必读清单超限应报错或告警）。
- 评估可比：`dev eval-trigger` / `dev eval-runtime` 基线在瘦身前后可对比，触发行为不回归。
- 度量：记录瘦身前后每个 skill 必读集合的文件数与字节数（作为验收证据）。
- 文档对齐：`README.md` / `README_CN.md` / `CLAUDE.md` / docs 站相关页面同步。

## 约束

- 不改变任何生命周期语义、真值边界、写面授权——本任务只动"契约如何被加载"，不动"契约说什么"。
- 不删除仍被其他 skill 或测试引用的 references 文件（合并需同步更新所有引用点）。
- 遵循 AGENTS.md：markdown 短标题、仓库术语精确。

## Acceptance Criteria

- [x] 每个 skill 的无条件必读清单 ≤ 6 个文件（或单一合并简报），必读集合总字节数较现状下降 ≥ 50%，数据记录在任务工件中（design.md 度量表：core 上限 3-6/skill，总字节 388,767 → 189,838，−51.2%）
- [x] 每条现行契约规则在新结构中可达（design.md 映射表齐全，无孤儿规则；重分层脚本断言 core ∪ on-demand == 原 reads 集合，未合并未删除任何文件）
- [x] `onkb --json dev contract-validate`、`dev audit-skills` 按新分层校验并通过（新增：core 上限 6、无重叠、文件存在、on-demand 小节逐条精确匹配）
- [x] `dev eval-trigger` 基线对比无回归（before/after 除 workspace 时间戳路径外逐字段一致）
- [x] `just ci` 全绿；`README.md` / `README_CN.md` 已补两层加载说明；根 `CLAUDE.md` 无必读机制表述，无需对齐

## Notes

- 复杂任务：`task.py start` 前需补 `design.md`（分层标准、registry schema 变更、references 合并映射表）与 `implement.md`。
- 建议排序：本任务动契约文本面较广，宜在另外两个子任务（各自也要改契约措辞）之后执行，或在 design 阶段明确文件级避让，避免同面冲突。
