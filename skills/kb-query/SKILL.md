---
name: kb-query
description: Query, search, and generate grounded outputs from the approved live layer of an Obsidian vault. Use this skill whenever the user asks what their approved notes say about something, wants a grounded answer, ranked local candidates, archived Q&A reuse, or a static web export from `wiki/live/`, says "query kb", "kb-search", "search live wiki", "导出静态知识站", "问知识库", or "搜索批准层并回答". This is the canonical read-side skill for approved retrieval, candidate ranking, grounded answers, archived answer reuse, and static web export. Do not use this skill for generic writing, open-ended web research, deterministic slide/report/chart/canvas rendering, or governance/maintenance passes that belong to `kb-render` or `kb-review`.
---

# KB Query

Search, answer, and generate outputs from the approved live layer.

In Karpathy's words: "good answers can be filed back into the wiki as new pages." `kb-query` is where explorations compound — every substantive answer should leave the wiki slightly better than before.

## Minimal loop

1. locate the best approved live pages and briefings
2. answer or draft the artifact from `wiki/live/`
3. archive the result when it is substantive
4. record the smallest durable delta that would improve the wiki next

## When this compounds the wiki

`kb-query` should not behave like disposable chat over live notes. Strong outputs should either become reusable archived artifacts or leave behind explicit candidates for a better concept page, synthesis page, relationship edge, governed question, or curated hub.

## When not to promote

Do not treat a polished answer as approved truth by itself. If the result discovers durable knowledge, it must still re-enter through `draft -> review -> live`.

## Read before querying

Read these files first:

- local `AGENTS.md`
- local `CLAUDE.md` if present
- `../obsidian-notes-karpathy/scripts/skill-contract-registry.json`
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

Treat `skill-contract-registry.json` as the canonical source for required references, baseline script, and expected write surfaces.

If `../obsidian-notes-karpathy/scripts/scan_query_scope.py` exists, run it first and treat its output as the deterministic baseline for live-layer retrieval boundaries.

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

`kb-query` must not treat `raw/` or `wiki/drafts/` as retrieval truth.

`MEMORY.md` is also outside the default knowledge retrieval boundary. Read it only when the user is asking about preferences, editorial priorities, or collaboration behavior rather than topic knowledge.

Those layers may be cited only as evidence if a human explicitly asks for source inspection. Default synthesis should stay on `wiki/live/`.
`outputs/episodes/` and `outputs/health/graph-snapshot.json` may help surface candidates, but they are not default truth surfaces.

## Source-backed answer discipline

- prefer approved summaries when grounding key conclusions
- use concept and entity pages as navigation and synthesis aids, not as a shortcut around approved evidence trails
- when sources disagree, say so explicitly instead of flattening the disagreement

## Modes

1. **search** — quickly locate approved pages and rank local candidates before synthesis
   - Example: "这个wiki里有没有关于 transformer attention 的内容"
   - Output: ranked page list with relevance snippets, no new file created

2. **research** — grounded answers archived into `outputs/qa/`
   - Example: "对比一下 RAG 和 LLM Wiki 的优缺点"
   - Output: structured Q&A markdown saved to `outputs/qa/`

3. **web** — static browseable web exports under `outputs/web/`
   - Example: "导出一个静态知识站"
   - Output: HTML package rooted at `outputs/web/{slug}/index.html`

4. **reflect-lite** — question resolution, synthesis notes, or gap reports that stay outside live until re-reviewed
   - Example: "这几个概念之间的关系还没理清楚，帮我整理一下"
   - Output: reflection note in `outputs/qa/`, with `followup_route: draft` if durable

When a substantive answer or artifact creates durable follow-up work, archive explicit `writeback_candidates`, `open_questions_touched`, `source_live_pages`, `writeback_status`, and a `followup_route` so the next compile/review or maintenance pass can decide what should happen next.

Prefer the smallest durable delta that improves future reuse:

- update an existing live concept or entity when the identity is already right
- draft a missing concept, entity, or synthesis when the knowledge is durable but absent
- strengthen `related` links or question tracking when the problem is graph quality rather than page absence
- create or expand a curated hub when repeated outputs keep traversing the same approved cluster

## Retrieval ladder

Default retrieval order:

1. `wiki/index.md`
2. `wiki/live/indices/*`
3. governance indices such as `QUESTIONS.md`, `GAPS.md`, and `ALIASES.md` when present
4. relevant `wiki/briefings/{role}.md`
5. prior `outputs/qa/`
6. local structured or metadata-driven search
7. optional semantic retrieval only as candidate surfacing

Semantic retrieval may help discover candidate pages, but approved live pages remain the truth source.

If the user explicitly says `kb-search`, treat that as direct wording for `kb-query` search mode.

If the user wants slides, reports, charts, or canvas derivatives from approved knowledge, hand off to `kb-render` instead of stretching `kb-query` into deterministic rendering work.

## Writeback contract

Compounding principle: every substantive query should leave the wiki at least slightly better — whether through a new draft page, a stronger relationship, an updated hub, or a promoted question. Writeback is the default posture, not an opt-in extra.

Use `../obsidian-notes-karpathy/references/query-writeback-lifecycle.md` as the detailed contract.

At minimum, substantive Q&A or publish outputs should:

- record `source_live_pages` when specific approved pages grounded the answer
- record `open_questions_touched` when the answer materially advances or reframes a standing question
- record `writeback_candidates` when the output discovers durable follow-up worth re-entering the wiki
- record `writeback_status` so later passes can see whether the work is still pending
- record `compounding_value` when future reuse, navigation value, or synthesis payoff should be explicit
- record `crystallized_from_episode` when the output came out of a broader episodic chain
- set `followup_route` to `none`, `draft`, or `review`

Use:

- `none` when the output is grounded and does not create durable follow-up work
- `draft` when the output suggests new or updated long-term knowledge that must re-enter draft -> review -> live
- `review` when the next action is an immediate human decision or a governance pass on already-prepared maintenance work

## Checkpoint

Before archiving a substantive answer, confirm:

- the answer is grounded in approved pages (not fabricated from general knowledge)
- the `followup_route` is set correctly: `none` when self-contained, `draft` when new knowledge was discovered, `review` when existing approved pages need updating
- any `writeback_candidates` are concrete and actionable, not vague placeholders

For simple search-mode lookups, no checkpoint is needed.

## Output to the user

Report:

1. which live pages and briefings were used
2. whether prior Q&A was reused
3. where the new artifact was saved
4. whether a `query` or `publish` log entry should be appended
5. whether any follow-up should go back into drafts instead of directly mutating live
