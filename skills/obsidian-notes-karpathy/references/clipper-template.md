# Web Clipper Template Guidance

Recommend a Web Clipper template that preserves enough metadata for later compilation and leaves the captured body intact.

```markdown
---
title: "{{title}}"
source: "{{url}}"
author: "{{author}}"
date: "{{published}}"
type: article
tags:
  - inbox/web
clipped_at: "{{date:YYYY-MM-DDTHH:mm:ssZ}}"
---

# {{title}}

## Source Snapshot

- URL: {{url}}
- Author: {{author}}
- Published: {{published}}

## Captured Content

{{content}}
```

Notes:

- If the clip includes meaningful images, prefer downloading them into `raw/assets/`.
- Keep metadata keys aligned with the vault schema so property search remains useful.
