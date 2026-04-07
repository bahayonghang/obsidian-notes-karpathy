# File Model

Canonical V2 layering:

```text
raw/            -> immutable capture intake
wiki/drafts/    -> compiled but unapproved draft knowledge
wiki/live/      -> approved long-term brain
wiki/briefings/ -> role-specific context built from live only
outputs/        -> reviews, Q&A, health reports, and publishable derivatives
```

Treat the vault like a codebase with a promotion gate:

- `raw/` is source evidence.
- `wiki/drafts/` is build output waiting for review.
- `wiki/live/` is the deployed truth layer.
- `wiki/briefings/` is generated runtime context for agents.
- `outputs/reviews/` is the decision ledger for promotion.

## Core rules

1. `raw/` is read-only from the workflow's point of view.
2. `kb-compile` writes only to `wiki/drafts/`, never directly to `wiki/live/`.
3. `kb-review` is the only skill that can promote draft knowledge into `wiki/live/`.
4. `kb-query` reads `wiki/live/`, `wiki/briefings/`, and prior `outputs/qa/`; it must not treat `raw/` or `wiki/drafts/` as retrieval truth.
5. `wiki/index.md` is the content-oriented landing page for the whole contract, including live, draft, and briefing state.
6. `wiki/log.md` is the append-only activity ledger for `ingest`, `review`, `brief`, `query`, `publish`, and `health`.
7. `outputs/qa/` stores durable research answers, not disposable chat residue.
8. `outputs/reviews/` stores reviewer decisions and scoring details.
9. Existing V1 vaults using `wiki/summaries/` and `wiki/concepts/` directly should be detected as `legacy-v1` and migrated before normal V2 operation.

## Expected directories

```text
vault/
├── raw/
│   ├── human/
│   │   ├── articles/
│   │   ├── papers/
│   │   ├── podcasts/
│   │   ├── repos/
│   │   └── assets/
│   └── agents/
│       └── {role}/
├── wiki/
│   ├── drafts/
│   │   ├── summaries/
│   │   ├── concepts/
│   │   ├── entities/
│   │   └── indices/
│   ├── live/
│   │   ├── summaries/
│   │   ├── concepts/
│   │   ├── entities/
│   │   └── indices/
│   ├── briefings/
│   ├── index.md
│   └── log.md
├── outputs/
│   ├── reviews/
│   ├── qa/
│   ├── health/
│   ├── reports/
│   ├── slides/
│   ├── charts/
│   └── content/
│       ├── articles/
│       ├── threads/
│       └── talks/
├── AGENTS.md
└── CLAUDE.md
```

## Raw capture classes

### Human captures

Live under `raw/human/**`.

- treated as curated evidence
- may be markdown notes or PDFs under a `papers/` subtree
- should preserve source metadata only

### Agent captures

Live under `raw/agents/{role}/**`.

- treated as untrusted until reviewed
- should preserve provenance and role identity
- must never be promoted directly into the live brain

### Legacy captures

Older vaults may still use V1 paths such as `raw/articles/` or `raw/papers/`.

- continue to detect them for migration
- do not silently reinterpret them as fully valid V2 support layers

## Draft summaries

Live under `wiki/drafts/summaries/**` and mirror raw captures.

Expected properties:

- `title`
- `source_file`
- `source_hash` or `source_mtime`
- `compiled_at`
- `draft_id`
- `compiled_from`
- `capture_sources`
- `review_state`
- `review_score`
- `blocking_flags`

## Live pages

Live under `wiki/live/{summaries,concepts,entities}/`.

Expected properties:

- `title`
- `approved_at`
- `approved_from`
- `review_record`
- `trust_level: approved`
- `updated_at`
- `sources`
- `related`

## Briefings

Live under `wiki/briefings/`.

Expected properties:

- `title`
- `brief_for`
- `built_from`
- `updated_at`
- `staleness_after`
- `source_live_pages`

## Review records

Live under `outputs/reviews/`.

Expected properties:

- `title`
- `decision`
- `accuracy`
- `provenance`
- `conflict_risk`
- `composability`
- `reviewed_at`

## Query outputs

`outputs/qa/` remains the durable answer archive, but all cited knowledge should trace back to `wiki/live/`.

## Naming and graph conventions

- use stable lowercase kebab-case paths
- keep `wiki/live/indices/` as the canonical derived navigation directory
- allow a promoted draft to keep the same basename when moved into `wiki/live/`
- keep alias-style wikilinks out of Markdown table cells
- treat `review_record` and `approved_from` as first-class provenance edges
