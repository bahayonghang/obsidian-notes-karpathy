# kb-init

初始化、修复或迁移到 review-gated 契约。

## 创建或修复的支撑层

- `raw/human/{articles,papers,podcasts,repos,assets}`
- `raw/agents/{role}/**`
- `MEMORY.md`
- `wiki/drafts/{summaries,concepts,entities,indices}`
- `wiki/live/{summaries,concepts,entities,indices}`
- `wiki/briefings/**`
- `outputs/reviews/**`
- `wiki/index.md`
- `wiki/log.md`
- `AGENTS.md`
- `CLAUDE.md`

## 治理与启动文件

`kb-init` 还应该补齐核心 live 索引：

- `wiki/live/indices/INDEX.md`
- `wiki/live/indices/CONCEPTS.md`
- `wiki/live/indices/SOURCES.md`
- `wiki/live/indices/RECENT.md`
- `wiki/live/indices/EDITORIAL-PRIORITIES.md`

如果用户想要更强的治理面，再按需创建 `wiki/live/indices/QUESTIONS.md`、`GAPS.md`、`ALIASES.md`。

`MEMORY.md` 是推荐的协作脚手架，用来承接偏好、编辑优先级和协作上下文，不是批准知识存储层。

`outputs/qa/**`、`outputs/health/**`、`outputs/content/**`、`outputs/reports/**`、`outputs/slides/**`、`outputs/charts/**` 等下游目录按需创建，不是最小初始化要求。

## 迁移姿态

遇到 legacy-layout 时，先搭好 review-gated 结构，再保留并迁移旧内容。直接落在 `wiki/summaries/` 或 `wiki/concepts/` 的旧页面要视作迁移任务，而不是普通编译输出。
