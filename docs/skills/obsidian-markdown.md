# Obsidian Markdown

> Companion skill reference. This page documents an adjacent ecosystem skill, not one of the six core skills shipped by this repository.

Obsidian Flavored Markdown syntax reference — the foundation for all wiki content.

## Overview

All content in the knowledge base uses **Obsidian Flavored Markdown**, which extends standard Markdown with additional features for knowledge management.

## Key Features

### Wikilinks

The most important feature. Create bidirectional links between notes:

```markdown
[[Concept Name]]
[[concept-name|Custom Link Text]]
[[path/to/note]]
```

Wikilinks are used extensively throughout the wiki to connect:
- Summaries to concepts
- Concepts to related concepts
- Concepts back to sources
- Reports to source material

### Callouts

Rich informational blocks with icons and colors:

```markdown
> [!info] Title
> Informational content

> [!warning] Title
> Warning content

> [!danger] Title
> Critical/dangerous content

> [!tip] Title
> Helpful tip

> [!abstract] Title
> Summary/abstract content

> [!example] Title
> Example content
```

Callouts are used in:
- Index files (info blocks about auto-maintenance)
- Summaries (abstract blocks for source info)
- Health reports (warning, danger, tip blocks)

### YAML Frontmatter

Metadata at the top of markdown files:

```yaml
---
title: "Document Title"
tags:
  - tag1
  - tag2
---
```

Used for:
- Source tracking (`compiled_at`, `clipped_at`)
- Concept metadata (`category`, `related`, `sources`)
- Organization and filtering

### Tags

Hierarchical tags using slashes:

```markdown
#topic/subtopic
#concept
#report
```

Tags enable:
- Filtering search results
- Grouping related content
- Identifying file types

## Usage in Knowledge Base

Every SKILL.md file and wiki document uses these features. The LLM must:

1. **Always use `[[wikilinks]]`** instead of standard markdown links for internal links
2. **Use callout blocks** for metadata and warnings
3. **Include proper frontmatter** according to the schemas in AGENTS.md
4. **Use hierarchical tags** for consistent organization

## Reference

- [Obsidian Help: Markdown](https://help.obsidian.md/Editing+and+formatting/Basic+formatting+syntax)
- [Obsidian Help: Wikilinks](https://help.obsidian.md/Linking+notes+and+attachments/Internal+links)
- [Obsidian Help: Callouts](https://help.obsidian.md/Editing+and+formatting/Callouts)
