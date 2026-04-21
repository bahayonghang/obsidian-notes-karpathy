# Draft Schema

Use this reference as the single-source schema for `wiki/drafts/**` pages. `kb-compile` writes drafts that match this schema; `kb-review` reads and gates them.

Type-specific templates (`summary-template.md`, `concept-template.md`, `entity-template.md`, `topic-template.md`, `procedure-template.md`) layer their own fields on top of this baseline.

## Required fields

Every draft, regardless of type, must include:

| Field | Purpose |
| --- | --- |
| `draft_id` | Stable identifier for cross-referencing between compile, review, and log entries. |
| `compiled_from` | Relative path of the raw capture(s) that produced this draft. |
| `capture_sources` | Full list of source paths when one draft synthesizes multiple captures. |
| `review_state` | `pending` on write; `kb-review` mutates to `approved` or `rejected`. |
| `review_score` | Compile-side heuristic score used to band the draft for the gate. |
| `blocking_flags` | Non-empty means the gate cannot auto-approve. |
| `evidence_coverage` | Measure of how much of the claim space is grounded in the cited captures. |
| `uncertainty_level` | Compile's confidence posture before review reconciles it. |
| `promotion_target` | `semantic` (into `wiki/live/concepts/`) or `procedural` (into `wiki/live/procedures/`). |
| `review_package_meta` | Pointer to the deterministic source package that generated the draft. |

## Conditional fields

Include these only when the described signal is present; omit otherwise so review gets a clean surface.

| Field | Include when |
| --- | --- |
| `alias_candidates` | Terminology overlap with existing approved vocabulary is visible. |
| `duplicate_candidates` | A draft or live page may already cover the same concept. |
| `boundary_conditions` | Conclusion depends on market, scale, geography, recency, or other scope limit. |
| `assumption_flags` | Claim rests on unstated premises the reviewer should inspect. |
| `transfer_targets` | Cross-domain analogy, migration value, or hub candidate discovered during compile. |
| `candidate_entities` | Reusable entity structure appeared alongside the draft. |
| `candidate_relationships` | Reusable relationship edges appeared alongside the draft. |
| `topic_candidates` | Stable browse-layer clustering signal emerged. |
| `confidence_inputs` | Draft is strong enough that a future confidence score is justified; review decides whether to promote to `confidence_score`. |

## Provenance and timestamps

Every draft that touches `compiled_at`, `last_verified_at`, `last_source_check_at`, or `last_confirmed_at` must use ISO-8601 UTC with second precision, e.g. `2026-04-20T11:32:00Z`. Avoid local time zones in draft frontmatter.

## Type-specific layering

On top of this schema, each draft type adds its own fields:

- `summary-template.md` — adds `title`, `source_file`, `source_hash` or `source_mtime`, `possibly_outdated`.
- `concept-template.md` — adds `canonical_name`, `aliases`, `domain_volatility` candidates.
- `entity-template.md` — adds entity-specific identity and role fields.
- `topic-template.md` — adds `topic_hub` candidates and hub-level aggregation fields.
- `procedure-template.md` — adds procedure-specific fields (`procedure_id`, `confidence_band`, `decay_class`, `next_review_due_at`).

When a field appears here and also in a type template, this reference is authoritative for the naming and conditional rules; the type template only specifies its incremental fields.

## Write-time checklist

Before handing a draft to `kb-review`, confirm:

- every required field is present with a non-placeholder value
- each conditional field is either populated with real signal or omitted
- `compiled_from` and `capture_sources` actually exist under `raw/**`
- `promotion_target` matches the content shape (workflow → procedural, definition/concept → semantic)
- `blocking_flags` is an empty list if and only if the draft is safe for auto-approval
