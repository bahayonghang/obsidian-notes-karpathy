# kb-init

初始化、修复或迁移到带审校与批准流程的契约。用户即使只说“LLM Wiki”“Karpathy 风格知识库”“把 Obsidian 当 IDE”“做个知识编译器”“做个人知识库 / second brain”“中文优先 wiki”或“raw/wiki/output 脚手架”，也应该优先落到这个技能，而不是要求他们先知道 `kb-init` 的名字。

## 创建或修复的支撑层

- `raw/human/{articles,papers,podcasts,repos,assets}`
- `raw/human/data`
- `raw/agents/{role}/**`
- `raw/_manifest.yaml`
- `MEMORY.md`
- `wiki/drafts/{summaries,topics,concepts,entities,indices}`
- `wiki/live/{summaries,topics,concepts,entities,indices}`
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
- `wiki/live/indices/TOPICS.md`
- `wiki/live/indices/RECENT.md`
- `wiki/live/indices/EDITORIAL-PRIORITIES.md`

如果用户想要更强的治理面，再按需创建 `wiki/live/indices/QUESTIONS.md`、`GAPS.md`、`ALIASES.md`。

`MEMORY.md` 是推荐的协作脚手架，用来承接偏好、编辑优先级和协作上下文，不是批准知识存储层。

## 确定性 helper

- `onkb --json init <vault-root> ...`：新建与非破坏式修复支撑层
- `onkb --json migrate <vault-root> ...`：迁移 legacy 单层 vault，并保留来源与迁移报告
- `onkb --json status <vault-root>`：初始化或修复后快速查看当前阶段与下一步

`outputs/qa/**`、`outputs/health/**`、`outputs/content/**`、`outputs/reports/**`、`outputs/slides/**`、`outputs/charts/**` 等下游目录按需创建，不是最小初始化要求。

## Profile 选择

`kb-init` 应在 starter files 里显式保留以下三种 profile 之一：

- `governed-team`
- `standard`
- `fast-personal`

## 迁移姿态

遇到旧版目录结构时，先搭好带审校与批准流程的结构，再保留并迁移旧内容。直接落在 `wiki/summaries/` 或 `wiki/concepts/` 的旧页面要视作迁移任务，而不是普通编译输出。

如果请求使用的是 `Chinese-LLM-Wiki` 的单层话术，也只把它当作 onboarding 语言。真正落地仍然是 `raw/` + `wiki/drafts/` + `wiki/live/` + `outputs/**`。


