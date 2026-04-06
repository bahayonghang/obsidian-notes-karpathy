# AGENTS.md and CLAUDE.md

The vault contract is expressed in two schema files:

- `AGENTS.md`
- `CLAUDE.md`

They should describe the same rules with minimal wrapper differences.

## What they define

- directory purposes
- frontmatter schemas
- immutable raw rule
- summary tracking fields such as `source_mtime` or source hash
- index and log responsibilities
- Q&A archival expectations

## Why both exist

Different coding agents consult different top-level guidance files. Generating both keeps the vault portable across agents without changing the workflow itself.
