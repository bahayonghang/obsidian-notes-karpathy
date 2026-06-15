# File Persistence And Manifest Guidelines

This project has no ORM, SQL database, migration framework, or server-side
database layer. Persistent state is the vault file tree plus structured
Markdown, YAML-like manifests, JSON payloads, and JSONL audit logs.

## Persistence Surfaces

Primary persisted surfaces:

- `raw/`: immutable evidence intake.
- `raw/_manifest.yaml`: source registry written by ingest logic.
- `wiki/drafts/`: reviewable draft knowledge, not query truth.
- `wiki/live/`: approved long-term knowledge and default query truth.
- `wiki/briefings/`: generated from approved live knowledge.
- `outputs/reviews/`: promotion decision ledger.
- `outputs/audit/operations.jsonl`: append-only machine-readable audit events.
- `outputs/qa/`, `outputs/content/`, `outputs/episodes/`, `outputs/web/`:
  downstream or episodic outputs that must not silently become approved truth.

Reference docs under `skills/obsidian-notes-karpathy/references/` define these
contracts. Check `file-model.md`, `source-manifest-contract.md`,
`lifecycle-matrix.md`, `query-writeback-lifecycle.md`, and
`archive-model.md` before changing persisted shapes.

## Read Patterns

- Parse Markdown through `src/kb/markdown.rs` helpers such as
  `load_markdown`, `parse_frontmatter`, `list_field`, and
  `iter_markdown_records`.
- Preserve path portability with `relative_posix`, `collapse_posix`, and
  `normalize_path_string`.
- Build lookup indexes with helpers such as `registry_for_records` and
  `resolve_target` instead of local string-only link matching.
- Keep output deterministic: sort paths, records, issue lists, and manifest
  entries before serializing when order is externally visible.

Examples:

- `src/kb/ingest/legacy_impl.rs` loads `raw/_manifest.yaml`, compares current
  raw sources, sorts entries by path, and writes a complete manifest.
- `src/kb/health/engine.rs` sorts audit issues by `kind`, `path`, and `line`
  before returning the health payload.
- `src/kb/markdown.rs` normalizes wiki links and path lookups across basename
  and relative-link forms.

## Write Patterns

- Prefer explicit write flags. Many commands scan by default and write only
  when passed `--write`, for example ingest sync, compile build, review
  governance, review graph, review automation, and render.
- Create parent directories before writing files.
- Use `write_markdown` for Markdown outputs that should end with one trailing
  newline and include path-specific error context.
- Use `src/support/audit_log.rs::append_event` for audit JSONL. It opens the
  file in append mode instead of reading and rewriting the whole log.
- Preserve existing user files unless the command explicitly supports
  overwrite. `src/kb/init/legacy_impl.rs::write_asset_file` returns
  `preserved` when a scaffold target already exists and `overwrite` is false.

## Manifest Conventions

`raw/_manifest.yaml` is a constrained YAML-like file, not a general YAML API.
Keep changes compatible with the parser and writer in
`src/kb/ingest/legacy_impl.rs`:

- Maintain `MANIFEST_VERSION`.
- Keep required fields in `MANIFEST_FIELDS`.
- Treat optional scalar/list fields explicitly.
- Preserve stable source ids derived from normalized relative paths.
- Keep `first_seen_at` stable across syncs and update `last_seen_at` when a
  source is re-evaluated.

If a manifest field changes, update:

- `skills/obsidian-notes-karpathy/references/source-manifest-contract.md`
- fixture vaults under `evals/skills/obsidian-notes-karpathy/fixtures/`
- JSON snapshots in `tests/snapshots/`
- integration tests such as `tests/payload_compat.rs` and
  `tests/query_health.rs`

## Migration Conventions

Vault migrations are conservative and file-based:

- Scaffold the review-gated support layer first.
- Copy legacy files into review-gated locations without deleting originals.
- Put migration backups and reports under `outputs/reviews/`.
- Skip target files that already exist and report the skip reason.

`src/kb/init/legacy_impl.rs::migrate_legacy_vault` is the current model.

## Common Mistakes

- Treating `MEMORY.md`, `outputs/episodes/`, `wiki/drafts/`, or `raw/` as
  approved query truth.
- Rewriting JSONL audit logs by reading the full file and writing it back.
- Using OS-native backslashes in payload fields that are compared by snapshots.
- Updating a persisted field without updating the contract references,
  fixtures, and payload snapshots.
- Adding a database abstraction when a deterministic file contract is the real
  persistence model.
