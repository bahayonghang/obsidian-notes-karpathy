# Taxonomy and Hubs

Use this reference when compile, review, query, or health work needs lightweight guidance on what kind of note should exist in `wiki/live/`, when to add relationships, and when to create a curated hub.

## Goal

Improve consistency and navigation without turning the vault into a rigid ontology project.

Keep this guidance review-enforced rather than compile-enforced.

## Core note classes

### Summary
Use a summary page when the note primarily preserves source-backed conclusions from one capture or a tightly related capture bundle.

Good fit:

- article summaries
- source briefings
- paper takeaways
- repo walkthrough summaries

### Concept
Use a concept page when the note captures a reusable idea, mechanism, or abstraction that should outlive any single source.

Good fit:

- review gate
- writeback loop
- provenance drift
- retrieval ladder

### Entity
Use an entity page when the note centers on a durable named thing.

Good fit:

- people
- organizations
- projects
- tools
- libraries
- datasets

### Overview
Use an overview page when the note synthesizes the high-level state of an entire topic area from approved live pages. Karpathy's LLM Wiki concept emphasizes `overview.md` as a birds-eye view that ties the whole knowledge area together.

Good fit:

- topic-area synthesis that evolves as the live layer grows
- research-program status pages that summarize where coverage stands
- onboarding summaries that help new readers orient before diving deeper

An overview should stay grounded in approved live pages and update when the underlying pages change. It is a synthesis view, not a shortcut around provenance.

### Comparison
Use a comparison page when the note systematically contrasts two or more approved concepts, entities, or approaches side by side. Karpathy explicitly lists `comparisons/` as a first-class wiki directory.

Good fit:

- tool-vs-tool or framework-vs-framework comparisons
- approach tradeoff analyses grounded in approved summaries
- methodology contrasts that recur across multiple queries

A comparison should cite the approved pages it draws from and flag where evidence is thin or contested.

## Relationship-first posture

Before creating a new live page, ask whether the durable improvement is better expressed as one of these smaller moves:

- add or strengthen `related` links between existing pages
- add aliases to the existing canonical page
- connect a page to a `topic_hub`
- expand `question_links` for a page participating in an open thread
- update an approved summary so it points more clearly at the concept or entity pages it already supports

Prefer relationship upgrades before page proliferation when the underlying knowledge already exists.

## Pattern posture

If a repeated structure or operating principle keeps appearing across approved notes, it may be worth documenting as a concept or pattern-style concept page before adding a new filesystem class.

Prefer lightweight documentation first over structural expansion.

## When to create a curated hub

Create a curated hub / MOC-like page when:

- several approved pages belong to the same durable theme
- users repeatedly need a stable entry point into a topic
- navigation or onboarding suffers without a human-curated map
- aliases alone are not enough to explain how pages relate
- a creator program needs a durable planning or coverage surface that should stay navigational rather than pretending to be a new truth page
- the main missing value is synthesis of relationships across approved pages rather than new evidence

A curated hub should:

- point to approved live pages
- explain the organizing logic briefly
- remain a navigation surface, not a shortcut around provenance
- support creator planning, topic balance, or prior-coverage reuse without pretending the hub itself is new evidence
- avoid restating unsupported conclusions without linking back to the underlying approved pages
- make obvious which pages are central, adjacent, unresolved, or thinly supported

## Curated hub relationship posture

To keep a compounding wiki navigable, a curated hub should usually make the underlying relationships explicit instead of just listing links.

Good hub relationships include:

- canonical entry pages for the topic
- adjacent concepts that readers often confuse
- durable entities, sources, or summaries that anchor the topic
- linked open questions, gaps, or backlog themes when the topic still has unresolved coverage debt
- prior approved coverage that new publish artifacts should reuse instead of restating from scratch

## Relationship heuristics

Use `related` links when pages should stay separate but future readers should traverse between them quickly.

Good fit:

- adjacent mechanisms that are often confused
- a summary and the concepts it materially supports
- an entity and the concepts most central to its role
- sibling topic pages within the same hub

Prefer a hub over many pairwise `related` links when several pages need a stable shared map.

Prefer aliases over `related` when two names are really the same durable identity.

Prefer merge review over either approach when two live pages appear to duplicate the same concept or entity.

For ordinary live pages, prefer lightweight minimum relationships over a heavy ontology:

- summaries should link outward to the main concepts, entities, or hubs they materially support
- concept pages should link to nearby concepts and to a hub when the concept belongs to a broader durable program
- entity pages should link to the concepts, summaries, or hubs that explain why the entity matters

## Naming posture

- keep one canonical durable identity for each approved concept or entity
- use `aliases` to absorb terminology variation and cross-language overlap
- avoid creating multiple live truth pages for wording variants that should share one identity
- keep hub names stable and human-browsable; they should feel like entry points, not internal implementation details

## Review posture

Review should decide:

- whether a page is really a summary, concept, or entity
- whether a relationship should stay as `related`, become a merge, or be represented through a hub
- whether a topic deserves a curated hub
- whether overlapping pages should be merged, linked, or kept distinct

Health should surface drift, duplicates, alias collisions, weak relationship coverage, and hub maintenance needs, but should not silently reclassify notes with semantic judgment.
