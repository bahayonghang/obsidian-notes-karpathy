---
name: kb-compile
description: 'Incrementally compile tracked raw captures into reviewable draft knowledge. Use this skill whenever the user says "compile wiki", "compile kb", "sync drafts", "digest these captures", "turn my clips into drafts", "编译wiki", "更新草稿层", "同步草稿", or wants tracked material under `raw/human/**`, `raw/agents/{role}/**`, bootstrap `raw/*.md`, `raw/**/assets/*`, or `raw/**/data/*` turned into reviewable summaries, topics, concepts, entities, and draft indices. This is also the right skill when Chinese-LLM-Wiki wording such as `来源页`, `主题页`, `实体页`, `综合页草稿`, `整理成来源页`, `补主题页`, or `补实体页草稿` clearly means “build draft pages from raw evidence first.” Do not treat `raw/**/papers/*.pdf` as a normal compile trigger: those paper ingests still belong to `paper-workbench`, and `kb-compile` should only surface or defer them.'
---

# KB Compile

Incrementally turn immutable raw captures into reviewable draft knowledge.

In Karpathy's pattern, this is where "the LLM reads it, extracts the key information, and integrates it into the existing wiki — updating entity pages, revising topic summaries, noting where new data contradicts old claims." A single source might touch 10-15 wiki pages. In this contract, compile shapes candidates for the review gate rather than writing directly to the live brain.

## Minimal loop

1. detect new or changed captures
2. compile them into reviewable summaries, topics, concepts, entities, relationships, and indices
3. apply the default `浓缩 -> 质疑 -> 对标` compile method
4. surface conflicts, alias overlap, duplicate risk, and hub candidates
5. hand the package to `kb-review`

## When this compounds the wiki

`kb-compile` is where raw evidence first becomes reusable structure. Good compile work should reduce future rediscovery by turning captures into durable draft packages and by surfacing where the graph needs better links, syntheses, or hubs.

## When not to promote

Compile shapes candidates. It does not approve truth and it should not bypass review just because a draft looks strong.

## Read before compiling

Read these files first:

- local `AGENTS.md`
- local `CLAUDE.md` if present
- `../obsidian-notes-karpathy/scripts/skill-contract-registry.json`
- `../obsidian-notes-karpathy/references/file-model.md`
- `../obsidian-notes-karpathy/references/compile-method.md`
- `../obsidian-notes-karpathy/references/lifecycle-matrix.md`
- `../obsidian-notes-karpathy/references/schema-template.md`
- `../obsidian-notes-karpathy/references/summary-template.md`
- `../obsidian-notes-karpathy/references/concept-template.md`
- `../obsidian-notes-karpathy/references/entity-template.md`
- `../obsidian-notes-karpathy/references/activity-log-template.md`
- `../obsidian-notes-karpathy/references/provenance-and-alias-policy.md`
- `../obsidian-notes-karpathy/references/paper-ingestion-lifecycle.md`
- `../obsidian-notes-karpathy/references/memory-lifecycle.md`
- `../obsidian-notes-karpathy/references/graph-contract.md`
- `../obsidian-notes-karpathy/references/source-manifest-contract.md`
- `../obsidian-notes-karpathy/references/topic-template.md`
- `../obsidian-notes-karpathy/references/procedure-template.md`
- `../obsidian-notes-karpathy/references/draft-schema.md`

Treat `skill-contract-registry.json` as the canonical source for required references, baseline script, and allowed write surfaces.

If `../obsidian-notes-karpathy/scripts/scan_compile_delta.py` exists, run it first.

## Non-negotiable rules

- do not rewrite `raw/`
- write only to `wiki/drafts/` and draft indices
- never promote directly into `wiki/live/`
- keep human captures and agent captures distinguishable in provenance
- keep PDF paper handling strict: `raw/**/papers/*.pdf` still routes through `paper-workbench`
- prefer tracked sources from `raw/_manifest.yaml` when it is present

## Source discovery

Accept:

- markdown captures under `raw/human/**`
- markdown captures under `raw/agents/{role}/**`
- markdown captures directly under `raw/` in bootstrap vaults
- image assets under `raw/**/assets/*`
- data assets under `raw/**/data/*`
- legacy-layout markdown captures under older paths only during migration
- paper PDFs under any `papers/` subtree inside raw

## Compile posture

Chinese-LLM-Wiki compatibility note:

- `来源页` maps to source-grounded draft summaries first
- `主题页` and `实体页` map to draft topic/entity updates first
- `综合页` only becomes approved truth after review, so compile should treat it as a draft synthesis candidate

Before shaping drafts:

- follow the shared compile method in `compile-method.md`
- normalize source metadata such as `source_hash`, `source_mtime`, `last_verified_at`, and `possibly_outdated`
- surface alias and duplicate candidates rather than silently creating competing concept/entity drafts
- treat duplicate/alias surfacing as both governance input and authoring leverage, so later query/publish work can reuse prior approved coverage instead of restating the same concept from scratch
- preserve cross-language or terminology overlap as review input, not as automatic merges
- surface repeated concept clusters, repeated question clusters, and likely hub candidates when the durable improvement is navigational rather than just another standalone page
- prefer strengthening relationships between likely draft/live neighbors when the knowledge exists but the graph is weak
- treat process-level takeaways as candidates for `wiki/drafts/procedures/**`, not as forced semantic pages
- treat cross-domain transfer value as a first-class output, not an optional flourish

## Main outputs

- `wiki/drafts/summaries/**`
- `wiki/drafts/topics/**`
- `wiki/drafts/concepts/**`
- `wiki/drafts/entities/**` when needed
- `wiki/drafts/procedures/**` when the durable delta is a workflow rather than a semantic page
- `wiki/drafts/indices/*`
- `wiki/drafts/indices/packages/**`
- batch `ingest` entry in `wiki/log.md`

The compile pass exists to hand clean draft packages to `kb-review`, which then writes `outputs/reviews/**`, promotes approved pages into `wiki/live/**`, and rebuilds `wiki/briefings/**`.

## Draft requirements

Follow `../obsidian-notes-karpathy/references/draft-schema.md` for the authoritative list of required and conditional draft fields (`draft_id`, `compiled_from`, `capture_sources`, `review_state`, `review_score`, `blocking_flags`, `evidence_coverage`, `uncertainty_level`, `promotion_target`, `review_package_meta`, plus conditional alias/duplicate/boundary/assumption/transfer/candidate/topic/confidence fields). Type-specific templates layer their own fields on top of that schema.

Keep source claims and compiler inferences cleanly separated — review depends on that separation to judge accuracy without re-reading the raw capture. Drafts should be shaped for review, not for final polish.

## Checkpoint

Before writing draft packages, confirm with the user:

- how many captures will be compiled and which ones
- whether any sources look ambiguous, duplicated, or potentially conflicting with existing live pages
- whether alias or duplicate candidates should be surfaced now or deferred to review

For single-source incremental compiles with no ambiguity, proceed without pausing.

## Output to the user

Always report:

1. how many captures were new, changed, or unchanged
2. how many draft summaries were created or updated
3. how many draft concepts or entities were touched
4. whether any PDFs were skipped because `paper-workbench` was unavailable
5. whether alias or duplicate candidates were surfaced for review
6. whether boundary conditions, assumption flags, or transfer targets were surfaced
7. whether the next step is `kb-review`
