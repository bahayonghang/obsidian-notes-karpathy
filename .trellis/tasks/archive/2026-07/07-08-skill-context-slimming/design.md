# 设计：SKILL 契约上下文瘦身分层

对应 `prd.md`。证据档案：`../07-08-llm-wiki-gap-optimization/research/llm-wiki-gap-analysis.md`（P3）。

## 决策总览

1. **纯重分层，不合并不删除任何 references 文件**。PRD 允许合并（需映射表），但合并会同时动"契约说什么"与所有引用点；重分层只动"契约如何被加载"，语义无损性可机械验证（所有文件仍在，规则可达性 = core 直读 + on-demand 触发条件）。映射表因此就是下方的触发条件表。
2. **registry 双键表达分层**：`reads` 收窄为 core（无条件必读，硬上限 6），新增 `reads_on_demand`：`[{"file": "...", "when": "<触发条件>"}]`。registry 保持 canonical——SKILL.md 的两个小节都由它驱动并被 `contract-validate` 逐条校验。
3. **SKILL.md 双小节**："## Read before ..." 保留 AGENTS.md / CLAUDE.md / registry.json + core 引用；紧随其后新增 "## Load on demand"，每条 `- `<path>` — <when>`。
4. **校验机械化**（`src/dev/contract.rs` + `reference_blocks.rs`）：
   - core 上限：`reads.len() > 6` 报错。
   - `reads` 与 `reads_on_demand` 不得重叠；两者引用的文件必须存在于 `references/`。
   - 每条 on-demand 必须以 `- `{prefix}references/{file}` — {when}` 精确出现在 "Load on demand" 小节（`build_on_demand_bullets` 生成期望，`extract_on_demand_bullets` 解析实际，二者比对）。
   - 既有 core 校验不变（`build_shared_reference_bullets` ⊆ "Read before" 小节）。
5. **`dev render-reference-block` 扩展**：非空时同时渲染 on-demand 块，供维护者再生 SKILL 小节。

## 分层判据

core = 该 skill 每次执行都会用到、缺了会直接出错的最小集合：真值边界（`file-model.md`）、该 skill 的主方法/主产出模板、以及无法从 SKILL prose 自身恢复的硬契约。其余全部转 on-demand，触发条件必须具体（出现某类文件、用户用某类词汇、进入某个 mode、要写某个面）。

既有先例：所有 SKILL 已内联 "If `onkb` is missing, follow the install fallback in `lifecycle-matrix.md`" —— 这本身就是条件加载指针，本设计将该模式推广成一等机制。

## 每 skill 分层表（核心决策 + 度量）

字节数为当前文件实测。触发条件在 registry 中为 canonical 英文措辞（下表为中文概述）。

### obsidian-notes-karpathy（router）：13 → core 3

| 层 | 文件 |
| --- | --- |
| core（24,545B） | `file-model`(15,872)、`lifecycle-matrix`(5,736)、`chinese-llm-wiki-compat`(2,937) |
| on-demand | `archive-model`（归档真值/复用问题）、`search-upgrades`（检索调优）、`activity-log-template`（写 log）、`provenance-and-alias-policy`（别名/重复冲突）、`questions-and-reflection-policy`（问题治理）、`memory-lifecycle`（MEMORY/episodic 边界）、`graph-contract`（图导出/关系）、`source-manifest-contract`（manifest 字段）、`profile-contract`（账号 profile）、`automation-hooks`（automation 接线） |

### kb-init：21 → core 6

| 层 | 文件 |
| --- | --- |
| core（29,664B） | `file-model`、`lifecycle-matrix`、`chinese-llm-wiki-compat`（legacy 迁移是本职）、`index-home-template`(1,560)、`source-manifest-contract`(1,595)、`memory-lifecycle`(1,964) |
| on-demand | 修复对应面时加载：`schema-template`、`summary-template`、`review-template`、`briefing-template`、`activity-log-template`、`questions-template`、`topic-template`、`procedure-template`、`episode-template`；冲突/治理场景：`provenance-and-alias-policy`、`query-writeback-lifecycle`、`taxonomy-and-hubs`、`graph-contract`、`profile-contract`、`automation-hooks` |

### kb-ingest：6 → core 3

| 层 | 文件 |
| --- | --- |
| core（23,203B） | `file-model`、`lifecycle-matrix`、`source-manifest-contract` |
| on-demand | `activity-log-template`（写 log）、`paper-ingestion-lifecycle`（出现 `raw/**/papers/*.pdf`）、`profile-contract`（source_profile 元数据） |

### kb-compile：16 → core 4

| 层 | 文件 |
| --- | --- |
| core（28,959B） | `file-model`、`compile-method`(2,477)、`draft-schema`(6,905)、`summary-template`(3,705) |
| on-demand | `lifecycle-matrix`（onkb 缺失/阶段交接不清）、`schema-template`（draft schema 之外的全库字段）、`concept/entity/topic/procedure-template`（产该类型草稿时）、`activity-log-template`、`provenance-and-alias-policy`（浮出别名/重复候选）、`paper-ingestion-lifecycle`（PDF）、`memory-lifecycle`、`graph-contract`（relationship 候选）、`source-manifest-contract`（manifest 异常） |

