# Obsidian Notes Karpathy

面向多 Agent 的、带显式审校门的 Obsidian 知识库技能包。

```text
raw/            -> 不可变的人类/Agent 捕获层
wiki/drafts/    -> 编译出的草稿知识层
wiki/live/      -> 已批准的长期知识层
wiki/briefings/ -> 只从 live 生成的角色 briefing
outputs/        -> reviews、Q&A、health 报告和对外交付物
```

核心思想不再只是“把笔记编译成 wiki”，而是“把生产和裁决分开，避免未审草稿进入可复用真相层并持续复利。”

## 技能列表

- `obsidian-notes-karpathy`：生命周期路由
- `kb-init`：初始化、修复或迁移到 V2
- `kb-compile`：把 raw 编译到 `wiki/drafts/`
- `kb-review`：批准 / 拒绝 / 升级人工复核，并重建 `wiki/briefings/`
- `kb-query`：只从 `wiki/live/` 检索和产出
- `kb-health`：审计 `wiki/live/`、`wiki/briefings/`、`outputs/qa/`、`outputs/reviews/`

## 关键契约

- `raw/` 永远不可变。
- `kb-compile` 只能写 `wiki/drafts/`。
- 只有 `kb-review` 可以把草稿提升到 `wiki/live/`。
- `wiki/briefings/` 只能从 approved live 生成。
- `kb-query` 读取 live、briefings 和历史 Q&A，不把 drafts 当真相层。
- 旧的 V1 Vault 会被识别出来，并应先迁移再进入正常 V2 工作流。

## 确定性脚本

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
