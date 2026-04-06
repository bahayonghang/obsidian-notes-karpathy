---
layout: home

hero:
  name: "Obsidian Notes Karpathy"
  text: "LLM-Compiled Knowledge Bases for Obsidian"
  tagline: Keep raw sources immutable, compile a persistent wiki, and let questions compound into reusable artifacts.
  image:
    src: /hero-image.svg
    alt: Obsidian Notes Karpathy
  actions:
    - theme: brand
      text: Get Started
      link: /guide/introduction
    - theme: alt
      text: View Skills
      link: /skills/overview
    - theme: alt
      text: 中文文档
      link: /zh/guide/introduction

features:
  - title: Package Entry + 4 Operations
    details: One package-level entry skill routes requests into kb-init, kb-compile, kb-query, or kb-health.
    link: /skills/obsidian-notes-karpathy
  - title: Immutable Raw Layer
    details: Source notes stay untouched. Compilation state lives in wiki summaries and health outputs.
    link: /guide/directory-structure
  - title: Persistent Q&A Memory
    details: Substantive answers are archived to outputs/qa and can feed back into concept pages and future research.
    link: /skills/kb-query
  - title: Publish From The Wiki
    details: kb-query can turn grounded research into article drafts, social threads, and talk outlines under outputs/content.
    link: /workflow/query
  - title: index.md + log.md
    details: The wiki has both a content-oriented entry point and an append-only operational timeline.
    link: /workflow/overview
  - title: Deep Health Checks
    details: kb-health runs report-oriented linting for contradictions, stale claims, alias drift, weak links, orphan pages, and provenance gaps.
    link: /skills/kb-health
  - title: Markdown First, Search Later
    details: Start with markdown indices, Backlinks, and Properties search. Add DuckDB, Dataview, or heavier retrieval only when scale demands it.
    link: /workflow/query
---
