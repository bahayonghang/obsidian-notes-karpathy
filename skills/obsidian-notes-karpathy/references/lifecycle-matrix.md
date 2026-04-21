# Lifecycle Matrix

Use this table as the shared routing contract for the package entry skill and the operational skills.

## Structural diagnosis first

Prefer a structural diagnosis over user phrasing alone.

If `onkb` is available, run `onkb --json status <vault-root>` before deciding how to route.

If the shell reports `onkb` is not installed, install it from GitHub first, then rerun the same command:

- `cargo install --locked --git https://github.com/bahayonghang/obsidian-notes-karpathy.git onkb`
- repo maintainers working from a local clone may use `cargo install --path . --locked` instead

The CLI is the deterministic baseline for:

- setup-needed vault detection
- repair-needed vault detection
- legacy-layout migration detection
- compile-delta detection
- source-manifest drift detection
- pending review queue detection
- briefing refresh detection
- maintenance-needed integrity flags over the approved layer
- latest lifecycle health flags such as confidence gaps, supersession gaps, episodic backlog, graph gaps, procedural promotion gaps, and audit trail gaps

## Routing table

| Structural state | Default route | Why | Must inspect next | Typical writes |
| --- | --- | --- | --- | --- |
| `needs-setup` | `kb-init` | The support layer does not exist yet. | target root, desired topic, whether this is a sub-vault | support layer + starter files |
| `needs-repair` | `kb-init` | The vault has some KB signals but later skills would fail on missing support files. | missing support files, existing content that must be preserved | repaired support layer |
| `needs-migration` | `kb-init` | The vault still uses the old direct-compiled layout and should be migrated before normal operation. | migration path, old compiled files, missing companions | migration guidance or repair |
| `needs-ingest` | `kb-ingest` | Raw sources and the canonical source manifest disagree. | `onkb --json ingest scan`, `raw/_manifest.yaml`, deferred sources | refreshed `raw/_manifest.yaml`, `wiki/log.md` |
| `needs-compilation` | `kb-compile` | New or changed raw captures are ahead of the draft layer. | `onkb --json compile scan`, raw captures, matching draft summaries | `wiki/drafts/`, draft indices, `wiki/log.md` |
| `needs-review` | `kb-review` | Draft knowledge exists and still needs an explicit gate decision. | `onkb --json review queue`, overlapping live pages, referenced raw captures | `outputs/reviews/`, `wiki/live/`, `wiki/briefings/`, `wiki/log.md` |
| `needs-briefing-refresh` | `kb-review` | The approved brain changed after the last briefing build. | briefing sources, latest live timestamps | regenerated `wiki/briefings/`, `wiki/log.md` |
| `ready-for-query` | `kb-query` | The live layer exists and there is no obvious source delta, review backlog, or stale briefing. | `wiki/live/index`, live indices, prior `outputs/qa/`, relevant briefings | `outputs/qa/`, `outputs/content/`, `wiki/log.md` |
| `needs-maintenance` | `kb-review` (`maintenance` mode) | The approved layer has drift, integrity, or provenance problems that are more urgent than another query. | `onkb --json review lint`, health rubric, local guidance | `outputs/health/`, `wiki/live/indices/`, `wiki/log.md`, deterministic mechanical fixes in approved surfaces only |

## Symptom overrides

These symptoms should push routing toward `kb-review` maintenance mode even when the structure alone might allow `kb-query`:

- the live notes feel contradictory or unreliable
- briefings seem wrong even after a recent review pass
- there are obvious duplicate live concepts or approved conflicts
- aliases appear split across multiple live notes
- source hashes or verification timestamps suggest provenance drift
- the approved layer renders badly in Obsidian
- archived answers have pending writeback work piling up
- archived outputs need hygiene work such as reuse drift, stale claims, or scope leakage
- collaboration memory and approved knowledge appear to be mixing
- open questions and gap reports are accumulating without clear ownership
- pages using the latest lifecycle metadata are missing confidence metadata, supersession bookkeeping, or due-date refreshes
- episodic or audit scaffolding exists but is falling behind

These symptoms should push routing toward `kb-init` even when `raw/` and `wiki/` both exist:

- `AGENTS.md` is missing
- `wiki/index.md` or `wiki/log.md` is missing
- the review-gated directories `wiki/drafts/`, `wiki/live/`, `wiki/briefings/`, or `outputs/reviews/` are missing
- a vault is still clearly legacy-layout and has not been migrated

These symptoms should be surfaced as repair targets without blocking compile/query/review/health by themselves:

- only `CLAUDE.md` is missing
- a single noncanonical filename such as `agents.md` or `claude.md` exists but the contract is otherwise usable
- optional governance indices like `QUESTIONS.md` or `ALIASES.md` are absent

## Operational reminders

- `kb-init` owns contract creation, migration, repair, and optional governance scaffolding.
- `kb-ingest` owns source registration, manifest refresh, and deferred-source visibility.
- `kb-compile` owns source-to-draft updates, source metadata normalization, and alias-candidate surfacing.
- `kb-review` owns draft promotion, rejection, contradiction handling, briefing refresh, and maintenance-mode governance passes.
- `kb-query` owns synthesis, local-first retrieval, archival, and static web export from the approved layer only.
- `kb-render` owns deterministic derivative rendering from approved knowledge.
- backward compatibility rule: old vaults do not become `needs-repair` just because they lack the latest lifecycle fields; this drift is surfaced through `kb-review` maintenance flags instead.
