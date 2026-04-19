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

## Karpathy operations vs this bundle

Karpathy's public note describes three operations:

- ingest
- query
- lint

This bundle maps them like this:

- ingest -> `kb-ingest` + `kb-compile`
- query -> `kb-query` + `kb-render` when the result is a deterministic derivative
- lint -> `kb-review` maintenance mode

The extra split is deliberate. This repository keeps source registration, draft compilation, approval, archive reuse, and maintenance separable.

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

## Archive extension

Karpathy's note emphasizes immutable raw sources and a maintained wiki. This bundle adds a more explicit archive model:

- retained sources stay in `raw/**` and `raw/_manifest.yaml`
- durable outputs accumulate under `outputs/**`
- archived outputs are reusable and maintenance-visible
- archived outputs still do not bypass `draft -> review -> live`

## Next steps

- [Directory Structure](/guide/directory-structure)
- [AGENTS.md and CLAUDE.md](/guide/agents-schema)
- [Archive Model](/architecture/archive-model)
- [Skills Overview](/skills/overview)
