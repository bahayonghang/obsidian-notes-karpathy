# LLM Wiki 对标差距分析（2026-07-08）

调研目的：以 Karpathy LLM Wiki 原始模式与社区实践教训为基准，核查本仓库实现的覆盖与缺口。本文件是父任务 `07-08-llm-wiki-gap-optimization` 及其子任务的共享证据档案。

## 一、外部基准

### 1. Karpathy 原始 gist（权威定义）

来源：<https://gist.github.com/karpathy/442a6bf555914893e9891c11519de94f>（2026-04-02 发布，配套 X 帖 1600 万+浏览）

模式要点：

- 三层：raw sources（不可变）/ the wiki（LLM 全权维护）/ the schema（CLAUDE.md 等，一个与用户共同演化的文件）
- 三操作：
  - **Ingest**：读源 → 与用户讨论 → 写 summary 页 → 更新 index → 跨 wiki 更新 entity/concept 页（单源触达 10-15 页）→ 追加 log
  - **Query**：查 index → 读页 → 带引用综合回答；**好答案应写回 wiki 成为新页**（"your explorations compound"）
  - **Lint**：矛盾、被新源取代的陈旧结论、孤儿页、缺页概念、缺失交叉引用、可用 web search 填补的数据缺口
- `index.md`（内容目录，每次 ingest 更新，查询先读）+ `log.md`（时间线，可 grep 前缀 `## [date] op | title`）
- 规模判断：index-first 到 ~100 sources / 数百页够用，之后建议 qmd（本地 BM25/hybrid + LLM rerank，CLI + MCP）
- tips：Obsidian Web Clipper、图像本地化（`raw/assets/` + LLM 分步查看图像）、graph view、Marp、Dataview、git
- gist 刻意抽象："pick what's useful, ignore what isn't"——schema 保持极简并与用户共同演化

### 2. 社区验证的失败模式

