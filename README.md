# Obsidian Notes Karpathy

Review-gated, multi-agent-friendly Obsidian knowledge-base skills inspired by Karpathy-style markdown wikis.

This bundle is aimed at users who may ask for an `LLM Wiki`, a `Karpathy wiki`, an `Obsidian IDE` for notes, a `knowledge compiler`, a personal knowledge base, or a markdown-first `second brain` without knowing the internal `kb-*` command names yet.

```text
raw/            -> immutable human + agent captures
raw/_manifest.yaml -> canonical source registry
MEMORY.md       -> collaboration memory and editorial context
outputs/episodes/ -> episodic memory / crystallized work arcs
wiki/drafts/    -> compiled draft knowledge
wiki/live/      -> approved long-term brain
wiki/live/topics/ -> approved browse-layer topic maps
wiki/live/procedures/ -> approved procedural memory
wiki/briefings/ -> role-specific context generated from live
outputs/        -> reviews, Q&A, health reports, audit logs, and publishable artifacts
```

The core idea is no longer just "compile notes into a wiki". It is "separate production from judgment so unreviewed drafts never become compound retrieval truth."

## Skill set

- `obsidian-notes-karpathy` - lifecycle router
- `kb-init` - initialize, repair, or migrate a vault into the review-gated layout
- `kb-ingest` - register raw sources into `raw/_manifest.yaml`
- `kb-compile` - compile raw markdown captures into `wiki/drafts/`
- `kb-review` - approve, reject, or escalate drafts; rebuild `wiki/briefings/`
- `kb-query` - canonical read-side lane for search, grounded answers, archived Q&A reuse, and static web export from `wiki/live/`
- `kb-render` - deterministic slides, reports, charts, and canvas derivatives from approved knowledge

## Companion skill matrix

| Need | Core route | Companion route |
| --- | --- | --- |
| Fresh setup, repair, or legacy migration | `kb-init` | none |
| Register raw markdown, assets, or data into the manifest | `kb-ingest` | none |
| Compile raw markdown into drafts | `kb-compile` | none |
| Review and promote draft knowledge | `kb-review` | none |
| Query, rank approved candidates, reuse archived answers, or export a static knowledge site from approved live knowledge | `kb-query` | none |
| Render deterministic slides, reports, charts, or canvas artifacts from approved knowledge | `kb-render` | none |
| Legacy `kb-search` wording | `kb-query` | literal wording is absorbed by the canonical query skill |
| Legacy `kb-health` wording, approved-layer drift, or backlog maintenance | `kb-review` | use maintenance mode under the canonical governance skill |
| Ingest `raw/**/papers/*.pdf` | not a core route | `paper-workbench` |

The core bundle should own the review-gated lifecycle. Companion skills take over only for clearly external lanes such as paper PDFs or canvas-specific authoring.

## Contract highlights

- `raw/` is immutable.
- treat `raw/` as the durable source library; editorial notes, Q&A, and publish artifacts belong in downstream surfaces rather than mutating source captures.
- `raw/_manifest.yaml` is the canonical source registry for tracked inputs.
- `MEMORY.md` is collaboration memory, not retrieval truth.
- `outputs/episodes/` stores episodic memory, not approved topic truth.
- `kb-compile` writes only to `wiki/drafts/`.
- raw paper PDFs under `raw/**/papers/*.pdf` remain a `paper-workbench` routing exception, not a normal `kb-compile` ingest.
- `kb-review` is the only promotion gate into `wiki/live/`.
- `wiki/live/procedures/` is the procedural-memory lane for approved workflows and playbooks.
- `wiki/live/topics/` is the browse-layer entry surface over approved knowledge clusters.
- `wiki/briefings/` is generated from approved live pages only.
- `kb-query` reads `wiki/live/`, briefings, and prior Q&A, not drafts.
- `outputs/audit/operations.jsonl` is the machine-readable audit trail for automation and derived exports.
- legacy-layout vaults are detected and should be migrated before normal operation.
- alias alignment, source integrity, stale-page checks, and question tracking are absorbed as governance rules without collapsing the review gate.
- curated hubs and creator-planning surfaces may exist, but they remain navigation/maintenance layers rather than truth-boundary shortcuts.

## Why this is not a single-layer source/concept/synthesis wiki

This package deliberately does **not** treat a freshly generated `wiki/` page as durable truth.

Instead:

- raw captures are first registered into `raw/_manifest.yaml`
- raw captures become reviewable drafts
- drafts become approved live notes only after `kb-review`
- query and publish work stay grounded in approved summaries and live pages
- health and reflection outputs surface governance work, but durable changes still re-enter draft -> review -> live

