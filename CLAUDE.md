# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Claude Code **skills-only** project — there is no application code, build system, or test suite. The deliverables are SKILL.md files that implement an LLM-driven knowledge base workflow for Obsidian, inspired by Andrej Karpathy's approach.

Core workflow: `raw/` (human adds sources) → `kb-compile` (LLM compiles wiki) → `wiki/` (LLM maintains) → `kb-query` → `outputs/`

## Repository Structure

- `skills/obsidian-notes-karpathy/` — The distributable skills (kb-init, kb-compile, kb-query)
- `ref/` — Read-only reference material (original Karpathy tweet thread). Do not modify or distribute.
- `README.md` (English) and `README_CN.md` (Chinese) — Must be kept in sync when either is updated.

## Skill Development

Each skill lives at `skills/obsidian-notes-karpathy/{skill-name}/SKILL.md` with this format:

```yaml
---
name: skill-name
description: When and how to use this skill (used for matching)
---

# Skill instructions in markdown
```

- Follow the patterns established in existing SKILL.md files for structure and detail level
- Skills must use **Obsidian Flavored Markdown**: `[[wikilinks]]`, `> [!callout]` blocks, YAML frontmatter with properties
- Include both English and Chinese trigger phrases in the description field (e.g., "compile wiki" / "编译wiki")
- Reference dependent skills by name: `obsidian-markdown`, `obsidian-cli`, `obsidian-canvas-creator` (from [kepano/obsidian-skills](https://github.com/kepano/obsidian-skills))

## Installation (for end users)

```bash
cp -r skills/obsidian-notes-karpathy/* ~/.claude/skills/
```
