# File Model

Canonical review-gated layering:

```text
raw/            -> immutable capture intake
MEMORY.md       -> collaboration memory and editorial context
wiki/drafts/    -> compiled but unapproved draft knowledge
wiki/live/      -> approved long-term brain
wiki/briefings/ -> role-specific context built from live only
outputs/        -> reviews, Q&A, health reports, and publishable derivatives
```

Treat the vault like a codebase with a promotion gate:

- `raw/` is source evidence.
- `MEMORY.md` is the coordination surface for preferences, priorities, and collaboration rules.
- `wiki/drafts/` is build output waiting for review.
- `wiki/live/` is the deployed truth layer.
- `wiki/briefings/` is generated runtime context for agents.
- `outputs/reviews/` is the decision ledger for promotion.
- `outputs/qa/` and `outputs/content/` can surface writeback candidates and unresolved questions, but they still re-enter the system through draft -> review -> live.

## Core rules

1. `raw/` is read-only from the workflow's point of view.
2. `kb-compile` writes only to `wiki/drafts/`, never directly to `wiki/live/`.
3. `kb-review` is the only skill that can promote draft knowledge into `wiki/live/`.
4. `kb-query` reads `wiki/live/`, `wiki/briefings/`, and prior `outputs/qa/`; it must not treat `raw/` or `wiki/drafts/` as retrieval truth.
5. `kb-query` must not treat `MEMORY.md` as domain knowledge truth; it is only for collaboration or editorial context.
6. `wiki/index.md` is the content-oriented landing page for the whole contract, including live, draft, question, and briefing state.
7. `wiki/log.md` is the append-only activity ledger for `ingest`, `review`, `brief`, `query`, `publish`, and `health`.
8. `outputs/qa/` stores durable research answers, not disposable chat residue.
9. `outputs/reviews/` stores reviewer decisions and scoring details.
10. Existing older vaults using `wiki/summaries/` and `wiki/concepts/` directly should be detected as `legacy-layout` and migrated before normal operation.
11. Alias alignment, source integrity, stale-page checks, and duplicate detection are part of governance, but they must respect the review gate rather than bypass it.

## Required vs optional support

The minimum support layer for `kb-init` is:

- `raw/`
- `wiki/drafts/`
- `wiki/live/`
- `wiki/briefings/`
- `wiki/index.md`
- `wiki/log.md`
- `outputs/reviews/`
- `AGENTS.md`
- `CLAUDE.md`

Downstream output surfaces such as `outputs/qa/`, `outputs/health/`, `outputs/reports/`, `outputs/slides/`, `outputs/charts/`, and `outputs/content/**` are valid parts of the full contract, but they are created on demand when later stages need them. `MEMORY.md` is recommended collaboration scaffolding rather than a blocking support-layer requirement.

Optional governance scaffolding may also be created when the user wants richer maintenance surfaces:

- `wiki/live/indices/QUESTIONS.md`
- `wiki/live/indices/GAPS.md`
- `wiki/live/indices/ALIASES.md`

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
├── MEMORY.md
└── CLAUDE.md
```

## Raw capture classes

### Human captures

Live under `raw/human/**`.

- treated as curated evidence
- may be markdown notes or PDFs under a `papers/` subtree
- should preserve source metadata only
- may carry `last_verified_at` and `possibly_outdated` as intake hints

### Agent captures

Live under `raw/agents/{role}/**`.

- treated as untrusted until reviewed
- should preserve provenance and role identity
- must never be promoted directly into the live brain

### Legacy captures

Older vaults may still use legacy-layout paths such as `raw/articles/` or `raw/papers/`.

- continue to detect them for migration
- do not silently reinterpret them as fully valid review-gated support layers

### Bootstrap root captures

Some partially bootstrapped vaults may place markdown directly under `raw/`.

- accept these files as valid compile inputs
- preserve raw immutability exactly as with nested capture classes
- surface any missing support-layer directories separately instead of rejecting the source format itself

## Draft summaries

Live under `wiki/drafts/summaries/**` and mirror raw captures.

Expected properties:

- `title`
- `source_file`
- `source_hash` or `source_mtime`
- `last_verified_at`
- `possibly_outdated`
- `compiled_at`
- `draft_id`
- `compiled_from`
- `capture_sources`
- `review_state`
- `review_score`
- `blocking_flags`
- `alias_candidates`
- `duplicate_candidates`

## Live pages

Live under `wiki/live/{summaries,concepts,entities}/`.

Expected properties:

- `title`
- `canonical_name`
- `aliases`
- `domain_volatility`
- `approved_at`
- `approved_from`
- `review_record`
- `trust_level: approved`
- `updated_at`
- `last_reviewed_at`
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
- `open_questions_touched`

## Collaboration memory

`MEMORY.md` lives at the vault root.

Expected content:

- stable preferences
- editorial priorities
- collaboration rules
- current focus areas

It should not become a shadow knowledge base full of topic conclusions or source-grounded claims.

## Review records

Live under `outputs/reviews/`.

Expected properties:

- `title`
- `decision`
- `accuracy`
- `provenance`
- `conflict_risk`
- `composability`
- `source_integrity`
- `alias_alignment`
- `duplication_risk`
- `staleness_risk`
- `reviewed_at`

## Query outputs

`outputs/qa/` remains the durable answer archive, but all cited knowledge should trace back to `wiki/live/` and the relevant approved summaries whenever possible.

## Naming and graph conventions

- use stable lowercase kebab-case paths
- keep `wiki/live/indices/` as the canonical derived navigation directory
- allow a promoted draft to keep the same basename when moved into `wiki/live/`
- keep alias-style wikilinks out of Markdown table cells
- treat `review_record` and `approved_from` as first-class provenance edges
- use aliases and canonical names to support cross-language linking without creating parallel truth pages
