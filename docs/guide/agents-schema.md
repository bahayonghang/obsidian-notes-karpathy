# AGENTS.md and CLAUDE.md

The vault contract is anchored by `AGENTS.md`.

`kb-init` should also generate `CLAUDE.md` as a companion file so different coding agents read the same vault rules.

## What they define

- directory purposes
- frontmatter schemas
- immutable raw rule
- summary tracking fields such as `source_mtime` or source hash
- index and log responsibilities
- Q&A archival expectations

## Why both exist

Different coding agents consult different top-level guidance files. Generating both keeps the vault portable across agents without changing the workflow itself.

Practical contract strength:

- `AGENTS.md` is required local guidance
- `CLAUDE.md` is the generated companion
- if only `CLAUDE.md` is missing in an otherwise usable vault, treat it as a repair target rather than a reason to block compile, query, or health work
