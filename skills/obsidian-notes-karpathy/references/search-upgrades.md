# Search Upgrades

Default posture: stay local-first, markdown-first, and auditable.

Do not jump straight to RAG or a vector database just because the user mentions AI search.

For repository or vault text search, prefer `rg` (ripgrep) over `grep` because it is faster and scales better across large markdown trees. On Windows, avoid using `rg` against wildcarded absolute paths; prefer exact file paths or platform-native alternatives when needed.

## Stage 1: Native markdown navigation

Use this first for small to medium vaults.

- read `wiki/index.md`
- read `wiki/live/indices/INDEX.md`, `wiki/live/indices/CONCEPTS.md`, and `wiki/live/indices/SOURCES.md`
- check optional governance indices such as `wiki/live/indices/QUESTIONS.md`, `GAPS.md`, and `ALIASES.md` when they exist
- use ordinary file search over markdown, preferring `rg` when CLI search is needed
- follow real wikilinks between summaries, concepts, entities, prior Q&A, and approved question pages

This stage is enough for many vaults with hundreds of high-signal notes.

## Stage 2: Obsidian-native graph and metadata affordances

Use this before introducing extra infrastructure.

- use Backlinks to inspect linked mentions and unlinked mentions
- use Properties view and property search to find notes by `type`, `tags`, `author`, `aliases`, `domain_volatility`, or other standardized metadata
- use alias coverage on concept and entity pages to improve linkability and discoverability
- use derived indices such as `RECENT.md`, alias maps, and question registries to surface drift

Recommend this stage when the problem is disconnected notes, weak links, metadata inconsistency, or unresolved question clusters rather than raw search scale.

## Stage 3: Local structured search

Use this when plain markdown navigation is no longer enough but the user still wants local, transparent search.

Suggested upgrades:

1. qmd for local markdown search with BM25 or hybrid retrieval
2. DuckDB markdown parsing for frontmatter, links, images, and section-level analysis
3. DuckDB full-text search for vault-wide text retrieval
4. Dataview or Datacore for metadata-driven views inside Obsidian

Recommend this stage when:

- the vault has grown large enough that manual indices lag behind
- the user wants structured audits, metadata reports, section-level retrieval, or question-gap analysis
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

If the user's problem is malformed indices, broken table rendering, syntactically wrong markdown, provenance drift, or duplicate approved notes, do not treat it as a search-upgrade problem first. Route that work through health or repair before adding more retrieval infrastructure.
