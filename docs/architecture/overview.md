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
- `MEMORY.md` is the collaboration-memory surface, outside default topic retrieval
- `wiki/live/indices/EDITORIAL-PRIORITIES.md` is the editor-in-chief surface
- optional governance views such as `QUESTIONS.md`, `GAPS.md`, and `ALIASES.md` sit under `wiki/live/indices/`

The routing and ownership contract lives primarily in `skills/obsidian-notes-karpathy/scripts/skill-contract-registry.json` and `skills/obsidian-notes-karpathy/references/lifecycle-matrix.md`. Read those two files when you need the canonical mapping between lifecycle states, owning skills, and allowed write surfaces.

## Design themes

- Separate production from judgment so draft errors do not compound.
- Keep retrieval local-first and auditable.
- Treat good answers as assets that can re-enter the wiki through review.
- Keep collaboration memory separate from source-grounded topic knowledge.

## Current pressure points

The main areas still evolving are:

- stronger anti-rot checks for weak evidence and knowledge drift
- better governance refresh workflows for recurring open questions and alias drift
- cleaner writeback review loops from durable outputs back into drafts

Use the contract pages under Guide, Skills, and Workflow for the current source of truth. Use Architecture pages to understand the rationale and forward direction.
