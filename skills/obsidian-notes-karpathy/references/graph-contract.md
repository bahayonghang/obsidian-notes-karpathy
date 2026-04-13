# Graph Contract

Use this reference when adding the latest graph structure without replacing the markdown wiki.

## Graph surfaces

- Page body and frontmatter remain the primary human-readable source.
- `related`, `topic_hub`, `question_links`, `relationship_notes`, `supersedes`, and `superseded_by` are the canonical explicit graph hints in markdown.
- `wiki/live/indices/ENTITIES.md` and `wiki/live/indices/RELATIONSHIPS.md` are derived navigation views.
- `outputs/health/graph-snapshot.json` is a machine-readable export for local candidate retrieval, audits, and tooling.

## Edge types

Prefer typed edges over vague adjacency:

- `related`
- `supports`
- `contrasts`
- `extends`
- `supersedes`
- `superseded_by`
- `related-question`

`relationship_notes` may carry the human-readable nuance, but durable traversal should still land on explicit `related` or supersession fields when possible.

## Retrieval boundary

The graph augments retrieval. It does not widen truth.

- The graph can help discover candidate pages.
- Final answers should still cite approved `wiki/live/**` pages.
- Episode nodes and graph snapshot exports are candidate-only surfaces.

## Health posture

Surface `graph_gap` when a page clearly signals graph intent but lacks durable edges, for example:

- `relationship_notes` exists without `related`
- graph-required pages remain disconnected
- repeated hub signals never become explicit links