### kb-review：21 → core 6

| 层 | 文件 |
| --- | --- |
| core（34,417B） | `file-model`、`review-template`(1,874)、`schema-template`(6,255)、`health-rubric`(5,677)、`provenance-and-alias-policy`(3,625)、`briefing-template`(1,114) |
| on-demand | `lifecycle-matrix`、`chinese-llm-wiki-compat`（中文维护词汇）、`archive-model`（归档面 hygiene）、`activity-log-template`、`search-upgrades`、`questions-and-reflection-policy`、`query-writeback-lifecycle`（写回积压处置）、`memory-lifecycle`、`graph-contract`、`source-manifest-contract`（来源完整性核查）、`topic-template`（topic 提升）、`profile-contract`（creator 一致性）、`automation-hooks`、`episode-template`、`procedure-template`（procedural 提升） |

### kb-query：17 → core 4

| 层 | 文件 |
| --- | --- |
| core（29,760B） | `file-model`、`archive-model`(2,661)、`qa-template`(2,652)、`query-writeback-lifecycle`(8,575) |
| on-demand | `chinese-llm-wiki-compat`（SKILL prose 已内联三条关键映射，词汇密集时再全读）、`lifecycle-matrix`、`briefing-template`（用到角色 briefing）、`content-output-template`（publish 模式）、`web-export-template`（web 模式）、`render-template`（转交 kb-render）、`activity-log-template`、`questions-and-reflection-policy`（reflect-lite/问题推进）、`search-upgrades`、`memory-lifecycle`、`graph-contract`、`profile-contract`、`episode-template` |

### kb-render：7 → core 3

| 层 | 文件 |
| --- | --- |
| core（19,290B） | `file-model`、`render-template`(1,804)、`obsidian-safe-markdown`(1,614) |
| on-demand | `lifecycle-matrix`、`archive-model`（归档复用问题）、`content-output-template`（brief 复用 content 产物）、`profile-contract` |

### 度量汇总（验收证据）

| skill | 现状 files/bytes | core files/bytes | 降幅 |
| --- | --- | --- | --- |
| router | 13 / 50,887 | 3 / 24,545 | −51.8% |
| kb-init | 21 / 73,133 | 6 / 29,664 | −59.4% |
| kb-ingest | 6 / 29,131 | 3 / 23,203 | −20.3%（file-model 地板） |
| kb-compile | 16 / 61,208 | 4 / 28,959 | −52.7% |
| kb-review | 21 / 77,498 | 6 / 34,417 | −55.6% |
| kb-query | 17 / 64,998 | 4 / 29,760 | −54.2% |
| kb-render | 7 / 31,912 | 3 / 19,290 | −39.6%（已经很薄） |
| **合计** | **101 / 388,767** | **29 / 189,838** | **−51.2%** |

实施收口时已用脚本重测：以上数字与实测一致（2026-07-09，`dev eval-trigger --dry-run` 前后基线除 workspace 时间戳路径外逐字段一致）。

## 校验与测试面

- `src/dev/registry.rs`：`SkillEntry` 增 `reads_on_demand: Vec<OnDemandRead>`（`{file, when}`，serde default）。
- `src/dev/reference_blocks.rs`：`build_on_demand_bullets` / `extract_on_demand_bullets`（扫描 "## Load on demand" 小节，`—` 前取路径）；`render_shared_reference_block` 输出追加 on-demand 块。
- `src/dev/contract.rs`：新增 4 项校验（上限 6 / 不重叠 / 文件存在 / SKILL 小节逐条精确匹配）。
- `tests/contract_routing.rs` 既有镜像断言天然兼容（core ⊆ Read before）；新增用例断言：全 registry core ≤ 6、on-demand 与 core 无交集、每个 on-demand 条目在对应 SKILL 中可达、字节降幅可复算。
- eval 面：trigger/runtime manifest 与 "Read before" 零耦合（已 grep 验证）；`dev eval-trigger --dry-run` 计划在改动前后比对留档。

## 兼容与回滚

- registry 消费方（`skill_audit`、`doc_fragments`、`payload` 安装面）不读 `reads` 之外的清单语义，serde 新键向后兼容。
- 已安装的旧版 skill 包不受影响（分层只改 SKILL 文本与 registry）；`onkb skill install --overwrite` 后获得新分层。
- 回滚 = revert 对应 commit；无数据迁移。

## 已否决的备选

- **合并 references 成每 skill 单一简报**：制造 30+ 文件 → 7 份简报的内容复制，同一规则多份拷贝会漂移，违背 single-source；且动"契约说什么"。
- **删除低频 references**：PRD 明令禁止删除仍被引用的文件；低频 ≠ 无用。
- **拆分 `file-model.md` 成 core 卡片 + 详版**：能再省 ~10K/skill，但属于内容手术（需全引用点同步 + 映射表），超出"只动加载方式"的本任务边界；留作后续任务候选。
- **在 skill_audit 而非 contract-validate 落校验**：audit 是评分面（warnings），contract-validate 是 `just lint` 阻断面；上限与一致性属于硬契约。
