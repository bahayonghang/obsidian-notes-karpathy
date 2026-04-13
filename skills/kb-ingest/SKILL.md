---
name: kb-ingest
description: Register and normalize raw source intake for an Obsidian knowledge base. Use this skill whenever the user says "kb ingest", "ingest sources", "refresh manifest", "register new raw files", "scan raw library", "同步素材清单", "登记新来源", "更新 manifest", or wants `raw/**` captures recorded into `raw/_manifest.yaml` before compile runs. Do not use it for draft generation, review, query, or maintenance once the source registry is already current.
---

# KB Ingest

Track raw sources in a canonical manifest before compile shapes draft knowledge.

## Minimal loop

1. discover raw sources
2. compare them against `raw/_manifest.yaml`
3. register new or changed entries
4. mark deferred sources such as paper PDFs explicitly instead of hiding them

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

Treat `skill-contract-registry.json` as the canonical source for required references, baseline script, and write surfaces.

If available, run:

- `../obsidian-notes-karpathy/scripts/scan_ingest_delta.py`
- `../obsidian-notes-karpathy/scripts/sync_source_manifest.py` when the task is writable

## Rules

- never rewrite `raw/**` content
- always record paper PDFs in the manifest even when compile must defer them
- keep image and data assets visible at the manifest layer even if downstream compile remains metadata-first
- preserve `first_seen_at` when an entry already exists
- use `raw/_manifest.yaml` as the user-visible canonical registry

## Output to the user

Report:

1. whether the manifest was missing, current, or stale
2. how many sources were new, changed, unchanged, or removed
3. which sources were deferred and why
4. whether the next step is `kb-compile`
