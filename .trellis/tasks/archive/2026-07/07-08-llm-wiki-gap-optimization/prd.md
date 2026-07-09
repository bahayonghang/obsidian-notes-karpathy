# 对标 Karpathy LLM Wiki 模式与社区教训的实现优化

## Goal

以 Karpathy 的 LLM Wiki 原始模式（gist）与 2026-04 以来社区实践暴露的失败模式为基准，修补本仓库实现中已验证的结构性缺口，同时不削弱本仓库的核心差异化设计（`draft -> review -> live` 评审门与确定性 `onkb` CLI）。

本任务为父任务：持有问题清单、跨子任务验收口径与最终集成评审。实现工作由子任务承载。

## 问题清单（按优先级）

每条问题都有外部证据（社区/原始模式）与仓库证据（文件:行）双重支撑。详细证据链见 `research/llm-wiki-gap-analysis.md`。

### 高优先级 → 已建子任务

- **P1 检索入口索引漂移**：`wiki/index.md` 与 `wiki/live/indices/{INDEX,CONCEPTS,SOURCES,TOPICS}.md` 仅在 init 脚手架时写入（`src/kb/init/legacy_impl.rs:62-67`），之后全靠 LLM 手工同步；`review governance` 只刷新治理索引（QUESTIONS/GAPS/ALIASES/ENTITIES/RELATIONSHIPS）。而 `kb-query` 的第一步导航就是读这些索引（`skills/kb-query/SKILL.md:54-64`）。这是 TiddlyWiki 作者在 HN 点名的 index drift 失败模式：LLM 跨会话维持 index 同步不可靠，陈旧索引直接毒化检索入口。→ 子任务 `07-08-deterministic-index-rebuild`
- **P2 写回闭环有契约无通路**：`query-writeback-lifecycle.md` 定义了 6 状态生命周期，health 有 `writeback_backlog_issues` 检测，但 `kb-compile` 的 source discovery 只接受 `raw/**` 捕获（`skills/kb-compile/SKILL.md:67-77`），`outputs/qa/**` 不是合法 compile 输入。candidate 从 `pending → drafted` 没有任何 skill/命令拥有这一步。Karpathy 模式中 "good answers can be filed back into the wiki" 是复利的一半来源，当前只在 prose 里成立。→ 子任务 `07-08-writeback-compile-lane`
- **P3 契约层上下文超重**：kb-review 要求执行前先读 24 个文件（`skills/kb-review/SKILL.md:34-59`），kb-compile 18 个，router 14 个。Karpathy 的 schema 刻意极简（一个共同演化的文件）。当前每次 skill 调用上下文成本巨大，且实际执行中模型大概率跳读，契约退化为名义契约——相当于把 Karpathy 消灭掉的维护负担搬进了契约层。→ 子任务 `07-08-skill-context-slimming`

### 中优先级 → backlog（另行同意后再立子任务）

- **P4 查询排序对中文近乎失效**：`rank_query_candidates` 是朴素词面重叠打分（`lexical_overlap*10 + metadata*2 + graph`，`src/kb/query.rs:181-298`），中文无分词场景下 lexical overlap 几乎不工作；`search-upgrades.md` 的 Stage 3/4（qmd/BM25/hybrid）只是 prose 建议无实现。社区参照：gist 建议超过 ~100 sources 后接 qmd；green-dalii 插件用 wikilink 图上的 Personalized PageRank（零 embedding 成本，思路与本仓库 graph_score 同源，可加强）。
- **P5 语义 lint 无增量范围**：Rust 确定性检查全量跑没问题（无 token 成本），但 kb-review maintenance 模式交给 LLM 的语义审查（矛盾、confidently-wrong）没有 delta 范围输出。社区实测：100 篇全量语义 lint ≈ 300K tokens/次；delta-lint（只查上次之后变更页 + 链接邻居）省约 80%。health engine 可基于 `outputs/audit/operations.jsonl` 时间戳输出"本轮应审查页面集"。
- **P6 引用精度无机械校验**：`summary-template.md` 要求 Key Evidence 摘录，但 `draft-schema.md` 只有粗粒度 `evidence_coverage`，没有 lint 规则验证"摘录字符串确实存在于 raw 源文件"。社区称之为 "auditing the auditor" 问题；逐条 quote 存在性检查是可自动化的。

