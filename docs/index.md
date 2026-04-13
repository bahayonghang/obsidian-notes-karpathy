---
layout: home

hero:
  name: "Obsidian Notes Karpathy"
  text: "Review-Gated Knowledge Bases for Obsidian"
  tagline: Separate capture, draft compilation, approval, and briefing so multi-agent knowledge compounds on approved truth instead of raw hallucinations.
  image:
    src: /hero-image.svg
    alt: Obsidian Notes Karpathy
  actions:
    - theme: brand
      text: Quick Start
      link: /guide/quick-start
    - theme: alt
      text: Choose a Skill
      link: /skills/overview
    - theme: alt
      text: Architecture
      link: /architecture/overview
    - theme: alt
      text: 中文文档
      link: /zh/guide/introduction

features:
  - title: Route by Lifecycle Signal
    details: The package entry skill diagnoses whether the vault needs initialization, source registration, compilation, review, querying, rendering, or a health pass.
    link: /skills/obsidian-notes-karpathy
  - title: Canonical Source Registry
    details: "`kb-ingest` keeps `raw/_manifest.yaml` aligned with raw captures before compile starts shaping drafts."
    link: /skills/kb-ingest
  - title: Draft Before Live
    details: "`kb-compile` turns immutable captures into `wiki/drafts/`; nothing becomes durable truth until `kb-review` promotes it."
    link: /skills/kb-compile
  - title: Independent Review Gate
    details: "`kb-review` scores drafts, writes review records, promotes only approved knowledge into `wiki/live/`, and rebuilds briefings."
    link: /skills/kb-review
  - title: Persistent Research Memory
    details: "Substantive answers live in `outputs/qa/`, but only approved live pages and briefings should feed query-time synthesis."
    link: /skills/kb-query
  - title: Publish From Grounded Notes
    details: "Reports, article drafts, social threads, and talk outlines are generated from `wiki/live/` evidence."
    link: /workflow/overview
  - title: Unified Read-Side Lane
    details: "`kb-query` now owns local-first retrieval, grounded synthesis, archived answer reuse, and static web export."
    link: /skills/kb-query
  - title: Immutable Raw Layer
    details: "`raw/human/**` and `raw/agents/{role}/**` are evidence intake only, never the deployed brain."
    link: /guide/directory-structure
  - title: Live Brain + Briefings
    details: "`wiki/live/` stores approved knowledge and `wiki/briefings/` turns it into role-specific context for agents."
    link: /workflow/overview
  - title: Governance Lane
    details: "`kb-review` now owns both the immediate gate and maintenance-mode audits across approved knowledge, stale briefings, backlog, and provenance."
    link: /skills/kb-review
  - title: Markdown First, Search Later
    details: Start with approved live indices, backlinks, and briefings before upgrading retrieval infrastructure.
    link: /workflow/query
  - title: Boundaries Before Retrieval
    details: Keep collaboration memory, draft knowledge, approved live pages, and writeback candidates in separate surfaces so the wiki can grow without rotting.
    link: /architecture/boundaries
---
