# 设计：writeback 候选的 compile 车道

对应 `prd.md`。证据档案：`../07-08-llm-wiki-gap-optimization/research/llm-wiki-gap-analysis.md`（P2）。

## 决策总览

1. **新增子命令 `onkb compile writeback <vault-root> [--write]`**（`CompileCommand::Writeback`），不扩展 `compile scan` / `compile build`。
   - 理由：scan/build 的 delta 语义（new/changed/unchanged 基于 raw hash）与 counts 契约不适合混入归档产物；`compile scan` 是 registry 登记的 baseline 命令，语义必须保持稳定。独立子命令让审计事件、契约措辞、skill 路由都干净。
   - 已否决：新增 automation mode（车道是深思熟虑的 compile 动作，不是定时体检动作；backlog 检测已由 governance/health 覆盖）。
2. **镜像 governance/navigation 的双函数范式**，放置于新模块 `src/kb/compile/writeback.rs`（`legacy_impl.rs` 已 43.5K，遵循模块拆分习惯）：
   - `build_writeback_scaffolds(vault_root) -> Result<Value>` — 纯计算：扫 `outputs/qa/**` + `outputs/content/**`，逐一判定资格并生成脚手架计划（含每个待写文件的完整内容），不落盘。
   - `write_writeback_scaffolds(vault_root, payload) -> Result<Vec<String>>` — 落盘草稿 + 评审元包、推进 `writeback_status`、`audit_log::append_event`。
   - CLI 接线照抄 `ReviewCommand::Indices` 臂：dry-run 输出 build 结果；`--write` 后回填 `write: true` + `written_paths`。

## 资格判定（全部满足才 eligible）

| 条件 | 不满足时的 `skip_reason` |
| --- | --- |
| 路径在 `outputs/qa/**` 或 `outputs/content/**` | （不在则不进入 items） |
| `writeback_status == "pending"`（精确匹配，小写化后） | `writeback_status_<status>` / `writeback_status_missing` |
| `followup_route == "draft"` | `followup_route_<route>` / `followup_route_missing` |
| `writeback_candidates` 非空 | `no_writeback_candidates` |
| 接地引用非空：取 `source_live_pages`，为空则回退 `sources` 中以 `[[wiki/live/` 或 `[[raw/` 开头的引用 | `missing_live_grounding` |
| 目标草稿路径不存在 | `draft_already_exists`（幂等护栏，永不覆盖既有草稿） |

PRD 约束逐条落点：`rejected` / `triaged` 不被拾取（第 2 行）；`followup_route: review` 的积压归 `kb-review` maintenance，不进草稿车道（第 3 行）；自引用污染由接地规则阻断（第 5 行——草稿证据基座是批准页/raw，绝不是归档产物本身）。

## 脚手架产物（每个 eligible 产一组，slug = 归档文件 basename）

| 文件 | 内容 |
| --- | --- |
| `wiki/drafts/summaries/writeback/<slug>.md` | 写回草稿脚手架（见下方 frontmatter 设计）；落在 `REVIEWABLE_DRAFT_ROOTS` 内（`src/kb/review.rs:10`），review queue 自动拾取 |
| `wiki/drafts/indices/packages/writeback/<slug>.md` | 评审元包，镜像 raw 车道 `build_draft_packages` 的 Review Package 形状 |

### provenance 字段设计（PRD 遗留决策点）

- `compiled_from` + `capture_sources` = **接地引用（批准页/raw 证据）**，不是归档产物。评审者按 draft-schema 检查单沿 `compiled_from` 审核证据时，落在批准层，链路经批准页自身的 provenance 传递回 raw——这是对"summary-of-summary"自引用污染的结构性阻断。
- 新增条件字段 `writeback_source: "[[outputs/qa/<slug>]]"` 承载触发链路：记录"哪个归档产物触发了本草稿"，明确它是 trigger 而非真值来源。`draft-schema.md` 条件字段表补一行，write-time 检查单的 "`compiled_from` and `capture_sources` actually exist under `raw/**`" 措辞放宽为 "exist under `raw/**`（source drafts）or `wiki/live/**`（writeback drafts）"。
- `writeback_candidates` 原样带入草稿，评审者可见预期的持久增量。

### 草稿 frontmatter（覆盖 draft-schema 全部必填字段）

```yaml
title: "Writeback: <question 或 title，缺省 slug>"
draft_id: "writeback--<slug>"
compiled_from: [接地引用列表]
capture_sources: [同上]
writeback_source: "[[outputs/qa/<slug>]]"
writeback_candidates: [带入]
review_state: "pending"
review_score: "0.70"
blocking_flags: []
evidence_coverage: "0.50"
uncertainty_level: "medium"
promotion_target: "semantic"
review_package_meta: "[[wiki/drafts/indices/packages/writeback/<slug>]]"
```

正文骨架（formatter-stable 风格，标题后空行）：`## Writeback Provenance`（触发产物 + 接地页清单 + trigger-not-truth 说明）、`## Core Conclusions` / `## Key Evidence`（占位行，LLM 按 `浓缩 -> 质疑 -> 对标` 填充）、`## Writeback Candidates`（带入清单）。占位措辞镜像 raw 车道（"No core conclusions extracted yet." 式样）。

## 状态推进（`--write` 时）