This keeps provenance visible and prevents fast ingest from silently hardening into long-term truth.

## Karpathy alignment

This project implements [Karpathy's LLM Wiki pattern](https://gist.github.com/karpathy/442a6bf555914893e9891c11519de94f) with one key extension: an explicit review gate.

| Karpathy's pattern | This project | Why the extension |
| --- | --- | --- |
| Raw sources (immutable) | `raw/` + `raw/_manifest.yaml` | Added a canonical manifest for tracked intake |
| The wiki (LLM-maintained) | `wiki/drafts/` → `wiki/live/` | Split into draft and approved layers with a promotion gate |
| The schema (CLAUDE.md) | `AGENTS.md` + `CLAUDE.md` + shared `references/` | Expanded into a full contract registry |
| Ingest | `kb-ingest` + `kb-compile` | Separated source registration from draft compilation |
| Query | `kb-query` + `kb-render` | Separated grounded answers from deterministic derivatives |
| Lint | `kb-review` maintenance mode | Made the health check a first-class governance lane |

The core metaphor is preserved: "Obsidian is the IDE; the LLM is the programmer; the wiki is the codebase." The user curates sources and asks questions. The LLM handles all the bookkeeping that makes knowledge compound over time.

## Required support vs optional outputs

The required support layer is `raw/`, `wiki/drafts/`, `wiki/live/`, `wiki/briefings/`, `wiki/index.md`, `wiki/log.md`, `outputs/reviews/`, `AGENTS.md`, and `CLAUDE.md`.

Downstream output surfaces are created when the later stages need them:

- `outputs/qa/` - durable research answers from `kb-query`
- `outputs/content/` - publish artifacts from `kb-query`
- `outputs/slides/`, `outputs/reports/`, and `outputs/charts/` - deterministic derivative artifacts from `kb-render`
- `outputs/web/` - static browseable exports from `kb-query`
- `outputs/health/` - maintenance reports from `kb-review` maintenance mode
- `MEMORY.md` - recommended collaboration memory and editorial context

Optional governance indices such as `wiki/live/indices/QUESTIONS.md`, `GAPS.md`, and `ALIASES.md` may be created when the user wants richer maintenance surfaces.

Substantive answers and publish artifacts may also carry structured writeback candidates so later compile/review passes can decide whether they should feed back into the wiki.

For creator-style workflows, a practical mapping is:

- source library / clipped research -> `raw/`
- editorial memory -> `MEMORY.md`
- session crystallization -> `outputs/episodes/`
- reusable research answers or drafting notes -> `outputs/qa/`
- outward-facing publish artifacts -> `outputs/content/`
- durable approved knowledge -> `wiki/live/`
- durable approved workflows -> `wiki/live/procedures/`
- curated topic maps / planning surfaces -> `wiki/live/indices/` or approved hub pages

## Route ownership

- `kb-init` owns setup, repair, and legacy-layout migration.
- `kb-ingest` owns source registration, manifest refresh, and deferred intake surfacing.
- `kb-compile` owns raw-to-draft updates, including bootstrap `raw/*.md` captures.
- `kb-review` owns the governance lane: immediate draft promotion plus maintenance-mode drift checks, backlog pressure, provenance audits, alias drift, question/gap resurfacing, and safe mechanical fixes in approved surfaces only.
- `kb-query` reads approved live knowledge only and owns search, grounded answers, archived Q&A reuse, and static web export.
- `kb-render` owns deterministic derivatives from approved knowledge.

## Deterministic helpers

- `skills/obsidian-notes-karpathy/scripts/skill-contract-registry.json`
- `scripts/detect_lifecycle.py`
- `scripts/scan_ingest_delta.py`
- `scripts/sync_source_manifest.py`
- `scripts/scan_compile_delta.py`
- `scripts/build_draft_packages.py`
- `scripts/bootstrap_review_gated_vault.py`
- `scripts/migrate_legacy_vault.py`
- `scripts/scan_review_queue.py`
- `scripts/scan_query_scope.py`
- `scripts/rank_query_candidates.py`
- `scripts/render_live_artifact.py`
- `scripts/vault_status.py`
- `scripts/render_reference_block.py`
- `scripts/lint_obsidian_mechanics.py`
- `scripts/build_memory_episodes.py`
- `scripts/build_graph_snapshot.py`
- `scripts/runtime_eval.py`
- `scripts/trigger_eval.py`

## Install

```bash
cp -r skills/* ~/.claude/skills/
cp -r skills/* ~/.codex/skills/
```
