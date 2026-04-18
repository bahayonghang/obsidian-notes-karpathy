# Compile Method

Use this reference whenever `kb-compile` needs the public contract for turning raw captures into draft knowledge.

## Core posture

Compile should not stop at summarization. The default creator-ready method is:

`浓缩 -> 质疑 -> 对标`

The goal is to turn raw evidence into reusable draft packages that preserve signal, surface fragility, and identify durable transfer value.

## Step 1: 浓缩

Reduce the source to the minimum set of claims that would materially change understanding.

Output expectations:

- no more than 3 core conclusions
- each conclusion paired with the key evidence that justifies it
- direct source claims kept separate from compiler-added framing

Good compression should make it obvious what the source is really saying without flattening all nuance into one vague paragraph.

## Step 2: 质疑

Stress-test each core conclusion before it hardens into reusable draft knowledge.

Check explicitly:

- which assumptions must hold for the conclusion to remain valid
- boundary conditions such as market, industry, scale, geography, or time horizon
- sample-size, source-quality, and freshness limits
- counterexamples, failure cases, or reasons the conclusion may not transfer

Output expectations:

- `assumption_flags`
- `boundary_conditions`
- review notes when evidence is thin, outdated, or context-bound

## Step 3: 对标

Look for cross-domain analogies and migration value.

Ask:

- what similar phenomenon exists in another domain
- which workflows, concepts, or hubs this source should strengthen
- whether the durable delta is semantic knowledge, a procedure, or a hub/relationship upgrade

Output expectations:

- `transfer_targets`
- concept / procedure / hub candidates
- relationship candidates when the best durable improvement is connective rather than page-creating

## Promotion guidance

- If the durable delta is a repeatable workflow, prefer `wiki/drafts/procedures/**`.
- If the durable delta is a cross-domain pattern or reusable idea, prefer concept or hub candidates.
- If the source is valuable mainly because it strengthens links between existing approved pages, prefer relationship or hub upgrades over another standalone page.

## Machine-readable posture

Draft summaries should expose these compile-time outputs in frontmatter when available:

- `boundary_conditions`
- `assumption_flags`
- `transfer_targets`

Those fields are review inputs. They do not widen the truth boundary by themselves.
