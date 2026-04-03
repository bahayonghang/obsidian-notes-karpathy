# Query & Output

Extract value from your compiled knowledge base through search, Q&A, and multi-format output generation.

## Overview

Once your wiki is compiled, the `kb-query` skill lets you **extract value** from all that curated knowledge. This is where the investment pays off.

## Three Capabilities

### 1. Search

**When**: You want to find specific information in the wiki

**Trigger**: "search kb", "搜索知识库", "find information about..."

**Process**:
1. Start with `wiki/indices/INDEX.md` for overview
2. Check `wiki/indices/CONCEPTS.md` for concept map
3. Full-text search with `obsidian-cli` or Grep
4. Filter by tags in frontmatter

**Output**: Structured list of relevant articles with excerpts

### 2. Q&A Research

**When**: You have a complex question

**Trigger**: "query kb", "问知识库", "research", "what does the wiki say about..."

**Process**:
1. Parse the question for key concepts
2. Navigate wiki indices and concept articles
3. Follow wikilinks to related content
4. Synthesize answer with full citations

**Output**: Comprehensive answer with source traceability

### 3. Multi-Format Output

**When**: You want to share knowledge

**Trigger**: "create report", "create slides", "generate diagram", "生成报告", "生成幻灯片"

**Formats**:
- **Markdown reports** → `outputs/reports/`
- **Marp slides** → `outputs/slides/`
- **Mermaid diagrams** → `outputs/charts/`
- **Canvas files** → `outputs/charts/`

## Query Examples

### Simple Search

```
User: search for transformer variants

Assistant: ## Search Results: "transformer variants"

Found 5 relevant articles:

1. **[[wiki/concepts/vision-transformer]]** — ViT applies transformers to image patches
   > Vision Transformers (ViT) treat image patches as tokens, applying standard transformer architecture...
   > Tags: #architecture/transformer #application/cv

2. **[[wiki/concepts/swin-transformer]]** — Hierarchical transformer with shifted windows
   > Swin Transformer uses shifted windowing for efficient hierarchical vision...
   > Tags: #architecture/transformer #application/cv

_Searched 23 articles in wiki/_
```

### Complex Q&A

```
User: How has attention mechanism evolved from the original transformer?

Assistant: ## Answer: Evolution of Attention Mechanisms

The attention mechanism has evolved significantly from the original "Attention Is All You Need" paper...

### Key Findings

1. **Multi-Head Attention (2017)** — The original transformer introduced multi-head self-attention
   - Source: [[wiki/summaries/attention-is-all-you-need]]

2. **Sparse Attention (2019-2020)** — Reduced quadratic complexity
   - Sources: [[wiki/concepts/sparse-transformer]], [[wiki/summaries/reformer]]

3. **Linear Attention (2020+)** — O(n) complexity approaches
   - Sources: [[wiki/concepts/linear-attention]], [[wiki/summaries/performer]]

### Analysis

The evolution trend shows a clear pattern: maintaining expressiveness while reducing computational complexity...

### References

- [[wiki/concepts/attention-mechanism]] — Core concept definition
- [[wiki/concepts/transformer-architecture]] — Original architecture
- [[wiki/summaries/attention-is-all-you-need]] — Original paper
```

### Output Generation

```
User: create slides about the history of transformer architectures

Assistant: Generating slide deck...

✓ Created outputs/slides/2026-04-03-transformer-history.md

The slide deck includes:
- 15 slides covering the evolution from 2017 to 2026
- Key milestones and innovations
- Mermaid timeline diagram
- Full citations to wiki sources

Open in Obsidian with Marp plugin to view.
```

## Output Formats

### Markdown Reports

**Best for**: Comprehensive research, literature reviews, detailed analysis

**Structure**:
- Executive summary
- Detailed sections with wikilinks
- Conclusions and implications
- Reference table

**Example**:

```markdown
---
title: "State of Transformer Research"
date: 2026-04-03
tags:
  - report
  - transformers
sources_consulted: 23
---

# State of Transformer Research

## Executive Summary

This report synthesizes findings from 23 sources...

## Transformer Variants

[[wiki/concepts/vision-transformer]] has been applied to...

## References

| Source | Type | Key Contribution |
|--------|------|-----------------|
| [[attention-is-all-you-need]] | paper | Original transformer |
```

### Marp Slides

**Best for**: Presentations, talks, teaching materials

**Guidelines**:
- 10-20 slides for comprehensive topics
- 5-7 bullet points max per slide
- Include speaker notes for context
- Use images with `![bg right](image.png)`

**View slides**: Open in Obsidian with Marp Slides plugin, or export to PDF/PPTX

### Mermaid Diagrams

**Best for**: Visualizing relationships, timelines, concept maps

**Types**:
- **Graph diagrams**: Concept relationships
- **Timeline**: Chronological evolution
- **Mind map**: Hierarchical topic breakdown

**Render**: Diagrams render natively in Obsidian

### Canvas Files

**Best for**: Interactive knowledge maps, visual exploration

**Features**:
- Drag and arrange nodes
- Create custom groupings
- Add visual annotations
- Export as image

**View**: Open `.canvas` file in Obsidian Canvas

## Archiving Insights

During Q&A, the LLM may discover **new connections** or **emerging concepts**. It will offer to archive them:

```
Assistant: This answer revealed some interesting connections between sparse attention and efficient transformers. 
Would you like me to archive this as:
1. A new concept article: [[wiki/concepts/efficient-attention]]
2. A research report: outputs/reports/2026-04-03-efficient-attention.md
3. Update existing concepts: Add these connections to [[wiki/concepts/sparse-attention]]
```

**Recommendation**: Archive insights regularly — this grows the wiki organically.

## Best Practices

### 1. Ask Complex Questions

The bigger the wiki, the better the answers. Don't ask "what is X?" — ask "how has X evolved and what are the trade-offs?"

### 2. Always Review Outputs

Generated content is a starting point, not final. Review for:
- Accuracy (check source links)
- Completeness (is anything missing?)
- Clarity (does it make sense?)

### 3. Share Widely

Outputs are designed for sharing:
- Reports → Send to team, publish as blog
- Slides → Present at meetings, conferences
- Diagrams → Include in papers, documentation

### 4. Iterate

Query → Generate → Archive → Recompile → Query again

Each cycle improves the wiki.

## Next Steps

- [**Health Checks**](/workflow/health-checks) — Maintain wiki quality
- [**kb-query Skill**](/skills/kb-query) — Detailed skill reference
- [**Multi-Format Output**](/skills/kb-query#capability-3-multi-format-output) — Output format guide
