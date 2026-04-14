# Obsidian Notes Karpathy

面向多 Agent 的、带显式审校与批准流程的 Obsidian 知识库技能包。

这个 bundle 也面向那些不会先说 `kb-init`、`kb-compile` 的用户。他们可能会说“帮我做个 LLM Wiki”“做个 Karpathy 风格知识库”“把 Obsidian 当 IDE”“做个知识编译器”“搭个个人知识库 / second brain”，这些都应该自然落到正确技能。

```text
raw/            -> 不可变的人类/Agent 捕获层
raw/_manifest.yaml -> 来源登记表
MEMORY.md       -> 协作记忆与编辑上下文
outputs/episodes/ -> 情节化记忆 / 工作链路结晶
wiki/drafts/    -> 编译出的草稿知识层
wiki/live/      -> 已批准的长期知识层
wiki/live/topics/ -> 已批准知识的浏览层主题图
wiki/live/procedures/ -> 已批准的流程 / playbook 记忆
wiki/briefings/ -> 只从 live 生成的角色简报
outputs/        -> reviews、Q&A、health 报告、审计轨和对外交付物
```

核心思想不再只是“把笔记编译成 wiki”，而是“把生产和裁决分开，避免未审草稿进入可复用真相层并持续复利。”

## 技能列表

- `obsidian-notes-karpathy`：生命周期路由
- `kb-init`：初始化、修复或迁移到带审校与批准流程的布局
- `kb-ingest`：把 raw 来源登记到 `raw/_manifest.yaml`
- `kb-compile`：把 raw Markdown 捕获编译到 `wiki/drafts/`
- `kb-review`：批准 / 拒绝 / 升级人工复核，并重建 `wiki/briefings/`
- `kb-query`：统一读侧入口，负责检索、排序、有依据的回答、归档复用和静态网站导出
- `kb-render`：把已批准知识渲染成 slides / reports / charts / canvas

## 搭配技能矩阵

| 需求 | 核心路由 | 搭配技能 |
| --- | --- | --- |
| 新建、修复或迁移带审校与批准流程的 vault | `kb-init` | 无 |
| 把 raw Markdown、asset、data 登记进 manifest | `kb-ingest` | 无 |
| 把 raw Markdown 编译成 drafts | `kb-compile` | 无 |
| 审校并提升 draft knowledge | `kb-review` | 无 |
| 只从已批准知识层检索 / 排序 / 归档 / 导出静态知识站 | `kb-query` | 无 |
| 把已批准知识渲染成 slides / reports / charts / canvas | `kb-render` | 无 |
| 旧的 `kb-search` 说法 | `kb-query` | 直接吸收到统一查询技能 |
| 旧的 `kb-health` 说法，或已批准知识层的 drift / backlog 维护 | `kb-review` | 走统一治理技能的 `维护模式` |
| 处理 `raw/**/papers/*.pdf` | 不属于核心路由 | `paper-workbench` |

核心 bundle 负责带审校与批准流程的生命周期本身。只有在明显超出核心边界的场景，例如论文 PDF 或 canvas 专项生产时，才切到搭配技能。

## 关键契约

- `raw/` 永远不可变。
- 应把 `raw/` 视为长期素材库；编辑笔记、Q&A 和对外内容应落在下游工作面，而不是回写 source captures。
- `raw/_manifest.yaml` 是跟踪输入来源的来源登记表。
- `MEMORY.md` 是协作记忆层，不是检索真相层。
- `outputs/episodes/` 是情节记忆，不是批准知识。
- `kb-compile` 只能写 `wiki/drafts/`。
- `raw/**/papers/*.pdf` 下的论文 PDF 仍然是 `paper-workbench` 的路由例外，不属于普通 `kb-compile` 入口。
- 只有 `kb-review` 可以把草稿提升到 `wiki/live/`。
- `wiki/live/topics/` 是批准知识的默认浏览层。
- `wiki/live/procedures/` 承载被批准的流程 / 工作法，而不是再塞进概念页。
- `wiki/briefings/` 只能从已批准的 live 页面生成。
- `kb-query` 读取 live、角色简报和历史 Q&A，不把 drafts 当真相层。
- `outputs/audit/operations.jsonl` 是自动化与派生操作的机器可读审计轨。
- 旧版目录结构 vault 会被识别出来，并应先迁移再进入正常工作流。
- alias 对齐、source integrity、stale 页面检查、开放问题跟踪都会被纳入治理规则，但不会绕过 review gate。
- curated hub 和创作者规划面可以存在，但它们仍然是导航 / 维护层，不是绕过真相边界的捷径。

## 为什么不是单层 source/concept/synthesis wiki

这个包**故意不**把新生成的 `wiki/` 页面直接当作长期真相。

而是：

- raw 捕获会先进入 `raw/_manifest.yaml`
- raw 捕获先变成可审校 drafts
- drafts 只有经过 `kb-review` 才进入 `wiki/live/`
- query 和 publish 默认只建立在已批准摘要与 live 页面上
- health / reflect 类输出可以发现治理问题，但若要进入长期知识层，仍需走 draft -> review -> live

