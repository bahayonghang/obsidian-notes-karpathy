---
name: kb-query
description: Query, search, and generate grounded outputs from the approved live layer of an Obsidian vault. Use this skill whenever the user asks what approved notes already say, wants grounded answers, ranked local candidates, archived Q&A reuse, publishable artifacts, or static web exports from `wiki/live/`; says "query kb", "kb-search", "archive this answer", "归档这个回答", "复用旧 Q&A", "写成公众号/thread", "对外内容", "一鱼多吃", "问知识库", or "搜索批准层并回答"; or uses Chinese-LLM-Wiki terms such as `综合页`, `output/analyses`, `中文优先分析`, `原文证据摘录`, or `先读 wiki/index.md 再回答`. This is the canonical read-side skill for approved retrieval, candidate ranking, answer archiving/reuse, creator-facing publishing, and static export. Do not use for generic writing, open web research, deterministic rendering, or governance/maintenance work handled by `kb-render` or `kb-review`.
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
- `../obsidian-notes-karpathy/references/file-model.md`
- `../obsidian-notes-karpathy/references/archive-model.md`
- `../obsidian-notes-karpathy/references/qa-template.md`
- `../obsidian-notes-karpathy/references/query-writeback-lifecycle.md`

Treat `skill-contract-registry.json` as the canonical source for required references, baseline command, and expected write surfaces.

If `onkb` is available, run `onkb --json query scope <vault-root>` first and treat its output as the deterministic baseline for live-layer retrieval boundaries.

If `onkb` is missing, follow the install fallback in `../obsidian-notes-karpathy/references/lifecycle-matrix.md`, then rerun the same command.

Then start with:

- `wiki/index.md`
- `wiki/live/indices/INDEX.md`
- `wiki/live/indices/CONCEPTS.md`
- `wiki/live/indices/SOURCES.md`
- `wiki/live/indices/TOPICS.md` when present
- `wiki/live/indices/QUESTIONS.md` when present
- relevant `wiki/briefings/{role}.md` when the request maps to a role
- prior `outputs/qa/`

The navigation indices (`INDEX.md`, `CONCEPTS.md`, `SOURCES.md`, `TOPICS.md`, `RECENT.md`) and the managed `onkb:indices` block in `wiki/index.md` are rebuilt deterministically by `onkb review indices`, so treat them as trustworthy entry points. If they look stale relative to `wiki/live/**`, run `onkb --json review indices <vault-root>` as a dry drift check and route the repair through `kb-review` maintenance mode instead of hand-editing them.

## Load on demand

Load these only when the trigger applies:

- `../obsidian-notes-karpathy/references/chinese-llm-wiki-compat.md` — when the request leans on Chinese-LLM-Wiki vocabulary beyond the mappings inlined in this SKILL
- `../obsidian-notes-karpathy/references/lifecycle-matrix.md` — when `onkb` is missing (install fallback) or a stage handoff is unclear
- `../obsidian-notes-karpathy/references/briefing-template.md` — when rebuilding or repairing `wiki/briefings/**`
- `../obsidian-notes-karpathy/references/content-output-template.md` — when producing publish-mode artifacts under `outputs/content/`
- `../obsidian-notes-karpathy/references/activity-log-template.md` — when appending an entry to `wiki/log.md`
- `../obsidian-notes-karpathy/references/questions-and-reflection-policy.md` — when standing questions, reflection notes, or `QUESTIONS.md` governance is involved
- `../obsidian-notes-karpathy/references/search-upgrades.md` — when retrieval quality tuning or scaling beyond the default ranking is on the table
- `../obsidian-notes-karpathy/references/memory-lifecycle.md` — when `MEMORY.md` or episodic memory boundaries are involved
- `../obsidian-notes-karpathy/references/graph-contract.md` — when graph exports, `related` edges, or relationship candidates are involved
- `../obsidian-notes-karpathy/references/render-template.md` — when handing off to `kb-render`
- `../obsidian-notes-karpathy/references/profile-contract.md` — when account profiles or `source_profile` metadata is involved
- `../obsidian-notes-karpathy/references/episode-template.md` — when `outputs/episodes/**` work is involved
- `../obsidian-notes-karpathy/references/web-export-template.md` — when running web export mode

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

Candidates left at `writeback_status: pending` with `followup_route: draft` are picked up later by the deterministic writeback lane (`onkb --json compile writeback <vault-root>` in `kb-compile`), which scaffolds a reviewable draft grounded in the recorded `source_live_pages` and advances the status to `drafted`. Keep `source_live_pages` accurate: the lane grounds draft provenance in those approved pages, never in the archived artifact itself.

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
