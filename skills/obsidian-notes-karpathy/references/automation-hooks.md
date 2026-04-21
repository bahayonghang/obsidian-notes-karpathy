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

## Invocation

The CLI harness for these hooks is `skills/obsidian-notes-karpathy/scripts/run_automation_hooks.py`. It dispatches on `--mode` and calls the underlying helpers (`scan_compile_delta`, `scan_ingest_delta`, `build_governance_indices`, `build_graph_snapshot`, `build_memory_episodes`) and always appends a matching audit event.

Usage pattern:

```
python run_automation_hooks.py --vault-root <vault> --mode <hook> [--write]
```

Supported `--mode` values mirror the hook names above: `on_new_source`, `on_session_end`, `on_query_archive`, `scheduled-health`. Without `--write`, the script runs as a dry report.

Treat this script as the CI/automation entrypoint. Human-driven lifecycle work still goes through the corresponding `kb-*` skill rather than this harness.

## Append responsibility

Who may append to `outputs/audit/operations.jsonl`:

| Skill | Direct append | Via `run_automation_hooks.py` |
| --- | --- | --- |
| `kb-init` | scaffolds the file on fresh setup | n/a |
| `kb-ingest` | no | `on_new_source` |
| `kb-compile` | no | `on_new_source` |
| `kb-review` | gate and maintenance events | `scheduled-health` |
| `kb-query` | no | `on_query_archive`, `on_session_end` |
| `kb-render` | no | `on_session_end` |

Every appended line must carry `timestamp` (ISO-8601 UTC, e.g. `2026-04-20T11:32:00Z`), `action` (hook or skill event name), and a `payload` object describing the effect. Skills that do not append directly rely on the automation harness to attribute events. `operations.jsonl` is append-only; compaction or rotation belongs to maintenance work, not to the emitting skill.
