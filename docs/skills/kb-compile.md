# kb-compile — Incremental Wiki Compilation

The core engine of the Karpathy knowledge base workflow. Transforms raw source materials into a structured, interlinked wiki.

## When to Use

- After adding new sources to `raw/` (via Web Clipper or manually)
- User says "compile", "编译", "更新知识库", "sync wiki"
- User says "lint", "health check", "检查知识库" (runs Phase 3 only or full pipeline)
- Periodically to keep the wiki fresh and well-connected

**This is the core skill** — run it after adding new sources to `raw/`.

## Compilation Philosophy

> Raw data is "source of truth", the wiki is a "compiled artifact". Like a compiler, this process is deterministic and incremental — only new or changed sources trigger recompilation.

## Prerequisites

- Knowledge base must be initialized (`kb-init` run previously)
- `AGENTS.md` must exist at the vault/project root
- Read `AGENTS.md` first to understand the schema and conventions

## The Compilation Pipeline

### Phase 1: Preprocess raw/ (Ingest)

Scan `raw/` and prepare source materials for compilation.

#### 1.1 Discover Sources

List all `.md` files in `raw/` (excluding `raw/assets/`). For each file:

1. Read its frontmatter
2. Check if `compiled_at` is null or missing → **new source, needs compilation**
3. Check if file mtime > `compiled_at` → **updated source, needs recompilation**
4. Otherwise → **skip** (already compiled and unchanged)

**Report**: "Found X new sources, Y updated sources, Z unchanged (skipped)."

#### 1.2 Validate and Enrich Frontmatter

For each new/updated source, verify required frontmatter fields exist. If missing, infer them:

| Field | Inference Strategy |
|-------|-------------------|
| `title` | Extract from first `# heading` or filename |
| `source` | Look for URLs in the content |
| `date` | Use file creation date or extract from content |
| `type` | Infer from content (article, paper, tweet, repo, etc.) |
| `tags` | Generate 2-5 relevant topic tags from content analysis |
| `clipped_at` | Use file creation time if missing |

Update the raw file's frontmatter in-place with enriched fields.

#### 1.3 Update SOURCES.md

Rebuild `wiki/indices/SOURCES.md` with a complete table of all raw sources, their types, dates, and compilation status.

### Phase 2: Incremental Wiki Compilation

The core compilation step. Process each new/updated source and integrate it into the wiki.

#### 2.1 Generate Summaries

For each source to compile, create or update `wiki/summaries/{source-filename}.md`:

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

#### 2.2 Extract and Compile Concepts

After summarizing, identify distinct concepts that deserve their own wiki article. A **concept** is a recurring idea, technique, person, tool, or theme that appears across multiple sources or is significant enough to warrant dedicated coverage.

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
- **Preserve existing content** — append and refine, never delete

#### 2.3 Maintain Wikilinks

After compiling all sources, scan all wiki articles and ensure:

1. **Forward links**: Every mention of a known concept is wrapped in `[[wikilinks]]`
2. **Backlinks**: Obsidian handles this automatically, but verify that `related` fields in frontmatter are bidirectional
3. **Cross-references**: Summaries link to concepts, concepts link to summaries and other concepts

#### 2.4 Update Index Files

Rebuild all index files to reflect the current state of the wiki.

**`wiki/indices/INDEX.md`** — Master index:

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

**`wiki/indices/CONCEPTS.md`** — Concept map by category:

```markdown
## {Category 1}
- [[Concept A]] — {one-line description}
  - Related: [[Concept B]], [[Concept C]]
- [[Concept B]] — {one-line description}

## {Category 2}
...
```

**`wiki/indices/RECENT.md`** — Recent changes:

```markdown
## {Today's Date}
- Compiled {N} new sources
- Created concepts: [[Concept X]], [[Concept Y]]
- Updated concepts: [[Concept Z]]

## {Previous Date}
...
```

#### 2.5 Mark Sources as Compiled

For each successfully compiled source, update its frontmatter:

```yaml
compiled_at: {current datetime}
```

This prevents recompilation on the next run unless the file is modified.

### Phase 3: Health Check (Lint)

Run automatically after compilation, or independently when the user requests a health check.

#### 3.1 Consistency Checks

- Scan for contradictory statements across concept articles
- Flag outdated information (sources older than a threshold)
- Verify that all wikilinks resolve to existing files (no broken links)

#### 3.2 Missing Data Detection

- Identify concept articles with sparse content (< 100 words)
- Find raw sources without tags or with incomplete frontmatter
- Detect summaries missing `key_concepts`

#### 3.3 Connection Discovery

- Compare all concept articles pairwise for potential but unmade connections
- Suggest new wikilinks between articles that share themes or keywords
- Identify concepts that are mentioned in text but don't have their own article yet

#### 3.4 Orphan Detection

- Find raw sources with no corresponding summary
- Find concept articles with no incoming links
- Find summaries that link to no concepts

#### 3.5 Generate Health Report

Write results to `outputs/reports/health-check-{date}.md`:

```markdown
---
title: "Health Check Report"
date: {datetime}
---

# Knowledge Base Health Check

## Summary
- Overall health: {Good / Needs Attention / Poor}
- Sources: {total} ({new} new, {orphaned} orphaned)
- Concepts: {total} ({sparse} sparse, {well-connected} well-connected)
- Broken links: {count}

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

## Execution Notes

- **Always read `AGENTS.md` first** to understand this knowledge base's specific schema
- Use `obsidian-markdown` skill for all Markdown syntax
- Use `obsidian-cli` skill for searching and reading vault content when available
- **Process sources in chronological order** (oldest first) for consistent concept evolution
- When the wiki is large (>50 concepts), read `wiki/indices/INDEX.md` before compilation to understand existing structure and avoid creating duplicate concepts
- For very large compilations (>10 new sources), consider batching: compile 5 at a time, update indices, then continue
- **Always report progress**: "Compiling source 3/7: {filename}..."

## Next Steps

- [**kb-query**](/skills/kb-query) — Search and query the compiled wiki
- [**Health Checks**](/workflow/health-checks) — Understanding the lint pipeline
- [**Workflow: Compile**](/workflow/compile) — Detailed compilation workflow
