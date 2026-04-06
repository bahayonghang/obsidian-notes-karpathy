# obsidian-notes-karpathy

Package-level entry skill for the Karpathy-style workflow.

## What it does

This skill does not perform the whole workflow itself. It detects the current lifecycle stage and routes to the correct operational skill.

## Routing logic

- missing vault structure -> `kb-init`
- new or changed raw sources -> `kb-compile`
- question, report, slide, or publish request -> `kb-query`
- lint, stale-claim review, contradiction audit, or disconnected-note diagnosis -> `kb-health`

## Why this exists

Without a package entry skill, broad prompts such as "help me set up a Karpathy workflow in Obsidian" are easy to under-trigger. This page-level skill gives the bundle a discoverable front door.

## Key doctrine

- treat `raw/` as immutable
- start from `wiki/index.md` and `wiki/log.md`
- archive substantive Q&A by default
- prefer markdown-first search before infrastructure upgrades
- use backlinks and properties before heavier retrieval infrastructure
