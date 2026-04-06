---
layout: home

hero:
  name: "Obsidian Notes Karpathy"
  text: "LLM-Compiled Knowledge Bases for Obsidian"
  tagline: Route the vault by lifecycle, keep raw sources immutable, and turn questions into durable research artifacts.
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
      text: 中文文档
      link: /zh/guide/introduction

features:
  - title: Route by Lifecycle Signal
    details: The package entry skill diagnoses whether the vault needs initialization, compilation, querying, or a health pass.
    link: /skills/obsidian-notes-karpathy
  - title: Compile, Do Not Improvise
    details: Source notes stay untouched. The LLM updates summaries, concept pages, indices, logs, and optional entities in the compiled layer.
    link: /skills/kb-compile
  - title: Persistent Research Memory
    details: Substantive answers live in outputs/qa instead of disappearing into chat history.
    link: /skills/kb-query
  - title: Publish From Grounded Notes
    details: Reports, article drafts, social threads, and talk outlines are generated from wiki-backed evidence.
    link: /workflow/overview
  - title: Immutable Raw Layer
    details: Raw markdown is human-curated input, not a place to store compilation state or generated edits.
    link: /guide/directory-structure
  - title: index.md + log.md
    details: The vault keeps both a content-oriented entry surface and an append-only operational history.
    link: /workflow/overview
  - title: Deep Health Checks
    details: kb-health scores completeness, consistency, connectivity, freshness, and provenance, then separates safe fixes from judgment calls.
    link: /skills/kb-health
  - title: Markdown First, Search Later
    details: Start with index pages, backlinks, unlinked mentions, and Properties search before adding heavier retrieval infrastructure.
    link: /workflow/query
---
