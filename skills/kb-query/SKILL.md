---
name: kb-query
description: Query, search, and generate grounded outputs from the approved live layer of an Obsidian vault. Use this skill whenever the user asks what the approved notes already say, wants a grounded answer, ranked local candidates, archived Q&A reuse, a publishable outward artifact from approved knowledge, a static web export from `wiki/live/`, says "query kb", "kb-search", "search live wiki", "archive this answer", "归档这个回答", "把回答存回去", "复用旧 Q&A", "写成公众号", "写成 thread", "对外内容", "一鱼多吃", "复用批准知识出稿", "导出静态知识站", "问知识库", "搜索批准层并回答", or uses Chinese-LLM-Wiki wording such as `综合页`, `综合页式分析`, `output/analyses`, `中文优先分析`, `原文证据摘录`, `先读 wiki/index.md 再回答`, or `先放 output/analyses`. This is the canonical read-side skill for approved retrieval, candidate ranking, grounded answers, archive and reuse of durable answers, creator-facing publish mode, and static web export. Do not use this skill for generic writing, open-ended web research, deterministic slide/report/chart/canvas rendering, or governance/maintenance passes that belong to `kb-render` or `kb-review`.
---

# KB Query

Search, answer, and generate outputs from the approved live layer.

In Karpathy's words: "good answers can be filed back into the wiki as new pages." In this bundle, that compounding still respects `draft -> review -> live`. Strong query results archive cleanly, then feed later writeback or maintenance work.

When a user speaks in the simpler `raw/wiki/output` language from `Chinese-LLM-Wiki`, translate it instead of widening the truth boundary:

- `综合页` usually means a grounded reusable analysis, so start in `outputs/qa/**`
- `output/analyses` maps to archived grounded analysis, not approved truth
- `先读 wiki/index.md` remains the default navigation posture

## Minimal loop

1. locate the best approved live pages and briefings
2. answer or draft the artifact from `wiki/live/`
3. archive the result when it is substantive
4. record the smallest durable delta that should flow back through review later

## Read before querying

- local `AGENTS.md`
- local `CLAUDE.md` if present
- `../obsidian-notes-karpathy/scripts/skill-contract-registry.json`
- `../obsidian-notes-karpathy/references/chinese-llm-wiki-compat.md`
- `../obsidian-notes-karpathy/references/archive-model.md`
- `../obsidian-notes-karpathy/references/file-model.md`
- `../obsidian-notes-karpathy/references/lifecycle-matrix.md`
- `../obsidian-notes-karpathy/references/briefing-template.md`
- `../obsidian-notes-karpathy/references/qa-template.md`
- `../obsidian-notes-karpathy/references/content-output-template.md`
- `../obsidian-notes-karpathy/references/activity-log-template.md`
- `../obsidian-notes-karpathy/references/search-upgrades.md`
- `../obsidian-notes-karpathy/references/questions-and-reflection-policy.md`
- `../obsidian-notes-karpathy/references/query-writeback-lifecycle.md`
- `../obsidian-notes-karpathy/references/memory-lifecycle.md`
- `../obsidian-notes-karpathy/references/graph-contract.md`
- `../obsidian-notes-karpathy/references/render-template.md`
- `../obsidian-notes-karpathy/references/profile-contract.md`
- `../obsidian-notes-karpathy/references/episode-template.md`
- `../obsidian-notes-karpathy/references/web-export-template.md`

Treat `skill-contract-registry.json` as the canonical source for required references, baseline command, and expected write surfaces.

If `onkb` is available, run `onkb --json query scope <vault-root>` first and treat its output as the deterministic baseline for live-layer retrieval boundaries.

Then start with:

- `wiki/index.md`
- `wiki/live/indices/INDEX.md`
- `wiki/live/indices/CONCEPTS.md`
- `wiki/live/indices/SOURCES.md`
- `wiki/live/indices/TOPICS.md` when present
- `wiki/live/indices/QUESTIONS.md` when present
- relevant `wiki/briefings/{role}.md` when the request maps to a role
- prior `outputs/qa/`

## Hard boundary

- `wiki/live/**` is the default truth source
- `raw/` and `wiki/drafts/` are not retrieval truth
- `MEMORY.md` is collaboration context, not topic truth
- `outputs/episodes/` and graph snapshots can surface candidates, but they do not outrank approved live pages
- archived outputs are reusable, but they remain artifact archive rather than approved truth

If a human explicitly asks for source inspection, you may cite raw evidence. Default synthesis should still stay on approved live coverage.

## Modes

### search

- use when the user wants ranked local candidates first
- output a ranked page list with short relevance notes
- do not create a new archive file unless the user asks

### research

- use when the user wants a grounded reusable answer
- save substantive outputs under `outputs/qa/`
- this is the closest current mapping to `output/analyses`

### publish

- use when the user wants creator-facing prose grounded in approved knowledge
- save article / thread / newsletter / talk-outline artifacts under `outputs/content/`
- reuse prior archived Q&A when it already contains the needed grounded synthesis

### web

- use when the user wants a static browseable export from approved knowledge
- save the package under `outputs/web/{slug}/index.html`
- follow `../obsidian-notes-karpathy/references/web-export-template.md` for the site skeleton, per-page frontmatter, slug rules, navigation structure, and `search.json` shape

### reflect-lite

- use when the user wants a synthesis note, question-resolution memo, or gap note that should stay outside live
- archive it under `outputs/qa/`
- set `followup_route: draft` if the result exposes durable knowledge work

If the user wants slides, deterministic reports, chart briefs, or canvas outputs from approved knowledge, hand off to `kb-render`.

## Writeback contract

Every substantive query should leave explicit follow-up metadata behind. At minimum, record:

- `source_live_pages`
- `open_questions_touched` when relevant
- `writeback_candidates`
- `writeback_status`
- `followup_route`

Use:

- `none` when the artifact is grounded and self-contained
- `draft` when durable knowledge should re-enter `draft -> review -> live`
- `review` when the next step is a governance or approval pass on already-prepared material

Prefer the smallest durable delta:

- extend an existing live page when the identity is already correct
- draft a new concept / entity / synthesis when the knowledge is durable but absent
- strengthen relationships or question tracking when the graph is the weak point

## Checkpoint

Before archiving a substantive answer, confirm:

- the answer is grounded in approved pages
- archive reuse did not outrank live grounding
- `followup_route` is concrete
- any `writeback_candidates` are actionable instead of vague

For simple search-mode lookups, no checkpoint is needed.

## Output to the user

Report:

1. which live pages and briefings were used
2. whether prior Q&A was reused
3. where the new artifact was saved
4. whether a `query` or `publish` log entry should be appended
5. whether follow-up work belongs in drafts or maintenance instead of directly mutating live
