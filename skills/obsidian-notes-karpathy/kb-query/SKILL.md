---
name: kb-query
description: Query, search, and generate outputs from a compiled Obsidian knowledge base. Use this skill whenever the user asks what their notes say about something, wants to search or summarize the wiki, asks for a report, thread, post, article, slide deck, or talk outline from their notes, says "query kb", "search kb", "问知识库", "搜索知识库", "帮我研究", "summarize what I know about", "write a report on", "把笔记写成文章", "生成推文串", "生成报告", "生成幻灯片", or wants a substantive answer to be archived instead of disappearing into chat.
---

# KB Query

Search, answer, and generate outputs from the compiled wiki.

The key principle is that substantive research answers become persistent knowledge artifacts. Do not treat valuable Q&A as disposable chat.

## Read before querying

Read these files first:

- local `AGENTS.md`
- local `CLAUDE.md`
- `../references/file-model.md`
- `../references/qa-template.md`
- `../references/content-output-template.md`
- `../references/search-upgrades.md`
- `../references/activity-log-template.md`

Then start with:

- `wiki/index.md`
- `wiki/indices/INDEX.md`
- `wiki/indices/CONCEPTS.md`
- `wiki/indices/SOURCES.md`
- `wiki/indices/ENTITIES.md` when it exists

## Mode selection

Choose one of three modes:

1. search mode for finding relevant notes and evidence quickly
2. research mode for answering a substantive question
3. publish mode for generating outward-facing artifacts grounded in the wiki

## Search mode

For search-like requests:

1. inspect `wiki/index.md`
2. inspect concept, source, and optional entity indices
3. use ordinary file search or `obsidian-cli`
4. use backlinks, unlinked mentions, or property search when they can surface related evidence faster than plain text search
5. return a structured result list with short relevance notes and a few excerpts

Prefer markdown-first navigation before suggesting any search infrastructure upgrade.

## Research mode

For complex questions, follow this order:

### Step 1: understand the question

Identify:

- key concepts
- key entities when relevant
- expected answer shape
- whether the question is factual, comparative, analytical, or exploratory

### Step 2: check existing Q&A

Search `outputs/qa/` first.

If you find a strong prior answer:

- use it as a starting point
- update or extend it when needed
- avoid re-deriving the same answer from scratch

### Step 3: navigate the wiki

Read the most relevant concept pages, entity pages, and summaries, following wikilinks where they add evidence or contrast.

### Step 4: synthesize with provenance

Answer using wiki citations and explicit uncertainty when sources disagree or evidence is thin.

### Step 5: archive by default

If the answer is substantive, save it to `outputs/qa/{date}-{slug}.md` using `../references/qa-template.md`.

Only skip archival when:

- the user explicitly says not to save it, or
- the interaction is trivial and purely operational

### Step 6: append the query log

When the answer is archived or materially updated, append a `query` entry to `wiki/log.md` using `../references/activity-log-template.md`.

### Step 7: feed useful insights back into the wiki

After archiving, check whether the answer revealed:

- a new concept page that should exist
- a new entity page that should exist
- new evidence for an existing concept or entity
- a missing connection between pages

Update the wiki when the improvement is mechanical and well-supported.

## Publish mode

Use this when the user wants content for an audience rather than only an internal answer.

Supported artifacts:

- `outputs/content/articles/` for article drafts
- `outputs/content/threads/` for X or social threads
- `outputs/content/talks/` for talk outlines
- `outputs/reports/` for markdown reports
- `outputs/slides/` for Marp decks
- `outputs/charts/` for Mermaid or Canvas outputs

For every generated artifact:

- look for an existing supporting Q&A note first
- if no strong supporting Q&A exists, create one before or immediately after the publish artifact
- use `../references/content-output-template.md` when the output is audience-facing
- cite the wiki pages and prior Q&A consulted
- keep filenames date-prefixed and slugged
- append a `publish` entry to `wiki/log.md` when the artifact is substantive

## Output requirements

In your user-facing summary, report:

1. which wiki pages or prior Q&A you relied on
2. which mode you used
3. where the new artifact was saved
4. whether you updated any concept or entity pages as a result
5. whether you appended a `query` or `publish` entry to `wiki/log.md`

## Tooling notes

- Use `obsidian-markdown` for all markdown outputs and wikilinks.
- Use `obsidian-cli` when available for vault-aware search.
- Use `obsidian-canvas-creator` only when the user explicitly wants a canvas-style artifact.
