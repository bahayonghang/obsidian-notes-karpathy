# Lifecycle Matrix

Use this table as the shared routing contract for the package entry skill and the operational skills.

## Structural diagnosis first

Prefer a structural diagnosis over user phrasing alone.

If `../scripts/detect_lifecycle.py` is available, run it against the target vault before deciding how to route.

The script is the deterministic baseline for:

- setup-needed vault detection
- repair-needed vault detection
- legacy-layout migration detection
- compile-delta detection
- pending review queue detection
- briefing refresh detection
- maintenance-needed integrity flags over the approved layer

## Routing table

| Structural state | Default route | Why | Must inspect next | Typical writes |
| --- | --- | --- | --- | --- |
| `needs-setup` | `kb-init` | The support layer does not exist yet. | target root, desired topic, whether this is a sub-vault | support layer + starter files |
| `needs-repair` | `kb-init` | The vault has some KB signals but later skills would fail on missing support files. | missing support files, existing content that must be preserved | repaired support layer |
| `legacy-layout` | `kb-init` | The vault still uses the old direct-compiled layout and should be migrated before normal operation. | migration path, old compiled files, missing companions | migration guidance or repair |
| `needs-compilation` | `kb-compile` | New or changed raw captures are ahead of the draft layer. | `scan_compile_delta.py`, raw captures, matching draft summaries | `wiki/drafts/`, draft indices, `wiki/log.md` |
| `needs-review` | `kb-review` | Draft knowledge exists and still needs an explicit gate decision. | `scan_review_queue.py`, overlapping live pages, referenced raw captures | `outputs/reviews/`, `wiki/live/`, `wiki/briefings/`, `wiki/log.md` |
| `needs-briefing-refresh` | `kb-review` | The approved brain changed after the last briefing build. | briefing sources, latest live timestamps | regenerated `wiki/briefings/`, `wiki/log.md` |
| `ready-for-query` | `kb-query` | The live layer exists and there is no obvious source delta, review backlog, or stale briefing. | `wiki/live/index`, live indices, prior `outputs/qa/`, relevant briefings | `outputs/qa/`, `outputs/content/`, `wiki/log.md` |
| `needs-maintenance` | `kb-health` | The approved layer has drift, integrity, or provenance problems that are more urgent than another query. | `lint_obsidian_mechanics.py`, health rubric, local guidance | `outputs/health/`, `wiki/log.md`, deterministic mechanical fixes in approved surfaces only |

## Symptom overrides

These symptoms should push routing toward `kb-health` even when the structure alone might allow `kb-query`:

- the live notes feel contradictory or unreliable
- briefings seem wrong even after a recent review pass
- there are obvious duplicate live concepts or approved conflicts
- the approved layer renders badly in Obsidian
- archived answers have pending writeback work piling up
- collaboration memory and approved knowledge appear to be mixing

These symptoms should push routing toward `kb-init` even when `raw/` and `wiki/` both exist:

- `AGENTS.md` is missing
- `wiki/index.md` or `wiki/log.md` is missing
- the review-gated directories `wiki/drafts/`, `wiki/live/`, `wiki/briefings/`, or `outputs/reviews/` are missing
- a vault is still clearly legacy-layout and has not been migrated

These symptoms should be surfaced as repair targets without blocking compile/query/review/health by themselves:

- only `CLAUDE.md` is missing
- a single noncanonical filename such as `agents.md` or `claude.md` exists but the contract is otherwise usable

## Operational reminders

- `kb-init` owns contract creation, migration, and repair.
- `kb-compile` owns source-to-draft updates.
- `kb-review` owns draft promotion, rejection, and briefing refresh.
- `kb-query` owns synthesis, archival, and publish artifacts from the approved layer only.
- `kb-health` owns deep maintenance passes over live knowledge, briefings, review backlog, and deterministic mechanical fixes in approved surfaces only.
