# Architecture Overview

This section explains why the bundle is shaped the way it is and where the design is heading.

## Current contract

The shipped workflow is review-gated:

- `raw/` is immutable evidence
- `wiki/drafts/` is reviewable build output
- `wiki/live/` is approved retrieval truth
- `wiki/briefings/` is regenerated runtime context
- `outputs/qa/` and `outputs/content/` are durable downstream artifacts

The routing and ownership contract lives primarily in `skills/obsidian-notes-karpathy/scripts/skill-contract-registry.json` and `skills/obsidian-notes-karpathy/references/lifecycle-matrix.md`. Read those two files when you need the canonical mapping between lifecycle states, owning skills, and allowed write surfaces.

## Design themes

- Separate production from judgment so draft errors do not compound.
- Keep retrieval local-first and auditable.
- Treat good answers as assets that can re-enter the wiki through review.
- Keep collaboration memory separate from source-grounded topic knowledge.

## Planned / evolving areas

The following areas are actively designed but may be only partially shipped depending on the current contract:

- `MEMORY.md` as explicit collaboration memory
- `EDITORIAL-PRIORITIES.md` as the editor-in-chief surface
- structured writeback from `outputs/qa/` and `outputs/content/`
- stronger anti-rot checks for weak evidence and knowledge drift

Use the contract pages under Guide, Skills, and Workflow for the current source of truth. Use Architecture pages to understand the rationale and forward direction.
