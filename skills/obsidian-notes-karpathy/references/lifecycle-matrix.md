# Lifecycle Matrix

Use this table as the shared routing contract for the package entry skill and the operational skills.

## Structural diagnosis first

Prefer a structural diagnosis over user phrasing alone.

If `../scripts/detect_lifecycle.py` is available, run it against the target vault before deciding how to route.

The script is the deterministic baseline for:

- fresh vault detection
- partial support-layer repair detection
- compile-delta detection
- health-first flags such as duplicate concepts, stale Q&A, or Obsidian render breakage

Human judgment still matters when the user's symptom should override a structurally query-ready vault.

## Routing table

| Structural state | Default route | Why | Must inspect next | Typical writes |
| --- | --- | --- | --- | --- |
| `fresh` | `kb-init` | The raw/wiki/outputs contract does not exist yet. | target root, desired topic, whether this is a sub-vault | support layer + starter files |
| `partial` | `kb-init` | The vault has some KB signals but later skills would fail on missing support files. | missing support files, existing content that must be preserved | repaired support layer |
| `compile-ready` | `kb-compile` | New or changed raw sources are ahead of the compiled layer. | `scan_compile_delta.py`, raw notes, matching summaries | `wiki/`, derived indices, `wiki/log.md` |
| `query-ready` | `kb-query` | The compiled layer exists and there is no obvious source delta or urgent health drift. | `wiki/index.md`, indices, prior `outputs/qa/` | `outputs/`, sometimes `wiki/`, `wiki/log.md` |
| `health-first` | `kb-health` | Drift or mechanical breakage is more urgent than another compile or query pass. | `lint_obsidian_mechanics.py`, health rubric, local guidance | `outputs/health/`, safe mechanical fixes, `wiki/log.md` |

## Symptom overrides

These symptoms should push routing toward `kb-health` even when the structure alone might allow `kb-query`:

- the notes feel disconnected or contradictory
- stale Q&A is more likely than missing compilation
- an index or table renders incorrectly in Obsidian
- there are obvious duplicate concepts or entity aliases drifting apart

These symptoms should push routing toward `kb-init` even when `raw/` and `wiki/` both exist:

- `AGENTS.md` is missing
- `wiki/index.md` or `wiki/log.md` is missing
- the `wiki/indices/` support layer is incomplete

## Operational reminders

- `kb-init` owns contract creation and repair, not content compilation.
- `kb-compile` owns source-to-wiki updates, not full maintenance diagnosis.
- `kb-query` owns synthesis, archival, and publish artifacts once the compiled layer is trustworthy.
- `kb-health` owns deep maintenance passes and mechanical repair recommendations.
