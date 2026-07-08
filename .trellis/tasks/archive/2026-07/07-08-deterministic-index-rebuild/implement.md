# 执行计划：确定性检索索引重建命令

前置：`prd.md`（需求与验收）、`design.md`（技术决策）。上下文清单见 `implement.jsonl` / `check.jsonl`。

## 前置阅读（实施前必读）

1. `.trellis/spec/backend/index.md` 及其 Pre-Development Checklist 指向的五份 guide
2. `src/kb/governance.rs:67`（build）与 `:612`（write + audit）——要镜像的范式
3. `design.md` 的每文件重建规则表

## 执行步骤

### Step 1 — 核心计算模块

- [x] 新建 `src/kb/navigation.rs`：`build_navigation_indices(vault_root) -> Result<Value>`
  - 复用 `collect_markdown_records` / `live_records` / `list_field`
  - 按 design.md 规则表生成 5 个索引文件内容 + `wiki/index.md` 受管区块内容
  - 与 on-disk 内容对比，产出 per-file `status` 与 `drift_count`
  - 零时间戳、BTreeMap/显式排序
- [x] 在 `src/kb/mod.rs`（或等效模块声明处）注册模块
- 验证：`cargo test`（编译通过）+ 临时用 `ready-for-query` fixture 手跑函数（可先接 Step 2 的 CLI 再验）

### Step 2 — CLI 接线（dry-run 先行）

- [x] `src/cli/args.rs`：`ReviewCommand::Indices { vault, #[arg(long)] write: bool }`
- [x] `src/cli/dispatch.rs`：接线到 build（dry-run 路径），JSON 输出对齐 design.md 形状
- 验证：`cargo run -- --json review indices evals/skills/obsidian-notes-karpathy/fixtures/ready-for-query` 返回合法 JSON、状态字段齐全
- 回滚点 A：以上为纯新增，revert 即回滚

### Step 3 — 写路径 + 审计

- [x] `write_navigation_indices(vault_root, payload)`：仅写 `drifted`/`missing` 文件；`unmanaged`/`excluded` 永不写
- [x] `audit_log::append_event(vault_root, "rebuild_navigation_indices", …)`，payload 含 `written_paths` + `drift_count`
- 验证：临时拷贝 fixture → `--write` → 索引内容与 live 层一致、`outputs/audit/operations.jsonl` 落审计行；再跑一次 `--write` 报零漂移且文件字节不变（幂等）

### Step 4 — scheduled-health 接线

- [x] `src/kb/automation.rs` `"scheduled-health"` 臂并入 `navigation_indices` 检查（dry-run 汇总；`--write` 时重建）
- 验证：`cargo run -- --json review automation <fixture> --mode scheduled-health` 输出含新键
- **评审门 1**：核心行为完成，向用户展示 dry-run/write/幂等三个 JSON 样例，确认输出形状后再进入契约同步

### Step 5 — init 模板与脚手架对齐

- [x] `skills/kb-init/assets/wiki/index.md`：加 `<!-- onkb:indices:begin/end -->` 受管标记
- [x] `skills/kb-init/assets/wiki/live/indices/{INDEX,CONCEPTS,SOURCES,TOPICS,RECENT}.md`：frontmatter 对齐重建产物形状（移除 `generated_at`，加 `managed_by`），使"新 init 的 vault 首跑 `review indices` 即零漂移"
- [x] 检查 `src/kb/init/legacy_impl.rs` 模板变量渲染不受影响
- 验证：init 一个临时 vault → `review indices` dry-run 报零漂移
- 注意：动模板可能影响 `tests/init_workflow.rs` / `payload_compat.rs` 快照，同步更新

### Step 6 — 测试固化

- [x] 新增 `tests/review_indices.rs`（或并入 `tests/query_health.rs`，以 `just test` 现有组织为准）：
  - 弄脏索引 → dry-run 报漂移 → `--write` 修复 → 复跑零漂移（PRD 验收 1、2）
  - `wiki/index.md` 无标记 → `unmanaged` 且 `--write` 不触碰
  - `EDITORIAL-PRIORITIES.md` 永远 `excluded`
  - RECENT 排序与 `unlisted_count`
  - scheduled-health 含 `navigation_indices`（PRD 验收 3）
- [x] 需要时新增 fixture（如 `needs-index-rebuild/`），或基于 `ready-for-query` 临时拷贝构造
- 验证：`just test` 全绿
- 回滚点 B：Steps 3-6 为一个行为单元，问题时整体 revert 后回到回滚点 A 状态

### Step 7 — 契约与文档同步

- [x] `skills/kb-review/SKILL.md`、`skills/kb-query/SKILL.md`、`skill-contract-registry.json`、`references/automation-hooks.md`、`references/search-upgrades.md` 按 design.md 同步面清单更新
- [x] `README.md` / `README_CN.md` / `CLAUDE.md`：Deterministic helpers 清单 + Karpathy alignment 表
- 验证：`cargo run -- --json dev contract-validate` + `cargo run -- --json dev audit-skills` 通过

### Step 8 — 全量收口

- [x] `just ci`（lint + test + docs-build）全绿
- [x] 对照 `prd.md` Acceptance Criteria 全部勾选
- [x] 按 AGENTS.md 惯例分 commit（建议：`feat(知识库): 新增 review indices 确定性索引重建` / `test(知识库): …` / `docs(知识库): …`，一 commit 一关注点）

## 验证命令速查

```bash
just test
just ci
cargo run -- --json review indices <vault> [--write]
cargo run -- --json review automation <vault> --mode scheduled-health
cargo run -- --json dev contract-validate
cargo run -- --json dev audit-skills
```

## 风险与注意

- Windows 路径：fixture 对比注意 `relative_posix` 归一化（repo 已有 helper，勿手拼路径）。
- 快照测试（insta）受模板变更影响时，先人工确认 diff 合理再 `cargo insta accept`（如工作流可用）。
- 不要顺手修改无关 references 措辞（surgical changes 原则）。
