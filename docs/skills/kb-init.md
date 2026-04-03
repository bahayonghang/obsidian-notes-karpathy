# kb-init — Knowledge Base Initialization

One-time setup skill that creates the standard directory structure and AGENTS.md schema.

## When to Use

- User says "初始化知识库", "kb init", "create knowledge base", "karpathy setup"
- Starting a new LLM-driven knowledge base project
- Setting up Obsidian for structured knowledge management with LLM compilation

**Important**: This is a **one-time setup** skill — run it once per vault/project.

## What It Does

### Step 1: Determine Vault Root

Asks the user which directory to initialize. This should be:
- An Obsidian vault root, or
- A subdirectory within a vault dedicated to this knowledge base topic

If the user provides a topic name (e.g., "deep learning research"), it customizes the AGENTS.md and index files.

### Step 2: Create Directory Structure

Creates the standard Karpathy directory tree:

```
{root}/
├── raw/                     # Raw source materials
│   └── assets/              # Images and attachments from raw sources
├── wiki/                    # LLM-compiled wiki (DO NOT manually edit)
│   ├── concepts/            # Concept articles (one per key concept)
│   ├── summaries/           # Summaries of each raw source
│   └── indices/             # Index and navigation files
├── outputs/                 # Query results and generated content
│   ├── reports/             # Research reports
│   ├── slides/              # Marp slide decks
│   └── charts/              # Mermaid diagrams and visualizations
└── AGENTS.md                # Schema definition for LLM agents
```

### Step 3: Generate AGENTS.md

Creates `AGENTS.md` at the root with:

- **Overview**: Knowledge base topic and purpose
- **Directory structure table**: Purpose and ownership (who writes)
- **Frontmatter schemas**: For raw sources, concept articles, and summaries
- **Compilation rules**: Seven core rules for deterministic compilation
- **File naming conventions**: Consistent naming patterns

The topic field is customized based on user input.

### Step 4: Create Initial Index Files

Creates four starter index files:

#### `wiki/indices/INDEX.md`

Master index with statistics and article listing:

```markdown
---
title: Knowledge Base Index
updated_at: {current datetime}
---

# Knowledge Base Index

> [!info] Auto-maintained
> This index is automatically maintained by `kb-compile`. Do not edit manually.

## Statistics

- Total sources: 0
- Total concepts: 0
- Total summaries: 0
- Last compiled: Never

## All Articles

_No articles yet. Run `kb-compile` after adding sources to `raw/`._
```

#### `wiki/indices/CONCEPTS.md`

Concept map grouped by category:

```markdown
---
title: Concept Map
updated_at: {current datetime}
---

# Concept Map

> [!info] Auto-maintained
> This concept map is automatically maintained by `kb-compile`.

_No concepts yet. Run `kb-compile` after adding sources to `raw/`._
```

#### `wiki/indices/SOURCES.md`

Registry of all raw sources:

```markdown
---
title: Source Registry
updated_at: {current datetime}
---

# Source Registry

> [!info] Auto-maintained
> This registry tracks all raw sources and their compilation status.

| Source | Type | Date | Compiled |
|--------|------|------|----------|
| _No sources yet_ | | | |
```

#### `wiki/indices/RECENT.md`

Log of recent changes:

```markdown
---
title: Recent Updates
updated_at: {current datetime}
---

# Recent Updates

> [!info] Auto-maintained
> Shows the most recent changes to the knowledge base.

_No updates yet._
```

### Step 5: Confirm Initialization

Reports to the user:
- What was created (directory tree)
- How to add raw sources (Web Clipper or manual)
- Next steps: add sources to `raw/`, then run `kb-compile`

## Implementation Details

### Tools Used

- **`obsidian-markdown` skill**: For proper Obsidian Flavored Markdown syntax (wikilinks, callouts, properties)
- **`obsidian-cli` skill**: If available, for file creation within the vault
- **Write tool**: Directly create files when CLI is not available

### Safety Checks

- If the directory already has content, **warn the user** before overwriting
- Always use UTF-8 encoding
- Validate that the target directory exists and is accessible

## Post-Initialization

After running `kb-init`, the user should:

1. **Add sources** to `raw/` using Web Clipper or manually
2. **Run `kb-compile`** to process sources into the wiki
3. **Query the wiki** using `kb-query`

## Example Usage

```
User: I want to set up a knowledge base for transformer research
Assistant: I'll initialize a Karpathy-style knowledge base for "transformer research"...

✓ Created directory structure:
  raw/
  raw/assets/
  wiki/concepts/
  wiki/summaries/
  wiki/indices/
  outputs/reports/
  outputs/slides/
  outputs/charts/

✓ Generated AGENTS.md with transformer-specific schema
✓ Created initial index files

Next steps:
1. Add sources to raw/ (use Web Clipper or create .md files)
2. Run "compile wiki" to build the wiki
3. Use "query kb" to ask questions
```

## Next Steps

- [**kb-compile**](/skills/kb-compile) — The core compilation engine
- [**Directory Structure**](/guide/directory-structure) — Understanding the layout
- [**AGENTS.md Schema**](/guide/agents-schema) — The rulebook