| 失败模式 | 来源 | 要点 |
| --- | --- | --- |
| **Index drift** | jermolene（TiddlyWiki 作者）于 HN，转引自 [artificial-ideas.com](https://artificial-ideas.com/learn/karpathy-llm-wiki/) | LLM 跨会话维持 index.md 同步"是 LLM 不擅长的事，陈旧化跨会话蔓延"；缓解 = lint 时确定性全量重建（数百源规模下够用），或用计算视图取代物化索引 |
| **KB poisoning / 自引用污染** | [Anand Lahoti](https://foundanand.medium.com/the-hidden-flaw-in-karpathys-llm-wiki-e3a86a94b459) | 摘要引用摘要，回到原始源的 chain of custody 悄然断裂；"内部一致性是谎言"；write-time 综合是"向未来正确性借贷" |
| **Confidently-wrong 无法被 lint 捕获** | HN Abby_101，Mehul Gupta | lint 抓矛盾，抓不到"自洽但错误"；六个月后你有一批"自信地错着"的条目 |
| **全量 lint token 成本** | [proudfrog](https://proudfrog.com/en/insights/llm-wiki-skeptics-guide)、[youcanbuildthings](https://youcanbuildthings.com/articles/llm-wiki-maintenance-lint/) | 100 篇 × 3K tokens ≈ 300K tokens/次全量语义 lint；delta-lint（只查变更页+链接邻居）省 ~80%；~100 条目是切换阈值；"1 → 4 → 16"错误级联，周度 lint 在成本有界时切断 |
| **Append-only 陈旧化** | [Eugeniu Ghelbur](https://theaioperator.io/p/i-rebuilt-karpathys-llm-wiki-heres) | 只增不改的 wiki 是"带内链的日志簿"；实体页不重写为当前最优答案，未来查询得到含糊对冲回答；修复 = 重写/调和/定时代理 |
| **引用精度** | proudfrog | 人类 wiki 可回溯到"第 47 页第 3 段"；LLM wiki 引用本身可能错——"auditing the auditor"；机械校验（摘录存在性、链接有效性、mtime 新旧）可自动化 |
| **Agent 生成 markdown 结构噪声** | [Leandro Bernardo](https://pub.towardsai.net/i-built-karpathys-llm-wiki-twice-once-as-code-once-as-a-md-heres-what-each-one-gives-up-08b31170999a) | 似是而非的 markdown（双 H1、YAML 冒号炸裂、指向空处的链接、重生成时段落顺序漂移）污染每个下游自动化阶段；规模化需要确定性管线 |
| **Ingest 成本前置** | [Adi Insights](https://pub.towardsai.net/i-used-karpathys-llm-wiki-to-build-a-research-brain-that-updates-itself-ff02dda47335) | 单源 ingest ≈ 5-8× 源 token 量（触达 ~12 文件）；每次 ingest 后 `git diff` 人审 90 秒是真正的安全网（两周抓到 2 个错误） |
| **信任崩塌恢复路径** | proudfrog、Adi Insights | raw 不可变 ⇒ wiki 可视为可再生缓存，失去信任可整体重建——这是以往笔记工具没有的属性 |

其他参照实现：[green-dalii/obsidian-llm-wiki](https://github.com/green-dalii/obsidian-llm-wiki)（Obsidian 插件：entity/concept 页 + 会话式查询，检索用 wikilink 图上的 Personalized PageRank，零 embedding 成本、支持本地离线模型）。

## 二、仓库覆盖核查（证据）

| 检查点 | 判定 | 证据 |
| --- | --- | --- |
| 检索入口索引确定性重建 | **缺失** | `wiki/index.md`、`wiki/live/indices/{INDEX,CONCEPTS,SOURCES}.md` 仅出现在 init 脚手架 `src/kb/init/legacy_impl.rs:62-67` 与契约路径清单 `src/dev/contract.rs:15-20`；无任何 refresh/rebuild 写入路径。`review governance` 仅刷新治理索引（QUESTIONS/GAPS/ALIASES/ENTITIES/RELATIONSHIPS，见 `skills/kb-review/SKILL.md:68`）。`src/kb/index.rs` 是内存 wikilink 解析注册表，与 index.md 无关 |
| 确定性 lint 覆盖 | **良好** | `src/kb/health/rules/link_integrity.rs`：`broken_wikilink_issues`、`orphan_page_issues:52`、`alias_wikilink_table_issues`；`src/kb/health/signals.rs:6-8`：`stale_qa`、`volatile_page_stale`、`supersession_gap`、`review_backlog`、`weak_live_sources`、`writeback_backlog`、`procedural_promotion_gap`；另有 `rules/identity.rs` |
| 语义 lint 增量范围 | **缺失** | `src/kb` 全树无 `incremental|delta_lint|last_lint|lint_state|since_last` 匹配；maintenance 模式无"本轮应审查页面集"输出 |
| 查询排序 | **薄弱** | `src/kb/query.rs:181-298` `rank_query_candidates`：`score = lexical_overlap*10 + metadata_score*2 + graph_score`，无分词、无 BM25、无 PPR；中文场景词面重叠近乎失效。`references/search-upgrades.md` Stage 3（qmd/DuckDB）/ Stage 4（hybrid）为 prose 建议，无实现 |
| 写回闭环 | **断裂** | 契约完备：`references/query-writeback-lifecycle.md`（6 状态）、health 有 `writeback_backlog_issues`；但 `skills/kb-compile/SKILL.md:67-77` source discovery 只接受 `raw/**`，`outputs/qa/**` 非法输入；`pending → drafted` 无属主 |
| 引用粒度 | **部分** | `references/summary-template.md:66-95` 要求 Key Evidence 摘录（"{quote} - {where}"）；`references/draft-schema.md:19` 仅粗粒度 `evidence_coverage`；无"摘录存在于 raw 源"的机械校验规则 |
| 自动化 | **部分** | `references/automation-hooks.md`：`onkb review automation --mode on_new_source|on_session_end|on_query_archive|scheduled-health [--write]` harness 已在；缺调度接线示例（cron/CI/hook）与 delta 范围 |
| rebuild/replay | **缺失** | `src/cli/args.rs` 无 `rebuild|replay|regenerate` 命令；live 层状态不可从 raw + `outputs/reviews/**` 重放 |
| markdown 结构保障 | **已有** | `src/kb/markdown.rs`（"obsidian-safe markdown"，含 stale/broken 处理逻辑 6 处匹配） |
| 契约上下文成本 | **超重** | 执行前必读清单：kb-review 24 文件（`skills/kb-review/SKILL.md:34-59`）、kb-compile 18 文件（`SKILL.md:30-51`）、router 14 文件（`SKILL.md:31-48`）；references/ 共 30+ 文件 |

## 三、结论映射

- P1（index drift）→ 子任务 `07-08-deterministic-index-rebuild`
- P2（写回断裂）→ 子任务 `07-08-writeback-compile-lane`
- P3（契约超重）→ 子任务 `07-08-skill-context-slimming`
- P4 查询排序 / P5 delta lint / P6 引用校验 / P7 replay / P8 调度接线 / P9 图像契约 → 父任务 PRD backlog
- 评审门 + 确定性 CLI 是本仓库对社区两大最重担忧（poisoning、不可复现）的既有优势，所有优化不得削弱

## 四、外部资料清单

- Karpathy gist: <https://gist.github.com/karpathy/442a6bf555914893e9891c11519de94f>
- Karpathy X 帖: <https://x.com/karpathy/status/2039805659525644595>
- VentureBeat 报道: <https://venturebeat.com/data/karpathy-shares-llm-knowledge-base-architecture-that-bypasses-rag-with-an>
- 怀疑论综述（成本/污染/引用精度）: <https://proudfrog.com/en/insights/llm-wiki-skeptics-guide>
- delta-lint 成本测算: <https://youcanbuildthings.com/articles/llm-wiki-maintenance-lint/>
- KB poisoning: <https://foundanand.medium.com/the-hidden-flaw-in-karpathys-llm-wiki-e3a86a94b459>
- append-only 批评与五扩展: <https://theaioperator.io/p/i-rebuilt-karpathys-llm-wiki-heres>
- 确定性管线对比: <https://pub.towardsai.net/i-built-karpathys-llm-wiki-twice-once-as-code-once-as-a-md-heres-what-each-one-gives-up-08b31170999a>
- 两周实测（ingest 成本、git diff 安全网）: <https://pub.towardsai.net/i-used-karpathys-llm-wiki-to-build-a-research-brain-that-updates-itself-ff02dda47335>
- HN/社区综述: <https://artificial-ideas.com/learn/karpathy-llm-wiki/>
- 参照插件: <https://github.com/green-dalii/obsidian-llm-wiki>
