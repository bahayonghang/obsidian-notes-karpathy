---
name: kb-compile
description: Incrementally compile an LLM knowledge base wiki from raw sources. Scans raw/ for new or updated materials, generates summaries and concept articles in wiki/, maintains wikilinks and index files, and runs health checks with a quantitative health score. Use this skill when the user says "compile wiki", "编译wiki", "更新知识库", "compile kb", "sync wiki", "lint知识库", "检查知识库", "health check", "process new articles", "编译", "同步wiki", "build wiki", "refresh wiki", "消化文章", or wants to process newly clipped articles into the wiki. Also trigger when the user has added new files to raw/ and asks what to do next, or mentions their wiki feels outdated or disconnected — this skill handles both compilation and diagnostic health checks. This is the core skill — run it after adding new sources to raw/.
---

# KB Compile — Incremental Wiki Compilation

The core engine of the Karpathy knowledge base workflow. Transforms raw source materials into a structured, interlinked wiki that LLMs can efficiently query and humans can browse in Obsidian.

The compilation philosophy: raw data is "source of truth", the wiki is a "compiled artifact". Like a compiler, this process is deterministic and incremental — only new or changed sources trigger recompilation.

## When to Use

- After adding new sources to `raw/` (via Web Clipper or manually)
- User says "compile", "编译", "更新知识库", "sync wiki"
- User says "lint", "health check", "检查知识库" (runs Phase 3 only or full pipeline)
- Periodically to keep the wiki fresh and well-connected

## Prerequisites

- Knowledge base must be initialized (`kb-init` run previously)
- `AGENTS.md` must exist at the vault/project root
- Read `AGENTS.md` first to understand the schema and conventions for this specific knowledge base

## Compilation Pipeline

### Phase 1: Preprocess raw/ (Ingest)

Scan `raw/` and prepare source materials for compilation.

#### 1.1 Discover sources

List all `.md` files in `raw/` (excluding `raw/assets/`). For each file:

1. Read its frontmatter
2. Check if `compiled_at` is null or missing → **new source, needs compilation**
3. Check if file mtime > `compiled_at` → **updated source, needs recompilation**
4. Otherwise → **skip** (already compiled and unchanged)

Report discovery results: "Found X new sources, Y updated sources, Z unchanged (skipped)."

#### 1.2 Validate and enrich frontmatter

For each new/updated source, verify required frontmatter fields exist. If missing, infer them:

- `title`: Extract from first `# heading` or filename
- `source`: Look for URLs in the content
- `date`: Use file creation date or extract from content
- `type`: Infer from content (article, paper, tweet, repo, etc.)
- `tags`: Generate 2-5 relevant topic tags from content analysis
- `clipped_at`: Use file creation time if missing

Update the raw file's frontmatter in-place with the enriched fields. Use `obsidian-markdown` skill for proper YAML frontmatter syntax.

#### 1.3 Update SOURCES.md

Rebuild `wiki/indices/SOURCES.md` with a complete table of all raw sources, their types, dates, and compilation status.

### Phase 2: Incremental Wiki Compilation

The core compilation step. Process each new/updated source and integrate it into the wiki.

#### 2.1 Generate summaries

For each source to compile, create or update `wiki/summaries/{source-filename}.md`:

The summary should be a concise yet thorough distillation of the source:

```markdown
---
title: "Summary: {Source Title}"
source_file: "[[raw/{filename}]]"
source_url: "{url if available}"
key_concepts:
  - "[[Concept A]]"
  - "[[Concept B]]"
created_at: {datetime}
---

# Summary: {Source Title}

> [!abstract] Source
> **Type**: {type} | **Author**: {author} | **Date**: {date}
> **Source**: [[raw/{filename}]]

## Key Points

- {Bullet point 1 — the most important takeaway}
- {Bullet point 2}
- {Bullet point 3}
...

## Detailed Summary

{2-4 paragraphs covering the main content, arguments, data, and conclusions}

## Key Concepts

- [[Concept A]] — {how this source relates to the concept}
- [[Concept B]] — {how this source relates to the concept}

## Notable Quotes / Data

> {Direct quote or specific data point from source}

## Related Sources

- [[wiki/summaries/other-source]] — {relationship}
```

