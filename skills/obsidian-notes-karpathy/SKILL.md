---
name: obsidian-notes-karpathy
description: Diagnose and route ambiguous, workflow-level review-gated Obsidian vault requests. Use this skill when the user is talking about an Obsidian vault workflow as a whole, asks which lifecycle step should run next, says things like "what should I run first", "which stage am I in", "LLM Wiki", "Karpathy wiki", "Obsidian IDE", "knowledge compiler", "creator knowledge compiler", "personal knowledge base", "second brain", "archive this answer", "归档这个回答", "存回知识库", "复用归档内容", "清理 archive backlog", "先读 wiki/index.md", "这个知识库下一步该做什么", "现在应该初始化/摄取/编译/审校/检索/发布/渲染/体检哪个步骤", or uses Chinese-LLM-Wiki wording such as `中文优先`, `来源页`, `主题页`, `实体页`, `综合页`, `output/analyses`, `output/reports`, `原文证据摘录`, `先读 wiki/index.md 再判断`, or `先判断该做来源页还是综合页` without making the operation explicit. Prefer the operation-specific skills when the user already clearly means init, ingest, compile, review, query, publish, render, or archive maintenance, and only route through this package entry skill when the workflow step is genuinely ambiguous.
---

# Obsidian Notes Karpathy

Package-level entry point for the review-gated knowledge workflow.

Use this skill when the user talks about the workflow as a whole, not just one operation. Diagnose the lifecycle stage first, then route to the correct operational skill. If the operation is already obvious, skip the router and go straight to the matching `kb-*` skill.

Archive semantics are split deliberately:

- raw retention archive: `raw/**` plus `raw/_manifest.yaml`
- artifact archive: durable outputs under `outputs/**`

Neither archive surface bypasses the `draft -> review -> live` truth boundary.

When the user brings in the simpler `raw/wiki/output` vocabulary from `Chinese-LLM-Wiki`, route by meaning rather than mirroring that older structure literally.

## Minimal loop

- `kb-ingest` registers raw sources into `raw/_manifest.yaml`
- `kb-compile` builds reviewable candidates from immutable captures
- `kb-review` decides what deserves durable truth
- `kb-query` reuses approved knowledge for search, grounded answers, creator-facing publish artifacts, archived Q&A reuse, and static web export
- `kb-render` turns approved knowledge into deterministic derivative artifacts
- `kb-review` also owns the maintenance lane when approved knowledge drifts or backlog accumulates

## Karpathy alignment

This package implements the LLM Wiki pattern: "Obsidian is the IDE; the LLM is the programmer; the wiki is the codebase." The user curates sources, asks questions, and thinks about meaning. The LLM handles summarizing, cross-referencing, filing, and bookkeeping that makes knowledge compound over time.

The key extension beyond Karpathy's original idea is an explicit review gate: unreviewed drafts never silently harden into long-term truth. This adds a `draft → review → live` promotion step that Karpathy's pattern leaves implicit.

The creator-ready extension is that compile should behave like a knowledge compiler, not just a summarizer: collect -> ingest -> compile (`浓缩 -> 质疑 -> 对标`) -> review -> query/publish -> maintenance.

## When this compounds the wiki

The package should behave like a persistent markdown wiki operator, not just a staged pipeline. Each step should either improve approved knowledge directly through the gate or leave behind clearer navigation, better reuse surfaces, and explicit next actions.

## Read before routing

Read these shared references first:

- `./scripts/skill-contract-registry.json`
- `./references/chinese-llm-wiki-compat.md`
- `./references/archive-model.md`
- `./references/file-model.md`
- `./references/lifecycle-matrix.md`
- `./references/search-upgrades.md`
- `./references/activity-log-template.md`
- `./references/provenance-and-alias-policy.md`
- `./references/questions-and-reflection-policy.md`
- `./references/memory-lifecycle.md`
- `./references/graph-contract.md`
- `./references/source-manifest-contract.md`
- `./references/profile-contract.md`
- `./references/automation-hooks.md`

Treat `skill-contract-registry.json` as the canonical list of package roles, required shared references, baseline scripts, and output surfaces.

If the target vault already exists, inspect:

- local `AGENTS.md`
- local `CLAUDE.md` if present
- the top of `wiki/index.md`
- the most recent entries in `wiki/log.md` when available

If `./scripts/detect_lifecycle.py` exists, run it first and treat its JSON output as the deterministic baseline.

If `./scripts/vault_status.py` exists, use it as the user-facing status wrapper after lifecycle detection when the user mainly wants a concise "where am I and what next?" summary.

## Lifecycle signals

### Init signals

Route to `kb-init` when:

- the support layer does not exist yet
- `AGENTS.md` is missing
- `wiki/index.md` or `wiki/log.md` is missing
- `wiki/drafts/`, `wiki/live/`, `wiki/briefings/`, or `outputs/reviews/` is missing
- the vault is a `needs-migration` / legacy-layout case that must be migrated first
- optional governance scaffolding such as `QUESTIONS.md` or `ALIASES.md` is explicitly requested

### Compile signals

