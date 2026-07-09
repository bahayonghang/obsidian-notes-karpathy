# 执行计划：writeback 候选的 compile 车道

前置：`prd.md`（需求与验收）、`design.md`（技术决策）。上下文清单见 `implement.jsonl` / `check.jsonl`。

## 前置阅读（实施前必读）

1. `.trellis/spec/backend/index.md` 及其 Pre-Development Checklist 指向的五份 guide
2. `src/kb/compile/legacy_impl.rs:675`（`build_draft_packages` 的脚手架范式）与 `src/kb/navigation.rs`（build/write 双函数 + 审计事件范式）
3. `design.md` 的资格判定表与 provenance 字段设计

## 执行步骤

### Step 1 — 核心计算模块（dry-run 先行）

- [x] 新建 `src/kb/compile/writeback.rs`：`build_writeback_scaffolds(vault_root) -> Result<Value>`
  - 复用 `collect_markdown_records` / `list_field`；items 按 path 字典序
  - 按 design.md 资格表逐条判定，eligible 项内联生成草稿与元包的完整内容行
  - 零时间戳、BTreeMap/显式排序、formatter-stable markdown 风格
- [x] `src/kb/compile/mod.rs` 注册模块并导出
- [x] `src/cli/args.rs`：`CompileCommand::Writeback { vault, #[arg(long)] write: bool }`
- [x] `src/cli/dispatch.rs`：照抄 `ReviewCommand::Indices` 臂的接线形状
- 验证：`cargo run -- --json compile writeback evals/skills/obsidian-notes-karpathy/fixtures/writeback-backlog` 输出 eligible 项与 skip 分类
- 回滚点 A：以上为纯新增，revert 即回滚

### Step 2 — 写路径 + 状态推进 + 审计

- [x] `write_writeback_scaffolds(vault_root, payload)`：写草稿 + 元包（`crate::common::write_markdown`）
- [x] 归档产物外科式行级编辑：`writeback_status` 行替换 + `writeback_draft` 指针插入，其余字节保留
- [x] `audit_log::append_event(vault, "compile_writeback", …)`；零脚手架时跳过
- 验证：临时拷贝 fixture → `--write` → 草稿/元包/状态/审计四面齐 → 复跑零 eligible 且文件字节不变

### Step 3 — health 跳过集对齐

- [x] `src/kb/health/engine.rs:244`：跳过集 `"compiled" | "rejected"` → `"compiled" | "drafted" | "reviewed" | "rejected"`
- 验证：推进后 vault 跑 `review lint` 无该产物的 `writeback_backlog` issue；`tests/query_health.rs` 既有断言不回归

### Step 4 — 测试固化

- [x] 新增 `tests/compile_writeback.rs`，按 design.md 测试计划 6 条落用例（幂等复跑含目录字节级快照对比，沿用 `tests/review_indices.rs` 的 tree_snapshot 手法）
- [x] 不改共享 `writeback-backlog` fixture；资格边界变体在临时 vault 内构造
- 验证：`just test` 全绿
- **评审门 1**：dry-run/write/幂等三个 JSON 样例成形后再进契约同步

### Step 5 — 契约与文档同步

- [x] `skills/kb-compile/SKILL.md`（source discovery + 写面措辞 + writeback lane 小节）
- [x] `skills/kb-query/SKILL.md`（writeback contract 下一跳）
- [x] `skills/kb-review/SKILL.md`（maintenance 对积压的处置动作）
- [x] `references/query-writeback-lifecycle.md`（drafted 属主 + simple path 落命令）
- [x] `references/draft-schema.md`（`writeback_source` 条件字段 + 检查单措辞）
- [x] `scripts/skill-contract-registry.json`（kb-compile writes 登记）
- [x] `README.md` / `README_CN.md` / 根 `CLAUDE.md` 对齐
- 验证：`cargo run -- --json dev contract-validate` + `cargo run -- --json dev audit-skills` 通过；docs 页若被 contract-validate 点名则同步

### Step 6 — 全量收口

- [x] `just ci`（lint + test + docs-build）全绿
- [x] 对照 `prd.md` Acceptance Criteria 逐条勾选
- [x] 分 commit：`feat(知识库): 新增 compile writeback 写回车道`（代码+接线+health 对齐）/ `test(知识库): …` / `docs(知识库): …`

## 验证命令速查

```bash
just test
just ci
cargo run -- --json compile writeback <vault> [--write]
cargo run -- --json review queue <vault>
cargo run -- --json review lint <vault>
cargo run -- --json dev contract-validate
cargo run -- --json dev audit-skills
```

## 风险与注意

- 外科式 frontmatter 编辑必须只动目标行：状态行匹配用行首 `writeback_status:` 前缀，不做 YAML 重序列化。
- PostToolUse Markdown 格式化 hook 会整形手写模板/文档；生成器输出保持 formatter-stable 风格（上一任务已验证的对策）。
- `load_markdown` 的相对路径守卫 bug（`src/kb/markdown.rs:56`）是既有问题：测试与验证一律传绝对路径，不在本任务顺手修。
- 不改 `compile scan`/`build` 与 automation 的任何 JSON 形状。
