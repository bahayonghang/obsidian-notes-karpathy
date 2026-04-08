# Introduction

Obsidian Notes Karpathy is a bundle of Obsidian-oriented skills for running a review-gated, compile-first knowledge base inside a vault.

## What the bundle owns

```text
raw/            -> immutable capture intake
MEMORY.md       -> collaboration memory and editorial context
wiki/drafts/    -> compiled draft knowledge
wiki/live/      -> approved long-term brain
wiki/briefings/ -> role-specific context generated from live
outputs/        -> reviews, Q&A, health reports, and publishable derivatives
```

The workflow is not "retrieve from whatever the agents last said." It is "compile captures into drafts, review them, promote only approved knowledge, and then reuse the result."

`MEMORY.md` is outside the default knowledge retrieval truth boundary. Use it for preferences, editorial priorities, and collaboration rules rather than source-grounded topic conclusions.

## Core skills

This bundle is organized as one shared package home plus five operation skills. `skills/obsidian-notes-karpathy/` holds the shared package home for `references/`, `scripts/`, `evals/`, and the routing contract, while the top-level `skills/kb-*` directories are the operation skills that carry out each lifecycle step.

| Skill | Responsibility | Reach for it when |
| --- | --- | --- |
| `obsidian-notes-karpathy` | Package entry and lifecycle router | The user talks about the workflow as a whole or asks what to do next |
| `kb-init` | Create, repair, or migrate the vault contract | The support layer is missing, partial, or still in a legacy-layout |
| `kb-compile` | Turn raw captures into reviewable draft pages | New or changed evidence is waiting |
| `kb-review` | Decide what deserves to persist and rebuild briefings | Drafts are pending or briefings are stale |
| `kb-query` | Search, answer, archive, and publish from the approved layer | The vault already knows enough and the user wants an answer or artifact |
| `kb-health` | Audit drift, backlog, briefings, and provenance | The approved layer feels unreliable or disconnected |
