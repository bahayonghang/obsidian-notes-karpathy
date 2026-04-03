---
# https://vitepress.dev/reference/default-theme-home-page
layout: home

hero:
  name: "Obsidian Notes Karpathy"
  text: "LLM-Driven Knowledge Base"
  tagline: Transform raw information into structured, queryable wisdom — inspired by Andrej Karpathy's workflow
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
  - title: 🔄 Karpathy Workflow
    details: Raw data → LLM compilation → Structured wiki → Q&A and outputs. A complete knowledge management pipeline.
    link: /guide/karpathy-workflow
  - title: 🤖 LLM-Compiled Wiki
    details: The wiki is the LLM's domain. Humans rarely edit it manually — the LLM builds and maintains it from raw sources.
    link: /skills/kb-compile
  - title: 🔍 Intelligent Query
    details: Ask complex questions and get synthesized answers with full traceability back to original sources.
    link: /skills/kb-query
  - title: 📊 Multi-Format Output
    details: Generate reports, slide decks, diagrams, and Canvas visualizations from your compiled knowledge.
    link: /workflow/query
  - title: 🏗️ Three Core Skills
    details: kb-init for setup, kb-compile for wiki building, kb-query for search and Q&A. Simple but powerful.
    link: /skills/overview
  - title: 🔗 Obsidian Integration
    details: Built on Obsidian Flavored Markdown with wikilinks, callouts, and Canvas support.
    link: /skills/obsidian-markdown
---
