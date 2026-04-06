# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Claude Code **skills-only** project. The deliverables are SKILL.md files, shared references, docs pages, and eval fixtures that implement an LLM-driven knowledge base workflow for Obsidian, inspired by Andrej Karpathy's approach.

Core workflow: `raw/` (human adds immutable sources) → `kb-compile` (LLM compiles wiki) → `wiki/` (LLM maintains) → `kb-query` / `kb-health` → `outputs/`

## Repository Structure

- `skills/obsidian-notes-karpathy/SKILL.md` — Package-level entry skill
- `skills/obsidian-notes-karpathy/references/` — Shared templates, lifecycle matrix, and conventions used by multiple skills
- `skills/obsidian-notes-karpathy/scripts/` — Deterministic helpers for lifecycle detection, compile delta scanning, and mechanical linting
- `skills/obsidian-notes-karpathy/evals/` — Eval prompts, fixtures, and trigger-eval coverage
- `skills/kb-init/SKILL.md`, `skills/kb-compile/SKILL.md`, `skills/kb-query/SKILL.md`, `skills/kb-health/SKILL.md` — Operational skills
- `ref/` — Read-only reference material (original Karpathy tweet thread). Do not modify or distribute.
- `README.md` (English) and `README_CN.md` (Chinese) — Must be kept in sync when either is updated.

## Skill Development

Operational skills live at `skills/<skill-name>/SKILL.md`. The package entry skill lives at `skills/obsidian-notes-karpathy/SKILL.md`.

Every `SKILL.md` follows this format:

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
- Keep `raw/` immutable in all user-facing guidance. Compilation state belongs in `wiki/` metadata or health outputs, never in raw source files.
- Keep generated `AGENTS.md` and `CLAUDE.md` templates aligned whenever initialization behavior changes.

## Installation (for end users)

```bash
cp -r skills/* ~/.claude/skills/
cp -r skills/* ~/.codex/skills/
```
