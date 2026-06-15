# Error Handling

The Rust CLI uses `anyhow::Result` at command and workflow boundaries, plus
typed serde payloads for successful machine-readable output. Errors should
carry enough path or contract context for a developer to reproduce the issue
without changing the stable success payload shape.

## Error Types

- Public command functions generally return `anyhow::Result<serde_json::Value>`
  or `anyhow::Result<T>`.
- Stable success payloads live in `src/payload/mod.rs` and are serialized with
  `serde`.
- CLI output formatting lives in `src/cli/output.rs`; keep error handling out
  of display-only code unless the display conversion itself can fail.
- Developer-facing validators should collect multiple contract errors and
  return a JSON payload with `status` and `errors` instead of failing fast when
  that gives better review feedback. `src/dev/contract.rs::validate_bundle`
  is the model.

## Propagation Patterns

- Use `?` to propagate IO, JSON, command, and parser failures.
- Add path context around filesystem reads/writes where the failing path matters:
  `load_markdown` uses `with_context(|| format!("read markdown {}", path.display()))`.
- Use explicit `anyhow!` errors for missing internal assets or impossible
  contract states, for example missing embedded `kb-init` assets.
- Return empty/default payloads only when absence is a valid state. For example,
  `load_source_manifest` returns a manifest-shaped value when
  `raw/_manifest.yaml` is absent.

## CLI Response Pattern

Commands should build a payload first and let `CommandOutcome` decide how it is
rendered:

- `--json` prints pretty JSON.
- `doctor` and `version` use their dedicated render modes.
- Default output prints `summary`, then `content`, then JSON.

Avoid command-specific printing inside domain modules. Domain modules should
return structured values so tests can assert exact fields.

## Validation And Audit Patterns

- For contract validation, collect all detected errors in a vector and return
  `{"status":"error","errors":[...]}`. This makes `just lint` actionable.
- For health review, return structured issue objects with stable `kind` and
  `path` fields.
- For route/status decisions, return machine-readable `state`, `route`,
  `signals`, and `counts`.
- For audit events, append a valid JSON object per line with timestamp, action,
  and payload through the shared audit-log helper.

## Common Mistakes

- Swallowing `walkdir` or file IO errors in paths where the command should
  fail loudly. Only skip bad walk entries when the current code already treats
  them as non-blocking discovery noise.
- Returning a human sentence where tests or downstream tools expect JSON.
- Dropping path context from filesystem failures.
- Panicking in runtime command paths. `expect` is acceptable in tests and for
  static regex initialization, not for ordinary user-vault input.
- Converting missing support files into successful "ready" states. Missing
  support belongs in status signals such as `needs-setup`, `needs-repair`, or
  `needs-migration`.