#### 2.2 Extract and compile concepts

After summarizing, identify distinct concepts that deserve their own wiki article. A concept is a recurring idea, technique, person, tool, or theme that appears across multiple sources or is significant enough to warrant dedicated coverage.

For each concept, either **create** a new article or **update** an existing one at `wiki/concepts/{concept-name}.md`:

```markdown
---
title: "{Concept Name}"
aliases:
  - "{Alternative Name}"
category: "{Parent Category}"
tags:
  - concept
  - {topic/subtopic}
related:
  - "[[Other Concept]]"
sources:
  - "[[raw/source-1]]"
  - "[[raw/source-2]]"
created_at: {datetime}
updated_at: {datetime}
---

# {Concept Name}

## Definition

{Clear, concise definition of this concept — 1-2 sentences}

## Overview

{Comprehensive explanation drawn from all sources that mention this concept.
This section grows as more sources are compiled.}

## Key Aspects

### {Aspect 1}

{Details from sources}

### {Aspect 2}

{Details from sources}

## Connections

- [[Related Concept 1]] — {how they relate}
- [[Related Concept 2]] — {how they relate}

## Sources

- [[wiki/summaries/source-1]] — {what this source contributes}
- [[wiki/summaries/source-2]] — {what this source contributes}
```

**When updating an existing concept:**
- Merge new information into existing sections (don't duplicate)
- Add the new source to the `sources` list in frontmatter
- Update `updated_at` timestamp
- Add any new connections discovered
- Preserve existing content — append and refine, never delete

#### 2.3 Maintain wikilinks

After compiling all sources, scan all wiki articles and ensure:

1. **Forward links**: Every mention of a known concept is wrapped in `[[wikilinks]]`
2. **Backlinks**: Obsidian handles this automatically, but verify that `related` fields in frontmatter are bidirectional
3. **Cross-references**: Summaries link to concepts, concepts link to summaries and other concepts

#### 2.4 Update index files

Rebuild all index files to reflect the current state of the wiki:

**`wiki/indices/INDEX.md`** — Master index with all articles:
```markdown
## Statistics
- Total sources: {count}
- Total concepts: {count}
- Total summaries: {count}
- Last compiled: {datetime}

## Concepts
| Concept | Category | Sources | Updated |
|---------|----------|---------|---------|
| [[concept-name]] | {category} | {count} | {date} |

## Summaries
| Summary | Source Type | Date |
|---------|------------|------|
| [[summary-name]] | {type} | {date} |
```

**`wiki/indices/CONCEPTS.md`** — Concept map grouped by category:
```markdown
## {Category 1}
- [[Concept A]] — {one-line description}
  - Related: [[Concept B]], [[Concept C]]
- [[Concept B]] — {one-line description}

## {Category 2}
...
```

**`wiki/indices/RECENT.md`** — Recent changes log:
```markdown
## {Today's Date}
- Compiled {N} new sources
- Created concepts: [[Concept X]], [[Concept Y]]
- Updated concepts: [[Concept Z]]

## {Previous Date}
...
```

#### 2.5 Mark sources as compiled

For each successfully compiled source, update its frontmatter:
```yaml
compiled_at: {current datetime}
```

This prevents recompilation on the next run unless the file is modified.

### Phase 3: Health Check (Lint)

Run automatically after compilation, or independently when the user requests a health check.

#### 3.1 Consistency checks

- Scan for contradictory statements across concept articles
- Flag outdated information (sources older than a threshold)
- Verify that all wikilinks resolve to existing files (no broken links)

#### 3.2 Missing data detection

- Identify concept articles with sparse content (< 100 words)
- Find raw sources without tags or with incomplete frontmatter
- Detect summaries missing `key_concepts`

#### 3.3 Connection discovery

- Compare all concept articles pairwise for potential but unmade connections
- Suggest new wikilinks between articles that share themes or keywords
- Identify concepts that are mentioned in text but don't have their own article yet

#### 3.4 Orphan detection

- Find raw sources with no corresponding summary
- Find concept articles with no incoming links
- Find summaries that link to no concepts

#### 3.5 Staleness detection

Check for content that may need refreshing:

- Flag raw sources where `compiled_at` is older than 30 days — they may benefit from recompilation to pick up new concept connections established since then
- Identify concept articles in `wiki/concepts/` not updated in 60+ days
- List these in the health report as "potentially stale" entries

#### 3.6 Generate health report

Write results to `outputs/health/health-check-{date}.md`:

```markdown
---
title: "Health Check Report"
date: {datetime}
tags:
  - health-check
---

# Knowledge Base Health Check

## Health Score: {score}/100

| Dimension | Score | Details |
|-----------|-------|---------|
| Completeness | {0-100} | {N} raw sources missing summaries, {N} concepts with sparse content |
| Consistency | {0-100} | {N} definition conflicts found |
| Connectivity | {0-100} | {N} orphan nodes, {N} missing cross-links |
| Freshness | {0-100} | {N} concepts not updated in 60+ days, {N} stale compilations |

## Summary
- Sources: {total} ({new} new, {orphaned} orphaned)
- Concepts: {total} ({sparse} sparse, {well-connected} well-connected)
- Broken links: {count}
- Stale entries: {count}

## Issues Found

### Critical
> [!danger] {Issue description}
> {Details and suggested fix}

### Warnings
> [!warning] {Issue description}
> {Details and suggested fix}

### Suggestions
> [!tip] {Suggestion}
> {Details}

## Recommended Actions
1. {Action item with specific file references}
2. {Action item}
```

The health score is calculated as a weighted average: Completeness (30%), Consistency (30%), Connectivity (20%), Freshness (20%). This gives a quick at-a-glance indicator of knowledge base quality over time.

## First-Time vs Incremental Compilation

The compiler behaves differently depending on whether the wiki already exists:

### First-time compilation (empty `wiki/`)

When `wiki/concepts/` and `wiki/summaries/` are empty or don't exist yet:

1. **Read all sources first** before writing anything — scan every file in `raw/` to build a mental map of all concepts, themes, and connections across the entire corpus
2. **Plan the concept taxonomy** — decide which concepts deserve their own articles and how they relate to each other, aiming for a coherent structure rather than a bag of isolated articles
3. **Compile in chronological order** (oldest source first) — this mirrors the natural evolution of ideas
4. **After all sources are compiled**, do a final cross-linking pass: re-read all concept articles and add connections that only became visible after the full corpus was processed
5. **Generate all index files** from scratch

This "read everything, then write" approach produces a much more coherent initial wiki than processing sources one at a time. Expect the first compilation to take significantly longer.

### Incremental compilation (existing `wiki/`)

When the wiki already has content:

1. **Read `wiki/indices/INDEX.md` first** to understand the existing concept structure
2. **Only process new/updated sources** (Phase 1 discovery handles this)
3. **Merge into existing concepts** — update, don't recreate
4. **Look for new connections** between the new source and existing concepts
5. **Update indices incrementally** — append rather than rebuild from scratch (unless the index is corrupted)

Incremental runs are much faster and should be the normal operating mode after initial setup.

## Execution Notes

- Always read `AGENTS.md` first to understand this knowledge base's specific schema
- Use `obsidian-markdown` skill for all Markdown syntax
- Use `obsidian-cli` skill for searching and reading vault content when available
- Process sources in chronological order (oldest first) for consistent concept evolution
- When the wiki is large (>50 concepts), read `wiki/indices/INDEX.md` before compilation to understand existing structure and avoid creating duplicate concepts
- For very large compilations (>10 new sources), consider batching: compile 5 at a time, update indices, then continue
- Always report progress: "Compiling source 3/7: {filename}..."
