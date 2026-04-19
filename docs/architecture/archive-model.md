# Archive Model

This bundle uses the word `archive` in two different ways.

## Two archive surfaces

1. **source retention archive**
2. **artifact archive**

They are both durable. They are not the same as approved truth.

## Source retention archive

The source retention archive is:

- `raw/**`
- `raw/_manifest.yaml`

This means the vault retains original captures and keeps them registered.

It does **not** mean:

- moving sources into `raw/09-archive/`
- deleting sources after compile
- treating raw captures as query-time truth

This repository intentionally keeps raw files in place. Archive here means retention, not relocation.

## Artifact archive

The artifact archive is the downstream output layer:

- `outputs/qa/**`
- `outputs/content/**`
- `outputs/episodes/**`
- `outputs/reviews/**`
- `outputs/health/**`
- `outputs/web/**`

These outputs are durable because the workflow wants reuse, maintenance, and explicit follow-up instead of disposable chat residue.

## Truth, reuse, and maintenance boundaries

| Surface | Primary role | Truth status | Reuse status | Maintenance status |
| --- | --- | --- | --- | --- |
| `raw/**` + `raw/_manifest.yaml` | retained evidence | not approved truth | reusable as evidence | can trigger ingest / maintenance |
| `wiki/live/**` | approved knowledge | approved truth | primary reuse surface | can trigger review / maintenance |
| `outputs/**` | artifact archive | not approved truth | reusable working surface | can trigger archive hygiene and writeback |

The rule is simple:

- `wiki/live/**` is still the only approved truth layer.
- archived outputs can be reused.
- archived outputs can surface `writeback_candidates`.
- archived outputs can create backlog and drift signals.
- archived outputs do not auto-promote into truth.

## Reuse order

When a user asks for a new answer or publish artifact, the safe reuse order is:

1. approved live pages
2. live indices and briefings
3. prior archived Q&A
4. prior archived publish artifacts when they already reuse approved coverage cleanly

This keeps archive useful without letting polished artifacts outrank grounded live pages.

## Why the repo does not use `raw/09-archive/`

Karpathy's public note talks about immutable raw sources and a maintained wiki. It does not require physical relocation of processed sources.

This bundle therefore keeps:

- raw retention stable
- source registration explicit
- downstream outputs clearly separated

If the project ever wants physical raw-file archiving, that should be treated as a separate architecture change with its own migration and lifecycle rules.
