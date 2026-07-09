# 执行计划：SKILL 契约上下文瘦身分层

前置：`prd.md`（需求与验收）、`design.md`（分层表、registry schema、校验面）。上下文清单见 `implement.jsonl` / `check.jsonl`。

## 前置阅读（实施前必读）

1. `.trellis/spec/backend/index.md` 的 Pre-Development Checklist（本任务主要触 `src/dev/**` 与 SKILL 文本）
2. `src/dev/reference_blocks.rs`（bullet 生成/解析既有机制）与 `src/dev/contract.rs:181-230`（校验落点）
3. `design.md` 的每 skill 分层表——SKILL 小节改写以它为准

## 执行步骤

### Step 0 — 基线留档

- [x] `cargo run -- dev eval-trigger --dry-run` 输出存 `target/trigger-baseline-before.json`
- [x] 度量脚本跑一遍现状（files/bytes per skill），与 design.md 表核对

### Step 1 — registry schema 与校验机制

- [x] `src/dev/registry.rs`：`OnDemandRead { file, when }` + `SkillEntry.reads_on_demand`
- [x] `src/dev/reference_blocks.rs`：`build_on_demand_bullets` / `extract_on_demand_bullets`；`render_shared_reference_block` 追加 on-demand 块
- [x] `src/dev/contract.rs`：4 项新校验（core ≤ 6 / 无重叠 / 文件存在 / on-demand 小节逐条精确匹配）
- 验证：`cargo build` + 现状 registry（尚无 reads_on_demand）下 `dev contract-validate` 仍 ok（向后兼容）
- 回滚点 A

### Step 2 — registry 数据分层

- [x] `skill-contract-registry.json`：7 个 skill 的 `reads` 收窄为 core，其余移入 `reads_on_demand` 并写 canonical `when` 条件（照 design.md 表）
- 验证：此时 `dev contract-validate` 应报"SKILL 缺 on-demand 小节"类错误——证明校验真的在咬合（红灯预期）

### Step 3 — SKILL.md 双小节改写

- [x] 7 个 SKILL.md："Read before" 收窄为 AGENTS/CLAUDE/registry + core；紧随新增 "## Load on demand" 小节（用 `dev render-reference-block` 生成的 bullets 回填，避免手写漂移）
- [x] 各 SKILL 中原必读清单承载的语义指针若在 prose 有引用（如 install fallback 指向 lifecycle-matrix），确认措辞仍成立
- 验证：`dev contract-validate` 全绿；`dev audit-skills` 全 ok
- 回滚点 B：Steps 2-3 是一个契约单元

### Step 4 — 测试固化

- [x] `tests/contract_routing.rs`（或新增 `tests/context_slimming.rs`）：core ≤ 6 全 registry 断言、core/on-demand 无交集、on-demand 文件存在且 SKILL 可达、`extract_on_demand_bullets` 解析用例
- 验证：`just test` 全绿

### Step 5 — 基线对比与度量回填

- [x] `cargo run -- dev eval-trigger --dry-run` 存 `target/trigger-baseline-after.json`，与 before 比对（计划集应一致）
- [x] 重跑度量脚本，实际数字回填 design.md 汇总表（验收：core ≤ 6/skill，总字节 ≥ −50%）

### Step 6 — 文档对齐

- [x] `README.md` / `README_CN.md`：契约注册表相关段补一句两层加载模型（core 无条件 + on-demand 触发条件）
- [x] 根 `CLAUDE.md`：契约规则区若有必读表述则对齐（无则不动）
- [x] docs 站相关页（`docs/skills/*.md` 不含读清单，预计零改动；以 contract-validate/doc 测试为准）
- 验证：`cargo test --test docs_contract --test doc_fragments`

### Step 7 — 全量收口

- [x] `just ci` 全绿
- [x] `prd.md` Acceptance Criteria 逐条勾选（含度量数据指向 design.md 表）
- [x] 分 commit：`feat(知识库): registry 分层机制 + 校验`（src/dev/**）/ `docs(知识库): SKILL 双小节 + registry 数据 + README`（契约文本面）/ `test(知识库): 分层契约用例`

## 验证命令速查

```bash
just test
just ci
cargo run -- --json dev contract-validate
cargo run -- dev audit-skills --json
cargo run -- dev eval-trigger --dry-run
cargo run -- dev render-reference-block <skill>
```

## 风险与注意

- PostToolUse Markdown hook 会整形 SKILL/README 编辑——bullets 用 render 命令产物回填可保持与校验期望逐字节一致；若 hook 整形了 `—` 分隔风格需在 extract 中兼容（以实测为准）。
- `extract_reference_bullets` 的 fallback 分支（无小节时全表扫描）不得被新小节意外触发：core 小节始终在前，解析在下一个 `## ` 停止。
- 不动任何 `description:` frontmatter（trigger evals 基线依赖）；不动 `reads` 之外的 registry 语义键。
- kb-ingest / kb-render 降幅低于 50% 是 file-model 地板效应，验收口径按全局总字节（design.md 已注明），PRD 勾选时说明。
