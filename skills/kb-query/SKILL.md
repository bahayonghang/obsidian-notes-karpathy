---
name: kb-query
description: Query, search, and generate grounded outputs from the approved live layer of an Obsidian vault. Use this skill whenever the user asks what their approved notes say about something, wants a report, thread, talk, or slides grounded in `wiki/live/`, says "query kb", "search live wiki", "问知识库", "搜索批准层", "生成报告", or wants a substantive answer reused from prior approved outputs and archived back into the vault. Do not use this skill for generic writing, open-ended web research, or maintenance/audit passes when the task is not anchored in approved live knowledge.
---

# KB Query

Search, answer, and generate outputs from the approved live layer.

## Read before querying

Read these files first:

- local `AGENTS.md`
- local `CLAUDE.md` if present
- `../obsidian-notes-karpathy/scripts/skill-contract-registry.json`
- `../obsidian-notes-karpathy/references/file-model.md`
- `../obsidian-notes-karpathy/references/lifecycle-matrix.md`
- `../obsidian-notes-karpathy/references/briefing-template.md`
- `../obsidian-notes-karpathy/references/activity-log-template.md`
- `../obsidian-notes-karpathy/references/search-upgrades.md`
- `../obsidian-notes-karpathy/references/questions-and-reflection-policy.md`
- `../obsidian-notes-karpathy/references/query-writeback-lifecycle.md`

Treat `skill-contract-registry.json` as the canonical source for required references, baseline script, and expected write surfaces.

If `../obsidian-notes-karpathy/scripts/scan_query_scope.py` exists, run it first and treat its output as the deterministic baseline for live-layer retrieval boundaries.

Then start with:

- `wiki/index.md`
- `wiki/live/indices/INDEX.md`
- `wiki/live/indices/CONCEPTS.md`
- `wiki/live/indices/SOURCES.md`
- `wiki/live/indices/QUESTIONS.md` when present
- relevant `wiki/briefings/{role}.md` when the request maps to a role
- prior `outputs/qa/`

## Hard boundary

`kb-query` must not treat `raw/` or `wiki/drafts/` as retrieval truth.

`MEMORY.md` is also outside the default knowledge retrieval boundary. Read it only when the user is asking about preferences, editorial priorities, or collaboration behavior rather than topic knowledge.

Those layers may be cited only as evidence if a human explicitly asks for source inspection. Default synthesis should stay on `wiki/live/`.

## Source-backed answer discipline

- prefer approved summaries when grounding key conclusions
- use concept and entity pages as navigation and synthesis aids, not as a shortcut around approved evidence trails
- when sources disagree, say so explicitly instead of flattening the disagreement

## Modes

1. search mode for quickly locating approved pages
2. research mode for grounded answers archived into `outputs/qa/`
3. publish mode for reports, threads, talks, and slides derived from approved knowledge
4. reflect-lite mode for question resolution, synthesis notes, or gap reports that should stay outside live until re-reviewed

When a substantive answer or artifact surfaces durable follow-up work, archive explicit `writeback_candidates`, `open_questions_touched`, `source_live_pages`, and a `followup_route` so the next compile/review or health pass can decide what should happen next.

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

## Writeback contract

Use `../obsidian-notes-karpathy/references/query-writeback-lifecycle.md` as the detailed contract.

At minimum, substantive Q&A or publish outputs should:

- record `source_live_pages` when specific approved pages grounded the answer
- record `open_questions_touched` when the answer materially advances or reframes a standing question
- record `writeback_candidates` when the output discovers durable follow-up worth re-entering the wiki
- set `followup_route` to `none`, `draft`, `review`, or `health`

Use:

- `none` when the output is grounded and does not create durable follow-up work
- `draft` when the output suggests new or updated long-term knowledge that must re-enter draft -> review -> live
- `review` when the next action is an immediate human decision about an already-prepared candidate
- `health` when the output mainly exposes governance, drift, alias, backlog, or stale-archive maintenance work

## Output to the user

Report:

1. which live pages and briefings were used
2. whether prior Q&A was reused
3. where the new artifact was saved
4. whether a `query` or `publish` log entry should be appended
5. whether any follow-up should go back into drafts instead of directly mutating live