Route to `kb-compile` when:

- new or changed raw captures exist under `raw/human/**`, `raw/agents/{role}/**`, or directly under `raw/` in a bootstrap vault
- the draft summary matching a raw capture is missing or outdated
- alias candidates, duplicate candidates, or source-integrity drift need to be surfaced before review
- a legacy-layout raw source still needs to be converted into the draft layer during migration

### Ingest signals

Route to `kb-ingest` when:

- `raw/_manifest.yaml` exists but is stale relative to `raw/**`
- the user explicitly wants to refresh the source registry or inspect deferred raw sources
- a paper PDF, image asset, or data asset should be registered before compile runs
- a manifest drift signal appears before draft compilation work starts

### Review signals

Route to `kb-review` when:

- draft knowledge exists under `wiki/drafts/**` and still has `review_state: pending`
- the user explicitly asks to run the quality gate, approve drafts, reject drafts, or rebuild briefings
- briefings are stale relative to the live layer and the next immediate step is to rebuild them through the gate
- a draft has unresolved contradiction, alias-alignment, or duplication-risk questions that require approval judgment

`kb-review` owns the immediate gate: pending draft decisions, promotion into `wiki/live/`, and briefing rebuilds that should happen as part of the current review pass.

### Query signals

Route to `kb-query` when:

- `wiki/live/**` is trustworthy and current
- the user wants an answer, report, thread, slides, or other artifact grounded in the approved brain
- the user wants to reuse prior approved coverage or archived outputs before drafting a new outward-facing artifact
- the user wants to archive a substantive answer or reuse a prior archived answer/content artifact
- the user wants question-resolution candidates or reflection outputs that should stay outside live until re-reviewed
- the user explicitly uses older `kb-search` wording for local-first retrieval or candidate ranking

### Render signals

Route to `kb-render` when:

- the user explicitly wants slides, a report, chart brief, or canvas output
- the task is deterministic format generation from approved knowledge rather than a normal grounded answer
- the user already has the source pages or archived answer and now wants a derivative artifact

### Health signals

Route to `kb-review` in `maintenance` mode when:

- the live layer feels contradictory, weakly linked, or poorly provenanced
- approved pages appear to have bypassed review
- review backlog, stale briefings, or writeback pressure have become longer-horizon maintenance issues rather than the next immediate gate
- archived answers have pending writeback work
- archived outputs need hygiene work such as stale reuse, archive backlog cleanup, or private/shared scope leakage
- collaboration memory and approved knowledge appear to be mixing
- the user wants a maintenance baseline, drift audit, duplicate pass, alias audit, coverage-gap review, or report-first cleanup pass across approved surfaces
- the user wants planning surfaces such as curated hubs or editorial-gap views refreshed without widening the truth boundary
- the user wants to know which repeated outputs, question clusters, or weakly connected topics should become syntheses or hubs next

`kb-review` owns both the immediate gate and the longer-horizon maintenance lane: approved-layer drift, backlog pressure, archived-output hygiene, source-integrity drift, alias splits, graph weakness, hub backlog, and safe mechanical fixes after the immediate review gate has passed.

## Companion skills

These skills are referenced by the lifecycle but live outside this bundle and are not maintained here:

- `paper-workbench` — external companion for paper PDF intake and normalization. Any `raw/**/papers/*.pdf` defers to this skill; when it is absent, `kb-ingest` surfaces the PDF as `ingest_status: deferred` and `kb-compile` refuses to treat it as a normal source.
- `web-access` / Obsidian Web Clipper — upstream collection lane for web sources before they land under `raw/**`. Routed through `kb-ingest` once the capture is on disk.

Companion skills are declared in `scripts/skill-contract-registry.json` under the top-level `companion_skills` field. Audit tooling treats them as external and does not report them as missing from the bundle.

## Contract rules

- Treat `raw/` as immutable evidence intake.
- Treat `raw/_manifest.yaml` as the canonical source registry.
- Treat `wiki/drafts/` as reviewable knowledge, not query truth.
- Treat `wiki/live/` as the only approved long-term brain.
- Treat `wiki/live/topics/` as the default browse layer over approved knowledge.
- Treat `wiki/live/procedures/` as approved procedural memory, not just another concept bucket.
- Treat `wiki/briefings/` as per-role context generated from live only.
- Treat `outputs/episodes/` as episodic memory and `outputs/audit/operations.jsonl` as the machine-readable audit trail.
- Treat archived `outputs/qa/**` and `outputs/content/**` as reusable artifact archive, not as approved truth.
- Treat `outputs/reviews/` as the durable decision ledger.
- Keep `wiki/index.md` and `wiki/log.md` as complementary navigation surfaces.
- Absorb stronger governance signals such as source integrity, alias alignment, stale-page checks, and question tracking without collapsing the review gate.

## Output requirements

When the user asks what to do next, answer with:

1. the lifecycle stage you detected
2. the concrete signals that led you there
3. the operational skill you are routing to
4. the next concrete action in the vault
5. any assumption you had to make
6. any repair target, migration warning, or missing companion guidance
