# Skills 优化计划

> 基于 `ref/notes.md`（Karpathy 知识库方法论原文）与现有三个 skill 的深度对比分析。

## 一、核心差距总览

| 维度 | notes.md 设计 | 现有 Skills 实现 | 差距等级 |
|------|-------------|-----------------|---------|
| Q&A 沉淀 | **默认行为**，每次对话自动存档到 `outputs/qa/` | 可选的"归档到 wiki" | **严重** |
| Q&A 复用 | 先查已有 Q&A，避免重复推导 | 无此逻辑 | **严重** |
| 目录结构 | `outputs/qa/` + `outputs/health/` + `raw/{分类}/` | 缺 qa/、health/；raw/ 只有 assets/ | **中等** |
| 健康检查归档 | 报告存到 `outputs/health/`，每周一份 | 存到 `outputs/reports/`，无周期概念 | **中等** |
| 首次 vs 增量 | 首次编译 30min-1hr，后续增量很快 | 无差异化引导 | **轻微** |
| 编译规范 | CLAUDE.md 定义编译规范 | AGENTS.md 承担此角色（可接受的设计选择） | **可忽略** |
| 入门引导 | 两周最小闭环路线图 | 无 | **轻微** |

## 二、优化方案

---

### P0：kb-query — Q&A 沉淀机制重构

**问题**：文章的核心洞见是"你每跟 AI 聊一次，知识库就增加一层"。现有 kb-query 把 Q&A 归档当作可选步骤，这直接违反了这个设计哲学。

**改动范围**：`skills/obsidian-notes-karpathy/kb-query/SKILL.md`

#### 改动 1：Q&A 归档从"可选"变为"默认"

**现状**（Step 4）：
> "If the answer reveals new insights worth preserving, offer to archive it"
> "Always ask the user before archiving"

**优化后**：
- 每次 Q&A 研究**默认**将结果保存到 `outputs/qa/{date}-{slug}.md`
- 不再询问"是否归档"，而是直接归档后告知用户
- 仅在用户明确说"不要保存"时跳过

#### 改动 2：Q&A 文件格式对齐文章标准

现有格式不完整。对齐为：

```yaml
---
question: "RAG 和轻量索引的适用边界？"
asked_at: 2026-04-03
sources:
  - "[[raw/source-1]]"
  - "[[wiki/concepts/concept-a]]"
tags:
  - qa
  - topic/subtopic
---
```

正文结构增加：
- `## TL;DR` — 一句话结论
- `## 结论` — 详细回答
- `## 证据` — 链接回原始来源
- `## 不确定性` — 知识空白和待验证点

#### 改动 3：研究前先查已有 Q&A

在 "Step 2: Navigate the wiki" **之前**插入新步骤：

> **Step 1.5: Check existing Q&A**
> 1. 搜索 `outputs/qa/` 中是否有相似问题
> 2. 如有匹配，先读取已有 Q&A 作为研究起点
> 3. 如已有 Q&A 已充分回答，直接引用并补充（而非从零开始）

这实现了文章中"下次遇到类似问题，Claude 直接读已有的 Q&A，不用重新推导"的逻辑。

#### 改动 4：Q&A 反哺 wiki

每次 Q&A 归档后，检查是否发现：
- 新概念 → 自动创建 `wiki/concepts/` 条目
- 现有概念的新证据 → 更新对应概念文章
- 概念间新关联 → 更新 `related` 字段

---

### P1：kb-init — 目录结构对齐

**问题**：缺少 Q&A 和健康检查的专用目录；raw/ 缺少分类子目录。

**改动范围**：`skills/obsidian-notes-karpathy/kb-init/SKILL.md`

#### 改动 1：补充目录结构

```diff
  {root}/
  ├── raw/
+ │   ├── articles/           # Web Clipper 剪藏的文章
+ │   ├── podcasts/           # Podwise 等播客转录
+ │   ├── papers/             # 学术论文
  │   └── assets/             # 图片和附件
  ├── wiki/
  │   ├── concepts/
  │   ├── summaries/
  │   └── indices/
  ├── outputs/
+ │   ├── qa/                 # Q&A 问答沉淀（kb-query 自动写入）
+ │   ├── health/             # 健康检查报告（kb-compile 写入）
  │   ├── reports/
  │   ├── slides/
  │   └── charts/
  └── AGENTS.md
```

#### 改动 2：AGENTS.md 目录说明表更新

对应新增目录，更新"Who writes"列：

| Directory | Purpose | Who writes |
|-----------|---------|------------|
| `raw/articles/` | Web Clipper 剪藏的文章 | Human |
| `raw/podcasts/` | 播客转录导出 | Human / Podwise |
| `raw/papers/` | 学术论文 | Human |
| `outputs/qa/` | Q&A 问答沉淀 | LLM (kb-query) |
| `outputs/health/` | 健康检查报告 | LLM (kb-compile) |

#### 改动 3：Step 5 增加入门路线图

初始化完成后，向用户展示"两周最小闭环"路线图：

