# Memory Lifecycle

Use this reference when the bundle needs to talk about the latest lifecycle behavior without collapsing the review gate.

## Core posture

- `raw/**` stays immutable working evidence.
- `outputs/episodes/**` stores episodic memory: compacted chains of work, not approved truth.
- `wiki/live/**` stores semantic memory: approved reusable topic knowledge.
- `wiki/live/procedures/**` stores procedural memory: approved workflows and repeated decision patterns.
- `MEMORY.md` remains collaboration/editorial context only.

## Confidence

Treat confidence as explicit metadata, not an implied vibe.

Preferred fields on approved pages using the latest lifecycle metadata:

- `confidence_score`
- `confidence_band`
- `support_count`
- `contradiction_count`
- `last_confirmed_at`
- `next_review_due_at`
- `decay_class`

Confidence should strengthen with repeated support and weaken with time or contradiction. Old vaults can adopt this gradually; missing fields are maintenance issues, not structural repair failures.

## Supersession

When a newer page replaces or narrows an older one, prefer explicit supersession over silent overwrite.

Preferred fields:

- `supersedes`
- `superseded_by`
- `superseded_at`
- `supersession_reason`

## Promotion posture

- Promote to semantic memory when the durable delta is topical knowledge.
- Promote to procedural memory when the durable delta is a reusable workflow, playbook, or decision pattern.
- Keep the smallest durable delta possible: often the right move is a relationship edge, a hub update, or a question promotion rather than a brand-new page.

## Episodic backlog

Episode pages should not pile up forever at `consolidation_status: pending`.

Common next moves:

- `draft` when the episode points to a missing semantic or procedural note
- `health` when the episode mainly exposes drift, duplicate risk, or graph weakness
- `review` when a concrete candidate already exists and needs a decision
