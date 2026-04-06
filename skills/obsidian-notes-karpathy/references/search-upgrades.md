# Search Upgrades

Default posture: stay local-first, markdown-first, and auditable.

Do not jump straight to RAG or a vector database just because the user mentions AI search.

## Stage 1: Native markdown navigation

Use this first for small to medium vaults.

- read `wiki/index.md`
- read `wiki/indices/INDEX.md`, `wiki/indices/CONCEPTS.md`, and `wiki/indices/SOURCES.md`
- use ordinary file search over markdown
- follow real wikilinks between summaries, concepts, entities, and prior Q&A

This stage is enough for many vaults with hundreds of high-signal notes.

## Stage 2: Obsidian-native graph and metadata affordances

Use this before introducing extra infrastructure.

- use Backlinks to inspect linked mentions and unlinked mentions
- use Properties view and property search to find notes by `type`, `tags`, `author`, or other standardized metadata
- use alias coverage on concept and entity pages to improve linkability and discoverability
- use derived indices such as `RECENT.md` and alias maps to surface drift

Recommend this stage when the problem is disconnected notes, weak links, or metadata inconsistency rather than raw search scale.

## Stage 3: Local structured search

Use this when plain markdown navigation is no longer enough but the user still wants local, transparent search.

Suggested upgrades:

1. qmd for local markdown search with BM25 or hybrid retrieval
2. DuckDB markdown parsing for frontmatter, links, images, and section-level analysis
3. DuckDB full-text search for vault-wide text retrieval
4. Dataview or Datacore for metadata-driven views inside Obsidian

Recommend this stage when:

- the vault has grown large enough that manual indices lag behind
- the user wants structured audits, metadata reports, or section-level retrieval
- full-text retrieval is needed but auditability still matters
- the user wants better local retrieval without jumping straight to a hosted vector stack

## Stage 4: Hybrid or vector retrieval

Only suggest this when earlier stages stop being sufficient.

Use this stage when:

- the vault is very large
- semantic recall is the main bottleneck
- the user explicitly wants embeddings, reranking, or RAG infrastructure

If you suggest this stage, explain why the earlier markdown-first stages are no longer enough.

## Decision rule

Prefer the cheapest stage that solves the user's problem while preserving traceability:

- navigation problem -> Stage 1 or 2
- metadata/reporting problem -> Stage 2 or 3
- search-scale problem -> Stage 3
- semantic-recall-at-scale problem -> Stage 4

If the user's problem is malformed indices, broken table rendering, or syntactically wrong markdown, do not treat it as a search-upgrade problem first. Route that work through health or repair before adding more retrieval infrastructure.