这样可以保持 provenance 清晰，避免快速摄入直接固化成长期真相。

## Karpathy 对齐

本项目实现了 [Karpathy 的 LLM Wiki 模式](https://gist.github.com/karpathy/442a6bf555914893e9891c11519de94f)，并增加了一个关键扩展：显式的 review gate（draft → review → live 提升流程）。

| Karpathy 原始模式 | 本项目 | 扩展理由 |
| --- | --- | --- |
| Raw sources（不可变） | `raw/` + `raw/_manifest.yaml` | 增加来源登记表用于跟踪摄入 |
| The wiki（LLM维护） | `wiki/drafts/` → `wiki/live/` | 拆分为草稿层和已批准层，中间有提升门控 |
| The schema（CLAUDE.md） | `AGENTS.md` + `CLAUDE.md` + 共享 `references/` | 扩展为完整的契约注册表 |
| Ingest | `kb-ingest` + `kb-compile` | 将来源登记与草稿编译分离 |
| Query | `kb-query` + `kb-render` | 将有依据的回答与确定性派生物分离 |
| Lint | `kb-review` 维护模式 | 将健康检查提升为一等治理通道 |

核心隐喻保持不变："Obsidian 是 IDE；LLM 是程序员；wiki 是代码库。" 用户负责策展来源和提问。LLM 负责所有让知识持续复利的记账工作。

## 必需支撑层与可选输出

初始化最小支撑层是 `raw/`、`wiki/drafts/`、`wiki/live/`、`wiki/briefings/`、`wiki/index.md`、`wiki/log.md`、`outputs/reviews/`、`AGENTS.md`、`CLAUDE.md`。

其余输出面按阶段按需创建：

- `outputs/qa/`：`kb-query` 归档的持久问答
- `outputs/content/`：`kb-query` 生成的对外交付物
- `outputs/slides/`、`outputs/reports/`、`outputs/charts/`：`kb-render` 产出的确定性派生物
- `outputs/web/`：`kb-query` 导出的静态浏览站
- `outputs/health/`：`kb-review` `维护模式` 输出的体检报告
- `MEMORY.md`：推荐的协作记忆与编辑上下文

可选治理索引如 `wiki/live/indices/QUESTIONS.md`、`GAPS.md`、`ALIASES.md` 可按需创建，用于跟踪开放问题、知识空白和别名映射。

高价值回答和对外内容也可以携带结构化 writeback candidates，供后续 compile / review 决定是否回流进 wiki。

面向创作者的常见工作面可这样映射：

- 素材库 / 网页摘录 -> `raw/`
- 编辑协作记忆 -> `MEMORY.md`
- 工作链路结晶 -> `outputs/episodes/`
- 可复用的研究问答或写作笔记 -> `outputs/qa/`
- 对外交付物 -> `outputs/content/`
- 长期批准知识 -> `wiki/live/`
- 批准后的流程 / playbook -> `wiki/live/procedures/`
- 选题地图 / 规划面 -> `wiki/live/indices/` 或经 review 批准的 hub 页面

## 路由职责

- `kb-init` 负责初始化、修复和旧版目录结构迁移。
- `kb-ingest` 负责来源登记表、manifest 刷新与 deferred intake 说明。
- `kb-compile` 负责 raw 到 draft 的更新，也接受 bootstrap 阶段的 `raw/*.md`。
- `kb-review` 统一负责治理路径：既处理 pending drafts 与角色简报重建，也负责 `维护模式` 下的 drift、backlog、provenance、alias 和已批准层面的安全机械修复。
- `kb-query` 只读取已批准知识层，并负责 search / 有依据的 Q&A / archived Q&A reuse / static web export。
- `kb-render` 负责确定性派生产物。

## 确定性脚本

- `skills/obsidian-notes-karpathy/scripts/skill-contract-registry.json`
- `scripts/detect_lifecycle.py`
- `scripts/scan_ingest_delta.py`
- `scripts/sync_source_manifest.py`
- `scripts/scan_compile_delta.py`
- `scripts/build_draft_packages.py`
- `scripts/bootstrap_review_gated_vault.py`
- `scripts/migrate_legacy_vault.py`
- `scripts/scan_review_queue.py`
- `scripts/scan_query_scope.py`
- `scripts/rank_query_candidates.py`
- `scripts/render_live_artifact.py`
- `scripts/vault_status.py`
- `scripts/render_reference_block.py`
- `scripts/lint_obsidian_mechanics.py`
- `scripts/build_memory_episodes.py`
- `scripts/build_graph_snapshot.py`
- `scripts/runtime_eval.py`
- `scripts/trigger_eval.py`

## 安装

```bash
cp -r skills/* ~/.claude/skills/
cp -r skills/* ~/.codex/skills/
```


