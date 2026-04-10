# Architecture Overview

This section explains why the bundle is shaped the way it is and where the design is heading.

## Current contract

The shipped workflow is review-gated:

- `raw/` is immutable evidence
- `wiki/drafts/` is reviewable build output
- `wiki/live/` is approved retrieval truth
- `wiki/briefings/` is regenerated runtime context
- `outputs/reviews/` is the required decision ledger
- `outputs/qa/`, `outputs/content/`, and `outputs/health/` are downstream artifacts that appear when later stages need them
- `raw/` doubles as the durable source-library surface, while creator-facing drafting, reuse, and publish shaping happen downstream
- `MEMORY.md` is the collaboration-memory surface, outside default topic retrieval
- `wiki/live/indices/EDITORIAL-PRIORITIES.md` is the editor-in-chief surface
- governance views such as `QUESTIONS.md`, `GAPS.md`, and `ALIASES.md` sit under `wiki/live/indices/` for mature vaults, but they must stay grounded in approved live pages

The routing and ownership contract lives primarily in `skills/obsidian-notes-karpathy/scripts/skill-contract-registry.json` and `skills/obsidian-notes-karpathy/references/lifecycle-matrix.md`. Read those two files when you need the canonical mapping between lifecycle states, owning skills, and allowed write surfaces.

## Design themes

- Separate production from judgment so draft errors do not compound.
- Keep retrieval local-first and auditable.
- Treat good answers as assets that can re-enter the wiki through review.
- Keep collaboration memory separate from source-grounded topic knowledge.
- Let archived outputs feed maintenance and writeback loops without silently promoting them into approved truth.

## Current pressure points

The main areas still evolving are:

- stronger anti-rot checks for weak evidence and knowledge drift
- better governance refresh workflows for recurring open questions and alias drift
- cleaner writeback review loops from durable outputs back into drafts
- stronger creator-program surfaces for prior-coverage reuse, curated hubs, and planning views that stay outside the truth boundary
- more explicit retrieval ladders and follow-up routing across query and health surfaces

Use the contract pages under Guide, Skills, and Workflow for the current source of truth. Use Architecture pages to understand the rationale and forward direction.