> **第一周**：搭 raw → wiki 的最小循环
> - 往 `raw/` 里添加 5-10 篇资料
> - 运行 `kb-compile` 做首次编译
>
> **第二周**：让 Q&A 开始积累 + 首次健康检查
> - 对知识库提复杂问题，结果自动存到 `outputs/qa/`
> - 运行 `kb-compile` 的健康检查，查看首份报告

---

### P2：kb-compile — 健康检查增强

**问题**：健康报告路径混淆；缺少周期性概念；首次编译引导不足。

**改动范围**：`skills/obsidian-notes-karpathy/kb-compile/SKILL.md`

#### 改动 1：健康报告输出路径修正

```diff
- Write results to `outputs/reports/health-check-{date}.md`
+ Write results to `outputs/health/health-check-{date}.md`
```

语义更清晰：`outputs/reports/` 是用户主动生成的研究报告，`outputs/health/` 是系统级体检报告。

#### 改动 2：增加"过期检测"维度

Phase 3 新增检查项：

> **3.6 Staleness detection**
> - 标记 `compiled_at` 超过 30 天的 raw 源（可能需要重新编译以吸收新的概念关联）
> - 检查 wiki/concepts/ 中超过 60 天未更新的概念条目
> - 在报告中列出"可能过期"的条目清单

#### 改动 3：首次编译 vs 增量编译引导

在 "Phase 2" 开头增加说明：

> **首次编译注意事项：**
> - 首次编译会处理 raw/ 中所有文件，耗时较长（5-10 篇约 15-30 分钟）
> - 建议首次编译不超过 10 篇 raw，先跑通流程
> - 首次编译后，后续每次只处理新增/变更的 raw，速度会快很多
>
> **大批量编译（>10 篇新源）：**
> - 分批处理，每批 5 篇
> - 每批完成后更新索引，再处理下一批
> - 报告进度："批次 2/3 完成，已编译 10/15 篇..."

#### 改动 4：健康检查增加评分体系

健康报告增加量化评分：

```markdown
## Health Score: 78/100

| 维度 | 分数 | 说明 |
|------|------|------|
| 完整性 | 85 | 2 篇 raw 缺摘要 |
| 一致性 | 70 | 1 处定义冲突 |
| 连通性 | 90 | 3 个孤岛节点 |
| 新鲜度 | 65 | 5 个概念超 30 天未更新 |
```

---

### P3：描述字段（description）触发优化

三个 skill 的 description 已经不错，但可以更"主动触发"。

#### kb-init

```diff
- ...or wants to create a structured wiki workspace for LLM-driven knowledge management. This is a one-time setup — run it once per vault/project.
+ ...or wants to create a structured wiki workspace for LLM-driven knowledge management. Also use when setting up a new Obsidian vault for research, content creation, or learning. This is a one-time setup — run it once per vault/project. Trigger phrases include "新建知识库", "setup vault", "karpathy workflow", "知识管理初始化".
```

#### kb-query

```diff
  ... or wants to explore and extract insights from their collected knowledge.
+ Also trigger when the user asks ANY question that could be answered by their knowledge base, even if they don't explicitly say "query" — for example "what do my notes say about X", "summarize what I've collected on Y", "之前那篇文章说了什么", "帮我整理一下关于X的资料".
```

---

### P4（可选）：考虑新增 kb-health 独立技能

**理由**：文章将健康检查描述为独立的周期性活动（"每周任务"），而非编译的附属步骤。

**方案**：
- 从 kb-compile Phase 3 提取为独立的 `kb-health` skill
- kb-compile 仍在编译后自动运行轻量检查
- kb-health 提供完整深度检查 + 历史趋势对比

**暂不实施的理由**：当前三个 skill 已覆盖功能，拆分会增加复杂度。建议先优化 P0-P2，观察用户反馈后再决定。

---

## 三、实施顺序

| 阶段 | 改动 | 涉及文件 | 预计工作量 |
|------|------|---------|-----------|
| **Phase 1** | P0: kb-query Q&A 沉淀重构 | kb-query/SKILL.md | 中 |
| **Phase 2** | P1: kb-init 目录结构对齐 | kb-init/SKILL.md | 小 |
| **Phase 3** | P2: kb-compile 健康检查增强 | kb-compile/SKILL.md | 中 |
| **Phase 4** | P3: description 触发优化 | 三个 SKILL.md | 小 |
| **Phase 5** | 同步更新 README.md + README_CN.md | 两个 README | 小 |

## 四、不改动的部分

以下是**有意保留**的设计选择，不需要修改：

1. **AGENTS.md 而非 CLAUDE.md**：文章作者用 CLAUDE.md 定义编译规范，但我们的 AGENTS.md 承担了相同角色且命名更语义化（对知识库场景来说"Agent 规范"比"Claude 规范"更准确）
2. **平台内容目录**（x/, 公众号/, 小红书/）：这是文章作者的个人用例，不是通用知识库的必要结构。用户可以在 kb-init 时自行添加
3. **Claudian 特定集成**：文章提到在 Obsidian 里用 Claudian 插件，但 skill 应保持工具无关性
4. **RAG 相关讨论**：文章说"别上来就搞 RAG"——这是使用建议而非 skill 功能需求
