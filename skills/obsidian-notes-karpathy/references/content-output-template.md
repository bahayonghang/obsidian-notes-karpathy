# Content Output Template

Use this when the user wants to turn the knowledge base into publishable content.

```markdown
---
title: "{Artifact Title}"
created_at: "{datetime}"
artifact_type: article | thread | newsletter | talk-outline
channel: "{channel}"
sources:
  - "[[wiki/live/concepts/concept-a]]"
  - "[[wiki/live/entities/entity-b]]"
  - "[[outputs/qa/question-a]]"
source_live_pages:
  - "[[wiki/live/concepts/concept-a]]"
  - "[[wiki/live/entities/entity-b]]"
derived_from:
  - "[[outputs/qa/question-a]]"
reused_prior_coverage:
  - "[[wiki/live/indices/topic-hub]]"
related_pages:
  - "[[wiki/live/concepts/concept-c]]"
topic_hubs:
  - "[[wiki/live/indices/hub-a]]"
open_questions_touched:
  - "[[wiki/live/indices/QUESTIONS#question-a]]"
writeback_candidates:
  - "create or expand curated hub for topic-x"
writeback_status: none | pending | triaged | drafted | reviewed | rejected
followup_route: none | draft | review | health
confidence_posture: grounded | mixed-evidence | exploratory
compounding_value: low | medium | high
crystallized_from_episode: "[[outputs/episodes/example-episode]]"
visibility_scope: private | shared
---

# {Artifact Title}

## Brief

- Audience: {audience}
- Goal: {goal}
- Angle: {angle}
- Prior coverage reused: {existing approved page or archived artifact reused instead of re-explaining from scratch}
- Topic hub or program touched: {hub / coverage surface when relevant}
- Reusable payoff: {how this artifact should make future query, drafting, or hub-building easier}

## Core Claims

1. {claim 1}
2. {claim 2}
3. {claim 3}

## Draft

{full draft}

## Provenance

- [[wiki/live/concepts/concept-a]] - {what it contributed}
- [[wiki/live/entities/entity-b]] - {what it contributed}
- [[outputs/qa/question-a]] - {what it contributed}

## Relationship Notes

- Related pages worth linking or updating: {pages that should gain `related`, alias, or hub connections}
- Hub impact: {whether this artifact strengthens an existing hub or suggests a new one}

## Proposed Durable Delta

- Preferred move: {update live page | draft new page | strengthen relationships | create / expand curated hub}
- Target: {page or hub}
- Why this compounds the wiki: {future reuse, navigation gain, or reduced duplication}

## Writeback Candidates

- {new concept, entity, relationship, hub, or source gap discovered while drafting}
```

Rules:

- Publish artifacts should stay grounded in the wiki instead of drifting away from the evidence.
- Prefer reusing prior approved coverage or archived outputs explicitly before restating the same background explanation in a new artifact.
- If the draft introduces a new synthesis worth preserving, pair the artifact with explicit `writeback_candidates` or a supporting Q&A note.
- When the artifact came out of a larger research/debugging thread, keep the episodic breadcrumb through `crystallized_from_episode`.
- Writeback candidates may be relationship or hub upgrades, not only new pages.
- Keep raw source captures immutable; editorial shaping belongs in the artifact, Q&A archive, or the later draft/review lane.
- Prefer the smallest durable delta that improves the wiki after publication: sometimes this is a new synthesis page, but often it is a hub update, relationship upgrade, or governed question.
