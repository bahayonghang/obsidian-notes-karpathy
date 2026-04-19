# Archive Model

Use this reference whenever the workflow talks about archiving, reusing archived work, or cleaning archive backlog.

## Core idea

This bundle uses `archive` in two different but related senses:

1. **source retention archive**
2. **artifact archive**

Both are durable surfaces. Neither one overrides the approved truth boundary.

## Source retention archive

The source retention archive is:

- `raw/**`
- `raw/_manifest.yaml`

It means the vault keeps original evidence and its registry.

It does **not** mean:

- moving sources into `raw/09-archive/`
- deleting or rewriting source files after compile
- treating retained raw captures as query-time truth

The current contract keeps raw sources in place. The archive behavior here is retention plus registration, not physical relocation.

## Artifact archive

The artifact archive is the set of downstream durable outputs:

- `outputs/qa/**`
- `outputs/content/**`
- `outputs/episodes/**`
- `outputs/reviews/**`
- `outputs/health/**`
- `outputs/web/**`

These are durable working surfaces for reuse, review, maintenance, and export.

## Truth boundary

Archive is not the same thing as approved knowledge.

- `wiki/live/**` remains the approved truth layer.
- archived artifacts may be reused, ranked, inspected, and audited.
- archived artifacts may generate `writeback_candidates`, `followup_route`, and maintenance pressure.
- archived artifacts do not become approved truth unless the durable knowledge re-enters `draft -> review -> live`.

## Reuse boundary

Archive is valuable because it prevents repeated work.

Expected reuse order:

1. approved live pages
2. relevant live indices and briefings
3. prior archived Q&A
4. prior archived publish artifacts when they already reuse approved coverage cleanly

Archive reuse should reduce duplicated explanation, not bypass live grounding.

## Maintenance boundary

Archived artifacts are first-class maintenance inputs.

Maintenance should inspect archived artifacts for:

- stale outputs relative to newer live pages
- writeback backlog
- reuse gaps
- scope leaks such as private/shared mismatches
- creator consistency drift

Maintenance should fix archive hygiene, not silently upgrade archived artifacts into live truth.

## Physical archive note

This contract intentionally does not introduce a physical `raw/09-archive/` move step.

If a future workflow wants true source-file relocation after processing, treat that as a separate architecture change:

- it needs new directory rules
- it needs migration guidance
- it needs explicit lifecycle updates
- it should not be mixed into the current archive-surface clarification work
