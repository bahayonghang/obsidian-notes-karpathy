# 设计：确定性检索索引重建命令

对应 `prd.md`。证据档案：`../07-08-llm-wiki-gap-optimization/research/llm-wiki-gap-analysis.md`。

## 决策总览

1. **新增子命令 `onkb review indices <vault-root> [--write]`**（`ReviewCommand::Indices`），不扩展 `review governance`。
   - 理由：`governance` 在 skill 契约中已特指 QUESTIONS/GAPS/ALIASES/ENTITIES/RELATIONSHIPS 族；导航索引族（INDEX/CONCEPTS/SOURCES/TOPICS/RECENT）语义不同、消费者不同（`kb-query` 检索第一跳）。独立子命令使 `scheduled-health` 可分别调用、审计事件独立、契约措辞不混淆。
2. **镜像 governance 的双函数范式**（`src/kb/governance.rs:67` 与 `:612`）：
   - `build_navigation_indices(vault_root) -> Result<Value>` — 纯计算：扫 live 记录 → 生成每个索引文件的完整内容（`files` map）→ 与 on-disk 内容对比得出漂移状态。
   - `write_navigation_indices(vault_root, payload) -> Result<Vec<String>>` — 落盘 + `audit_log::append_event`。
   - 放置于新模块 `src/kb/navigation.rs`（`governance.rs` 已 24K，遵循 spec 的模块拆分习惯；复用 `collect_markdown_records` / `live_records` / `list_field` 等既有 helper）。

## 每个文件的重建规则

| 文件 | 处理 | 数据来源与规则 |
| --- | --- | --- |
| `wiki/live/indices/INDEX.md` | 全量重建 | 索引族链接（现模板形状）+ 每族页面计数（concepts/sources/topics/…） |
| `wiki/live/indices/CONCEPTS.md` | 全量重建 | `wiki/live/concepts/**` 每页一行：wikilink + title（frontmatter `title`，缺省 basename）+ 一行摘要（frontmatter 摘要字段存在时）；按 canonical slug / path 排序 |
| `wiki/live/indices/SOURCES.md` | 全量重建 | `wiki/live/summaries/**`：wikilink + title + `compiled_from` / `capture_sources` 溯源提示 |
| `wiki/live/indices/TOPICS.md` | 全量重建 | `wiki/live/topics/**` 目录 |
| `wiki/live/indices/RECENT.md` | 全量重建 | 全 live 页按 `approved_at`（缺省回退 `last_reviewed_at`）降序取前 20（同刻并列按 path 排序）；两字段皆缺的页不入列，计入 JSON `unlisted_count` |
| `wiki/live/indices/EDITORIAL-PRIORITIES.md` | **排除** | 编辑判断面（人工/LLM 拥有），仅在 JSON 报 `excluded` |
| `wiki/index.md` | **受管区块** | 仅替换 `<!-- onkb:indices:begin -->` … `<!-- onkb:indices:end -->` 之间内容（Approved Live Indices 链接 + 计数）。无标记的存量文件不写，报 `unmanaged`——避免覆盖用户/LLM prose。`kb-init` 资产模板同步加标记 |

governance 族五个索引不在本命令范围内（已由 `review governance` 拥有）。

## 确定性姿态

- 与 governance 相同：**生成内容零时间戳**（`governance.rs` 全文件无 `now_iso`/`generated_at`，本命令沿用；时间戳只出现在审计事件里）。现模板 frontmatter 的 `generated_at: {{GENERATED_AT}}` 在重建产物中移除，改为固定 `managed_by: "onkb review indices"` 标识。
- 排序容器用 `BTreeMap`/显式 sort，保证同一 vault 状态重复运行逐字节一致（PRD 验收第 2 条的实现基础）。
- 产物走既有 obsidian-safe markdown 写入 helper（`crate::common::write_markdown` / `markdown.rs` 约束）。

## JSON 输出形状

沿用 review 族命令风格（机器可断言，AGENTS.md 要求）：

```json
{
  "action": "review_indices",
  "vault": "<root>",
  "write": false,
  "files": {
    "wiki/live/indices/INDEX.md":   { "status": "in_sync | drifted | missing" },
    "wiki/live/indices/RECENT.md":  { "status": "drifted", "unlisted_count": 2 },
    "wiki/live/indices/EDITORIAL-PRIORITIES.md": { "status": "excluded" },
    "wiki/index.md":                { "status": "unmanaged | in_sync | drifted" }
  },
  "drift_count": 1,
  "written_paths": []
}
```

dry-run（默认）只报告；`--write` 落盘漂移文件并填 `written_paths`，审计 action 定为 `rebuild_navigation_indices`（对齐 `.trellis/spec/backend/logging-guidelines.md` 与 `write_governance_indices` 的事件形状：`written_paths` + 计数字段）。

## 自动化接线

`src/kb/automation.rs:29` 的 `"scheduled-health"` 臂追加导航索引检查：dry-run 结果并入该模式 payload（新键 `navigation_indices`）；`--write` 时执行重建。`references/automation-hooks.md` 的 `scheduled_health` 条目同步补一行。

## 契约与文档同步面

- `skills/kb-review/SKILL.md`：baseline 命令清单加 `onkb --json review indices <vault-root>`；maintenance 模式职责列表明确"检索导航索引确定性重建"。
- `skills/kb-query/SKILL.md`：导航入口小节注明索引由 `onkb` 确定性重建，怀疑漂移时先跑 `review indices`。
- `skills/obsidian-notes-karpathy/scripts/skill-contract-registry.json`：kb-review 的 baseline commands / 写面登记。
- `references/automation-hooks.md`、`references/search-upgrades.md`（Stage 1 注记索引可信度来源）。
- `skills/kb-init/assets/wiki/index.md` 与 `indices/*.md` 模板：加受管标记、frontmatter 对齐新形状（注意 `src/kb/init/legacy_impl.rs` 的模板渲染变量）。
- `README.md` / `README_CN.md` / `CLAUDE.md`：Deterministic helpers 清单与 Karpathy alignment 表补一行。

## 兼容与回滚

- 纯新增命令，不改既有命令语义与 JSON 形状。
- 存量 vault：`wiki/index.md` 无标记 → `unmanaged`（不破坏）；索引文件内容被 LLM 手写过 → 首次 `--write` 即转为机器拥有面（契约中明示这五个文件自此归 `onkb` 管理，编辑判断留在 `EDITORIAL-PRIORITIES.md` 与 `wiki/index.md` 非受管区）。
- 回滚 = revert 对应 commit；无数据迁移、无状态文件。

## 已否决的备选

- **扩展 `review governance` 覆盖导航索引**：governance 措辞在契约里已有精确含义，混入导航族会让 skill 路由与审计语义变糊。
- **计算视图（查询时即时生成、不落盘）**：Obsidian 前端需要真实文件支撑导航与 graph view，落盘保留。
- **`wiki/index.md` 全文件重建**：会覆盖 prose 与人工导航内容，改为受管区块方案。
