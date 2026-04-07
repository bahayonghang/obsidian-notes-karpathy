---
name: kb-query
description: Query, search, and generate outputs from the approved V2 live layer. Use this skill whenever the user asks what their approved notes say about something, wants a report or thread grounded in live knowledge, says "query kb", "search live wiki", "问知识库", "搜索批准层", "生成报告", or wants a substantive answer archived instead of disappearing into chat.
---

# KB Query

Search, answer, and generate outputs from the approved live layer.

## Read before querying

Read these files first:

- local `AGENTS.md`
- local `CLAUDE.md` if present
- `../obsidian-notes-karpathy/references/file-model.md`
- `../obsidian-notes-karpathy/references/lifecycle-matrix.md`
- `../obsidian-notes-karpathy/references/briefing-template.md`
- `../obsidian-notes-karpathy/references/activity-log-template.md`

Then start with:

- `wiki/index.md`
- `wiki/live/indices/INDEX.md`
- `wiki/live/indices/CONCEPTS.md`
- `wiki/live/indices/SOURCES.md`
- relevant `wiki/briefings/{role}.md` when the request maps to a role
- prior `outputs/qa/`

## Hard boundary

`kb-query` must not treat `raw/` or `wiki/drafts/` as retrieval truth.

Those layers may be cited only as evidence if a human explicitly asks for source inspection. Default synthesis should stay on `wiki/live/`.

## Modes

1. search mode for quickly locating approved pages
2. research mode for grounded answers archived into `outputs/qa/`
3. publish mode for reports, threads, talks, and slides derived from approved knowledge

## Output to the user

Report:

1. which live pages and briefings were used
2. whether prior Q&A was reused
3. where the new artifact was saved
4. whether a `query` or `publish` log entry should be appended
5. whether any follow-up should go back into drafts instead of directly mutating live
