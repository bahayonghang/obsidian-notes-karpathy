# Karpathy Workflow

This workflow treats a knowledge base like a codebase with source, build artifacts, and maintenance passes.

## Mental model

| Software | Knowledge base |
|----------|----------------|
| source code | `raw/` |
| source registry | `raw/_manifest.yaml` |
| compiler | `kb-compile` |
| build artifacts | `wiki/` |
| logs and reports | `outputs/` |
| lint / maintenance | `kb-review` (`maintenance` mode) |

## Practical consequences

- humans curate sources
- the workflow registers sources before compile runs
- the LLM maintains structure and cross-links
- useful Q&A gets stored as artifacts
- durable outputs can generate writeback work that re-enters drafts and review
- the wiki can be checked for drift, contradictions, and weak structure
- creator-facing prose can be produced from approved knowledge without collapsing the truth boundary

## Compile posture

This bundle's default compile method is not "write a generic summary".

Instead, compile should follow:

`浓缩 -> 质疑 -> 对标`

- `浓缩` keeps only the core conclusions plus evidence
- `质疑` records assumptions, boundary conditions, and failure cases
- `对标` looks for cross-domain transfer value and procedure / hub candidates

## Retrieval model

The default retrieval story is markdown-first:

1. read `wiki/index.md`
2. read derived indices
3. use topic pages as the default browse layer
4. use Backlinks, unlinked mentions, and Properties search
5. follow wikilinks
6. use file search when needed

Only add heavier search infrastructure when the vault outgrows this model. The next step should usually be local structured search such as DuckDB markdown parsing and full-text search, not an immediate jump to vector infrastructure.

## Next steps

- [Directory Structure](/guide/directory-structure)
- [AGENTS.md and CLAUDE.md](/guide/agents-schema)
- [Skills Overview](/skills/overview)
