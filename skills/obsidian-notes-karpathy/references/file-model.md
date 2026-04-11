# File Model

## Table of Contents

- [Core rules](#core-rules)
- [Creator workflow mapping](#creator-workflow-mapping)
- [Required vs optional support](#required-vs-optional-support)
- [Expected directories](#expected-directories)
- [Raw capture classes](#raw-capture-classes)
- [Draft summaries](#draft-summaries)
- [Live pages](#live-pages)
- [Briefings](#briefings)
- [Collaboration memory](#collaboration-memory)
- [Review records](#review-records)
- [Query outputs](#query-outputs)
- [Naming and graph conventions](#naming-and-graph-conventions)

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

- `raw/` is source evidence and the durable source library.
- `MEMORY.md` is the coordination surface for preferences, priorities, and collaboration rules.
- `wiki/drafts/` is build output waiting for review.
- `wiki/live/` is the deployed truth layer.
- `wiki/briefings/` is generated runtime context for agents.
- `outputs/reviews/` is the decision ledger for promotion.
- `outputs/qa/` and `outputs/content/` can surface writeback candidates, unresolved questions, and creator-facing derivatives, but they still re-enter the system through draft -> review -> live.

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

## Creator workflow mapping

For creator-style vaults, map common working surfaces onto the contract like this:

- source library / clipped research -> `raw/`
- editorial memory / collaboration preferences -> `MEMORY.md`
- temporary research answers or drafting notes worth preserving -> `outputs/qa/`
- publish-ready outward artifacts -> `outputs/content/`
- reusable approved concepts, entities, and summaries -> `wiki/live/`
- curated topic maps, hubs, or editorial navigation surfaces -> `wiki/live/indices/` or approved hub-style live pages when review says they are durable enough
- actionable writeback backlog and editorial triage surfaces -> governance indices, health reports, or a maintained backlog note derived from archived outputs rather than hidden inside one-off artifacts

The key boundary is that creator convenience must not widen the truth boundary. Reusable planning or publish surfaces can exist, but durable topic knowledge is still approved only through `draft -> review -> live`.

A compounding wiki should therefore keep both:

- a truth layer in `wiki/live/`
- a visible follow-up layer where archived outputs, open questions, and maintenance signals can be triaged into the next draft, review, or health pass

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

Optional governance scaffolding should be treated as the recommended default for mature vaults that want recurring maintenance surfaces:

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
│   │   ├── overviews/
│   │   ├── comparisons/
│   │   └── indices/
│   ├── live/
│   │   ├── summaries/
│   │   ├── concepts/
│   │   ├── entities/
│   │   ├── overviews/
│   │   ├── comparisons/
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

## TODO: Image and data raw source expansion

Future intake support should expand beyond markdown and PDF while preserving the same review-gated contract.

Planned next steps:

1. **Image captures under `raw/**/assets/`**
   - add deterministic metadata-sidecar handling for screenshots, diagrams, and clipped images
   - distinguish binary assets from image-derived markdown summaries
   - keep raw binaries immutable and route extracted insights through draft -> review -> live

2. **Structured data captures under `raw/**/data/`**
   - support CSV / JSON / tabular datasets as immutable evidence inputs
   - define a metadata contract for schema summary, source hash, and last verification timestamps
   - compile only derived markdown interpretations, never mutate the original dataset

3. **Routing and lifecycle updates**
   - extend `accepted_raw_sources()` and compile-delta scanning to classify image/data sources explicitly
   - add fixture vaults for image intake and data intake edge cases
   - document when a companion skill is required versus when the core bundle can ingest directly

4. **Query and provenance expectations**
   - ensure query outputs cite the derived approved markdown page, not the binary/data asset directly as truth
   - preserve traceability back to the underlying asset or dataset through metadata and review records

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

Live under `wiki/live/{summaries,concepts,entities,overviews,comparisons}/`.

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
- `last_source_check_at` when freshness needs to be audited explicitly
- `sources`
- `related`
- optional `relationship_notes` when the vault wants lightweight semantics such as `supports`, `contrasts`, `extends`, `supersedes`, or `related-question`
- optional `evidence_strength` or `source_density` when the vault tracks thin-support risk
- optional `question_links` or `open_questions` when the page participates in a standing governance thread
- optional `topic_hub` when the page belongs to a curated hub / MOC-like surface

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
- optional `brief_scope` or `brief_focus` when the same role has multiple stable briefing lenses

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

Expected operational fields for substantive Q&A and publish outputs:

- `source_live_pages` when specific approved pages grounded the output
- `open_questions_touched` when the output materially advances standing questions
- `writeback_candidates` when the output discovers durable follow-up worth re-entering the wiki
- `writeback_status` to show whether that follow-up is still pending
- `followup_route` as `none | draft | review | health`
- optional `confidence_posture` when the answer should advertise uncertainty explicitly
- optional `compounding_value` when the artifact should advertise expected long-term reuse value

These outputs can inform governance and maintenance surfaces, but they never become approved truth automatically.

Prefer concrete operational values over placeholders. For example, `writeback_candidates` should describe the exact durable delta proposed, and `compounding_value` should reflect whether the artifact meaningfully improves future reuse, navigation, or synthesis.

## Naming and graph conventions

- use stable lowercase kebab-case paths
- keep `wiki/live/indices/` as the canonical derived navigation directory
- allow a promoted draft to keep the same basename when moved into `wiki/live/`
- keep alias-style wikilinks out of Markdown table cells
- treat `review_record` and `approved_from` as first-class provenance edges
- use aliases and canonical names to support cross-language linking without creating parallel truth pages
