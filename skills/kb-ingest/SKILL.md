---
name: kb-ingest
description: Register and normalize raw source intake for an Obsidian knowledge base. Use this skill whenever the user says "kb ingest", "ingest sources", "refresh manifest", "register new raw files", "scan raw library", "同步素材清单", "登记新来源", "更新 manifest", or wants `raw/**` captures recorded into `raw/_manifest.yaml` before compile runs. Treat browser-side collection as an upstream companion lane: if the source is still on the web and not in `raw/` yet, use `web-access` or Web Clipper first, then come here. Do not use it for draft generation, review, query, or maintenance once the source registry is already current.
---

# KB Ingest

Track raw sources in a canonical manifest before compile shapes draft knowledge.

In Karpathy's LLM Wiki pattern, ingest is the moment raw evidence becomes visible to the system. A well-maintained manifest means compile can trust what it sees, deferred sources stay explicit instead of silently lost, and the user always knows what the knowledge base has ingested.

## Minimal loop

1. scan `raw/` for all source files
2. run `onkb --json ingest scan <vault-root>` to compare against `raw/_manifest.yaml`
3. present the delta to the user for confirmation
4. register new or changed entries via `onkb --json ingest sync <vault-root> --write`
5. mark deferred sources explicitly with reason
6. append an `ingest` entry to `wiki/log.md`

## When this compounds the wiki

Good ingest work makes the source library navigable and trustworthy. Every source should be visible in the manifest with enough metadata that compile, review, and query can trace provenance back to the original capture.

## Read before ingesting

- local `AGENTS.md`
- local `CLAUDE.md` if present
- `../obsidian-notes-karpathy/scripts/skill-contract-registry.json`
- `../obsidian-notes-karpathy/references/file-model.md`
- `../obsidian-notes-karpathy/references/lifecycle-matrix.md`
- `../obsidian-notes-karpathy/references/activity-log-template.md`
- `../obsidian-notes-karpathy/references/paper-ingestion-lifecycle.md`
- `../obsidian-notes-karpathy/references/source-manifest-contract.md`
- `../obsidian-notes-karpathy/references/profile-contract.md`

Treat `skill-contract-registry.json` as the canonical source for required references, baseline command, and write surfaces.

If `onkb` is available, run:

- `onkb --json ingest scan <vault-root>` to detect the delta
- `onkb --json ingest sync <vault-root> --write` when the task is writable

## Source discovery

Scan these paths for raw sources:

- `raw/human/articles/**` — web clips and article captures
- `raw/human/papers/**` — literature PDFs (deferred to `paper-workbench`)
- `raw/human/podcasts/**` — podcast and audio transcripts
- `raw/human/repos/**` — repository snapshots or code captures
- `raw/human/assets/**` — images and visual evidence
- `raw/human/data/**` — structured data files
- `raw/agents/{role}/**` — agent-generated captures
- `raw/*.md` — bootstrap root captures in early-stage vaults

## Manifest entry format

Each entry in `raw/_manifest.yaml` must include:

```yaml
- source_id: "{unique-slug}"
  path: "raw/human/articles/example-article.md"
  source_type: article | paper | podcast | repo | asset | data | agent-capture
  capture_origin: human | agent:{role}
  source_url_or_handle: "https://example.com/article"
  content_hash: "{sha256 or mtime}"
  first_seen_at: "2026-04-14"
  last_seen_at: "2026-04-14"
  ingest_status: ready-for-compile | deferred | deferred-missing-skill
  normalized_outputs: []
```

Optional fields:

- `deferred_to: paper-workbench` — for paper PDFs awaiting external processing
- `metadata_path: raw/human/papers/example.meta.yaml` — when metadata lives alongside the source
- `capture_method: web-clipper | browser-cdp | manual-markdown | agent-capture | file-drop`
- `linked_assets:` — local images or attachments tied to the markdown source
- `source_profile:` — creator/account/profile context used later for editorial consistency checks

## Non-negotiable rules

- never rewrite `raw/**` content
- always record paper PDFs in the manifest even when compile must defer them
- keep image and data assets visible at the manifest layer even if downstream compile remains metadata-first
- keep browser-collected or Web Clipper-collected intake distinguishable through `capture_method` when the metadata is available
- preserve attached local images or files through `linked_assets` instead of forcing compile to guess from the body
- preserve `first_seen_at` when an entry already exists
- use `raw/_manifest.yaml` as the user-visible canonical registry
- if the manifest file does not exist, create it with the entries discovered

## Checkpoint

Before writing the manifest update, present the delta summary:

```
New sources:      N (list paths)
Changed sources:  N (list paths with what changed)
Deferred sources: N (list paths with reason)
Removed sources:  N (list paths — only mark removed, never delete raw files)
```

Proceed only after the user confirms, or if all sources are straightforward new markdown captures with no ambiguity.

## Edge cases

- **Missing manifest**: create `raw/_manifest.yaml` from scratch by scanning all of `raw/`
- **Corrupted manifest**: warn the user, rebuild from the file system, preserve any `first_seen_at` dates that can be recovered
- **Mixed source types in one directory**: register each file individually with the correct `source_type`
- **Paper PDFs**: always register with `ingest_status: deferred` and `deferred_to: paper-workbench`; never skip them silently
- **Duplicate paths**: warn and deduplicate by keeping the entry with the earlier `first_seen_at`

## Output to the user

Report:

1. whether the manifest was missing, current, or stale
2. how many sources were new, changed, unchanged, or removed
3. which sources were deferred and why
4. whether the next step is `kb-compile`
5. any anomalies discovered during scanning (missing files, duplicates, unexpected formats)
