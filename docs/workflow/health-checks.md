# Health Checks

Maintain the quality and integrity of your knowledge base wiki.

## Overview

Health checks (also called "lint") are automated quality assessments that identify issues in your compiled wiki. They run automatically after each compilation, or can be triggered independently.

## When to Run

- **Automatically**: After every `kb-compile` run
- **On demand**: "health check", "lint", "检查知识库"
- **Periodically**: Weekly or monthly to catch drift

## What Gets Checked

### 1. Consistency Checks

**Purpose**: Ensure the wiki doesn't contain contradictory or outdated information.

**Checks**:
- **Contradictory statements**: Concept A says X is true, Concept B says X is false
- **Outdated information**: Sources older than a threshold (e.g., 2 years)
- **Broken wikilinks**: Links that point to non-existent files

**Example issues**:

```
> [!danger] Contradiction Detected
> [[wiki/concepts/batch-normalization]] claims batch norm is essential for training
> [[wiki/concepts/group-normalization]] claims group norm outperforms batch norm
> Both cite different sources — consider reconciling

> [!warning] Outdated Source
> [[wiki/concepts/rnn]] primarily cites sources from 2015-2017
> Consider adding recent developments (2020+)

> [!danger] Broken Link
> [[wiki/concepts/attention]] links to [[wiki/concepts/multi-head-attention]]
> but the target file does not exist
```

### 2. Missing Data Detection

**Purpose**: Identify incomplete or sparse content.

**Checks**:
- **Sparse concepts**: Articles with < 100 words
- **Incomplete frontmatter**: Raw sources missing tags or metadata
- **Missing key concepts**: Summaries without `key_concepts` field

**Example issues**:

```
> [!warning] Sparse Concept
> [[wiki/concepts/layer-normalization]] has only 67 words
> Consider expanding with more sources

> [!warning] Missing Tags
> [[raw/2026-04-03-some-article]] has no tags
> Add 2-5 tags for better searchability

> [!warning] Missing Key Concepts
> [[wiki/summaries/some-source]] has empty key_concepts
> Update summary to identify key concepts
```

### 3. Connection Discovery

**Purpose**: Find opportunities to improve wiki interconnectedness.

**Checks**:
- **Unmade connections**: Concepts that share themes but aren't linked
- **Missing concept articles**: Concepts mentioned in text but without their own article
- **Suggested wikilinks**: Places where `[[wikilinks]]` should be added

**Example issues**:

```
> [!tip] Suggested Connection
> [[wiki/concepts/attention]] and [[wiki/concepts/self-attention]] 
> share 80% keyword overlap but are not linked
> Consider adding a wikilink

> [!tip] Missing Concept Article
> "Positional encoding" is mentioned in 5 articles but has no concept page
> Consider creating [[wiki/concepts/positional-encoding]]
```

### 4. Orphan Detection

**Purpose**: Find content that isn't connected to the rest of the wiki.

**Checks**:
- **Orphaned sources**: Raw sources with no corresponding summary
- **Orphaned concepts**: Concept articles with no incoming links
- **Disconnected summaries**: Summaries that link to no concepts

**Example issues**:

```
> [!warning] Orphaned Source
> [[raw/2026-04-01-older-article]] has no summary
> Run kb-compile to generate one

> [!warning] Orphaned Concept
> [[wiki/concepts/obscure-technique]] has no incoming links from other articles
> Either remove it or add connections from related concepts

> [!warning] Disconnected Summary
> [[wiki/summaries/some-source]] doesn't link to any concepts
> Summary may be too generic — identify key concepts
```

## Health Report

After checks complete, a report is written to `outputs/reports/health-check-{date}.md`:

```markdown
---
title: "Health Check Report"
date: 2026-04-03T15:30:00
---

# Knowledge Base Health Check

## Summary
- Overall health: Good
- Sources: 45 (3 new, 0 orphaned)
- Concepts: 67 (2 sparse, 54 well-connected)
- Broken links: 1

## Issues Found

### Critical
> [!danger] Broken Link
> [[wiki/concepts/attention]] links to [[wiki/concepts/multi-head-attention]]
> Target file does not exist

### Warnings
> [!warning] Sparse Concept
> [[wiki/concepts/layer-normalization]] has only 67 words

### Suggestions
> [!tip] Suggested Connection
> [[wiki/concepts/attention]] and [[wiki/concepts/self-attention]] share keywords

## Recommended Actions
1. Create [[wiki/concepts/multi-head-attention]] or fix link in [[wiki/concepts/attention]]
2. Expand [[wiki/concepts/layer-normalization]] with more sources
3. Add wikilink between attention and self-attention articles
```

## Health Ratings

### Good

- Most concepts are well-connected (>80% have incoming links)
- No broken wikilinks
- Few or no sparse concepts
- All sources have summaries

### Needs Attention

- Some orphaned content
- A few broken links
- Several sparse concepts
- Missing some connections

### Poor

- Many broken links
- Significant orphaned content
- Contradictory statements
- Outdated information not flagged

## Fixing Issues

### Critical Issues (Fix Immediately)

- **Broken links**: Create missing files or update links
- **Contradictions**: Reconcile conflicting statements, cite sources

### Warnings (Address Soon)

- **Sparse concepts**: Add more sources, recompile
- **Orphaned sources**: Run compilation to generate summaries
- **Missing frontmatter**: Enrich source files manually or via recompilation

### Suggestions (Consider for Future)

- **Suggested connections**: Add wikilinks where appropriate
- **Missing concept articles**: Create articles for frequently mentioned concepts
- **Tag improvements**: Add missing tags to sources

## Automated vs. Manual Fixes

### The LLM Can Auto-Fix

- Create missing concept articles (if enough source material exists)
- Add wikilinks where concepts are mentioned
- Update indices and cross-references
- Enrich frontmatter for incomplete sources

### You Should Manually Fix

- Contradictory statements (requires judgment)
- Outdated information (requires domain knowledge)
- Structural reorganization (requires planning)

## Best Practices

### 1. Review Reports Regularly

Don't ignore health reports. Review them weekly and address critical issues promptly.

### 2. Recompile After Adding Sources

Many issues (sparse concepts, orphans) resolve automatically when you add more sources and recompile.

### 3. Track Trends

Compare health reports over time:
- Is the wiki getting more connected?
- Are broken links decreasing?
- Is concept quality improving?

### 4. Set Quality Thresholds

Define your own standards:
- Minimum concept length (e.g., 100 words)
- Maximum age before flagging as outdated
- Minimum incoming links per concept

## Next Steps

- [**Compile Wiki**](/workflow/compile) — Run compilation to auto-fix issues
- [**Query & Output**](/workflow/query) — Extract value from healthy wiki
- [**kb-compile Skill**](/skills/kb-compile) — Detailed compilation reference
