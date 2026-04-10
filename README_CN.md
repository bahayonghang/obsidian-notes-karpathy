# Obsidian Notes Karpathy

面向多 Agent 的、带显式审校门的 Obsidian 知识库技能包。

```text
raw/            -> 不可变的人类/Agent 捕获层
MEMORY.md       -> 协作记忆与编辑上下文
wiki/drafts/    -> 编译出的草稿知识层
wiki/live/      -> 已批准的长期知识层
wiki/briefings/ -> 只从 live 生成的角色 briefing
outputs/        -> reviews、Q&A、health 报告和对外交付物
```

核心思想不再只是“把笔记编译成 wiki”，而是“把生产和裁决分开，避免未审草稿进入可复用真相层并持续复利。”

## 技能列表

- `obsidian-notes-karpathy`：生命周期路由
- `kb-init`：初始化、修复或迁移到 review-gated 布局
- `kb-compile`：把 raw Markdown 捕获编译到 `wiki/drafts/`
- `kb-review`：批准 / 拒绝 / 升级人工复核，并重建 `wiki/briefings/`
- `kb-query`：只从 `wiki/live/` 检索和产出
- `kb-health`：审计 `wiki/live/`、`wiki/briefings/`、`outputs/qa/`、`outputs/reviews/`

## 关键契约

- `raw/` 永远不可变。
- 应把 `raw/` 视为长期素材库；编辑笔记、Q&A 和对外内容应落在下游 surfaces，而不是回写 source captures。
- `MEMORY.md` 是协作记忆层，不是检索真相层。
- `kb-compile` 只能写 `wiki/drafts/`。
- `raw/**/papers/*.pdf` 下的论文 PDF 仍然是 `paper-workbench` 的路由例外，不属于普通 `kb-compile` 入口。
- 只有 `kb-review` 可以把草稿提升到 `wiki/live/`。
- `wiki/briefings/` 只能从 approved live 生成。
- `kb-query` 读取 live、briefings 和历史 Q&A，不把 drafts 当真相层。
- 旧的 legacy-layout vault 会被识别出来，并应先迁移再进入正常工作流。
- alias 对齐、source integrity、stale 页面检查、开放问题跟踪都会被纳入治理规则，但不会绕过 review gate。
- curated hub 和创作者规划面可以存在，但它们仍然是导航 / 维护层，不是绕过真相边界的捷径。

## 为什么不是单层 source/concept/synthesis wiki

这个包**故意不**把新生成的 `wiki/` 页面直接当作长期真相。

而是：

- raw 捕获先变成可审校 drafts
- drafts 只有经过 `kb-review` 才进入 `wiki/live/`
- query 和 publish 默认只建立在 approved summaries 与 live pages 上
- health / reflect 类输出可以发现治理问题，但若要进入长期知识层，仍需走 draft -> review -> live

这样可以保持 provenance 清晰，避免快速摄入直接固化成长期真相。

## 必需支撑层与可选输出

初始化最小支撑层是 `raw/`、`wiki/drafts/`、`wiki/live/`、`wiki/briefings/`、`wiki/index.md`、`wiki/log.md`、`outputs/reviews/`、`AGENTS.md`、`CLAUDE.md`。

其余输出面按阶段按需创建：

- `outputs/qa/`：`kb-query` 归档的持久问答
- `outputs/content/`：`kb-query` 生成的对外交付物
- `outputs/health/`：`kb-health` 输出的体检报告
- `MEMORY.md`：推荐的协作记忆与编辑上下文

可选治理索引如 `wiki/live/indices/QUESTIONS.md`、`GAPS.md`、`ALIASES.md` 可按需创建，用于跟踪开放问题、知识空白和别名映射。

高价值回答和对外内容也可以携带结构化 writeback candidates，供后续 compile / review 决定是否回流进 wiki。

面向创作者的常见工作面可这样映射：

- 素材库 / 网页摘录 -> `raw/`
- 编辑协作记忆 -> `MEMORY.md`
- 可复用的研究问答或写作笔记 -> `outputs/qa/`
- 对外交付物 -> `outputs/content/`
- 长期批准知识 -> `wiki/live/`
- 选题地图 / 规划面 -> `wiki/live/indices/` 或经 review 批准的 hub 页面

## 路由职责

- `kb-init` 负责初始化、修复和 legacy-layout 迁移。
- `kb-compile` 负责 raw 到 draft 的更新，也接受 bootstrap 阶段的 `raw/*.md`。
- `kb-review` 是即时处理 pending drafts 与 briefing 重建的草稿提升门。
- `kb-query` 只读取 approved live 层。
- `kb-health` 是更长期的维护入口：默认先出报告，处理 drift、backlog pressure、provenance 审计、alias 漂移、开放问题堆积，以及 approved surfaces 上的安全机械修复。

## 确定性脚本

- `skills/obsidian-notes-karpathy/scripts/skill-contract-registry.json`
- `scripts/detect_lifecycle.py`
- `scripts/scan_compile_delta.py`
- `scripts/scan_review_queue.py`
- `scripts/scan_query_scope.py`
- `scripts/lint_obsidian_mechanics.py`
- `scripts/runtime_eval.py`
- `scripts/trigger_eval.py`

## 安装

```bash
cp -r skills/* ~/.claude/skills/
cp -r skills/* ~/.codex/skills/
```
