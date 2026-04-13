# Automation Hooks

Use this reference when designing CLI-first automation around the review-gated bundle.

## Near-term hooks

Start with file-system and command driven hooks before deeper runtime integration.

- `on_new_source`
  - detect new or changed captures
  - refresh compile delta
  - optionally precompute draft and graph candidates
- `on_session_end`
  - crystallize substantive outputs into `outputs/episodes/**`
  - append an audit event
- `on_query_archive`
  - ensure archived Q&A/content records `source_live_pages`, `writeback_candidates`, `followup_route`, and `confidence_posture`
  - optionally generate/update a linked episode page
- `scheduled_health`
  - run health lint
  - refresh governance indices
  - refresh `outputs/health/graph-snapshot.json`

## Audit requirement

Automation should leave a machine-readable breadcrumb in `outputs/audit/operations.jsonl`:

- `timestamp`
- `action`
- `payload`

## Boundary discipline

Automation can:

- create drafts
- refresh indices
- emit episodes
- export graph snapshots
- append audit events

Automation must not:

- mutate `raw/**`
- bypass draft -> review -> live
- silently promote candidate or episodic surfaces into approved truth
