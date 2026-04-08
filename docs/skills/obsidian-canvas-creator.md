# Canvas Creator

> Companion skill reference. This page documents an adjacent ecosystem skill, not one of the six core skills shipped by this repository.

Create Obsidian Canvas files for visual knowledge mapping.

## Overview

Obsidian Canvas is an infinite canvas where you can arrange notes, create connections, and visualize relationships. The `obsidian-canvas-creator` skill generates `.canvas` files programmatically.

## What is Canvas?

Canvas files (`.canvas`) are JSON files that define:
- **Nodes**: Notes, text blocks, images, web pages
- **Edges**: Connections between nodes with labels
- **Layout**: Position and size of each element

Rendered in Obsidian as interactive visual maps.

## Usage in Knowledge Base

The Karpathy skills use Canvas for:

### Concept Relationship Networks

Visualize how concepts connect:

```
Concept A ──related to──> Concept B
    │                        │
    │derived from            │extends
    ▼                        ▼
Concept C ──builds on──> Concept D
```

### Source-to-Concept Mapping

Show which sources contribute to which concepts:

```
Source 1 ──> Concept A
Source 2 ──> Concept A
Source 2 ──> Concept B
Source 3 ──> Concept C
```

### Topic Clusters

Group related concepts by category:

```
┌─ Category 1 ─────────────┐
│  Concept A  Concept B    │
│     Concept C            │
└──────────────────────────┘

┌─ Category 2 ─────────────┐
│  Concept D  Concept E    │
└──────────────────────────┘
```

## Output Location

Canvas files are saved to `outputs/charts/{topic}.canvas`.

## Installation

Install from [kepano/obsidian-skills](https://github.com/kepano/obsidian-skills):

```bash
cp -r obsidian-skills/obsidian-canvas-creator ~/.claude/skills/
```

## Trigger Phrases

- "知识图谱" / "canvas" / "visual knowledge map"
- "show relationships" / "concept map"
- "create canvas" / "generate canvas"

## Reference

- [kepano/obsidian-skills](https://github.com/kepano/obsidian-skills)
- [Obsidian Canvas Documentation](https://help.obsidian.md/Plugins/Canvas)
