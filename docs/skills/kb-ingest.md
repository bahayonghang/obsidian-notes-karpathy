# kb-ingest

Use `kb-ingest` when the review-gated support layer already exists but `raw/_manifest.yaml` is missing or stale.

## What it does

- scans `raw/**`
- registers tracked sources into `raw/_manifest.yaml`
- marks deferred inputs such as paper PDFs explicitly
- keeps `raw/**` immutable

## Typical triggers

- new raw files were added
- manifest drift is visible
- paper/image/data sources need explicit registration before compile
