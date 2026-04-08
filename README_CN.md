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
- `kb-compile`：把 raw 编译到 `wiki/drafts/`
- `kb-review`：批准 / 拒绝 / 升级人工复核，并重建 `wiki/briefings/`
- `kb-query`：只从 `wiki/live/` 检索和产出
- `kb-health`：审计 `wiki/live/`、`wiki/briefings/`、`outputs/qa/`、`outputs/reviews/`

## 关键契约

- `raw/` 永远不可变。
- `MEMORY.md` 是协作记忆层，不是检索真相层。
- `kb-compile` 只能写 `wiki/drafts/`。
- 只有 `kb-review` 可以把草稿提升到 `wiki/live/`。
- `wiki/briefings/` 只能从 approved live 生成。
- `kb-query` 读取 live、briefings 和历史 Q&A，不把 drafts 当真相层。
- 旧的 legacy-layout vault 会被识别出来，并应先迁移再进入正常工作流。

## 必需支撑层与可选输出

初始化最小支撑层是 `raw/`、`wiki/drafts/`、`wiki/live/`、`wiki/briefings/`、`wiki/index.md`、`wiki/log.md`、`outputs/reviews/`、`AGENTS.md`、`CLAUDE.md`。

其余输出面按阶段按需创建：

- `outputs/qa/`：`kb-query` 归档的持久问答
- `outputs/content/`：`kb-query` 生成的对外交付物
- `outputs/health/`：`kb-health` 输出的体检报告
- `MEMORY.md`：推荐的协作记忆与编辑上下文

高价值回答和对外内容也可以携带结构化 writeback 候选，供后续 compile / review 决定是否回流进 wiki。

## 路由职责

- `kb-init` 负责初始化、修复和 legacy-layout 迁移。
- `kb-compile` 负责 raw 到 draft 的更新，也接受 bootstrap 阶段的 `raw/*.md`。
- `kb-review` 是即时处理 pending drafts 与 briefing 重建的草稿提升门。
- `kb-query` 只读取 approved live 层。
- `kb-health` 是更长期的维护入口：默认先出报告，处理 drift、backlog pressure、provenance 审计，以及 approved surfaces 上的安全机械修复。

## 确定性脚本

- `skills/obsidian-notes-karpathy/scripts/skill-contract-registry.json`
- `scripts/detect_lifecycle.py`
- `scripts/scan_compile_delta.py`
- `scripts/scan_review_queue.py`
- `scripts/scan_query_scope.py`
- `scripts/lint_obsidian_mechanics.py`

## 安装

```bash
cp -r skills/* ~/.claude/skills/
cp -r skills/* ~/.codex/skills/
```
