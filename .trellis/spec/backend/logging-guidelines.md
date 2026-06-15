# Logging Guidelines

There is no general logging framework in the runtime CLI. The project relies on
structured command payloads, deterministic summaries, contract-test output, and
the vault audit JSONL surface.

## Runtime Output

- Do not introduce ad-hoc `println!` calls in domain modules.
- Return structured payloads from `src/kb/**`, `src/dev/**`, and
  `src/support/**`; render them through `src/cli/output.rs`.
- Keep JSON output stable for tests and downstream automation.
- Human-readable command output should come from `summary` or `content` fields
  when those are part of the payload contract.

## Audit Log

Use `src/support/audit_log.rs::append_event` for machine-readable operational
events written to `outputs/audit/operations.jsonl`.

Event shape:

```json
{"timestamp":"...Z","action":"<action>","payload":{}}
```

Rules:

- Create `outputs/audit/` if missing.
- Append one JSON object per line.
- Keep actions stable and specific.
- Store normalized relative paths where possible.
- Do not put raw source text, secrets, or private conversation content in audit
  payloads.

## Test And Developer Diagnostics

- Contract diagnostics should be JSON payload fields, not logs. Examples:
  `src/dev/contract.rs::validate_bundle`,
  `src/dev/skill_audit.rs`, and runtime/trigger eval helpers.
- Health diagnostics should use stable issue kinds. Examples include
  `stale_qa`, `memory_knowledge_mix`, `unapproved_live_page`, and
  `audit_trail_gap` in `src/kb/health/engine.rs`.
- Test helpers should parse stdout as JSON via `tests/common/mod.rs::run_json`
  when asserting command behavior.

## What To Log Or Surface

Surface:

- command status and next route (`status`, `route`, `signals`)
- written paths (`written_manifest`, `output_path`, migration reports)
- counts and issue kinds
- contract validation errors
- eval planning/execution metadata

Avoid:

- absolute local machine paths unless they are explicitly redacted or needed for
  a developer diagnostic
- raw article/paper/source body text
- API keys, environment variables, tokens, or account-specific private data
- noisy progress logs that make JSON CLI output invalid

## Common Mistakes

- Adding debug printing to a command that integration tests parse as JSON.
- Treating logs as the only record of a mutation. Mutating commands should
  return the written path or append an audit event when the workflow contract
  expects one.
- Logging raw Markdown content instead of a relative path and issue kind.
- Adding a logging dependency before proving the existing structured payload
  model is insufficient.
