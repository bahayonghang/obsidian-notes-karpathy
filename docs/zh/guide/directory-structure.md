# 目录结构

目录按“证据 -> 草稿 -> 批准层 -> 派生输出”分层。

```text
vault/
├── raw/
│   ├── human/
│   └── agents/{role}/
├── MEMORY.md
├── wiki/
│   ├── drafts/
│   ├── live/
│   ├── briefings/
│   ├── index.md
│   └── log.md
├── outputs/
│   ├── reviews/
│   ├── qa/      (query 开始归档后创建)
│   ├── health/  (health 报告开始产出后创建)
│   └── content/ (publish 流程启动后创建)
├── AGENTS.md
└── CLAUDE.md
```

`outputs/reviews/` 属于必需支撑层。其余 outputs 面是完整合同的一部分，但按阶段按需出现。

`MEMORY.md` 是推荐的协作记忆层，用来承接偏好、编辑优先级和协作约束，不应该变成专题知识页。

## 所有权模型

| 路径 | Owner | 作用 |
| --- | --- | --- |
| `raw/human/**` | Human | 人工整理后的证据输入 |
| `raw/agents/{role}/**` | Agents | 仍需审校的不可信捕获 |
| `MEMORY.md` | Human + agent 协作 | 偏好、编辑优先级与协作上下文 |
| `raw/*.md` | `kb-compile` bootstrap input | bootstrap 阶段可接受的输入，但不能替代完整支撑层 |
| `wiki/drafts/**` | `kb-compile` | 可审校的 summaries / concepts / entities / indices |
| `wiki/live/**` | `kb-review` promotion target | 已批准的长期知识层 |
| `wiki/briefings/**` | `kb-review` | 只从 live 构建的角色 briefing |
| `outputs/reviews/**` | `kb-review` | 审校决策记录 |
| `outputs/qa/**` | `kb-query` | 持久问答归档 |
| `outputs/content/**` | `kb-query` | 从批准层生成的对外内容 |
| `outputs/health/**` | `kb-health` | 维护与体检报告 |