- 对归档产物做**外科式行级 frontmatter 编辑**：仅替换 `writeback_status:` 行为 `writeback_status: "drafted"`，并在其后插入 `writeback_draft: "[[wiki/drafts/summaries/writeback/<slug>]]"`（键已存在则只更新状态行）；文件其余部分逐字节保留。不做整体 YAML 重序列化（会破坏既有排版与键序）。
- `raw/**` 全程不可触碰；被改写的只有 `outputs/qa|content/**` 的 frontmatter 两行。
- 审计事件：`append_event(vault, "compile_writeback", {written_paths, advanced_sources: [{path, from, to, draft}], eligible_count})`，对齐 `logging-guidelines.md` 与 `rebuild_navigation_indices` 的事件姿态。零脚手架时不写审计事件。

## health 语义对齐（一处既有错位的一并修复）

`src/kb/health/engine.rs:244` 的 `writeback_backlog_issues` 跳过集是 `"compiled" | "rejected"`，与 6 状态生命周期错位：车道推进到 `drafted` 后仍会被记为积压（`missing_or_open_writeback_status`），`reviewed` 同样误报。修复：跳过集改为 `"drafted" | "reviewed" | "rejected"`，保留 `"compiled"` 作为 legacy 同义值兼容。`pending` / `triaged` / 缺失状态仍在积压面（reason 措辞不变）。governance 的积压采集（`None | Some("pending")`）本就一致，不动。

## JSON 输出形状

```json
{
  "action": "compile_writeback",
  "vault_root": "<root>",
  "write": false,
  "counts": { "eligible": 1, "skipped": 2, "total": 3 },
  "items": [
    { "path": "outputs/qa/x.md", "status": "eligible",
      "draft_path": "wiki/drafts/summaries/writeback/x.md",
      "package_path": "wiki/drafts/indices/packages/writeback/x.md",
      "grounding": ["[[wiki/live/concepts/review-gate]]"] },
    { "path": "outputs/qa/y.md", "status": "skipped", "reason": "followup_route_review" }
  ],
  "written_paths": [],
  "advanced_sources": []
}
```

排序：items 按 path 字典序（BTreeMap 采集），同一 vault 状态重复运行输出逐字节一致。

## 契约与文档同步面

- `skills/kb-compile/SKILL.md`：source discovery 增补 writeback 输入行（限定 `pending` + `followup_route: draft`）；非谈判规则的写面措辞扩为"`wiki/drafts/**`、draft indices，及写回车道对 `outputs/qa|content/**` 的 `writeback_status` 推进"；新增 writeback lane 小节（trigger-not-truth、接地规则、命令用法、LLM 后续精修职责）。
- `skills/kb-query/SKILL.md`：writeback contract 小节补下一跳指引（pending 候选由 `onkb --json compile writeback` 拾取）。
- `skills/kb-review/SKILL.md`：maintenance 模式对 `writeback_backlog` 的标准处置动作 = 路由到 compile writeback 车道。
- `references/query-writeback-lifecycle.md`：`drafted` 状态语义补属主命令；"Simple writeback for personal vaults" 的 happy path 落到命令。
- `references/draft-schema.md`：`writeback_source` 条件字段 + 检查单措辞放宽（见上）。
- `scripts/skill-contract-registry.json`：kb-compile `writes` 增 `outputs/qa/`、`outputs/content/`、`outputs/audit/operations.jsonl`（粒度与 kb-review 既有登记一致，SKILL prose 负责收窄为"仅状态推进"）。
- `README.md` / `README_CN.md`：Karpathy 对齐表 "answers filed back into the wiki" 行 + 确定性工具清单加 `compile writeback`。
- 根 `CLAUDE.md`：既有 writeback 候选契约行补属主车道措辞。

## 测试计划落点

新增 `tests/compile_writeback.rs`，基座是既有 `writeback-backlog` fixture（`evals/.../fixtures/writeback-backlog/`，恰好是 pending + `followup_route: draft` + 接地页齐全的场景）：

1. dry-run：eligible 项与计划路径正确，零落盘。
2. 临时拷贝 + `--write`：草稿/元包存在且必填字段齐全、接地引用指向批准页、归档产物推进 `drafted` + `writeback_draft` 指针、审计事件落盘。
3. 复跑 `--write`：零 eligible（reason `writeback_status_drafted`），目录字节级快照不变（幂等）。
4. `review queue` 拾取脚手架（review_state pending）。
5. 资格边界：临时构造 `triaged` / `rejected` / `followup_route: review` / 无候选 变体，断言 skip reason；不改共享 fixture（`tests/query_health.rs` 断言依赖其现状）。
6. health 对齐：推进后 `review lint` 不再报该产物的 `writeback_backlog`。

## 兼容与回滚

- 纯新增命令 + health 跳过集一行修正；不改既有命令 JSON 形状；`compile scan`/`build` 不动。
- 存量 vault：无 pending 候选时命令输出空 eligible，零副作用；已被 LLM 手工 drafted 的产物因状态不是 pending 而天然跳过。
- 回滚 = revert 对应 commit；无数据迁移、无状态文件。

## 已否决的备选

- **扩展 `compile scan/build` 接受归档输入**：污染 raw-delta 语义与 baseline 命令契约。
- **逐 `writeback_candidates` 条目生成多个 typed 草稿（按 wikilink 目标落 concepts/entities）**：候选是自由文本，逐条解析脆弱；拆分/定型是 LLM 在 compile 精修阶段的判断，不是确定性 CLI 的职责。单一综合脚手架 + 候选清单带入已满足评审需要。
- **`compiled_from` 指向归档产物**：评审者按检查单会拿归档产物当证据基座审核，恰是 PRD 点名要阻断的 summary-of-summary 姿态。
- **自动写入 `wiki/log.md` 批次条目**：raw 车道的 `compile build` 也不写 log（log 是 skill 层职责），保持一致。
