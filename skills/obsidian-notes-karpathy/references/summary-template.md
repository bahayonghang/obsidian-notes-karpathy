# Summary Template

```markdown
---
title: "Summary: {Source Title}"
source_file: "[[{raw-link}]]"
source_url: "{source-url}"
source_type: "{type}"
source_mtime: "{mtime}"
source_hash: "{optional-hash}"
compiled_at: "{datetime}"
key_concepts:
  - "[[concept-a]]"
  - "[[concept-b]]"
key_entities:
  - "[[wiki/entities/entity-a]]"
---

# Summary: {Source Title}

> [!abstract] Source
> **Type**: {type} | **Author**: {author} | **Date**: {date}
> **Source note**: [[{raw-link}]]
> **Original URL**: {source-url}

## Thesis

{one-paragraph statement of what the source is really saying}

## Key Takeaways

- {takeaway 1}
- {takeaway 2}
- {takeaway 3}

## Detailed Summary

{2-4 paragraphs}

## Key Concepts

- [[concept-a]] - {relationship}
- [[concept-b]] - {relationship}

## Key Entities

- [[wiki/entities/entity-a]] - {relationship}
- [[wiki/entities/entity-b]] - {relationship}

## Evidence

- "{quote or concrete datapoint}" - {where it appears}
- "{second datapoint}" - {why it matters}

## Tensions or Caveats

- {uncertainty, disagreement, or limitation}

## Related Sources

- [[wiki/summaries/other-source]] - {relationship}
```

Rules:

- Prefer concrete evidence over abstract praise.
- If an image materially changes meaning, inspect it and mention that in the summary.
- Keep source provenance explicit enough that a later health check can audit the claim trail.
- Omit `key_entities` and the `Key Entities` section when the source does not introduce durable named entities.