### 低优先级 → backlog 记录

- **P7 无 rebuild/replay**：不能从 `raw/` + `outputs/reviews/**` 决策台账重放重建 live 层（社区："wiki 是可再生缓存"是信任崩塌后的修复路径）。
- **P8 定时维护缺接线指引**：`onkb review automation --mode scheduled-health` harness 已存在，但无 cron/CI/hook 接线示例文档；社区认为定时维护是防懈怠关键。
- **P9 图像资产契约薄**：仅注册 `raw/**/assets/*`；Karpathy tips 中"compile 时 LLM 查看本地图像获取上下文"无对应契约。

### 明确不设任务（已覆盖良好）

raw 不可变 + manifest、评审门、来源完整性元数据（`source_hash`/`source_mtime`/`last_verified_at`/`possibly_outdated`）、孤儿页/断链/别名确定性检测（`orphan_page_issues`、`broken_wikilink_issues`）、审计日志、obsidian-safe markdown、写回积压检测信号。社区最担心的两点——自引用污染（KB poisoning）与不可复现——本仓库靠评审门 + 确定性 CLI 已优于原始模式。

## Requirements

- 子任务各自独立可验证，父任务不直接承载实现。
- 所有改动不得削弱 `draft -> review -> live` 真值边界，不得使 `raw/` 可变。
- backlog 项（P4-P9）不自动升级为子任务，需另行确认后用 `task.py create --parent` 追加。
- 契约措辞变更需保持 `README.md`、`README_CN.md`、`CLAUDE.md` 对齐（AGENTS.md 要求）。

## Acceptance Criteria

- [x] 三个子任务全部完成并归档（`archive/2026-07/`：deterministic-index-rebuild、writeback-compile-lane、skill-context-slimming）
- [x] 跨子任务集成评审通过：`just ci`（lint + test + docs-build）全绿（三个子任务合入后终跑确认，87 测试）
- [x] README 的 Karpathy alignment 表反映新增能力（`Index upkeep` 行 + `Answers filed back` 行）
- [x] backlog 项（P4-P9）在本 PRD 中有明确处置记录（见下方"Backlog 处置记录"）

## Backlog 处置记录（2026-07-09）

P4-P9 全部**保留为 backlog**，不自动升级：每项都需另行同意后用 `task.py create --parent` 追加子任务。理由与优先级建议：

- **P4 查询排序（中文）**：保留。P1 落地后导航索引可信，P4 的收益窗口在库规模超过 ~100 sources 时打开；建议届时优先做 graph_score 加强（PPR 思路，零 embedding 成本）。
- **P5 delta 语义 lint**：保留。P2 的审计事件（`compile_writeback` 等）已在 `operations.jsonl` 累积时间戳素材，实现"本轮应审查页面集"时可直接复用。
- **P6 引用精度机械校验**：保留。quote 存在性检查可复用 P1/P2 已验证的确定性双函数范式，属低风险高确定性项。
- **P7 rebuild/replay**：保留。依赖 `outputs/reviews/**` 台账形状稳定，宜在库出现真实规模后再评估成本收益。
- **P8 定时维护接线示例**：保留。`scheduled-health` harness 与 `automation-hooks.md` 已就绪，缺的只是 cron/CI 文档页，可与 P5 合并成一个小任务。
- **P9 图像资产契约**：保留。等 compile 场景真实出现本地图像上下文需求再立项。

## Notes

- 证据链与外部资料链接见 `research/llm-wiki-gap-analysis.md`。
- 三个子任务均为复杂任务：`task.py start` 前需补 `design.md` + `implement.md`。
